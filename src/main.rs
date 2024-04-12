use anyhow::Ok;
use axum::response::IntoResponse;
use axum::{
    http::HeaderMap,
    extract::{
        State,
        ws::{WebSocket, WebSocketUpgrade}
    },
    routing::get, Router
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Building our application with a single Route
    let app = Router::new().route("/", get(root)).route("/ws", get(ws));
 
    // Run the server with hyper on http://127.0.0.1:3000
    let port = std::env::var("PORT").unwrap();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, Axum ❤️ WASIX!"
}


async fn ws(ws: WebSocketUpgrade, headers: HeaderMap) -> impl IntoResponse {
    println!("{:?}", headers);
    ws.on_upgrade(|socket| {
        async move {
            todo!()
        }
    });
}



fn aaa() -> anyhow::Result<()> {
     let config = std::fs::read_to_string("cluster.json")?;
// /     let map: ClusterMap = serde_json::from_str(&config)?;
    Ok(())
}