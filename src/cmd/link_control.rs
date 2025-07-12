//! Link Control commands [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-fe2a33d3-28f4-9fd1-4d08-62286985c05e)

use crate::cmd;
use crate::param::{
    AllowRoleSwitch, BdAddr, ClockOffset, ConnHandle, DisconnectReason, PacketType, PageScanRepetitionMode, Status,
    StatusBdAddrReturn,
};

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
    /// Inquiry Cancel command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-75fbb334-4adc-d07a-bd5c-80b85f6e7074)
    ///
    /// Stops the current Inquiry if the BR/EDR Controller is in Inquiry Mode.
    InquiryCancel(LINK_CONTROL, 0x0002) {
        Params = ();
        Return = Status;
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
            page_scan_repetition_mode: PageScanRepetitionMode,
            reserved: u8, // Reserved, shall be set to 0x00.
            clock_offset: ClockOffset,
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
            packet_type: PacketType,
            page_scan_repetition_mode: PageScanRepetitionMode,
            reserved: u8, // Reserved, shall be set to 0x00.
            clock_offset: ClockOffset,
            allow_role_switch: AllowRoleSwitch,
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
        Return = StatusBdAddrReturn;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cmd::{Cmd, OpcodeGroup};
    use crate::param::{
        AllowRoleSwitch, BdAddr, ClockOffset, ConnHandle, DisconnectReason, PacketType, PageScanRepetitionMode,
    };

    #[test]
    fn test_inquiry() {
        let _cmd = Inquiry::new(
            [0x9e, 0x8b, 0x33], // General/Unlimited Inquiry Access Code
            0x08,               // 10.24 seconds
            0x00,               // Unlimited number of responses
        );

        assert_eq!(Inquiry::OPCODE.group(), OpcodeGroup::new(0x01));
        assert_eq!(Inquiry::OPCODE.cmd(), 0x0001);
    }

    #[test]
    fn test_inquiry_cancel() {
        let _cmd = InquiryCancel::new();
        assert_eq!(InquiryCancel::OPCODE.group(), OpcodeGroup::new(0x01));
        assert_eq!(InquiryCancel::OPCODE.cmd(), 0x0002);
    }

    #[test]
    fn test_disconnect() {
        let _cmd = Disconnect::new(ConnHandle::new(0x0001), DisconnectReason::AuthenticationFailure);

        assert_eq!(Disconnect::OPCODE.group(), OpcodeGroup::new(0x01));
        assert_eq!(Disconnect::OPCODE.cmd(), 0x0006);
    }

    #[test]
    fn test_read_remote_version_information() {
        let _cmd = ReadRemoteVersionInformation::new(ConnHandle::new(0x0001));

        assert_eq!(ReadRemoteVersionInformation::OPCODE.group(), OpcodeGroup::new(0x01));
        assert_eq!(ReadRemoteVersionInformation::OPCODE.cmd(), 0x001d);
    }

    #[test]
    fn test_remote_name_request() {
        let _cmd = RemoteNameRequest::new(
            BdAddr::new([0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc]),
            PageScanRepetitionMode::R2,
            0x00, // reserved
            ClockOffset::new().set_clock_offset_0(true),
        );
        assert_eq!(RemoteNameRequest::OPCODE.group(), OpcodeGroup::new(0x01));
        assert_eq!(RemoteNameRequest::OPCODE.cmd(), 0x0019);
    }

    #[test]
    fn test_create_connection() {
        let _cmd = CreateConnection::new(
            BdAddr::new([0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc]),
            PacketType::new()
                .set_dh1_may_be_used(true)
                .set_dm3_may_be_used(true)
                .set_dh3_may_be_used(true),
            PageScanRepetitionMode::R2,
            0x00, // reserved
            ClockOffset::new(),
            AllowRoleSwitch::Allowed,
        );
        assert_eq!(CreateConnection::OPCODE.group(), OpcodeGroup::new(0x01));
        assert_eq!(CreateConnection::OPCODE.cmd(), 0x0005);
    }

    #[test]
    fn test_authentication_requested() {
        let _cmd = AuthenticationRequested::new(ConnHandle::new(0x0001));

        assert_eq!(AuthenticationRequested::OPCODE.group(), OpcodeGroup::new(0x01));
        assert_eq!(AuthenticationRequested::OPCODE.cmd(), 0x0011);
    }

    #[test]
    fn test_link_key_request_negative_reply() {
        let bd_addr = BdAddr::new([0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc]);
        let _cmd = LinkKeyRequestNegativeReply::new(bd_addr);

        assert_eq!(LinkKeyRequestNegativeReply::OPCODE.group(), OpcodeGroup::new(0x01));
        assert_eq!(LinkKeyRequestNegativeReply::OPCODE.cmd(), 0x000c);
    }

    #[test]
    fn test_pin_code_request_reply() {
        let _cmd = PinCodeRequestReply::new(
            BdAddr::new([0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc]),
            4,
            [b'1', b'2', b'3', b'4', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );

        assert_eq!(PinCodeRequestReply::OPCODE.group(), OpcodeGroup::new(0x01));
        assert_eq!(PinCodeRequestReply::OPCODE.cmd(), 0x000d);
    }
}
