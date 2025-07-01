use actix_multipart::Multipart;
use actix_web::{post, web, HttpResponse, Error};
use futures_util::StreamExt as _;
use std::fs::File;
use std::io::Write;
use uuid::Uuid;

use crate::llm::{generate_report, ReportRequest};

#[post("/api/generate")]
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
                let filename = content_disposition.get_filename().unwrap_or("file.txt").to_string();
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
// Function to save uploaded files to disk (optional, for debugging purposes)
fn save_file(filename: &str, data: &[u8]) -> std::io::Result<()> {
    let file_path = format!("uploads/{}", filename);
    let mut file = File::create(file_path)?;
    file.write_all(data)?;
    Ok(())
}