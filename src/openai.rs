use std::{env, io};

use openai_dive::v1::api::Client;
use openai_dive::v1::models::Gpt4Engine;
use openai_dive::v1::resources::chat::{ChatCompletionParameters, ChatMessage, ChatMessageContent, Role};


pub const OPENAI_ENV_KEY: &str = "OPENAI_API_KEY";

pub const PROMPT: &str = r"You generate modern shell commands (OS: {{OS}}) based on descriptions.
You always respond with 1 line only, being the command.
If needed use pipes, redirections, intermediate files and several commands.

Generate a command to '{{DESCRIPTION}}'.
";


fn get_prompt(description: &str) -> String {
    let os = env::var("OS").unwrap_or_else(|_| "unknown".to_string());

    return PROMPT
        .replace("{{OS}}", &os)
        .replace("{{DESCRIPTION}}", description);
}

fn get_api_key() -> String {
    let api_key = env::var(OPENAI_ENV_KEY);

    if api_key.is_err() {
        panic!("{}: {}", OPENAI_ENV_KEY, api_key.err().unwrap());
    }

    api_key.unwrap()
}

pub(crate) async fn generate_command(description: &str) -> Result<String, io::Error> {
    let api_key = get_api_key();

    if description.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Description is empty"));
    }

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

    let result = client.chat().create(parameters).await.unwrap();
    if result.choices.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "OpenAI response is empty"));
    }

    let choice = result.choices.first().unwrap();
    let content = match &choice.message.content {
        ChatMessageContent::Text(text) => text,
        _ => "",
    };

    Ok(content.to_string())
}