use self::super::protocol::{Lobby, Play, RoomLogin};
use futures::{
	future::{FusedFuture, FutureExt, pending, select_all},
	sink::SinkExt, stream::{StreamExt, iter},
	select
};
use serde_json::{Error, from_str, to_string};
use std::{borrow::Cow, mem::{replace, take}, pin::Pin};
use tokio::{sync::mpsc::Receiver, join};
use warp::filters::ws::{Message, WebSocket};

pub async fn session(mut channel: Receiver<WebSocket>) {
	let mut clients = Vec::<Client>::new();

	loop {
		// Store whether or not clients is empty to avoid immutably borrowing later.
		let clients_empty = clients.is_empty();
		// Vec of each futures corresponding to each client's next message.
		let mut message_futures = clients.iter_mut()
			.map(|client| async move {client.socket.next().await})
			.collect::<Vec<_>>();
		// Map of pinned references to those futures.
		let message_selector = message_futures.iter_mut()
			// SAFETY: We do not move message_futures before dropping.
			.map(|future| unsafe {Pin::new_unchecked(future)});
		// Get a select_all of all those message futures, IF and ONLY IF we have at
		// least one future. This is to avoid panicking.
		let mut message_selector = (!clients_empty)
			.then(|| select_all(message_selector).fuse());

		// Use dynamic dispatch to use a future that always pends if
		// message_selector is None.
		let mut pend = pending();
		let mut message_selector: &mut (dyn FusedFuture<Output=_> + Unpin) =
			if let Some(selector) = &mut message_selector {selector} else {&mut pend};

		// Poll our futures.
		select! {
			// We got a message from a socket.
			message = message_selector => match message {
				(Some(Ok(Message::Text(data))), index, _) => {
					drop(message_futures); // Drop message_futures now that we're done.
					println!("Message {} from {:?}.", data, &clients[index]);
					process_opcode(&mut clients, &data, index).await;
				},
				// Wow, warp is a really huge piece of shit and provides zero fucking
				// information on errors other than human readable strings which may
				// change across minor versions, how extremely fucking useful. I want to
				// fucking die why can nobody write a normal fucking library, please
				// hold me ;-;.
				(Some(Err(error)), index, _) => match &format!("{}", error) as &str {
					"WebSocket protocol error: Connection reset without closing handshake" => {
						// Imagine using a library where clients can just crash your server
						// all they want because you cannot tell what an error is because
						// the only useful information you can gauge from them is
						// information that may change formatting between minor crate
						// versions because it's actually just a human readable error
						// message that isn't meant to be read by your program.

						// please, hold me, please, i'm crying

						eprintln!("Client {} left without proper closing handshake.", index);

						// Same code as close.
						drop(message_futures); // Drop message_futures now that we're done.

						match clients.remove(index).state {
							ClientState::Lobby {username, ..} |
								ClientState::Play {username} =>
									process_user_leave(&mut clients, &username).await,
							_ => ()
						}
					},
					_ => panic!("{}", error)
				},
				(None, index, _) => {
					drop(message_futures); // Drop message_futures now that we're done.

					match clients.remove(index).state {
						ClientState::Lobby {username, ..} |
							ClientState::Play {username} =>
								process_user_leave(&mut clients, &username).await,
						_ => ()
					}
				},
				_ => ()
			},
			// A new client connected.
			websocket = channel.recv().fuse() => match websocket {
				None => break,
				Some(socket) => {
					drop(message_futures); // Drop message_futures now that we're done.
					clients.push(Client {socket, state: ClientState::Login})
				}
			}
		}
	}
}

