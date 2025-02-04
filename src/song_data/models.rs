use anyhow::Context;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Default, Clone, Serialize, Deserialize, FromRow)]
pub struct SongVector {
    pub name: String,
}

impl SongVector {
   pub fn to_json_string(&self) -> anyhow::Result<String> {
        serde_json::to_string(self).context("Failed to serialize SongVector to JSON string")
   }
}

