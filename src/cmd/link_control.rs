use super::cmd;
use crate::param::{ConnHandle, DisconnectReason};

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
