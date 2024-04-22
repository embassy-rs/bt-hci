//! Status parameters [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-40e8a930-65b3-c409-007e-388fd48e1041)

use super::cmd;
use crate::param::ConnHandle;

cmd! {
    /// Read RSSI command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-97d75ef1-ae03-1164-a6be-61c65dc9fb94)
    ReadRssi(STATUS_PARAMS, 0x0005) {
        Params = ConnHandle;
        ReadRssiReturn {
            rssi: i8,
        }
        Handle = handle: ConnHandle;
    }
}
