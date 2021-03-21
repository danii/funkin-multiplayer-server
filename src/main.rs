use self::messages::LoginState;
use futures::{future::{FusedFuture, FutureExt, pending, select_all}, SinkExt, StreamExt, join, select};
use serde_json::from_str;
use std::borrow::Cow;
use tokio::{sync::mpsc::{Receiver, Sender, channel}, main as async_main};
use warp::{
	filters::{path::path, ws::{Message, WebSocket, Ws, ws}},
	reply::Reply, Filter, serve
};

mod messages;

#[async_main]
async fn main() {
	let (sessioner_handle, sessioner_receiver) = channel(3);
	let sessioner = room_sessioner(sessioner_receiver);

	let websocket_endpoint = path("gateway").and(ws())
		.map(move |ws| gateway_handler(ws, sessioner_handle.clone()));
	let server = serve(websocket_endpoint)
		.run(([127, 0, 0, 1], 8080));

	join!(server, sessioner);
}

async fn room_sessioner(mut channel: Receiver<WebSocket>) {
	let mut sockets = Vec::<(WebSocket, ClientState)>::new();

	loop {
		let sockets_empty = sockets.is_empty();
		let mut message_futures = sockets.iter_mut()
			.map(|(socket, _)| async move {socket.next().await})
			.collect::<Vec<_>>();
		let message_selector = message_futures.iter_mut()
			.map(|future| unsafe {std::pin::Pin::new_unchecked(future)});
		let mut message_selector = (!sockets_empty)
			.then(|| select_all(message_selector).fuse());

		let mut pend = pending();
		let mut message_selector: &mut (dyn FusedFuture<Output=_> + Unpin) =
			if let Some(selector) = &mut message_selector {selector} else {&mut pend};

		select! {
			message = message_selector => match message {
				(Some(Ok(Message::Text(data))), index, _) => {
					drop(message_futures); // Drop Vec to avoid mixed borrows.

					match sockets[index].1 {
						ClientState::Login => {
							let data: LoginState = match from_str(&data) {
								Ok(data) => data,
								Err(error) => {
									let message = Cow::Owned(format!("{}", error));
									sockets[index].0.send(Message::Close(1007, message)).await
										.expect("error");
									continue;
								}
							};

							println!("{:?}", data);
						},
						ClientState::Lobby => {
							println!("Lobby");
						}
					}
				},
				(Some(Err(error)), ..) => panic!("{}", error),
				(None, index, _) => {
					drop(message_futures); // Drop Vec to avoid double mutable borrows.

					sockets.remove(index);
				},
				_ => ()
			},
			websocket = channel.recv().fuse() => match websocket {
				None => break,
				Some(websocket) => {
					drop(message_futures); // Drop Vec to avoid double mutable borrows.

					sockets.push((websocket, ClientState::Login))
				}
			}
		}
	}
}

#[derive(Clone, Copy)]
enum ClientState {
	Login,
	Lobby
}

fn gateway_handler(websocket: Ws, sessioner: Sender<WebSocket>) -> impl Reply {
	websocket.on_upgrade(|ws| async move {sessioner.send(ws).await.unwrap();})
}
