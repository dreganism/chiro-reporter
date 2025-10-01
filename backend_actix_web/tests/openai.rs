use backend_actix_web::services::openai::{call_openai_chat, generate_report, ReportRequest};
use std::env;

#[tokio::test]
async fn call_openai_chat_requires_api_key() {
    if env::var("OPENAI_API_KEY").is_err() {
        let err = call_openai_chat("Test prompt".to_string())
            .await
            .unwrap_err();
        assert!(err.to_string().contains("OPENAI_API_KEY"));
        return;
    }

    let output = call_openai_chat("Say hello in a medical style.".to_string())
        .await
        .expect("Expected OpenAI call to succeed with API key set");
    assert!(!output.trim().is_empty());
}

#[tokio::test]
async fn generate_report_builds_prompt() {
    if env::var("OPENAI_API_KEY").is_err() {
        // Without the API key the call will fail. Ensure the error surfaces cleanly.
        let request = ReportRequest {
            report_type: "Appeal".into(),
            denial_text: "Lacked documentation".into(),
            files: vec![("note.txt".into(), b"Subjective: pain".to_vec())],
        };

        let err = generate_report(request).await.unwrap_err();
        assert!(err.to_string().contains("OPENAI_API_KEY"));
        return;
    }

    let request = ReportRequest {
        report_type: "Appeal".into(),
        denial_text: "Lacked documentation".into(),
        files: vec![("note.txt".into(), b"Subjective: pain".to_vec())],
    };

    let response = generate_report(request)
        .await
        .expect("Expected report generation to succeed");
    assert!(!response.report.trim().is_empty());
}
