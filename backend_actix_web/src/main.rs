use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use std::env;

mod routes;
mod services; // For logic like OpenAI API calls // For HTTP endpoints
use routes::report; // Specific to POST /api/report

/// Simple health check endpoint for uptime monitoring or readiness checks
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({ "status": "ok" }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Load .env variables
    env_logger::init(); // Initialize structured logging

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port);

    println!("🚀 Starting server at http://{}/", addr);

    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive()) // ⚠ Relaxed CORS for dev. Lock this down in prod!
            .configure(report::configure) // Mount /api/report route
            .route("/health", web::get().to(health_check)) // Mount /health check
    })
    .bind(addr)?
    .run()
    .await
}
// This is the main entry point for the Actix web server.
// It initializes the server, sets up CORS, and mounts the routes.
// We'll circle back to this once we have the OpenAI service and routes ready so we can test the full flow.
