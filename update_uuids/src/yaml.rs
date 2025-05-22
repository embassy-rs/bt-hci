use std::collections::HashMap;
use std::error::Error;
use std::path::{Path, PathBuf};

use serde::Deserialize;
use walkdir::WalkDir;

use crate::gss::{GattSpecSupplement, GssCharacteristic};

// Data structure for the UUIDs
#[derive(Debug, Deserialize)]
pub struct UuidData {
    /// short UUID for this predefined value
    pub uuid: u16,
    /// human readable name associated to this UUID
    pub name: String,
    /// reference id such as: org.bluetooth.characteristic.acceleration
    pub id: Option<String>,
    /// Additional information about this Uuid
    pub gss: Option<GattSpecSupplement>,
}

#[derive(Debug, Deserialize)]
struct Uuids {
    uuids: Vec<UuidData>,
}

/// Load UUID data from a directory of YAML files.
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
    pub subcategory: Option<Vec<Subcategory>>,
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

/// Load the Gatt Specification Supplement information to combine with characteristic data.
pub fn load_gss(path: &PathBuf) -> Result<HashMap<String, GattSpecSupplement>, Box<dyn Error>> {
    let mut map = HashMap::new();
    if !path.is_dir() {
        return Err("Path must be a directory to load gss files".into());
    }
    for file in WalkDir::new(path) {
        let file = file?;
        let path = file.path();
        if path.extension().is_some_and(|ext| ext == "yaml") {
            let file_name = get_file_name(path).expect("Filename should exist");
            if file_name.starts_with("org.bluetooth.characteristic.") && file_name.ends_with(".yaml") {
                let data = std::fs::read_to_string(path)?;
                match serde_yaml::from_str(&data) {
                    Ok(GssCharacteristic {
                        characteristic: gss_data,
                    }) => map.insert(gss_data.identifier.clone(), gss_data),
                    Err(e) => panic!("error: {e} parsing file: {path:?} \n{data}"),
                };
            }
        }
    }
    Ok(map)
}
