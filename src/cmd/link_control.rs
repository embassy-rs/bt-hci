//! Bluetooth Core Specification Vol 4, Part E, §7.1

use crate::param::{ConnHandle, DisconnectReason};
use crate::{cmd, param};

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.1.6
    Disconnect(LINK_CONTROL, 0x0006) {
        Params = DisconnectParams;
        Return = ();
    }
}

param! {
    struct DisconnectParams {
        handle: ConnHandle,
        reason: DisconnectReason,
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.1.23
    ReadRemoteVersionInformation(LINK_CONTROL, 0x001d) {
        Params = ConnHandle;
    }
}
