use super::cmd;
use crate::param::ConnHandle;

cmd! {
    ReadRssi(STATUS_PARAMS, 0x0005) {
        Params {
            handle: ConnHandle,
        }
        ReadRssiReturn {
            handle: ConnHandle,
            rssi: i8,
        }
    }
}
