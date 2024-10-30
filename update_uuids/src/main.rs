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

    let local_path = Path::new("bluetooth-sig");
    fetch_repo(SIG_URL, local_path)?;

    // Load the YAML data from ./bluetooth-sig/assigned_numbers/uuids*
    let path = local_path.join("assigned_numbers").join("uuids");
    let map = yaml::load_uuid_data(&path)?;
    println!("{:?}", map);
    let output_folder = Path::new("../src/uuid");
    // Update the assigned numbers in the source code
    writer::update_uuids(output_folder, &map)?;
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
