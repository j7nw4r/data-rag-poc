#![allow(unused)]

use axum::Router;
use ollama_rs::Ollama;
use derive_builder::Builder;
use sqlx::{Pool, Postgres};

use crate::embedder::Embedder;

pub trait HttpController {
    fn create_router(&self, state: ApplicationState) -> Router;
}

pub struct SongDataHttpController;

#[derive(Clone, Builder)]
pub struct ApplicationState {
    pub embedder: Embedder,
    pub postgres_pool: Pool<Postgres>,
}

impl HttpController for SongDataHttpController {
    fn create_router(&self, state: ApplicationState) -> Router {
        Router::new()
            .with_state(state)
    }
}
