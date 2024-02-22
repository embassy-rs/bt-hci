//! Bluetooth Core Specification Vol 4, Part E, §7.3

use crate::cmd;
use crate::param::{
    ConnHandle, ConnHandleCompletedPackets, ControllerToHostFlowControl, Duration, EventMask, EventMaskPage2,
    PowerLevelKind,
};

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.3.1
    SetEventMask(CONTROL_BASEBAND, 0x0001) {
        Params = EventMask;
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.3.2
    Reset(CONTROL_BASEBAND, 0x0003) {
        Params = ();
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.3.35
    ReadTransmitPowerLevel(CONTROL_BASEBAND, 0x002d) {
        ReadTransmitPowerLevelParams {
            kind: PowerLevelKind,
        }
        /// Bluetooth Core Specification Vol 4, Part E, §7.3.35
        ReadTransmitPowerLevelReturn {
            tx_power_level: i8,
        }
        Handle = handle: ConnHandle;
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.3.38
    SetControllerToHostFlowControl(CONTROL_BASEBAND, 0x0031) {
        Params = ControllerToHostFlowControl;
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.3.39
    HostBufferSize(CONTROL_BASEBAND, 0x0033) {
        HostBufferSizeParams {
            host_acl_data_packet_len: u16,
            host_sync_data_packet_len: u8,
            host_total_acl_data_packets: u16,
            host_total_sync_data_packets: u16,
        }
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.3.40
    HostNumberOfCompletedPackets(CONTROL_BASEBAND, 0x0035)  {
        Params<'a> = &'a [ConnHandleCompletedPackets];
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.3.69
    SetEventMaskPage2(CONTROL_BASEBAND, 0x0063) {
        Params = EventMaskPage2;
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.3.93
    ReadAuthenticatedPayloadTimeout(CONTROL_BASEBAND, 0x007b) {
        Params = ConnHandle;
        /// Bluetooth Core Specification Vol 4, Part E, §7.3.93
        ReadAuthenticatedPayloadTimeoutReturn {
            timeout: Duration<10_000>,
        }
        Handle = handle: ConnHandle;
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.3.94
    WriteAuthenticatedPayloadTimeout(CONTROL_BASEBAND, 0x007c) {
        WriteAuthenticatedPayloadTimeoutParams {
            timeout: Duration<10_000>,
        }
        Return = ConnHandle;
        Handle = handle: ConnHandle;
    }
}
