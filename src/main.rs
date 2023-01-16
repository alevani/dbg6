use axum::{routing::get, Router};
use dbgg_resources::get_tasks_for_member;
use std::net::SocketAddr;

pub mod dbgg_resources;
pub mod errors;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    // build our application with a route
    let app = Router::new()
        .route("/", get(ping))
        .route("/tasks", get(get_tasks_for_member));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn ping() -> &'static str {
    "I am alive!"
}
