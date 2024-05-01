use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Deserialize, Serialize, sqlx::FromRow)]
pub struct HelloWorld {
    pub id: i32,
    pub message: String,
    pub created_at: DateTime<Utc>,
}
