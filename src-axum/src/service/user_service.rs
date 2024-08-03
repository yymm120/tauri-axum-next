use crate::model::user::{User, UserInfo};
use crate::service::error::Result;
use crate::startup::Application;



#[derive(Clone)]
pub struct UserService {
}

impl UserService {
    pub async fn new() -> Result<Self> {
        Ok(Self {})
    }


    pub async fn list_users(&self, Application {database_state, ..}: Application) -> Result<Vec<User>> {
        let users = sqlx::query_as::<_, User>(r#"SELECT id, username, occupation, email, phone FROM "user""#)
            .fetch_all(&database_state.model.db_pool)
            .await?;
        Ok(users)
    }

    pub async fn get_user_by_id(&self, Application {database_state, ..}: Application, id: i64) -> Result<User> {
        let user = sqlx::query_as::<_, User>(r#"SELECT id, username, occupation, email, phone FROM "user" WHERE id = $1"#)
            .bind(id)
            .fetch_one(&database_state.model.db_pool)
            .await?;
        Ok(user)
    }

    pub async fn create_user(&self, user: UserInfo, Application {database_state, ..}: Application) -> Result<()> {
        sqlx::query(r#"INSERT INTO "user" (username, occupation, email, phone) VALUES ($1, $2, $3, $4)"#)
            .bind(user.username)
            .bind(user.occupation)
            .bind(user.email)
            .bind(user.phone)
            .execute(&database_state.model.db_pool)
            .await?;
        Ok(())
    }

    pub async fn update_user(&self, id: i64, user: UserInfo, Application { database_state, .. }: Application) -> Result<()> {
        sqlx::query(r#"UPDATE "user" SET username = $1, occupation = $2, email = $3, phone = $4 WHERE id = $5"#)
            .bind(user.username)
            .bind(user.occupation)
            .bind(user.email)
            .bind(user.phone)
            .bind(id)
            .execute(&database_state.model.db_pool)
            .await?;
        Ok(())
    }

    pub async fn delete_user(&self, id: i64, Application { database_state, .. }: Application) -> Result<()> {
        sqlx::query(r#"DELETE FROM "user" WHERE id = $1"#)
            .bind(id)
            .execute(&database_state.model.db_pool)
            .await?;
        Ok(())
    }
}