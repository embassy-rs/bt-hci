use std::{
    collections::HashMap,
    error::Error,
    path::{Path, PathBuf},
};

use serde::Deserialize;
use walkdir::WalkDir;

// Data structure for the UUIDs
#[derive(Debug, Deserialize)]
pub struct UuidData {
    pub uuid: u16,
    pub name: String,
    pub id: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Uuids {
    uuids: Vec<UuidData>,
}

/// Load UUID data from a directory of YAML files, 
/// matches files in the bluetooth-sig/assigned_numbers/uuids folder.
pub fn load_uuid_data(path: &PathBuf) -> Result<HashMap<String, Vec<UuidData>>, Box<dyn Error>> {
    let mut map = HashMap::new();
    for entry in WalkDir::new(path) {
        let entry = entry?;
        let path = entry.path();
        if path.extension().is_some_and(|ext| ext == "yaml") {
            let file_name = get_file_name(path).expect("Filename should exist");
            let data = std::fs::read_to_string(path)?;
            let uuid_data: Uuids = serde_yaml::from_str(&data)?;
            map.insert(file_name, uuid_data.uuids);
        };
    }
    Ok(map)
}

#[derive(Debug, Deserialize)]
struct AppearanceValues {
    appearance_values: Vec<Category>,
}

#[derive(Debug, Deserialize)]
pub struct Category {
    pub category: u8,
    pub name: String,
    pub subcategory: Option<Vec<Subcategory>>
}

#[derive(Debug, Deserialize)]
pub struct Subcategory {
    pub value: u8,
    pub name: String,
}

/// Load UUID data from the appearance folder
/// This has a different structure than the UUIDs folder
pub fn load_appearance_data(path: &PathBuf) -> Result<Vec<Category>, Box<dyn Error>> {
    if path.file_name() != Some("appearance_values.yaml".as_ref()) {
        return Err("Invalid file name, must be appearance_values.yaml".into());
    }
    let data = std::fs::read_to_string(path)?;
    let parsed_data: AppearanceValues = serde_yaml::from_str(&data)?;
    Ok(parsed_data.appearance_values)
}

fn get_file_name(path: &Path) -> Option<String> {
    path.file_name()?.to_str().map(|s| s.to_string())
}
