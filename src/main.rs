use self::room::session;
use futures::join;
use tokio::{sync::mpsc::{Sender, channel}, main as async_main};
use warp::{
	filters::{path::path, ws::{WebSocket, Ws, ws}},
	reply::Reply, Filter, serve
};

mod protocol;
mod room;

#[async_main]
async fn main() {
	let (sessioner_handle, sessioner_receiver) = channel(3);
	let sessioner = session(sessioner_receiver);

	let websocket_endpoint = path("gateway").and(ws())
		.map(move |ws| gateway_handler(ws, sessioner_handle.clone()));
	let server = serve(websocket_endpoint)
		.run(([127, 0, 0, 1], 8080));

	join!(server, sessioner);
}

fn gateway_handler(websocket: Ws, sessioner: Sender<WebSocket>) -> impl Reply {
	websocket.on_upgrade(|ws| async move {sessioner.send(ws).await.unwrap();})
}
