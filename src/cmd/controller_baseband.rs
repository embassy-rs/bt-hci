use super::cmd;
use crate::param::events::{EventMask, EventMaskPage2};
use crate::param::{param, ConnHandle, Duration};

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

param! {
    enum PowerLevelKind {
        Current = 0,
        Maximum = 1,
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

param! {
    enum ControllerToHostFlowControl {
        Off = 0,
        AclOnSyncOff = 1,
        AclOffSyncOn = 2,
        BothOn = 3,
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
            timeout: Duration<16>,
        }
    }
}

cmd! {
    WriteAuthenticatedPayloadTimeout(CONTROL_BASEBAND, 0x007c) {
        Params {
            handle: ConnHandle,
            timeout: Duration<16>,
        }
        Return = ConnHandle;
    }
}
