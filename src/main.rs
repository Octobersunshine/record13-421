mod desensitize;
mod export;
mod handler;
mod model;
mod store;

use axum::routing::get;
use axum::Router;

use crate::handler::{export_orders, get_order};
use crate::store::OrderStore;

#[tokio::main]
async fn main() {
    let store = OrderStore::new();

    let app = Router::new()
        .route("/orders/export", get(export_orders))
        .route("/orders/:id", get(get_order))
        .with_state(store);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind address");

    println!("Server running on http://127.0.0.1:3000");
    println!("Try: GET http://127.0.0.1:3000/orders/1");
    println!("Try: GET http://127.0.0.1:3000/orders/export?start=2026-06-16&end=2026-06-18");
    println!("Try: GET http://127.0.0.1:3000/orders/export?start=2026-06-18&format=csv");

    axum::serve(listener, app)
        .await
        .expect("Server failed");
}
