use actix_web::{web, HttpResponse, Responder, Error};
use actix_multipart::Multipart;
use futures::TryStreamExt;
use serde_json;
use crate::kml_merger::merge_kml_files_to_string;
use crate::kml_types::{KmlFile, StyleSettings};
use std::collections::HashMap;

// Helper function to read content from a multipart field
async fn read_field_content(field: &mut actix_multipart::Field) -> Result<String, Error> {
    let mut content = Vec::new();
    while let Some(chunk) = field.try_next().await? {
        content.extend_from_slice(&chunk);
    }
    Ok(String::from_utf8_lossy(&content).to_string())
}

use actix_files::Files;


/// Handler for the index page
pub async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/index.html"))
}

/// Handler for uploading and merging KML files
pub async fn upload_and_merge(mut payload: Multipart) -> Result<HttpResponse, Error> {
    let mut kml_files: Vec<KmlFile> = Vec::new();
    let mut output_filename = String::from("merged"); // Default filename
    let mut styles_str: Option<String> = None;

    // Parse multipart payload
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_disposition = field.content_disposition().clone();
        let name = content_disposition
            .get_name()
            .unwrap_or("unknown")
            .to_string();

        // Only process fields named 'file*'
        if name.starts_with("file") {
            let filename = content_disposition
                .get_filename()
                .map(|s| s.to_string()) // Convert Option<&str> to Option<String>
                .unwrap_or_else(|| format!("{}.kml", name)) // Use field name if filename is missing
                .to_string();

            // Ensure it's a KML file based on filename extension
            if filename.to_lowercase().ends_with(".kml") {
                let file_content = read_field_content(&mut field).await?; 
                kml_files.push(KmlFile {
                    name: filename,
                    content: file_content,
                });
            } else {
                // Optionally log or handle non-KML files if needed
                println!("Skipping non-KML file: {}", filename);
            }
        } else if name == "outputFilename" {
            // Read the output filename field
            let name_str = read_field_content(&mut field).await?;
            if !name_str.trim().is_empty() {
                // Sanitize filename characters if necessary, basic example:
                output_filename = name_str.trim().replace(|c: char| !c.is_alphanumeric() && c != '-' && c != '_', "_");
            }
        } else if name == "styles" {
            styles_str = Some(read_field_content(&mut field).await?);
        }
    }

    if kml_files.is_empty() {
        return Ok(HttpResponse::BadRequest().body("No KML files uploaded"));
    }

    let styles: HashMap<String, StyleSettings> = match styles_str {
        Some(s) => {
            match serde_json::from_str(&s) {
                Ok(styles) => styles,
                Err(e) => {
                    eprintln!("Failed to parse styles JSON: {}, using default styles.", e);
                    HashMap::new()
                }
            }
        },
        None => {
            eprintln!("No styles data received, using default styles.");
            HashMap::new()
        }
    };

    match merge_kml_files_to_string(&kml_files, &styles) {
        Ok(merged_kml) => {
            // Determine the final filename
            let final_filename = if output_filename.is_empty() {
                "merged".to_string() // Default to "merged" if empty
            } else {
                output_filename // Use the sanitized custom name
            };

            Ok(HttpResponse::Ok()
                .content_type("application/vnd.google-earth.kml+xml")
                .append_header((
                    "Content-Disposition",
                    format!("attachment; filename=\"{}.kml\"", final_filename)
                ))
                .body(merged_kml))
        }
        Err(e) => {
            eprintln!("Merge error: {}", e); // Log the error server-side
            Ok(HttpResponse::InternalServerError().body(format!("Merge error: {}", e)))
        }
    }
}

/// Configure web server routes
pub fn configure_app(cfg: &mut web::ServiceConfig) {
    cfg.service(Files::new("/static", "./static")) // Removed .show_files_listing() for security
        .route("/", web::get().to(index))
        .route("/upload", web::post().to(upload_and_merge));
}
