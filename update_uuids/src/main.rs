//! This Application pulls the latest assigned numbers from
//! the Bluetooth SIG and updates the assigned numbers constants.
//!
//! The assigned numbers are used in the Bluetooth specification
//! to define the UUIDs for services, characteristics, and other
//! Bluetooth related values.

mod utils;
mod writer;
mod yaml;

use git2::Repository;
use std::{error::Error, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    // Download the latest assigned numbers from the Bluetooth SIG
    const SIG_URL: &str = "https://bitbucket.org/bluetooth-SIG/public.git";
    println!("Downloading the latest assigned numbers from the Bluetooth SIG...");

    let local_folder = Path::new("bluetooth-sig");
    let output_folder = Path::new("../src/uuid");
    fetch_repo(SIG_URL, local_folder)?;

    write_uuids(local_folder, output_folder)?; // assigned_numbers/uuids

    write_appearance(local_folder, output_folder)?; // assigned_numbers/core/appearance

    Ok(())
}

/// Parse and write the UUIDs to the source code
/// The UUIDs are loaded from the YAML files in the assigned_numbers/uuids folder
fn write_uuids(local_folder: &Path, output_folder: &Path) -> Result<(), Box<dyn Error>> {
    // Load the YAML data from ./bluetooth-sig/assigned_numbers/uuids*
    let path = local_folder.join("assigned_numbers").join("uuids");
    let uuid_map = yaml::load_uuid_data(&path)?;
    // Update the assigned numbers in the source code
    writer::update_uuids(output_folder, uuid_map)?;
    Ok(())
}

/// Parse and write the Appearance values to the source code
/// The Appearance values are loaded from the YAML files in the assigned_numbers/core/appearance folder
fn write_appearance(local_folder: &Path, output_folder: &Path) -> Result<(), Box<dyn Error>> {
    // Load the YAML data from ./bluetooth-sig/assigned_numbers/core/appearance_values.yaml
    let file_name = "appearance_values.yaml";
    let path = local_folder.join("assigned_numbers").join("core").join(file_name);
    let appearance_data = yaml::load_appearance_data(&path)?;
    println!("{:?}", appearance_data);
    // Update the appearance values in the source code
    writer::update_appearance(output_folder, &appearance_data)?;
    Ok(())
}

fn fetch_repo(repo_url: &str, local_path: &Path) -> Result<(), Box<dyn Error>> {
    // Fetch the repository from the given URL
    if local_path.exists() {
        // Pull the latest changes
        let repo = Repository::open(local_path)?;
        let mut remote = repo.find_remote("origin")?;
        remote.fetch(&["main"], None, None)?;
    } else {
        // Clone the repository
        Repository::clone(repo_url, local_path)?;
    }
    println!("Repository fetched successfully!");
    Ok(())
}
