use axum::{routing::{get, post}, Router};
use sqlx::PgPool;

use repository::repository::Repository;

use crate::api::hello::{hello, hello_worlds};
use crate::api::index::index;
use crate::api::user::create_user;


pub struct HTTPHandler {
    repo: Repository,
}


impl HTTPHandler {
    pub fn new(pool: PgPool) -> Self {
        let repo = Repository::new(pool);
        Self {
            repo,
        }
    }
    pub fn mount_routes(&self) -> Router {
        let repo = self.repo.clone();

        Router::new()
            .route("/", get(index))
            .route("/users", post(create_user))
            .route("/hello/:limit", get(hello))
            .route("/hello-worlds", get(move || hello_worlds(repo.clone())))
    }
}