#[derive(Debug)]
struct Client {
	socket: WebSocket,
	state: ClientState
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum ClientState {
	Login,
	Lobby {
		username: Box<str>,
		ready: bool
	},
	Play {
		username: Box<str>
	}
}

impl Default for ClientState {
	fn default() -> Self {
		Self::Login
	}
}

// Beware! Lots of strange usages of mutable borrows to (hopefully) speed up
// performance and please the borrow checker at the same time!
async fn process_opcode(clients: &mut Vec<Client>, data: &str, index: usize) {
	match &mut clients[index].state {
		ClientState::Login => match from_str(data) {
			Ok(RoomLogin::ClientInformation {username}) => {
				let taken = clients.iter()
					.filter_map(|client| match &client.state {
						ClientState::Lobby {username, ..} |
							ClientState::Play {username} =>
								Some(&*username as &str),
						_ => None
					})
					.any(|client| client == data);

				if taken {
					let message_data = RoomLogin::NameTaken;
					let message = to_string(&message_data).expect("serialization error");
					clients[index].socket.send(Message::Text(message)).await
						.expect("socket error");
					return;
				}

				let mut this_client = None;
				let tee = clients.iter_mut()
					.enumerate()
					// Extract the client that sent data, from this mutable borrow.
					.filter_map(|(client_index, client)| if client_index == index {
						this_client = Some(client);
						None
					} else {
						Some(client)
					})
					.filter_map(|client| match &client.state {
						ClientState::Lobby {username, ready} =>
							Some((&mut client.socket, username as &str, ready)),
						// TODO: Handle play?
						_ => None
					})
					.collect::<Vec<_>>();

				let message_data = RoomLogin::RoomInformation {
					users: tee.iter()
						.map(|(_, username, _)| *username)
						.collect()
				};
				let message = to_string(&message_data)
					.expect("serialization error");

				let readied_data = tee.iter()
					.filter_map(|(_, username, ready)|
						if **ready {Some(*username)} else {None})
					.collect::<Vec<_>>();
				let readied = if readied_data.is_empty() {
					let data = Lobby::UsersReadied {users: readied_data};
					Some(to_string(&data).expect("serialization error"))
				} else {None};

				// This future tells the new client about the other already logged in
				// clients.
				let this_client = async {
					let client = this_client.expect("internal error");

					client.socket.send(Message::Text(message)).await
						.expect("socket error");
					match readied {
						Some(message) => client.socket.send(Message::Text(message)).await
							.expect("socket error"),
						_ => ()
					}
				};

				let message_data = Lobby::UserJoin {user: &username};
				let message = &to_string(&message_data)
					.expect("serialization error");

				// This future tells the other already logged in clients of the new
				// client.
				let clients_future = iter(tee.into_iter().map(|(client, _, _)| client))
					.for_each_concurrent(None, |client| async move {
						client.send(Message::Text(message.clone())).await
							.expect("socket error");
					});

				// Run them both!
				join!(this_client, clients_future);

				// Set state.
				clients[index].state
					= ClientState::Lobby {username: username.into(), ready: false};
			},
			Ok(_) => server_opcode_error(&mut clients[index]).await,
			Err(error) => deserialize_error(error, &mut clients[index]).await
		},
		ClientState::Lobby {ready, ..} => match from_str(data) {
			Ok(Lobby::ClientReady) => {
				*ready = !*ready;

				// Use the same iterator for multiple purposes. First use is for
				// collecting a list of readied users. Second use is for collecting a
				// list of users who need to be updated with previously mentioned list.
				// This still allocates, but it saves allocating a lot of string data,
				// instead only allocating references.
				let tee = clients.iter_mut()
					.map(|client| match &client.state {
						ClientState::Lobby {username, ready} if *ready =>
							(Some(&mut client.socket), Some(username as &str)),
						ClientState::Lobby {..} =>
							(Some(&mut client.socket), None),
						_ => (None, None)
					})
					.collect::<Vec<_>>();

				let readied = tee.iter()
					.filter_map(|(_, readied)| *readied)
					.collect::<Vec<_>>();
				let message_data = if tee.len() != readied.len() && tee.len() > 1 {
					Lobby::UsersReadied {users: readied}
				} else {
          Lobby::UsersReadied {users: readied};
					Lobby::GameStart
				};
				let message = &to_string(&message_data)
					.expect("serialization error");

				iter(tee.into_iter().filter_map(|(client, _)| client))
					.for_each_concurrent(None, |client| async move {
						client.send(Message::Text(message.clone())).await
							.expect("socket error");
					}).await;

				// Update user states if the game has started.
				if message_data == Lobby::GameStart {
					clients.iter_mut()
						.for_each(|client| match take(&mut client.state) {
							ClientState::Lobby {username, ..} =>
								drop(replace(&mut client.state, ClientState::Play {username})),
							_ => ()
						});
				}
			},
			Ok(_) => server_opcode_error(&mut clients[index]).await,
			Err(error) => deserialize_error(error, &mut clients[index]).await
		},
		ClientState::Play {username} => match from_str(data) {
			Ok(Play::ClientScoreUpdate {score, health}) => {
				let message_data = Play::UserScoreUpdate {
					user: &username, score, health
				};
				let message = &to_string(&message_data).expect("serialization error");

				iter(clients.iter_mut()
						.filter(|client| matches!(client.state, ClientState::Play {..})))
					.for_each_concurrent(None, |client| async move {
						client.socket.send(Message::Text(message.clone())).await
							.expect("socket error");
					}).await;
			},
			Ok(_) => server_opcode_error(&mut clients[index]).await,
			Err(error) => deserialize_error(error, &mut clients[index]).await
		}
	}
}

async fn process_user_leave(clients: &mut Vec<Client>, user: &str) {
	let message_data = Lobby::UserLeft {user};
	let message = &to_string(&message_data)
		.expect("serialization error");

	let game_start = clients.iter()
		.all(|client| match client.state {
			ClientState::Login => true,
			ClientState::Lobby {ready, ..} => ready,
			_ => false
		})
		.then(|| to_string(&Lobby::GameStart).expect("serialization error"));

	iter(clients.iter_mut())
		.for_each_concurrent(None, |client| {
			let game_start = game_start.clone();

			async move {
				client.socket.send(Message::Text(message.clone())).await
					.expect("socket error");

				match game_start {
					Some(message) => client.socket.send(Message::Text(message)).await
						.expect("socket error"),
					_ => ()
				}
			}
		}).await;

	// Switch everyone's states to Play.
	if let Some(_) = game_start {
		clients.iter_mut()
			.for_each(|client| match take(&mut client.state) {
				ClientState::Lobby {username, ..} =>
					drop(replace(&mut client.state, ClientState::Play {username})),
				_ => ()
			});
	}
}

async fn deserialize_error(error: Error, client: &mut Client) {
	let message = Cow::Owned(format!("{}", error));
	client.socket.send(Message::Close(1007, message)).await
		.expect("error");
}

async fn server_opcode_error(client: &mut Client) {
	const ERROR: Cow<str> = Cow::Borrowed("sent opcode reserved for server use");
	client.socket.send(Message::Close(1007, ERROR)).await
		.expect("socket error");
}
