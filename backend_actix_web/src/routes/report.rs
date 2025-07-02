use actix_multipart::Multipart;
use actix_web::{post, web, Error, HttpResponse};
use futures_util::StreamExt;
use uuid::Uuid;

use crate::llm::{generate_report, ReportRequest};

/// POST /api/report - Generate GPT medical report
#[post("/api/report")]
pub async fn generate(mut payload: Multipart) -> Result<HttpResponse, Error> {
    let mut report_type = String::new();
    let mut denial_text = String::new();
    let mut files: Vec<(String, Vec<u8>)> = Vec::new();

    while let Some(item) = payload.next().await {
        let mut field = item?;
        let content_disposition = field.content_disposition().unwrap();

        let name = content_disposition.get_name().unwrap_or("").to_string();

        let mut data = Vec::new();
        while let Some(chunk) = field.next().await {
            let chunk = chunk?;
            data.extend_from_slice(&chunk);
        }

        match name.as_str() {
            "report_type" => {
                report_type = String::from_utf8_lossy(&data).to_string();
            }
            "denial_text" => {
                denial_text = String::from_utf8_lossy(&data).to_string();
            }
            "files[]" => {
                let filename = content_disposition
                    .get_filename()
                    .unwrap_or("file.txt")
                    .to_string();
                files.push((filename, data));
            }
            _ => (),
        }
    }

    let request = ReportRequest {
        report_type,
        denial_text,
        files,
    };

    match generate_report(request).await {
        Ok(report_response) => Ok(HttpResponse::Ok().json(report_response)),
        Err(e) => Ok(HttpResponse::InternalServerError().body(format!("Generation failed: {}", e))),
    }
}

/// Configures all /api/report routes
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(generate);
}
// This module defines the POST /api/report endpoint for generating medical reports using GPT.
// It handles multipart form data containing the report type, optional denial text, and file uploads,
// then calls the OpenAI service to generate a structured report.
//
// The `generate` function is the main request handler. It:
// - Accepts and parses multipart input (text fields + file uploads)
// - Constructs a `ReportRequest` struct
// - Calls the `generate_report` async function to invoke OpenAI
// - Returns a JSON response or an error message
//
// The `configure` function mounts this route onto the Actix Web application for use with `.configure(...)`.
//
// Key architectural points:
// - Uses Actix's `Multipart` to support file uploads, and `StreamExt` for async processing
// - `Uuid` can be used to uniquely identify file uploads (if needed for temp storage or caching)
// - `ReportRequest` encapsulates user input in a clear, structured format
// - `generate_report` is designed for easy mocking/stubbing in unit tests
//
// This modular separation keeps HTTP handling distinct from business logic, supporting:
// - Clean error handling with `Result<HttpResponse, Error>`
// - JSON serialization via `serde_json` for frontend/backend compatibility
// - Easy extensibility: new fields, endpoints, or validation logic can be added with minimal impact
//
// Future-ready design considerations include:
// - Plug-and-play support for additional endpoints (e.g., validation, export)
// - Compatibility with JavaScript frontends via consistent JSON responses
// - Optional token tracking, validation scoring, and usage metrics in `ReportResponse`
//
// This structure prioritizes maintainability, testability, and clear integration boundaries between
// the web layer, GPT engine logic, and any future services (e.g., OCR, analytics).
