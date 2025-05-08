// kml_types.rs
// Module defining KML data structures and types

use serde::Deserialize;
use std::collections::HashMap;

/// Represents a KML file with its name and content
#[derive(Debug, Clone)]
pub struct KmlFile {
    /// KML dosyasının adı
    pub name: String,
    /// KML dosyasının ham içeriği
    pub content: String,
}

/// KML elemanları için stil ayarları
#[derive(Clone, serde::Deserialize)]
pub struct StyleSettings {
    /// KML formatında çizgi rengi (aabbggrr)
    #[serde(alias = "line_color")]
    pub line_color: String,
    /// Sayı olarak çizgi kalınlığı
    #[serde(alias = "line_width")]
    pub line_width: u32,
    /// KML formatında dolgu rengi (aabbggrr)
    #[serde(alias = "fill_color")]
    pub fill_color: String,
}

impl Default for StyleSettings {
    fn default() -> Self {
        StyleSettings {
            line_color: "ffff0000".to_string(), // Tam opak kırmızı
            line_width: 2,                     
            fill_color: "7f00ff00".to_string(), // %50 opak yeşil
        }
    }
}

/// Bir KML Placemark elemanını temsil eder
#[derive(Debug, Clone)]
pub struct Placemark {
    /// Placemark'ın isteğe bağlı adı
    pub name: Option<String>,
    /// Placemark'ın isteğe bağlı açıklaması
    pub description: Option<String>,
    /// Anahtar-değer çiftleri olarak genişletilmiş veri
    pub extended_data: HashMap<String, String>,
    /// XML formatında geometri verisi
    pub geometry: String,
    /// İlişkili stil kimliği
    pub style_id: String,
}

impl Placemark {
    /// Yeni bir boş Placemark oluşturur
    pub fn new() -> Self {
        Placemark {
            name: None,
            description: None,
            extended_data: HashMap::new(),
            geometry: String::new(),
            style_id: String::new(),
        }
    }
}

/// Stil bilgisi KML elemanları için
#[derive(Debug, Clone)]
pub struct Style {
    /// Stil Kimliği
    pub id: String,
    /// KML formatında çizgi rengi (aabbggrr)
    pub line_color: String,
    /// Çizgi kalınlığı
    pub line_width: String,
    /// KML formatında dolgu rengi (aabbggrr)
    pub fill_color: String,
}

/// Bir KML Document elemanını temsil eder
#[derive(Debug, Clone)]
pub struct KmlDocument {
    /// Belgenin adı
    pub name: String,
    /// Stil listesi
    pub styles: Vec<Style>,
    /// Stil haritası listesi
    pub style_maps: Vec<StyleMap>,
    /// Yer işareti listesi
    pub placemarks: Vec<Placemark>,
}

impl KmlDocument {
    /// Verilen isimle yeni bir KML Belgesi oluşturur
    pub fn new(name: &str) -> Self {
        KmlDocument {
            name: name.to_string(),
            styles: Vec::new(),
            style_maps: Vec::new(),
            placemarks: Vec::new(),
        }
    }
}

/// StyleMap information for KML elements
#[derive(Debug, Clone)]
pub struct StyleMap {
    /// StyleMap Kimliği
    pub id: String,
    /// Normal stilin kimliği
    pub normal_style_id: String,
    /// Vurgu stilinin kimliği
    pub highlight_style_id: String,
}
