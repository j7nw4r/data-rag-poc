use clap::Parser;
use data_rag_poc::Configuration;
use ollama_rs::{generation::embeddings::request::{EmbeddingsInput, GenerateEmbeddingsRequest}, Ollama};
use pgvector::Vector;
use sqlx::postgres::PgPoolOptions;

/// Data RAG POC
#[derive(Parser)]
pub struct Cli {
}

#[tokio::main]
async fn main() {
    let _cli = Cli::parse();
    let _config = Configuration::default();
    test_fn().await;
}

async fn test_fn() {
    let postgres_pool = PgPoolOptions::new()
    .connect("postgresql://postgres:postgres@localhost:5432/postgres")
    .await.unwrap();

    let embeddings_request = GenerateEmbeddingsRequest::new("mxbai-embed-large".to_string(), EmbeddingsInput::Single("testing".to_owned()));
    let embeddings_response = Ollama::default().generate_embeddings(embeddings_request).await.unwrap();

    let v = Vector::from(embeddings_response.embeddings.first().unwrap().clone());

    sqlx::query("INSERT INTO song_plays (name, embedding) VALUES ($1, $2)")
    .bind("testing".to_owned())
    .bind(v)
    .execute(&postgres_pool)
    .await
    .unwrap();
    println!("{:?}", embeddings_response.embeddings);
}