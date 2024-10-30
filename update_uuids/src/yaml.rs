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

fn get_file_name(path: &Path) -> Option<String> {
    path.file_name()?.to_str().map(|s| s.to_string())
}
