use std::{collections::HashMap, path::Path};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

#[derive(Debug, Clone)]
pub struct Token {
    pub text: String,
    pub logprob: f32,
}

#[derive(Debug, Clone)]
pub struct GenerationParams {
    pub temperature: f32,
    pub top_p: f32,
    pub max_tokens: usize,
}

pub trait Backend: Send + Sync {
    fn name(&self) -> &str;
    fn load_model(&mut self, path: &Path) -> anyhow::Result<()>;
    fn generate(
        &self,
        prompt: &str,
        params: &GenerationParams,
    ) -> mpsc::Receiver<anyhow::Result<Token>>;
    fn generate_stream(
        &self,
        prompt: &str,
        params: &GenerationParams,
    ) -> ReceiverStream<anyhow::Result<Token>>;
}

pub struct BackendFactory {
    pub backends: HashMap<String, Box<dyn Backend>>,
}

impl BackendFactory {
    pub fn create(backend_type: &str) -> anyhow::Result<Box<dyn Backend>> {
        // Call the backend-specific create function if it exists
        todo!()
    }

    pub fn register(&mut self, backend: Box<dyn Backend>) {
        self.backends.insert(backend.name().to_string(), backend);
    }

    pub fn init_backend(&mut self, backend: Box<dyn Backend>) -> anyhow::Result<()> {
        todo!()
    }

    pub fn get(&self, backend_type: &str) -> Option<&Box<dyn Backend>> {
        self.backends.get(backend_type)
    }

    pub fn get_mut(&mut self, backend_type: &str) -> Option<&mut Box<dyn Backend>> {
        self.backends.get_mut(backend_type)
    }

    pub fn list(&self) -> Vec<String> {
        self.backends.keys().cloned().collect()
    }
}
