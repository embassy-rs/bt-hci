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
    pub structure: HashMap<String, GattSpecStructure>,
    /// The 'fields' key in the YAML provides detailed descriptions for certain
    /// fields defined in 'structure', often for flags.
    #[serde(default)] // Use default if 'fields' is not present
    pub fields: Vec<FieldInformation>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GattSpecStructure {
    #[serde(rename = "type")]
    pub ty: String,
    pub size: String,
    pub description: String,
    pub unit: Option<String>,
}

/// Represents an item in the 'structure' list in the YAML.
/// This is an intermediate helper struct for deserialization.
#[derive(Debug, Deserialize)]
struct StructureListItem {
    field: String, // Primary Key
    #[serde(rename = "type")]
    ty: String,
    size: String,
    description: String,
    unit: Option<String>,
}

/// Custom deserializer function to convert the YAML 'structure' list
/// into a HashMap<String, GattSpecStructure>.
fn deserialize_structure_list_to_map<'de, D>(deserializer: D) -> Result<HashMap<String, GattSpecStructure>, D::Error>
where
    D: Deserializer<'de>,
{
    let list_items: Vec<StructureListItem> = Vec::deserialize(deserializer)?;
    let mut map = HashMap::new();
    for item in list_items {
        map.insert(
            item.field,
            GattSpecStructure {
                ty: item.ty,
                size: item.size,
                description: item.description,
                unit: item.unit,
            },
        );
    }
    Ok(map)
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
            .replace("\n", "\n///\n/// ")
            .replace("The structure of this characteristic is defined below.", "");
        let structure: String = self.structure.iter().fold(String::new(), |mut acc, (k, v)| {
            let field_string: String = format!(
                "
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
/// {}
/// 
/// ----",
                k,
                v.ty.replace("[", "").replace("]", ""),
                v.size.replace("\n", " - "),
                v.description
                    .replace("\n", "\n///\n/// ")
                    .replace("\\autoref{", "`")
                    .replace("}", "`")
            );
            acc.push_str(&field_string);
            acc
        });
        format!(
            "
/// 
/// {}
/// 
/// ----
/// ## Structure
/// 
/// {}
/// 
/// [more information](https://bitbucket.org/bluetooth-SIG/public/src/main/gss/{}.yaml)",
            description, structure, self.identifier
        )
    }
}
