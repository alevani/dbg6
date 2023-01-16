use axum::{routing::get, Router};
use axum_macros::FromRef;
use dbgg_resources::get_members as members;

use std::net::SocketAddr;

#[derive(Clone, Debug)]
pub struct Envs {}

#[derive(Clone, FromRef)]
struct AppState {
    http_client: reqwest::Client,
    envs: Envs,
}

pub mod dbgg_resources;
pub mod errors;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    // build our application with a route
    let app = Router::new()
        .route("/", get(ping))
        .route("/members", get(members))
        .route("/tasks/:member", get(task));

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
