use derive_builder::Builder;
use ollama_rs::{generation::embeddings::request::{EmbeddingsInput, GenerateEmbeddingsRequest}, Ollama};
use serde::Serialize;

const EMBEDDING_MODEL_NAME: &str = "mxbai-embed-large";

#[derive(Builder, Clone)]
pub struct Embedder {
    ollama_client: Ollama,
}

impl Embedder {
    pub async fn embed<T>(&self, input: T) -> anyhow::Result<Vec<f32>>
    where 
        T: Serialize,
    {
        let input_json = serde_json::to_string(&input)?;
        let embeddings_request = GenerateEmbeddingsRequest::new(EMBEDDING_MODEL_NAME.to_owned(), EmbeddingsInput::Single(input_json));
        let embeddings_response = self.ollama_client.generate_embeddings(embeddings_request).await?;
        Ok(embeddings_response.embeddings.first().unwrap().clone())
    }
}  