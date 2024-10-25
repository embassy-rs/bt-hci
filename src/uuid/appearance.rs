//! The representation of the external appearance of the device.
//!
//! This is a list of some of the most common appearance values and demonstrates the pattern to use to define new appearance values.
//!
//! https://www.bluetooth.com/wp-content/uploads/Files/Specification/Assigned_Numbers.html#bookmark49

/// Construct a new appearance value for the GAP Service.
///
/// Follow the pattern of the examples below to create new appearance values.
/// Use UUIDs from the [Bluetooth Assigned Numbers list](https://www.bluetooth.com/wp-content/uploads/Files/Specification/Assigned_Numbers.html#bookmark49).
///
/// ## Example
///
/// ```rust
/// use trouble_host::prelude::*;
///
/// const GAMEPAD: &[u8; 2] = &appearance::new(0x00F, 0x040);
/// ```
pub const fn new(category: u8, subcategory: u8) -> [u8; 2] {
    (((category as u16) << 6) | (subcategory as u16)).to_le_bytes()
}

/// Generic Unknown device appearance.
pub const GENERIC_UNKNOWN: [u8; 2] = new(0x000, 0x000);
/// Generic Phone device appearance.
pub const GENERIC_PHONE: [u8; 2] = new(0x001, 0x000);
/// Generic Computer device appearance.
pub const GENERIC_COMPUTER: [u8; 2] = new(0x002, 0x000);
/// Smart Watch device appearance.
pub const SMART_WATCH: [u8; 2] = new(0x003, 0x020);
/// Generic Power device appearance.
pub const GENERIC_POWER: [u8; 2] = new(0x01E, 0x000);
/// Generic Sensor device appearance.
pub const GENERIC_SENSOR: [u8; 2] = new(0x015, 0x000);
/// Gamepad device appearance.
pub const GAMEPAD: [u8; 2] = new(0x00F, 0x040);