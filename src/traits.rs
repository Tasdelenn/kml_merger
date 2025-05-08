// traits.rs
// Defines interfaces for KML operations

use crate::error::AppResult;
use crate::kml_types::{KmlDocument, KmlFile, Placemark, Style, StyleMap, StyleSettings};

/// Trait for parsing KML files
pub trait KmlParserTrait {
    /// Extract placemarks from a KML file
    fn extract_placemarks(kml_file: &KmlFile) -> AppResult<Vec<Placemark>>;
    
    /// Helper method to get an attribute from an XML element
    fn get_attribute(e: &quick_xml::events::BytesStart, name: &str) -> Option<String>;
}

/// Trait for building KML output
pub trait KmlBuilderTrait {
    /// Build KML string from a KML document
    fn build_kml(document: &KmlDocument) -> AppResult<String>;
    
    /// Create styles from style settings
    fn create_styles_from_settings(
        idx: usize,
        style_settings: &StyleSettings
    ) -> (Style, Style, StyleMap);
}

/// Trait for merging KML files
pub trait KmlMergerTrait {
    /// Merge multiple KML files into a single KML document
    fn merge_kml_files(
        files: &[KmlFile],
        styles: &std::collections::HashMap<String, StyleSettings>,
    ) -> AppResult<KmlDocument>;
}

// KmlUtilsTrait kaldırıldı çünkü kml_utils.rs dosyası silindi.
