// main.rs
// Main entry point for the KML Merger application

mod error;
mod kml_builder;
mod kml_merger;
mod kml_parser;
mod kml_types;
// kml_utils modül bildirimi kaldırıldı
mod traits;
mod web;

use actix_web::{App, HttpServer};
use web::configure_app;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server starting at http://127.0.0.1:8080");
    
    HttpServer::new(|| {
        App::new()
            .configure(configure_app)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
