// kml_builder.rs
// Module for building KML output

use crate::error::AppResult;
use crate::kml_types::{KmlDocument, Placemark, Style, StyleMap, StyleSettings};
use crate::traits::KmlBuilderTrait;
use regex;

pub struct KmlBuilder;

impl KmlBuilderTrait for KmlBuilder {
    /// Build KML string from a KML document
    fn build_kml(document: &KmlDocument) -> AppResult<String> {
        let mut kml = String::from(r#"<?xml version="1.0" encoding="UTF-8"?>
<kml xmlns="http://www.opengis.net/kml/2.2">
<Document>
    <name>"#);
        
        kml.push_str(&document.name);
        kml.push_str("</name>\n");
        kml.push_str("    <open>1</open>\n");
        
        // Tüm placemarkları kapsayan görünüm ayarlarını ekle
        if let Some(lookat) = build_lookat(&document.placemarks) {
            kml.push_str(&lookat);
        }
        
        // Kullanıcı tarafından tanımlanan stilleri ekle
        for style in &document.styles {
            kml.push_str(&format!(r#"    <Style id="{}">
        <BalloonStyle>
        </BalloonStyle>
        <LineStyle>
            <color>{}</color>
            <width>{}</width>
        </LineStyle>
        <PolyStyle>
            <color>{}</color>
            <fill>1</fill>
            <outline>1</outline>
        </PolyStyle>
    </Style>
"#, style.id, style.line_color, style.line_width, style.fill_color));
        }

        // Style Map'leri ekle
        for style_map in &document.style_maps {
            kml.push_str(&format!(r#"    <StyleMap id="{}">
        <Pair>
            <key>normal</key>
            <styleUrl>#{}</styleUrl>
        </Pair>
        <Pair>
            <key>highlight</key>
            <styleUrl>#{}</styleUrl>
        </Pair>
    </StyleMap>
"#, style_map.id, style_map.normal_style_id, style_map.highlight_style_id));
        }
        
        // Placemark'lar
        for placemark in &document.placemarks {
            kml.push_str("    <Placemark>\n");
            if let Some(name) = &placemark.name {
                kml.push_str(&format!("        <name>{}</name>\n", name));
            }
            // Placemark için stil referansını kullan
            let style_id = format!("#{}", &placemark.style_id);
            kml.push_str(&format!("        <styleUrl>{}</styleUrl>\n", style_id));
            if let Some(description) = &placemark.description {
                kml.push_str(&format!("        <description>{}</description>\n", description));
            }
            if !placemark.extended_data.is_empty() {
                kml.push_str("        <ExtendedData>\n");
                for (key, value) in &placemark.extended_data {
                    kml.push_str(&format!(r#"            <Data name="{}">
                <value>{}</value>
            </Data>
"#, key, value));
                }
                kml.push_str("        </ExtendedData>\n");
            }
            // Geometri (orijinal koordinatlar korunur)
            kml.push_str(&format!("        {}\n", placemark.geometry));
            kml.push_str("    </Placemark>\n");
        }
        kml.push_str("</Document>\n</kml>");
        Ok(kml)
    }
    
    fn create_styles_from_settings(base_id: usize, settings: &StyleSettings) -> (Style, Style, StyleMap) {
        let normal_style_id = format!("normal_style_{}", base_id);
        let highlight_style_id = format!("highlight_style_{}", base_id);
        let style_map_id = format!("style_{}", base_id);

        let normal_style = Style {
            id: normal_style_id.clone(),
            line_color: settings.line_color.clone(),
            line_width: settings.line_width.to_string(),
            fill_color: settings.fill_color.clone(),
        };

        // Vurgu stili için çizgi kalınlığını artır
        let highlight_style = Style {
            id: highlight_style_id.clone(),
            line_color: settings.line_color.clone(),
            line_width: (settings.line_width + 2).to_string(), // Vurguda çizgiyi daha kalın yap
            fill_color: settings.fill_color.clone(),
        };

        let style_map = StyleMap {
            id: style_map_id,
            normal_style_id,
            highlight_style_id,
        };

        (normal_style, highlight_style, style_map)
    }
}

fn build_lookat(placemarks: &[crate::kml_types::Placemark]) -> Option<String> {
    if placemarks.is_empty() {
        return None;
    }

    let mut min_lon = 180.0f64;
    let mut max_lon = -180.0f64;
    let mut min_lat = 90.0f64;
    let mut max_lat = -90.0f64;
    let mut all_coords: Vec<(f64, f64)> = Vec::new();

    // <coordinates>...</coordinates> etiketleri arasındaki içeriği bulmak için Regex
    let coordinates_block_regex = match regex::Regex::new(r"<coordinates>([\s\S]*?)</coordinates>") {
        Ok(re) => re,
        Err(e) => {
            eprintln!("Failed to compile coordinates_block_regex: {}", e);
            return None;
        }
    };
    // Bireysel lon,lat,alt demetlerini ayrıştırmak için Regex
    let tuple_regex = match regex::Regex::new(r"(-?\d+\.?\d*)\s*,\s*(-?\d+\.?\d*)(?:\s*,\s*-?\d+\.?\d*)?") {
        Ok(re) => re,
        Err(e) => {
            eprintln!("Failed to compile tuple_regex: {}", e);
            return None;
        }
    };

    for placemark in placemarks {
        for cap in coordinates_block_regex.captures_iter(&placemark.geometry) {
            if let Some(coords_content_match) = cap.get(1) {
                let coords_content = coords_content_match.as_str();
                // Split the content by whitespace (handles multiple coordinate tuples)
                for coord_tuple_str in coords_content.trim().split_whitespace() {
                    if let Some(tuple_cap) = tuple_regex.captures(coord_tuple_str) {
                        // tuple_cap[1] is longitude, tuple_cap[2] is latitude
                        if let (Ok(lon), Ok(lat)) = (tuple_cap[1].parse::<f64>(), tuple_cap[2].parse::<f64>()) {
                            all_coords.push((lon, lat));
                        } else {
                            eprintln!("Failed to parse lon/lat from: {}", coord_tuple_str);
                        }
                    }
                }
            }
        }
    }

    if all_coords.is_empty() {
        return None; // Koordinat bulunamazsa LookAt oluşturma
    }

    for (lon, lat) in &all_coords {
        min_lon = min_lon.min(*lon);
        max_lon = max_lon.max(*lon);
        min_lat = min_lat.min(*lat);
        max_lat = max_lat.max(*lat);
    }

    let center_lon = (min_lon + max_lon) / 2.0;
    let center_lat = (min_lat + max_lat) / 2.0;

    // Görüş alanını (range) hesapla. Basit bir yaklaşım:
    // En geniş enlem veya boylam farkını al ve bunu bir faktörle çarp.
    // Dünya'nın eğriliği ve projeksiyonlar nedeniyle bu mükemmel olmayacaktır.
    let delta_lon = (max_lon - min_lon).abs();
    let delta_lat = (max_lat - min_lat).abs();

    // Basit bir range hesaplaması.
    let range = delta_lon.max(delta_lat) * 111320.0 * 1.5; // Dereceyi metreye çevirip biraz boşluk bırak

    Some(format!(r#"    <LookAt>
        <longitude>{}</longitude>
        <latitude>{}</latitude>
        <altitude>0</altitude>
        <heading>0</heading>
        <tilt>0</tilt>
        <range>{}</range>
        <altitudeMode>clampToGround</altitudeMode>
    </LookAt>
"#, center_lon, center_lat, range.max(1000.0))) // Minimum range 1000m
}
