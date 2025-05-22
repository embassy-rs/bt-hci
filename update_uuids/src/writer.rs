use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::utils::screaming_snake_case;
use crate::yaml::{Category, UuidData};

/// Update the UUIDs in the source code
pub fn update_uuids(
    output_folder: &Path,
    mut input: HashMap<String, Vec<UuidData>>,
    commit_hash: &str,
) -> Result<(), Box<dyn Error>> {
    // each key in the map is a module name
    // each value is a list of UUIDs, which will be written as constants
    // in the form `pub const UUID_NAME: BluetoothUuid16 = BluetoothUuid16::new(uuid);`
    for (file_name, uuids) in input.iter_mut() {
        let output_name = file_name.replace("_uuids", "").replace(".yaml", "");
        if output_name == "member" || output_name == "sdo" {
            continue; // skip the member and sdo modules
        }
        let (module_name, mut file) = setup_rust_file(&output_name, output_folder.to_path_buf())?;

        let constants: Vec<_> = uuids
            .iter()
            .map(|uuid| {
                let supplement = match &uuid.gss {
                    Some(gss) => gss.print_docstring(),
                    None => "".to_string(),
                };
                format!(
                    "/// Bluetooth {} UUID.
///
/// `0x{:04x}` {}{}
pub const {}: BluetoothUuid16 = BluetoothUuid16::new(0x{:x});",
                    module_name,
                    uuid.uuid,
                    uuid.name,
                    supplement,
                    screaming_snake_case(&uuid.name),
                    uuid.uuid,
                )
            })
            .collect();
        let tokens = constants.join("\n\n");

        write_rust_file(&mut file, &module_name, tokens, commit_hash)?;
    }
    Ok(())
}

/// Update the Appearance values in the source code
///
/// Subcategories are dealt with as submodules.
pub fn update_appearance(output_folder: &Path, input: &[Category], commit_hash: &str) -> Result<(), Box<dyn Error>> {
    let output_folder = output_folder.join("appearance");
    let (module_name, mut file) = setup_rust_file("categories", output_folder)?;
    let mut tokens = String::new();
    let modules: Vec<_> = input
        .iter()
        .map(|cat| {
            let module_name = cat.name.replace(' ', "_").to_lowercase();
            if cat.subcategory.is_none() {
                format!(
                    "/// Bluetooth Appearance UUID.
///
/// `0x{:04x}` Generic {}
pub const {}: BluetoothUuid16 = super::from_category(0x{:04x}, 0x{:04x});",
                    appearance(cat.category, 0x000),
                    cat.name,
                    screaming_snake_case(&cat.name),
                    cat.category,
                    0x000 // generic subcategory
                )
            } else {
                format!(
                    "pub mod {} {{
    //! Appearance {} with subcategories.
    //!
    //! Generic variant named `GENERIC_{}`.\n
    use super::super::{{from_category, BluetoothUuid16}};\n
    {}
}}",
                    module_name,
                    module_name,
                    screaming_snake_case(&cat.name),
                    appearance_subcategory(cat)
                )
            }
        })
        .collect();
    tokens.push_str(&modules.join("\n\n"));
    write_rust_file(&mut file, &module_name, tokens, commit_hash)?;
    Ok(())
}

/// If the category has a subcategory, create a submodule for it
fn appearance_subcategory(cat: &Category) -> String {
    let mut constants: Vec<_> = Vec::new();
    // add generic subcategory first
    constants.push(format!(
        "/// Bluetooth Appearance UUID.
    ///
    /// `0x{:04x}` Generic {}
    pub const GENERIC_{}: BluetoothUuid16 = from_category(0x{:04x}, 0x{:04x});",
        appearance(cat.category, 0x000),
        cat.name,
        screaming_snake_case(&cat.name),
        cat.category,
        0x000 // generic subcategory
    ));
    if let Some(subcats) = &cat.subcategory {
        for subcat in subcats {
            constants.push(format!(
                "    /// Bluetooth Appearance UUID.
    ///
    /// `0x{:04x}` {} | {}
    pub const {}: BluetoothUuid16 = from_category(0x{:04x}, 0x{:04x});",
                appearance(cat.category, subcat.value),
                cat.name,
                subcat.name,
                screaming_snake_case(&subcat.name),
                cat.category,
                subcat.value
            ));
        }
    }
    constants.join("\n\n")
}

fn setup_rust_file(output_name: &str, output_folder: PathBuf) -> Result<(String, File), Box<dyn Error>> {
    let file_path = output_folder.join(format!("{output_name}.rs"));
    let module_name = output_name.replace('_', " ");
    // clear the file if it exists
    if file_path.exists() {
        std::fs::remove_file(&file_path)?;
    }
    let file = File::create(file_path)?;
    Ok((module_name, file))
}

fn write_rust_file(file: &mut File, name: &str, tokens: String, commit_hash: &str) -> Result<(), Box<dyn Error>> {
    // construct the header docstrings
    writeln!(file, "//! UUIDs for the {} module.\n", name)?;
    writeln!(file, "// This file is auto-generated by the update_uuids application.")?;
    writeln!(file, "// Based on https://bitbucket.org/bluetooth-SIG/public.git")?;
    writeln!(file, "// Commit hash: {}\n", commit_hash,)?;

    writeln!(file, "use super::BluetoothUuid16;\n")?;

    write!(file, "{}", tokens)?; // write the file contents
    write!(file, "\n")?; // add a newline at the end
    Ok(())
}

fn appearance(cat: u8, subcat: u8) -> u16 {
    ((cat as u16) << 6) | (subcat as u16)
}
