//! Bluetooth Core Specification Vol 4, Part E, §7.1

use super::cmd;
use crate::param::{ConnHandle, DisconnectReason};

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.1.6
    Disconnect(LINK_CONTROL, 0x0006) {
        Params {
            handle: ConnHandle,
            reason: DisconnectReason,
        }
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.1.23
    ReadRemoteVersionInformation(LINK_CONTROL, 0x001d) {
        Params {
            handle: ConnHandle,
        }
    }
}
