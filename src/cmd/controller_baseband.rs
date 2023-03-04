use super::cmd;
use crate::param::{ConnHandle, ControllerToHostFlowControl, Duration, EventMask, EventMaskPage2, PowerLevelKind};

cmd! {
    SetEventMask(CONTROL_BASEBAND, 0x0001) {
        Params {
            mask: EventMask,
        }
        Return = ();
    }
}

cmd! {
    Reset(CONTROL_BASEBAND, 0x0003) {
        Params {}
        Return = ();
    }
}

cmd! {
    ReadTransmitPowerLevel(CONTROL_BASEBAND, 0x002d) {
        Params {
            handle: ConnHandle,
            kind: PowerLevelKind,
        }
        ReadTransmitPowerLevelReturn {
            handle: ConnHandle,
            tx_power_level: i8,
        }
    }
}

cmd! {
    SetControllerToHostFlowControl(CONTROL_BASEBAND, 0x0031) {
        Params {
            flow_control_enable: ControllerToHostFlowControl,
        }
        Return = ();
    }
}

cmd! {
    HostBufferSize(CONTROL_BASEBAND, 0x0031) {
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
    SetEventMaskPage2(CONTROL_BASEBAND, 0x0063) {
        Params {
            mask: EventMaskPage2,
        }
        Return = ();
    }
}

cmd! {
    ReadAuthenticatedPayloadTimeout(CONTROL_BASEBAND, 0x007b) {
        Params {
            handle: ConnHandle,
        }
        ReadAuthenticatedPayloadTimeoutReturn {
            handle: ConnHandle,
            timeout: Duration<10_000>,
        }
    }
}

cmd! {
    WriteAuthenticatedPayloadTimeout(CONTROL_BASEBAND, 0x007c) {
        Params {
            handle: ConnHandle,
            timeout: Duration<10_000>,
        }
        Return = ConnHandle;
    }
}
