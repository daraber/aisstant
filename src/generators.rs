use std::io;

pub(crate) trait CommandGenerator {
    fn generate_command(&self, description: &str) -> Result<String, io::Error>;
}

pub(crate) struct OpenAICommandGenerator {
    api_key: String,
}

impl OpenAICommandGenerator {
    pub(crate) fn new(api_key: String) -> Self {
        OpenAICommandGenerator { api_key }
    }
}

impl CommandGenerator for OpenAICommandGenerator {
    fn generate_command(&self, description: &str) -> Result<String, io::Error> {
        if description.is_empty() {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Description is empty"));
        }

        return Ok("echo todo".to_string());
    }
}
