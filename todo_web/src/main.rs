mod handlers;
mod models;
mod utils;

use handlers::todos::{delete_todo, get_todos, post_todo, put_todo, root};

use axum::{
    routing::{delete, get, post, put},
    Router,
};

use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/", get(root))
        .route("/todos", post(post_todo))
        .route("/todos", get(get_todos))
        .route("/todo/:id", put(put_todo))
        .route("/todo/:id", delete(delete_todo))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!(
        "Start listening on http://{}",
        listener.local_addr().unwrap()
    );

    axum::serve(listener, app).await.unwrap();
}
