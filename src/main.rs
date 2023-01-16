use axum::{routing::get, Router};
use axum_macros::FromRef;
use dbgg_resources::get_members;

use std::net::SocketAddr;

#[derive(Clone, Debug)]
pub struct Envs {}

#[derive(Clone, FromRef)]
struct AppState {
    http_client: reqwest::Client,
    envs: Envs,
}

pub mod errors;
pub mod dbgg_resources;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    // build our application with a route
    let app = Router::new()
        .route("/", get(ping))
        .route("/members", get(get_members));
    
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
