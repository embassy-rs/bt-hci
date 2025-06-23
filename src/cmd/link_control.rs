//! Link Control commands [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-fe2a33d3-28f4-9fd1-4d08-62286985c05e)

use crate::cmd;
use crate::param::{BdAddr, ConnHandle, DisconnectReason};

cmd! {
    /// Inquiry command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-2db7bf11-f361-99bd-6161-dc9696f86c6b)
    Inquiry(LINK_CONTROL, 0x0001) {
        InquiryParams {
            lap: [u8; 3],
            inquiry_length: u8,
            num_responses: u8,
        }
        Return = ();
    }
}

cmd! {
    /// Disconnect command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-6bb8119e-aa67-d517-db2a-7470c35fbf4a)
    Disconnect(LINK_CONTROL, 0x0006) {
        DisconnectParams {
            handle: ConnHandle,
            reason: DisconnectReason,
        }
        Return = ();
    }
}

cmd! {
    /// Read Remote Version Information command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-ebf3c9ac-0bfa-0ed0-c014-8f8691ea3fe5)
    ReadRemoteVersionInformation(LINK_CONTROL, 0x001d) {
        Params = ConnHandle;
    }
}

cmd! {
    /// Remote Name Request command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-cbd9cb09-59fd-9739-2570-8fae93d45bd7)
    ///
    /// Initiates a remote name request procedure for the specified Bluetooth device.
    RemoteNameRequest(LINK_CONTROL, 0x0019) {
        RemoteNameRequestParams {
            bd_addr: BdAddr,
            page_scan_repetition_mode: u8,
            reserved: u8,
            clock_offset: u16,
        }
        Return = ();
    }
}

cmd! {
    /// Create Connection command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-4150eaa8-3d28-1113-68cf-5bae5bae78fd)
    ///
    /// Initiates a connection to a remote Bluetooth device.
    CreateConnection(LINK_CONTROL, 0x0005) {
        CreateConnectionParams {
            bd_addr: BdAddr,
            packet_type: u16,
            page_scan_repetition_mode: u8,
            reserved: u8,
            clock_offset: u16,
            allow_role_switch: u8,
        }
        Return = ();
    }
}

cmd! {
    /// Authentication Requested command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-904095aa-072e-02c1-023a-e16571079cd2)
    ///
    /// Initiates authentication (pairing) for the given connection handle.
    AuthenticationRequested(LINK_CONTROL, 0x0011) {
        Params = ConnHandle;
        Return = ();
    }
}

cmd! {
    /// Link Key Request Negative Reply command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-1ca1324a-dd2c-15b6-2ccf-b469b18dbd3d)
    ///
    /// Used to respond to a Link Key Request event when no key is available.
    LinkKeyRequestNegativeReply(LINK_CONTROL, 0x000c) {
        Params = BdAddr;
        Return = BdAddr;
    }
}

cmd! {
    /// PIN Code Request Reply command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-55d6cf30-d90e-f769-5176-e44ac3c3292e)
    ///
    /// Used to reply to a PIN Code Request event with the PIN code.
    PinCodeRequestReply(LINK_CONTROL, 0x000d) {
        PinCodeRequestReplyParams {
            bd_addr: BdAddr,
            pin_code_len: u8,
            pin_code: [u8; 16],
        }
        Return = BdAddr;
    }
}
