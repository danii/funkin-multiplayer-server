use self::{room::session, util::{Argument, ArgumentParser}};
use futures::join;
use std::env::args;
use tokio::{sync::mpsc::{Sender, channel}, main as async_main};
use warp::{
	filters::{path::path, ws::{WebSocket, Ws, ws}},
	reply::Reply, Filter, serve
};

mod protocol;
mod room;
mod util;

#[async_main]
async fn main() {
	panic!("{:?}", parse());

	let (sessioner_handle, sessioner_receiver) = channel(3);
	let sessioner = session(sessioner_receiver);

	let websocket_endpoint = path("gateway").and(ws())
		.map(move |ws| gateway_handler(ws, sessioner_handle.clone()));
	let server = serve(websocket_endpoint)
		.run(([0, 0, 0, 0], 6969));

	join!(server, sessioner);
}

fn gateway_handler(websocket: Ws, sessioner: Sender<WebSocket>) -> impl Reply {
	websocket.on_upgrade(|ws| async move {sessioner.send(ws).await.unwrap();})
}

// server help
// server room
// -t --trust (DEFAULT) Trusts client's to provide assets.
// -T --no-trust Does not trust client's to provide any assets.
// -p --public (DEFAULT) Makes the room public and vote based.
// -r --private Makes the room private and ownership based.
// -s <password> --password <password> Makes the room private, password locked and ownership based.
// server house
// -u --public-rooms (DEFAULT) Allow public rooms.
// -U --no-public-rooms Don't allow public rooms.
// -r --private-rooms (DEFAULT) Allow private rooms.
// -R --no-private-rooms Don't allow private rooms.
// -s --pasword-rooms Allow private password protected rooms.
// -S --no-password-rooms (DEFAULT) Don't allow private password protected rooms.

#[derive(Debug)]
enum RoomType {
	Public,
	Private
}

#[derive(Debug)]
enum Arguments {
	Room {
		trust: bool,
		r#type: RoomType
	},
	Help,
	Version
}

fn parse() -> Arguments {
	let args = args().collect::<Vec<_>>();
	let args = args.iter().map(|arg| arg as &str).skip(1);
	let mut args = ArgumentParser::new_extractor(args, |_, _| None);

	match args.next() {
		Some(Argument::Short('v')) | Some(Argument::Long("version"))
			| Some(Argument::Normal("version")) => Arguments::Version,
		Some(Argument::Normal("room")) => {
			let mut trust = true;
			let mut r#type = RoomType::Public;

			args.for_each(|arg| match arg {
				Argument::Long("trust") | Argument::Short('t') =>
					trust = true,
				Argument::Long("no-trust") | Argument::Short('T') =>
					trust = false,
				Argument::Long("public") | Argument::Short('p') =>
					r#type = RoomType::Public,
				Argument::Long("private") | Argument::Short('r') =>
					r#type = RoomType::Private,
				_ => panic!("error")
			});

			Arguments::Room {trust, r#type}
		},
		_ => Arguments::Help,
	}
}
