use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use super::{http_controller::ApplicationState, models::SongVector};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SongPlay {
    pub name: String,
}

impl Into<SongVector> for SongPlay {
    fn into(self) -> SongVector {
        SongVector { name: self.name }
    }
}

pub async fn record_song_play_handler(
    State(state): State<ApplicationState>,
    Json(song_play): Json<SongPlay>,
) -> impl IntoResponse {
    let song_vector: SongVector = song_play.clone().into();
    let embedding_vector_result = state.embedder.embed(song_vector).await;
    let Ok(embedding_vector) = embedding_vector_result else {
        log::error!("Failed to embed song vector: {:?}", embedding_vector_result.unwrap_err());
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };

    let insert_result = sqlx::query(
        r#"
        INSERT INTO song_plays (name, embedding)
        VALUES ($1, $2)
        "#,
    )
    .bind(song_play.name)
    .bind(embedding_vector.clone())
    .fetch_optional(&state.postgres_pool)
    .await;

    let Ok(_pg_query_row_option) = insert_result else {
        log::error!("Failed to insert song play: {:?}", insert_result.unwrap_err().into_database_error());
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };

    (StatusCode::CREATED, format!("{:?}", embedding_vector)).into_response()
}
