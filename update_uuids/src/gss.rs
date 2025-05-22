use std::collections::HashMap;

use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
pub struct GssCharacteristic {
    pub characteristic: GattSpecSupplement,
}

#[derive(Debug, Deserialize)]
/// The official identifier, e.g., "org.bluetooth.characteristic.activity_goal"
pub struct GattSpecSupplement {
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
    field: String, // This will become the key in the HashMap
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
#[derive(Debug, Deserialize, Default)]
pub struct FieldInformation {
    pub name: String, // Name of the field being described (e.g., "Flags", "Presence Flags")
    pub description: String,
    pub section_title: Option<String>,
    pub table_caption: Option<String>,
    #[serde(default)]
    pub values: Vec<FieldValueDefinition>,
}

#[derive(Debug, Deserialize)]
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
