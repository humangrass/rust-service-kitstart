use axum::extract::Path;
use axum::{Json};
use log::error;
use serde::Deserialize;
use models::hello_world::HelloWorld;
use repository::repository::Repository;

use crate::entries::Hello;

#[derive(Deserialize)]
pub struct HelloParams {
    id: i64,
    limit: usize,
    offset: usize,
}


pub async fn hello(Path(params): Path<HelloParams>) -> Json<Vec<Hello>> {
    let mut messages = Vec::new();
    for i in 1..=params.limit.min(10) {
        let hello = Hello {
            message: format!("Hello, World! [{}]", i),
        };
        messages.push(hello);
    }
    Json(messages)
}

pub(crate) async fn hello_worlds(repo: Repository) -> Json<Vec<HelloWorld>> {
    match repo.get_all_hello_world().await {
        Ok(result) => Json(result),
        Err(err) => {
            error!("Failed to fetch hello_worlds: {}", err);
            Json(vec![])
        }
    }
}
