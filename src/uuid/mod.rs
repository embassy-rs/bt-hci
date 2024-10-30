pub mod appearance;
pub mod browse_group_identifiers;
pub mod characteristic;
pub mod declarations;
pub mod descriptors;
pub mod member;
pub mod mesh_profile;
pub mod object_types;
pub mod protocol_identifiers;
pub mod sdo;
pub mod service;
pub mod service_class;
pub mod units;

pub struct BleUuid {
    pub uuid: u16,
    pub id: Option<&'static str>,
}

impl BleUuid {
    pub const fn new(uuid: u16, id: Option<&'static str>) -> Self {
        Self { uuid, id }
    }
}
