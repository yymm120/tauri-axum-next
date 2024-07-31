mod _dev_utils;
mod model;
mod service;
mod handler;

use axum::{routing::{delete, get, post, put}, Extension, Router};
use tracing::info;
use tracing_subscriber::EnvFilter;
use service::user_service::UserService;
use crate::handler::user_handler::{update_user, get_user_by_id, list_users, create_user, delete_user};



#[tokio::main]
async fn main() {
    // Log Init
    tracing_subscriber::fmt()
        .without_time() // For early local development.
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    info!("--> into main");
    // DB Init
    _dev_utils::init_dev().await;
    println!("Starting Service..!");

    let service = UserService::new().await.unwrap();

    let app = Router::new()
        .route("/users", get(list_users))
        .route("/user/:id", get(get_user_by_id))
        .route("/user", post(create_user))
        .route("/user/:id", put(update_user))
        .route("user/:id", delete(delete_user))
        .layer(Extension(service));

    // println!()

    let listenner = tokio::net::TcpListener::bind("0.0.0.0:3002").await.unwrap();
    axum::serve(listenner, app).await.unwrap()

}
