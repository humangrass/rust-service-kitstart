use models::hello_world::HelloWorld;
use crate::repository::Repository;

use sqlx::{Error};

impl Repository {
    pub async fn create_hello_world(&self, message: &str) -> Result<HelloWorld, Error> {
        sqlx::query_as::<_, HelloWorld>(
            "INSERT INTO hello_world (message) VALUES ($1) RETURNING *"
        )
            .bind(message)
            .fetch_one(self.pool())
            .await
    }
    pub async fn get_all_hello_world(&self) -> Result<Vec<HelloWorld>, Error> {
        sqlx::query_as::<_, HelloWorld>("SELECT * FROM hello_world")
            .fetch_all(self.pool())
            .await
    }
    pub async fn get_hello_world_by_id(&self, id: i64) -> Result<HelloWorld, Error> {
        sqlx::query_as::<_, HelloWorld>("SELECT * FROM hello_world WHERE id = $1")
            .bind(id)
            .fetch_one(self.pool())
            .await
    }
    pub async fn update_hello_world(&self, id: i32, message: &str) -> Result<(), Error> {
        sqlx::query("UPDATE hello_world SET message = $1 WHERE id = $2")
            .bind(message)
            .bind(id)
            .execute(self.pool())
            .await?;
        Ok(())
    }
    pub async fn delete_hello_world(&self, id: i32) -> Result<(), Error> {
        sqlx::query("DELETE FROM hello_world WHERE id = $1")
            .bind(id)
            .execute(self.pool())
            .await?;
        Ok(())
    }
}
