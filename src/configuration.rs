use derive_builder::Builder;

#[derive(Debug, Clone, Builder)]
#[builder(default)]
pub struct Configuration {
    pub _ollama_url: String,
    pub db_url: String,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            _ollama_url: "http://localhost:11434".to_string(),
            db_url: "http://localhost:5432".to_string(),
        }
    }
}
