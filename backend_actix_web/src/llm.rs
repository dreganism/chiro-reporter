use serde::{Deserialize, Serialize};
use anyhow::Result;

use crate::services::openai::call_openai_chat;

/// Request payload for generating a medical report
#[derive(Debug, Serialize, Deserialize)]
pub struct ReportRequest {
    pub report_type: String,
    pub denial_text: String,
    pub files: Vec<(String, Vec<u8>)>,
}

/// Response returned after a report is generated
#[derive(Debug, Serialize, Deserialize)]
pub struct ReportResponse {
    pub content: String,
}

/// Generate a report using the OpenAI service
pub async fn generate_report(req: ReportRequest) -> Result<ReportResponse> {
    // Basic prompt construction. In a real implementation we would pass file
    // contents and additional context.
    let mut prompt = format!(
        "Generate a {} based on the following information:\n{}\n",
        req.report_type, req.denial_text
    );

    if !req.files.is_empty() {
        prompt.push_str("Attached files:\n");
        for (name, _) in &req.files {
            prompt.push_str(&format!("- {}\n", name));
        }
    }

    let content = call_openai_chat(prompt).await?;
    Ok(ReportResponse { content })
}
