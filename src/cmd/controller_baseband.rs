//! Bluetooth Core Specification Vol 4, Part E, §7.3

use crate::param::{
    ConnHandle, ConnHandleCompletedPackets, ControllerToHostFlowControl, Duration, EventMask, EventMaskPage2,
    PowerLevelKind,
};
use crate::{cmd, param};

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
        Params = ReadTransmitPowerLevelParams;
        /// Bluetooth Core Specification Vol 4, Part E, §7.3.35
        ReadTransmitPowerLevelReturn {
            handle: ConnHandle,
            tx_power_level: i8,
        }
    }
}

param! {
    struct ReadTransmitPowerLevelParams {
        handle: ConnHandle,
        kind: PowerLevelKind,
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
        Params = HostBufferSizeParams;
        Return = ();
    }
}

param! {
    struct HostBufferSizeParams {
        host_acl_data_packet_len: u16,
        host_sync_data_packet_len: u8,
        host_total_acl_data_packets: u16,
        host_total_sync_data_packets: u16,
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
            handle: ConnHandle,
            timeout: Duration<10_000>,
        }
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.3.94
    WriteAuthenticatedPayloadTimeout(CONTROL_BASEBAND, 0x007c) {
        Params = WriteAuthenticatedPayloadTimeoutParams;
        Return = ConnHandle;
    }
}

param! {
    struct WriteAuthenticatedPayloadTimeoutParams {
        handle: ConnHandle,
        timeout: Duration<10_000>,
    }
}
