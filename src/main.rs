use futures::sink::SinkExt;
use tokio::main as async_main;
use warp::{
	filters::{path::path, ws::{Message, Ws, ws}},
	reply::Reply, Filter, serve
};

#[async_main]
async fn main() {
	let websocket_endpoint = path("gateway").and(ws())
		.map(gateway_handler);

	serve(websocket_endpoint)
		.run(([127, 0, 0, 1], 8080)).await;
}

fn gateway_handler(websocket: Ws) -> impl Reply {
	websocket.on_upgrade(|mut websocket| async move {
		websocket.send(Message::text("Hello, world!")).await.unwrap();
	})
}
