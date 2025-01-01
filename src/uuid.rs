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

/// 0000xxxx-0000-1000-8000-00805F9B34FB;
///
/// BLUETOOTH CORE SPECIFICATION Version 6.0 | Vol 3, Part B | Page 1250
/// [(link)](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host/service-discovery-protocol--sdp--specification.html#UUID-ef710684-4c7e-6793-4350-4a190ea9a7a4)
///
/// The full 128-bit value of a 16-bit or 32-bit UUID may be computed by a simple arithmetic operation.
///
/// 128_bit_value = 16_bit_value × 2^96 + Bluetooth_Base_UUID
pub const BLUETOOTH_BASE_UUID: [u8; 16] = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x80, 0x00, 0x00, 0x80, 0x5F, 0x9B, 0x34, 0xFB,
];
#[cfg(feature = "uuid")]
const BASE_UUID: uuid::Uuid = uuid::Uuid::from_bytes_le(BLUETOOTH_BASE_UUID);

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
        // "0000xxxx-0000-1000-8000-00805F9B34FB"
        uuid::Uuid::from_u128(BASE_UUID.as_u128() | (u128::from(uuid.0) << 96))
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
        let result = uuid::Uuid::from(BluetoothUuid16::new(0x1234));
        let expected = "00001234-0000-0010-8000-00805f9b34fb".parse::<uuid::Uuid>().unwrap();

        // defmt::Format not implemented on uuid::Uuid
        assert_eq!(result.into_bytes(), expected.into_bytes());
    }
}
