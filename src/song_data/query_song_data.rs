use crate::song_data::http_controller::ApplicationState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use pgvector::Vector;
use serde::{Deserialize, Serialize};

use super::models::SongVector;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongPlayRequest {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongPlayResponse {
    pub names: Vec<String>,
}

pub async fn query_song_play_handler(
    State(_state): State<ApplicationState>,
    Json(_song_query): Json<SongPlayRequest>,
) -> impl IntoResponse {
    let embedding_result = _state.embedder.embed(_song_query).await;
    let Ok(embedding) = embedding_result else {
        log::error!("Failed to embed song vector: {:?}", embedding_result.unwrap_err());
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };
    let embedding_vector = Vector::from(embedding);
    let song_vector_row_option_result = sqlx::query_as::<_, SongVector>(
        r#"SELECT * FROM song_plays order by embedding <-> $1 LIMIT 1"#,
    ).bind(embedding_vector).fetch_optional(&_state.postgres_pool).await;

    let Ok(song_vector_option) = song_vector_row_option_result else {
        log::error!("Failed to query song vector: {:?}", song_vector_row_option_result.unwrap_err());
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };

    let mut song_data_response = SongPlayResponse { names: Vec::new() };
    let Some(song_vector) = song_vector_option else {
        return (StatusCode::NOT_FOUND, "Song not found").into_response();
    };
     song_data_response.names.push(song_vector.name);

    (StatusCode::OK, Json(song_data_response)).into_response()
}
