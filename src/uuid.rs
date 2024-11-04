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
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct BluetoothUuid16(u16);

impl BluetoothUuid16 {
    /// Create a new `BluetoothUuid16`.
    pub const fn new(uuid: u16) -> Self {
        Self(uuid)
    }
    /// Convert the `BluetoothUuid16` to a byte array as a const function.
    pub const fn to_le_bytes(self) -> [u8; 2] {
        self.0.to_le_bytes()
    }
    /// Convert from a byte array to a `BluetoothUuid16`.
    pub const fn from_le_bytes(bytes: [u8; 2]) -> Self {
        Self(u16::from_le_bytes(bytes))
    }
}

impl From<BluetoothUuid16> for u16 {
    fn from(uuid: BluetoothUuid16) -> u16 {
        uuid.0
    }
}

impl From<BluetoothUuid16> for [u8; 2] {
    fn from(uuid: BluetoothUuid16) -> [u8; 2] {
        uuid.0.to_le_bytes()
    }
}

impl Debug for BluetoothUuid16 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "BluetoothUuid16(0x{:04X})", self.0)
    }
}

impl Display for BluetoothUuid16 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "0x{:04X}", self.0)
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for BluetoothUuid16 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "BluetoothUuid16(0x{:04X})", self.0)
    }
}

#[cfg(feature = "uuid")]
impl From<BluetoothUuid16> for uuid::Uuid {
    fn from(uuid: BluetoothUuid16) -> uuid::Uuid {
        uuid::Uuid::from_u128(u128::from(uuid.0))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ble_uuid() {
        const BLE_UUID: BluetoothUuid16 = BluetoothUuid16::new(0x1234);
        assert_eq!(u16::from(BLE_UUID), 0x1234);
        let uuid: u16 = BLE_UUID.into();
        assert_eq!(uuid, 0x1234);
        const UUID: [u8; 2] = BLE_UUID.to_le_bytes();
        assert_eq!(UUID, [0x34, 0x12]);
    }

    #[cfg(feature = "uuid")]
    #[test]
    fn test_uuid_conversion() {
        const BLE_UUID: BluetoothUuid16 = BluetoothUuid16::new(0x1234);
        let result: uuid::Uuid = BLE_UUID.into();
        let expected: uuid::Uuid = uuid::Uuid::from_u128(0x1234);
        assert_eq!(result, expected);
    }
}
