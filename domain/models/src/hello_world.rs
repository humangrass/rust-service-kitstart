use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, sqlx::FromRow)]
pub struct HelloWorld {
    pub id: i32,
    pub message: String,
    pub created_at: DateTime<Utc>,
}
