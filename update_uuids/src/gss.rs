#![allow(unused)]

use std::collections::HashMap;

use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
pub struct GssCharacteristic {
    pub characteristic: GattSpecSupplement,
}

#[derive(Debug, Deserialize, Clone)]
/// The official identifier, e.g., "org.bluetooth.characteristic.activity_goal"
pub struct GattSpecSupplement {
    pub identifier: String, // Primary Key
    pub name: String,
    pub description: String,
    #[serde(deserialize_with = "deserialize_structure_list_to_map")]
    pub structure: Vec<GattSpecStructure>,
    /// The 'fields' key in the YAML provides detailed descriptions for certain
    /// fields defined in 'structure', often for flags.
    #[serde(default)] // Use default if 'fields' is not present
    pub fields: Vec<FieldInformation>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GattSpecStructure {
    field: String,
    #[serde(rename = "type")]
    pub ty: String,
    pub size: String,
    pub description: String,
    pub unit: Option<String>,
}

/// Custom deserializer function to convert the YAML 'structure' list
/// into a HashMap<String, GattSpecStructure>.
fn deserialize_structure_list_to_map<'de, D>(deserializer: D) -> Result<Vec<GattSpecStructure>, D::Error>
where
    D: Deserializer<'de>,
{
    let list_items: Vec<GattSpecStructure> = Vec::deserialize(deserializer)?;
    Ok(list_items)
}

/// Represents the detailed information for a field, often used for flags.
/// Corresponds to items in the 'fields' list in the YAML.
#[derive(Debug, Deserialize, Default, Clone)]
pub struct FieldInformation {
    pub name: String, // Name of the field being described (e.g., "Flags", "Presence Flags")
    pub description: String,
    pub section_title: Option<String>,
    pub table_caption: Option<String>,
    #[serde(default)]
    pub values: Vec<FieldValueDefinition>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum FieldValueDefinition {
    Bit {
        bit: String,
        description: String,
    },
    Value {
        value: String,
        description: String,
    },
    Field {
        field: String,
        data_type: String,
        size: String,
        description: String,
    },
    OpCodeParameter {
        op_code_value: String,
        definition: String,
        parameter: String,
        parameter_type: String,
        description: String,
    },
    OpCodeOperand {
        op_code_value: String,
        definition: String,
        operand: String,
        operator: Option<String>,
        operand_data_type: Option<String>,
        description: String,
    },
    ResponseCode {
        response_code_value: String,
        definition: String,
        response_parameter: Option<String>,
        description: String,
    },
}

impl GattSpecSupplement {
    pub fn print_docstring(&self) -> String {
        let description: String = self
            .description
            .lines()
            .map(|line| format!("/// {}", line.replace("\\autoref{", "`").replace("}", "`").trim_end()))
            .filter(|line| !line.contains("The structure of this characteristic is defined below."))
            .collect::<Vec<_>>()
            .join("\n");
        let structure: String = self.structure.iter().fold(String::new(), |mut acc, v| {
            let field_string: String = format!(
                "///
/// ### Data Type
///
/// |  |  |
/// |---|---|
/// | **Field** | {} |
/// | **Type** | {} |
/// | **Size** | {} |
///
/// ### Description
///
{}
///
/// ----",
                v.field,
                v.ty.replace("[", "").replace("]", ""),
                v.size.replace("\n", " - "),
                v.description
                    .lines()
                    .map(|line| { format!("/// {}", line.replace("\\autoref{", "`").replace("}", "`").trim_end()) })
                    .collect::<Vec<_>>()
                    .join("\n")
            );
            acc.push_str(&field_string);
            acc
        });
        format!(
            "
///
{}
///
/// ----
/// ## Structure
{}
///
/// [more information](https://bitbucket.org/bluetooth-SIG/public/src/main/gss/{}.yaml)",
            description, structure, self.identifier
        )
    }
}
