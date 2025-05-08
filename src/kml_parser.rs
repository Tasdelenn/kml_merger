// kml_parser.rs
// Module for parsing KML files

use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use crate::error::{AppError, AppResult};
use crate::kml_types::{KmlFile, Placemark};
use crate::traits::KmlParserTrait;

pub struct KmlParser;

impl KmlParserTrait for KmlParser {
    /// Bir KML dosyasından yer işaretlerini çıkarır
    fn extract_placemarks(kml_file: &KmlFile) -> AppResult<Vec<Placemark>> {
        let mut placemarks = Vec::new();
        let mut reader = Reader::from_str(&kml_file.content);
        reader.trim_text(true);

        let mut buf = Vec::new();
        let mut current_placemark: Option<Placemark> = None;
        let mut in_placemark = false;
        let mut in_name = false;
        let mut in_description = false;
        let mut in_extended_data = false;
        let mut in_data = false;
        let mut current_data_name = String::new();

        let mut in_geometry = false;
        let mut geometry_parts: Vec<String> = Vec::new();
        let mut geometry_tag_name: Option<String> = None; // To store the main geometry tag like "Point"

        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    let name_bytes = e.name();
                    let name_str = String::from_utf8_lossy(name_bytes.as_ref()).to_string();

                    if in_geometry {
                        // Etikete öznitelikleri ekle
                        let mut tag_with_attrs = format!("<{}", name_str);
                        for attr_result in e.attributes() {
                            if let Ok(attr) = attr_result {
                                let key = String::from_utf8_lossy(attr.key.as_ref());
                                let value = String::from_utf8_lossy(&attr.value);
                                tag_with_attrs.push_str(&format!(" {}=\"{}\"", key, value.replace('"', "&quot;")));
                            }
                        }
                        tag_with_attrs.push('>');
                        geometry_parts.push(tag_with_attrs);
                    } else {
                        match name_str.as_str() {
                            "Placemark" => {
                                in_placemark = true;
                                current_placemark = Some(Placemark::new());
                            }
                            "name" if in_placemark => {
                                in_name = true;
                            }
                            "description" if in_placemark => {
                                in_description = true;
                            }
                            "ExtendedData" if in_placemark => {
                                in_extended_data = true;
                            }
                            "Data" if in_extended_data => {
                                in_data = true;
                                if let Some(attr) = Self::get_attribute(e, "name") {
                                    current_data_name = attr;
                                }
                            }
                            "Point" | "LineString" | "Polygon" | "MultiGeometry" | "MultiTrack" | "gx:Track" if in_placemark => {
                                in_geometry = true;
                                geometry_tag_name = Some(name_str.clone());
                                geometry_parts.clear();
                                // Etikete öznitelikleri ekle
                                let mut tag_with_attrs = format!("<{}", name_str);
                                for attr_result in e.attributes() {
                                    if let Ok(attr) = attr_result {
                                        let key = String::from_utf8_lossy(attr.key.as_ref());
                                        let value = String::from_utf8_lossy(&attr.value);
                                        tag_with_attrs.push_str(&format!(" {}=\"{}\"", key, value.replace('"', "&quot;")));
                                    }
                                }
                                tag_with_attrs.push('>');
                                geometry_parts.push(tag_with_attrs);
                            }
                            _ => {}
                        }
                    }
                }
                Ok(Event::End(ref e)) => {
                    let name_bytes = e.name();
                    let name_str = String::from_utf8_lossy(name_bytes.as_ref()).to_string();

                    if in_geometry {
                        geometry_parts.push(format!("</{}>", name_str));
                        if Some(&name_str) == geometry_tag_name.as_ref() {
                            if let Some(placemark) = &mut current_placemark {
                                placemark.geometry = geometry_parts.join("");
                            }
                            in_geometry = false;
                            geometry_tag_name = None;
                            geometry_parts.clear();
                        }
                    } else {
                        match name_str.as_str() {
                            "Placemark" => {
                                in_placemark = false;
                                if let Some(placemark) = current_placemark.take() {
                                    placemarks.push(placemark);
                                }
                            }
                            "name" if in_placemark => {
                                in_name = false;
                            }
                            "description" if in_placemark => {
                                in_description = false;
                            }
                            "ExtendedData" => {
                                in_extended_data = false;
                            }
                            "Data" if in_extended_data => {
                                in_data = false;
                                current_data_name = String::new();
                            }
                            _ => {}
                        }
                    }
                }
                Ok(Event::Text(ref e)) => {
                    let text_content = String::from_utf8_lossy(&e.unescaped().unwrap_or_default()).to_string();
                    if in_geometry {
                        geometry_parts.push(text_content.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;"));
                    } else if in_name && in_placemark {
                        if let Some(placemark) = &mut current_placemark {
                            placemark.name = Some(text_content);
                        }
                    } else if in_description && in_placemark {
                        if let Some(placemark) = &mut current_placemark {
                            placemark.description = Some(text_content);
                        }
                    } else if in_data && in_extended_data && !current_data_name.is_empty() {
                        if let Some(placemark) = &mut current_placemark {
                            placemark.extended_data.insert(current_data_name.clone(), text_content);
                        }
                    }
                }
                Ok(Event::Empty(ref e)) => {
                    let name_bytes = e.name();
                    let name_str = String::from_utf8_lossy(name_bytes.as_ref()).to_string();
                    if in_geometry {
                        let mut tag_with_attrs = format!("<{}", name_str);
                        for attr_result in e.attributes() {
                            if let Ok(attr) = attr_result {
                                let key = String::from_utf8_lossy(attr.key.as_ref());
                                let value = String::from_utf8_lossy(&attr.value);
                                tag_with_attrs.push_str(&format!(" {}=\"{}\"", key, value.replace('"', "&quot;")));
                            }
                        }
                        tag_with_attrs.push_str(" />");
                        geometry_parts.push(tag_with_attrs);
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => {
                    return Err(AppError::XmlError(e));
                }
                _ => (),
            }
            buf.clear();
        }
        Ok(placemarks)
    }
    
    // XML özniteliğini çıkaran yardımcı fonksiyon
    fn get_attribute(e: &BytesStart, name: &str) -> Option<String> {
        e.attributes()
            .filter_map(Result::ok)
            .find(|attr| attr.key.as_ref() == name.as_bytes())
            .map(|attr| String::from_utf8_lossy(&attr.value).to_string())
    }
}
