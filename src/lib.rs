pub mod configuration;

use anyhow::Context;
use embedder::EmbedderBuilder;
use ollama_rs::{IntoUrlSealed, Ollama};
use song_data::http_controller::ApplicationStateBuilder;
use crate::song_data::query_song_data::query_song_play_handler;
use crate::song_data::record_song_play::record_song_play_handler;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::Router;
pub use configuration::Configuration;
use sqlx::postgres::PgPoolOptions;

pub mod song_data;
pub mod embedder;


pub fn create_config() -> Configuration {
    Configuration::default()
}


pub async fn create_router(config: Configuration) -> anyhow::Result<Router> {
    let ollama_url = config._ollama_url.into_url().context("could not convert ollama url to url ")?;
    let ollama_client = Ollama::from_url(ollama_url);
    let internal_embedder = EmbedderBuilder::default()
        .ollama_client(ollama_client)
        .build()
        .context("could not create internal_emebedder")?;
    let postgres_pool = PgPoolOptions::new()
        .connect(config.db_url.as_str())
        .await
        .context("Could not connect to database")?;
    let application_state = ApplicationStateBuilder::default()
        .embedder(internal_embedder)
        .postgres_pool(postgres_pool)
        .build()
        .context("Could not build application state")?;

    Ok(Router::new()
        .route("/ping", get(ping_pong))
        .route("/song-play", post(record_song_play_handler).get(query_song_play_handler))
        .with_state(application_state))
}


pub async fn ping_pong() -> impl IntoResponse {
    "pong".into_response()
}