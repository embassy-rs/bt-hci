//! Controller & Baseband commands [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-5ced811b-a6ce-701a-16b2-70f2d9795c05)

use crate::cmd;

cmd! {
    /// Reset command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-b0aaafb1-0601-865c-2703-4f4caa4dee2e)
    Reset(CONTROL_BASEBAND, 0x0003) {
        Params = ();
        Return = ();
    }
}
