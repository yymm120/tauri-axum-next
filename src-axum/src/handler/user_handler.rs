use axum::extract::State;
use axum::{extract::Path, Extension};
use axum::http::StatusCode;
use axum::Json;

use crate::model::user::{User, UserInfo};
use crate::model::ModelManager;
use crate::service::user_service::UserService;
use crate::startup::Application;

#[derive(Clone)]
pub struct DatabaseState {
    pub model: ModelManager
}

pub async fn list_users(State(state): State<Application>, service: Extension<UserService>) -> Result<Json<Vec<User>>, StatusCode> {
    tracing::info!("into list_users function.");
    match service.list_users(state).await {
        Ok(users) => Ok(Json(users)),
        Err(ex) => {
            eprintln!("{:?}", ex);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_user_by_id(service: Extension<UserService>, State(state): State<Application>, Path(id): Path<i64>) 
    -> Result<Json<User>, StatusCode> 
{
    // get user by id
    tracing::debug!("into get_user_by_id function.");
    match service.get_user_by_id(state, id).await {
        Ok(user) => Ok(Json(user)),
        Err(ex) => {
            eprintln!("{:?}", ex);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn create_user(service: Extension<UserService>, State(state): State<Application>, Json(user): Json<UserInfo>) -> StatusCode {
    // create user
    tracing::info!("into create_user function.");
    match service.create_user(user, state).await {
        Ok(_) => StatusCode::OK,
        Err(ex) => {
            eprintln!("{:?}", ex);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

pub async fn update_user(service: Extension<UserService>, State(state): State<Application>, Path(id): Path<i64>, Json(user): Json<UserInfo>) -> StatusCode {
    // update user
    tracing::info!("into update_user function.");
    match service.update_user(id, user, state).await {
        Ok(_) => StatusCode::OK,
        Err(ex) => {
            eprintln!("{:?}", ex);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

pub async fn delete_user(service: Extension<UserService>, State(state): State<Application>, Path(id): Path<i64>) -> StatusCode {
    // delete user
    tracing::debug!("into delete_user function.");
    match service.delete_user(id, state).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(ex) => {
            eprintln!("{:?}", ex);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

