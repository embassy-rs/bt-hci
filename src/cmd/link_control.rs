use super::cmd;
use crate::param::{param, ConnHandle};

param!(
    enum DisconnectReason {
        AuthenticationFailure = 0x05,
        RemoteUserTerminatedConn = 0x13,
        RemoteDeviceTerminatedConnLowResources = 0x14,
        RemoteDeviceTerminatedConnPowerOff = 0x15,
        UnsupportedRemoteFeature = 0x1A,
        PairingWithUnitKeyNotSupported = 0x29,
        UnacceptableConnParameters = 0x3b,
    }
);

cmd! {
    Disconnect(LINK_CONTROL, 0x0006) {
        Params {
            handle: ConnHandle,
            reason: DisconnectReason,
        }
        Return = ();
    }
}

cmd! {
    ReadRemoteVersionInformation(LINK_CONTROL, 0x001d) {
        Params {
            handle: ConnHandle,
        }
    }
}
