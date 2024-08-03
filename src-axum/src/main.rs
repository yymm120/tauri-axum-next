mod model;
mod service;
mod handler;
mod configuration;
mod startup;
mod constants;
mod error;

use axum::{routing::{delete, get, post, put}, Extension, Router};
use configuration::get_configuration;
use startup::Application;
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

    info!("Starting Service..!");
    let configuration = get_configuration().expect("Failed to read configuration.");

    let application = Application::build(configuration.clone())
            .await
            .expect("create application service occur error!");

    run(application).await;

}
pub async fn run(application: Application) {
    let service = UserService::new().await.unwrap();
    let app = Router::new()
        .route("/", get(list_users))
        .route("/users", get(list_users))
        .route("/user/:id", get(get_user_by_id))
        .route("/user", post(create_user))
        .route("/user/:id", put(update_user))
        .route("/user/:id", delete(delete_user))
        .layer(Extension(service))
        .with_state(application.clone());

    let listenner = tokio::net::TcpListener::bind(application.address).await.unwrap();

    tracing::info!("Stared server!!!");
    axum::serve(listenner, app).await.unwrap()
}