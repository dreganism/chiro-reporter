use actix_multipart::{Field, Multipart};
use actix_web::{error, post, web, Error, HttpResponse};
use futures_util::StreamExt;

use crate::llm::{generate_report, ReportRequest};

/// POST /api/report - Generate GPT medical report
#[post("/api/report")]
pub async fn generate(payload: Multipart) -> Result<HttpResponse, Error> {
    let request = parse_report_request(payload).await?;

    match generate_report(request).await {
        Ok(report_response) => Ok(HttpResponse::Ok().json(report_response)),
        Err(e) => Ok(HttpResponse::InternalServerError().body(format!("Generation failed: {}", e))),
    }
}

/// Configures all /api/report routes
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(generate);
}

/// Reads the multipart payload and extracts the report request data.
async fn parse_report_request(mut payload: Multipart) -> Result<ReportRequest, Error> {
    let mut report_type: Option<String> = None;
    let mut denial_text: Option<String> = None;
    let mut files: Vec<(String, Vec<u8>)> = Vec::new();

    while let Some(item) = payload.next().await {
        let mut field = item?;
        let content_disposition = match field.content_disposition().cloned() {
            Some(cd) => cd,
            None => continue,
        };

        let Some(name) = content_disposition.get_name() else {
            continue;
        };

        let data = read_field_bytes(&mut field).await?;

        match name {
            "report_type" => report_type = Some(bytes_to_string(data, name)?),
            "denial_text" => denial_text = Some(bytes_to_string(data, name)?),
            "files[]" => {
                let filename = content_disposition
                    .get_filename()
                    .map(ToOwned::to_owned)
                    .unwrap_or_else(|| "file.txt".to_string());
                files.push((filename, data));
            }
            _ => {}
        }
    }

    Ok(ReportRequest {
        report_type: report_type.unwrap_or_default(),
        denial_text: denial_text.unwrap_or_default(),
        files,
    })
}

/// Collects the bytes for a multipart field, ensuring the stream is fully read.
async fn read_field_bytes(field: &mut Field) -> Result<Vec<u8>, Error> {
    let mut data = Vec::new();
    while let Some(chunk) = field.next().await {
        let chunk = chunk?;
        data.extend_from_slice(&chunk);
    }
    Ok(data)
}

/// Converts UTF-8 encoded field data into a `String` with friendly error reporting.
fn bytes_to_string(data: Vec<u8>, field_name: &str) -> Result<String, Error> {
    String::from_utf8(data)
        .map_err(|_| error::ErrorBadRequest(format!("Invalid UTF-8 in field `{}`", field_name)))
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
