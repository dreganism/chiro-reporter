use anyhow::{anyhow, Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::env;

/// Structured representation of the multipart payload provided by the
/// `/api/report` endpoint.
#[derive(Debug, Clone, Default)]
pub struct ReportRequest {
    pub report_type: String,
    pub denial_text: String,
    pub files: Vec<(String, Vec<u8>)>,
}

/// Minimal JSON response returned to the frontend.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ReportResponse {
    pub report: String,
}

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

/// Converts a [`ReportRequest`] into an instruction prompt for the language model.
fn build_prompt(request: &ReportRequest) -> String {
    let mut sections = Vec::new();

    if !request.report_type.trim().is_empty() {
        sections.push(format!("Report type: {}", request.report_type.trim()));
    }

    if !request.denial_text.trim().is_empty() {
        sections.push(format!(
            "Denial details provided by payer: {}",
            request.denial_text.trim()
        ));
    }

    if !request.files.is_empty() {
        let mut file_section = String::from("Attached clinical notes:\n");
        for (filename, data) in &request.files {
            let content: Cow<'_, str> = String::from_utf8_lossy(data);
            file_section.push_str(&format!("--- {} ---\n{}\n", filename, content.trim()));
        }
        sections.push(file_section);
    }

    if sections.is_empty() {
        sections.push("No additional context was provided.".to_string());
    }

    format!(
        "You are a medical documentation expert. Using the following information, produce a concise, professional, and insurance-ready clinical report.\n\n{}",
        sections.join("\n\n")
    )
}

/// Creates a report using the OpenAI chat completion API.
pub async fn generate_report(request: ReportRequest) -> Result<ReportResponse> {
    let prompt = build_prompt(&request);
    let report = call_openai_chat(prompt).await?;

    Ok(ReportResponse {
        report: report.trim().to_string(),
    })
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
        .context("No choices returned in OpenAI response")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_prompt_includes_sections() {
        let request = ReportRequest {
            report_type: "Initial Evaluation".into(),
            denial_text: "Missing objective findings".into(),
            files: vec![("note.txt".into(), b"Patient reports pain".to_vec())],
        };

        let prompt = build_prompt(&request);
        assert!(prompt.contains("Initial Evaluation"));
        assert!(prompt.contains("Missing objective findings"));
        assert!(prompt.contains("Patient reports pain"));
    }

    #[tokio::test]
    async fn call_openai_chat_is_optional_without_api_key() {
        if env::var("OPENAI_API_KEY").is_err() {
            // Without credentials the API call will fail, but the error should be informative.
            let err = call_openai_chat("test".into()).await.unwrap_err();
            assert!(err.to_string().contains("OPENAI_API_KEY"));
            return;
        }

        // When the key is present we expect a successful call. The assertion here keeps
        // backwards compatibility for environments that do provide a key.
        let response = call_openai_chat("Say hello".into()).await;
        assert!(response.is_ok(), "Expected success, got: {:?}", response);
    }
}
