use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Serialize, Debug)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
}

#[derive(Deserialize, Debug)]
struct ChatChoice {
    message: ChatMessage,
}

#[derive(Deserialize, Debug)]
struct ChatResponse {
    choices: Vec<ChatChoice>,
}

/// Sends a prompt to OpenAI's chat completion endpoint using the GPT-4o model.
pub async fn call_openai_chat(prompt: String) -> Result<String> {
    let api_key = env::var("OPENAI_API_KEY")
        .map_err(|_| anyhow!("OPENAI_API_KEY is not set in environment variables"))?;

    let client = Client::new();
    let request_body = ChatRequest {
        model: "gpt-4o".to_string(),
        messages: vec![
            ChatMessage {
                role: "system".to_string(),
                content: "You are a medical documentation expert. Please produce detailed, professional, and insurance-appropriate clinical narratives.".to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: prompt,
            },
        ],
    };

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&request_body)
        .send()
        .await
        .map_err(|e| anyhow!("Failed to send OpenAI request: {}", e))?;

    if !response.status().is_success() {
        return Err(anyhow!(
            "OpenAI API responded with non-success status: {}",
            response.status()
        ));
    }

    let parsed: ChatResponse = response
        .json()
        .await
        .map_err(|e| anyhow!("Failed to parse OpenAI response JSON: {}", e))?;

    parsed
        .choices
        .get(0)
        .map(|c| c.message.content.clone())
        .ok_or_else(|| anyhow!("No choices returned in OpenAI response"))
}

// ✅ Inline unit test for OpenAI connectivity
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_call_openai_chat_works() {
        let prompt = "Say hello in a medical style.";
        let result = call_openai_chat(prompt.to_string()).await;
        assert!(result.is_ok(), "Expected Ok(_), got error: {:?}", result);

        let response = result.unwrap();
        assert!(
            response.to_lowercase().contains("hello"),
            "Response did not contain expected greeting. Got: {}",
            response
        );
    }
}

// TODO: Write unit tests for `call_openai_chat()` using mock OpenAI endpoints.
// This will require setting up a mock server or using a library like `mockito` to simulate OpenAI's API responses.
