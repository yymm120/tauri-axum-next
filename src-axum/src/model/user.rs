use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Deserialize, Serialize, FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub occupation: String,
    pub email: String,
    pub phone: String
}

#[derive(Deserialize, Serialize)]
pub struct UserInfo {
    pub name: String,
    pub occupation: String,
    pub email: String,
    pub phone: String
}