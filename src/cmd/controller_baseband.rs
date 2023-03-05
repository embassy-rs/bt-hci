//! Bluetooth Core Specification Vol 4, Part E, §7.3

use super::cmd;
use crate::param::{ConnHandle, ControllerToHostFlowControl, Duration, EventMask, EventMaskPage2, PowerLevelKind};

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.3.1
    SetEventMask(CONTROL_BASEBAND, 0x0001) {
        Params {
            mask: EventMask,
        }
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.3.2
    Reset(CONTROL_BASEBAND, 0x0003) {
        Params {}
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.3.35
    ReadTransmitPowerLevel(CONTROL_BASEBAND, 0x002d) {
        Params {
            handle: ConnHandle,
            kind: PowerLevelKind,
        }
        /// Bluetooth Core Specification Vol 4, Part E, §7.3.35
        ReadTransmitPowerLevelReturn {
            handle: ConnHandle,
            tx_power_level: i8,
        }
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.3.38
    SetControllerToHostFlowControl(CONTROL_BASEBAND, 0x0031) {
        Params {
            flow_control_enable: ControllerToHostFlowControl,
        }
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.3.39
    HostBufferSize(CONTROL_BASEBAND, 0x0033) {
        Params {
            host_acl_data_packet_len: u16,
            host_sync_data_packet_len: u8,
            host_total_acl_data_packets: u16,
            host_total_sync_data_packets: u16,
        }
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.3.69
    SetEventMaskPage2(CONTROL_BASEBAND, 0x0063) {
        Params {
            mask: EventMaskPage2,
        }
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.3.93
    ReadAuthenticatedPayloadTimeout(CONTROL_BASEBAND, 0x007b) {
        Params {
            handle: ConnHandle,
        }
        /// Bluetooth Core Specification Vol 4, Part E, §7.3.93
        ReadAuthenticatedPayloadTimeoutReturn {
            handle: ConnHandle,
            timeout: Duration<10_000>,
        }
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.3.94
    WriteAuthenticatedPayloadTimeout(CONTROL_BASEBAND, 0x007c) {
        Params {
            handle: ConnHandle,
            timeout: Duration<10_000>,
        }
        Return = ConnHandle;
    }
}
