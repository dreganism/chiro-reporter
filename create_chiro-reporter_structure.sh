#!/bin/bash

# Define the base directory
BASE="raptor_chiro_reporter"
mkdir -p $BASE

echo "Creating directory structure under $BASE..."

# Flutter frontend
mkdir -p $BASE/frontend_flutter

# Actix-Web backend
mkdir -p $BASE/backend_actix_web/src/{routes,handlers,models,utils,services,validators,exporters,prompts,templates}
touch $BASE/backend_actix_web/src/main.rs
echo "// Entry point for Actix-Web backend" > $BASE/backend_actix_web/src/main.rs
touch $BASE/backend_actix_web/Cargo.toml
echo "// Actix-Web project configuration" > $BASE/backend_actix_web/Cargo.toml
touch $BASE/backend_actix_web/.env
echo "# Environment variables" > $BASE/backend_actix_web/.env

# OCR microservice
mkdir -p $BASE/ocr_service/src
echo "// OCR service main entry" > $BASE/ocr_service/src/main.rs
echo "// OCR logic here" > $BASE/ocr_service/src/ocr.rs
echo "// OCR service dependencies" > $BASE/ocr_service/Cargo.toml

# Image analyzer microservice
mkdir -p $BASE/image_analyzer/src
echo "// Image analyzer service" > $BASE/image_analyzer/src/main.rs
echo "// Vision analysis logic" > $BASE/image_analyzer/src/vision.rs
echo "// Image analyzer dependencies" > $BASE/image_analyzer/Cargo.toml

# Export engine
mkdir -p $BASE/export_engine/src
echo "// Export engine entry" > $BASE/export_engine/src/main.rs
echo "// PDF export logic" > $BASE/export_engine/src/pdf.rs
echo "// DOCX export logic" > $BASE/export_engine/src/docx.rs
echo "// Export engine dependencies" > $BASE/export_engine/Cargo.toml

# Template engine
mkdir -p $BASE/template_engine/src
echo "// Template engine entry" > $BASE/template_engine/src/main.rs
echo "// Template management logic" > $BASE/template_engine/src/template.rs
echo "// Template engine dependencies" > $BASE/template_engine/Cargo.toml

# Validation engine
mkdir -p $BASE/validation_engine/src
echo "// Validation engine entry" > $BASE/validation_engine/src/main.rs
echo "// Style validation logic" > $BASE/validation_engine/src/style.rs
echo "// Structure validation logic" > $BASE/validation_engine/src/structure.rs
echo "// Validation engine dependencies" > $BASE/validation_engine/Cargo.toml

# NGINX configuration
mkdir -p $BASE/nginx
echo "# NGINX configuration" > $BASE/nginx/nginx.conf

# Docker
mkdir -p $BASE/docker
echo "# Backend Dockerfile" > $BASE/docker/Dockerfile.backend
echo "# Frontend Dockerfile" > $BASE/docker/Dockerfile.frontend
echo "# OCR Dockerfile" > $BASE/docker/Dockerfile.ocr
echo "# Docker Compose setup" > $BASE/docker/docker-compose.yml

# Root README
echo "# Raptor Chiro Reporter README" > $BASE/README.md

echo "✅ All folders and stub files have been created."
