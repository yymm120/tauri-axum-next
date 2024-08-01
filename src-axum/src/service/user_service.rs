use sqlx::{postgres::PgPoolOptions, PgPool, Error};
use tokio::sync::OnceCell;
use crate::model::user::{User, UserInfo};
use crate::model::ModelManager;


#[derive(Clone)]
pub struct UserService {
    pool: PgPool
}

impl UserService {
    pub async fn new() -> Result<Self, Error> {
        static INIT: OnceCell<PgPool> = OnceCell::const_new();
	    INIT.get_or_init(|| async {
            let ModelManager { pool } = ModelManager::app_dev_pool();
            pool
	    })
	    .await;
        Ok(Self {pool})
    }

    pub async fn new_force() -> Result<Self, Error> {
        let ModelManager { pool } = ModelManager::app_dev_pool();
        Ok(Self {pool})
    }

    pub async fn list_users(&self) -> Result<Vec<User>, Error> {
        let users = sqlx::query_as::<_, User>("SELECT id, name, occupation, email, phone FROM user")
            .fetch_all(&self.pool)
            .await?;
        Ok(users)
    }

    pub async fn get_user_by_id(&self, id: i32) -> Result<User, Error> {
        let user = sqlx::query_as::<_, User>("SELECT id, name, occupation, email, phone FROM user WHERE id = $1")
            .bind(id)
            .fetch_one(&self.pool)
            .await?;
        Ok(user)
    }

    pub async fn create_user(&self, user: UserInfo) -> Result<(), Error> {
        sqlx::query("INSERT INTO USER (name, occupation, email, phone) VALUES ($1, $2, $3, $4)")
            .bind(user.name)
            .bind(user.occupation)
            .bind(user.email)
            .bind(user.phone)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn update_user(&self, id: i32, user: UserInfo) -> Result<(), Error> {
        sqlx::query("UPDATE user SET name = $1, occupation = $2, email = $3, phone = $4 WHERE id = $5")
            .bind(user.name)
            .bind(user.occupation)
            .bind(user.email)
            .bind(user.phone)
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn delete_user(&self, id: i32) -> Result<(), Error> {
        sqlx::query("DELETE FROM user WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}