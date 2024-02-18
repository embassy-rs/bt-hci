//! Bluetooth Core Specification Vol 4, Part E, §7.5

use super::cmd;
use crate::param::ConnHandle;

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.5.4
    ReadRssi(STATUS_PARAMS, 0x0005) {
        Params = ConnHandle;
        /// Bluetooth Core Specification Vol 4, Part E, §7.5.4
        ReadRssiReturn {
            handle: ConnHandle,
            rssi: i8,
        }
    }
}
