use std::sync::Arc;

use askama::Template;
use askama_axum::IntoResponse;
use axum::{
    routing::{get, post},
    Router,
};
use config::{Config, Environment, File};
use routes::{get_index, post_search};
use tokio::net::TcpListener;
use tracing::info;

mod routes;
mod sncf;
mod views;

#[derive(Clone)]
struct Globals {
    client: sncf::Client,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let conf = Config::builder()
        .add_source(File::with_name("config.toml").required(false))
        .add_source(Environment::with_prefix("SB"))
        .build()
        .unwrap();
    let conf = conf
        .try_deserialize::<sncf_configuration::Config>()
        .unwrap();
    let client = sncf::Client::new(conf.token);
    let state = Arc::new(Globals { client });

    let router = Router::new()
        .route("/", get(get_index))
        .route("/search", post(post_search))
        .route("/dashboard/:id", get(routes::get_dashboard))
        .with_state(state);
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    info!("Starting");

    axum::serve(listener, router).await.unwrap();
}

mod sncf_configuration {
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct Config {
        pub token: String,
    }
}
