mod desensitize;
mod handler;
mod model;
mod store;

use axum::routing::get;
use axum::Router;

use crate::handler::get_order;
use crate::store::OrderStore;

#[tokio::main]
async fn main() {
    let store = OrderStore::new();

    let app = Router::new()
        .route("/orders/:id", get(get_order))
        .with_state(store);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind address");

    println!("Server running on http://127.0.0.1:3000");
    println!("Try: GET http://127.0.0.1:3000/orders/1");

    axum::serve(listener, app)
        .await
        .expect("Server failed");
}
