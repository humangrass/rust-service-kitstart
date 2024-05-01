use sqlx::{PgPool};

#[derive(Clone)]
pub struct Repository {
    database: PgPool
}

impl Repository {
    pub fn new(pool: PgPool) -> Self {
        Self {
            database: pool,
        }
    }
    pub fn pool(&self) -> &PgPool {
        &self.database
    }
}
