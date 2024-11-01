//! This module contains the UUIDs for Bluetooth.

use core::fmt::{Debug, Display};

pub mod appearance;
pub mod browse_group_identifiers;
pub mod characteristic;
pub mod declarations;
pub mod descriptors;
pub mod mesh_profile;
pub mod object_types;
pub mod protocol_identifiers;
pub mod service;
pub mod service_class;
pub mod units;

/// Bluetooth UUID.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct BleUuid(u16);

impl BleUuid {
    /// Create a new `BleUuid`.
    pub const fn new(uuid: u16) -> Self {
        Self(uuid)
    }
    /// Construct a new appearance value for the GAP Service.
    ///
    /// Follow the pattern of the examples below to create new appearance values.
    /// Use UUIDs from the [Bluetooth Assigned Numbers list](https://www.bluetooth.com/wp-content/uploads/Files/Specification/Assigned_Numbers.html#bookmark49).
    ///
    /// ## Example
    ///
    /// ```rust ignore
    ///
    /// const GAMEPAD: BleUuid = BleUuid::from_category(0x00F, 0x040);
    /// const GAMEPAD_BYTES: &[u8; 2] = &GAMEPAD.to_le_bytes();
    /// ```
    pub const fn from_category(category: u8, subcategory: u8) -> Self {
        let uuid = ((category as u16) << 6) | (subcategory as u16);
        Self(uuid)
    }
    /// Convert the `BleUuid` to a byte array as a const function.
    pub const fn to_le_bytes(self) -> [u8; 2] {
        self.0.to_le_bytes()
    }
}

impl From<BleUuid> for u16 {
    fn from(uuid: BleUuid) -> u16 {
        uuid.0
    }
}

impl From<BleUuid> for [u8; 2] {
    fn from(uuid: BleUuid) -> [u8; 2] {
        uuid.0.to_le_bytes()
    }
}

impl Debug for BleUuid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "BleUuid(0x{:04X})", self.0)
    }
}

impl Display for BleUuid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "0x{:04X}", self.0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ble_uuid() {
        const BLE_UUID: BleUuid = BleUuid::new(0x1234);
        assert_eq!(u16::from(BLE_UUID), 0x1234);
        let uuid: u16 = BLE_UUID.into();
        assert_eq!(uuid, 0x1234);
        const UUID: [u8; 2] = BLE_UUID.to_le_bytes();
        assert_eq!(UUID, [0x34, 0x12]);
    }
}
