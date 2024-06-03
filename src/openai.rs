use std::{env, io};

use openai_dive::v1::api::Client;
use openai_dive::v1::models::Gpt4Engine;
use openai_dive::v1::resources::chat::{ChatCompletionParameters, ChatMessage, ChatMessageContent, Role};


pub const OPENAI_ENV_KEY: &str = "OPENAI_API_KEY";

pub const PROMPT: &str = r"You generate modern shell commands (OS: {{OS}}) based on descriptions.
You only provide the command and nothing else. You do not begin with bash or similar.
If needed use pipes, redirections, intermediate files and several commands.
Generate a command to '{{DESCRIPTION}}'.";

fn get_prompt(description: &str) -> String {
    let os = env::var("OS").unwrap_or_else(|_| "unknown".to_string());

    return PROMPT
        .replace("{{OS}}", &os)
        .replace("{{DESCRIPTION}}", description);
}

fn get_api_key() -> String {
    return match env::var(OPENAI_ENV_KEY) {
        Ok(key) => key,
        Err(e) => panic!("{}: {}", OPENAI_ENV_KEY, e),
    };
}


pub(crate) async fn generate_command(description: &str) -> Result<String, io::Error> {
    if description.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Description is empty"));
    }

    let api_key = get_api_key();
    let prompt = get_prompt(description);
    let client = Client::new(api_key);
    let parameters = ChatCompletionParameters {
        model: Gpt4Engine::Gpt4O.to_string(),
        messages: vec![
            ChatMessage {
                role: Role::System,
                content: ChatMessageContent::Text(prompt),
                ..Default::default()
            },
        ],
        max_tokens: Some(125),
        ..Default::default()
    };

    return client.chat().create(parameters).await
        .map(|completion| {
            let choice = completion.choices.first().unwrap();
            let content = match &choice.message.content {
                ChatMessageContent::Text(text) => text,
                content => panic!("Unexpected completion content; expected text, got {:?}", content),
            };

            Ok(content.to_string())
        })
        .unwrap_or_else(|e| Err(io::Error::new(io::ErrorKind::Other, e.to_string())));
}