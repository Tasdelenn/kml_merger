// kml_merger.rs
// Module for merging KML files

use std::collections::HashMap;

use crate::error::{AppError, AppResult};
use crate::kml_builder::KmlBuilder;
use crate::kml_parser::KmlParser;
use crate::kml_types::{KmlDocument, KmlFile, StyleSettings};
use crate::traits::{KmlMergerTrait, KmlBuilderTrait, KmlParserTrait};

pub struct KmlMerger;

impl KmlMergerTrait for KmlMerger {
    /// Birden fazla KML dosyasını tek bir KML belgesinde birleştirir
    fn merge_kml_files(
        files: &[KmlFile],
        styles: &HashMap<String, StyleSettings>,
    ) -> AppResult<KmlDocument> {
        if files.is_empty() {
            return Err(AppError::ValidationError("No KML files provided".to_string()));
        }

        // Yeni bir KML Belgesi oluştur
        let mut document = KmlDocument::new("Merged KML File");
        
        for (idx, kml_file) in files.iter().enumerate() {
            // Bu dosya için stil ayarlarını al veya bulunamazsa varsayılanı kullan
            let style_settings = styles.get(&kml_file.name).cloned().unwrap_or_else(|| {
                eprintln!("No style found for file: {}, using default style.", kml_file.name);
                StyleSettings::default()
            });
            
            // Create styles
            let (normal_style, highlight_style, style_map) = 
                KmlBuilder::create_styles_from_settings(idx, &style_settings);
            
            // Add styles and style maps
            document.styles.push(normal_style);
            document.styles.push(highlight_style);
            document.style_maps.push(style_map);
            
            // KML dosyasından yer işaretlerini çıkar
            let mut placemarks = KmlParser::extract_placemarks(kml_file)
                .map_err(|e| AppError::ParseError(format!("Failed to parse file {}: {}", kml_file.name, e)))?;
            
            // Her yer işareti için stil kimliğini ayarla
            for placemark in &mut placemarks {
                placemark.style_id = format!("style_{}", idx);
            }
            
            // Add placemarks to the document
            document.placemarks.extend(placemarks);
        }
        
        Ok(document)
    }
}

/// KML dosyalarını birleştirip nihai KML dizesini döndüren bağımsız fonksiyon
pub fn merge_kml_files_to_string(
    files: &[KmlFile],
    styles: &HashMap<String, StyleSettings>,
) -> AppResult<String> {
    // KML dosyalarını bir KmlDocument nesnesine birleştir, stil haritasını ilet
    let document = KmlMerger::merge_kml_files(files, styles)?;
    KmlBuilder::build_kml(&document)
}
