use axum::extract::Path;
use axum::{Extension, Json};
use log::error;
use models::hello_world::HelloWorld;
use repository::repository::Repository;
use serde::Deserialize;

use crate::entries::Hello;

#[derive(Deserialize)]
pub struct HelloParams {
    id: i64,
    limit: usize,
    offset: usize,
}

pub async fn hello(Extension(repo): Extension<Repository>,Path(params): Path<HelloParams>) -> Json<Vec<Hello>> {
    let mut messages = Vec::new();
    for i in 1..=params.limit.min(10) {
        let hello = Hello {
            message: format!("Hello, World! [{}]", i),
        };
        messages.push(hello);
    }
    Json(messages)
}

pub(crate) async fn hello_worlds(Extension(repo): Extension<Repository>) -> Json<Vec<HelloWorld>> {
    match repo.get_all_hello_world().await {
        Ok(result) => Json(result),
        Err(err) => {
            error!("Failed to fetch hello_worlds: {}", err);
            Json(vec![])
        }
    }
}
