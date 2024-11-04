//! The representation of the external appearance of the device.
//!
//! https://www.bluetooth.com/wp-content/uploads/Files/Specification/Assigned_Numbers.html#bookmark49

#[allow(dead_code)]
mod categories;

pub use categories::*;

use super::BluetoothUuid16;

/// Construct a new appearance value for the GAP Service.
///
/// Follow the pattern of the examples below to create new appearance values.
/// Use UUIDs from the [Bluetooth Assigned Numbers list](https://www.bluetooth.com/wp-content/uploads/Files/Specification/Assigned_Numbers.html#bookmark49).
///
/// ## Example
///
/// ```rust ignore
///
/// const GAMEPAD: BluetoothUuid16 = appearance::from_category(0x00F, 0x040);
/// const GAMEPAD_BYTES: &[u8; 2] = &GAMEPAD.to_le_bytes();
/// ```
pub const fn from_category(category: u8, subcategory: u8) -> BluetoothUuid16 {
    let uuid = ((category as u16) << 6) | (subcategory as u16);
    BluetoothUuid16(uuid)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_appearance() {
        const CUSTOM_UUID: BluetoothUuid16 = from_category(0x002, 0x007);
        assert_eq!(u16::from(CUSTOM_UUID), 0x0087);
        let uuid: u16 = aircraft::LARGE_PASSENGER_AIRCRAFT.into();
        assert_eq!(uuid, 0x0984);
        const LABEL_BYTES: [u8; 2] = signage::ELECTRONIC_LABEL.to_le_bytes();
        assert_eq!(LABEL_BYTES, [0xc2, 0x0a]);
        const GAMEPAD: BluetoothUuid16 = power_device::GENERIC_POWER_DEVICE;
        assert_eq!(u16::from(GAMEPAD), 0x780);
    }
}
