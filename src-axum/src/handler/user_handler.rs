use axum::{extract::Path, Extension};
use axum::http::StatusCode;
use axum::Json;

use crate::model::user::{User, UserInfo};
use crate::service::user_service::UserService;

pub async fn list_users(service: Extension<UserService>) -> Result<Json<Vec<User>>, StatusCode> {
    // get users
    match service.list_users().await {
        Ok(users) => Ok(Json(users)),
        Err(ex) => {
            eprintln!("{:?}", ex);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_user_by_id(service: Extension<UserService>, Path(id): Path<i32>) 
    -> Result<Json<User>, StatusCode> 
{
    // get user by id
    match service.get_user_by_id(id).await {
        Ok(user) => Ok(Json(user)),
        Err(ex) => {
            eprintln!("{:?}", ex);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn create_user(service: Extension<UserService>, Json(user): Json<UserInfo>) -> StatusCode {
    // create user
    match service.create_user(user).await {
        Ok(_) => StatusCode::OK,
        Err(ex) => {
            eprintln!("{:?}", ex);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

pub async fn update_user(service: Extension<UserService>, Path(id): Path<i32>, Json(user): Json<UserInfo>) -> StatusCode {
    // update user
    match service.update_user(id, user).await {
        Ok(_) => StatusCode::OK,
        Err(ex) => {
            eprintln!("{:?}", ex);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

pub async fn delete_user(service: Extension<UserService>, Path(id): Path<i32>) -> StatusCode {
    // delete user
    match service.delete_user(id).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(ex) => {
            eprintln!("{:?}", ex);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

