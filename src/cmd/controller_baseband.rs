//! Controller & Baseband commands [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-5ced811b-a6ce-701a-16b2-70f2d9795c05)

use crate::cmd;
use crate::param::{
    ConnHandle, ConnHandleCompletedPackets, ControllerToHostFlowControl, Duration, EventMask, EventMaskPage2,
    PowerLevelKind, ReadStoredLinkKeyParams, ReadStoredLinkKeyReturn,
};

cmd! {
    /// Set Event Mask command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-9cf88217-77b4-aeb6-61fb-d1129d48a67c)
    SetEventMask(CONTROL_BASEBAND, 0x0001) {
        Params = EventMask;
        Return = ();
    }
}

cmd! {
    /// Reset command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-b0aaafb1-0601-865c-2703-4f4caa4dee2e)
    Reset(CONTROL_BASEBAND, 0x0003) {
        Params = ();
        Return = ();
    }
}

cmd! {
    /// Read Transmit Power Level command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-7205a3ee-15c7-cc48-c512-a959b4e3f560)
    ReadTransmitPowerLevel(CONTROL_BASEBAND, 0x002d) {
        ReadTransmitPowerLevelParams {
            kind: PowerLevelKind,
        }
        ReadTransmitPowerLevelReturn {
            tx_power_level: i8,
        }
        Handle = handle: ConnHandle;
    }
}

cmd! {
    /// Set Controller To Host Flow Control command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-64d757fc-e1da-329f-6d6a-16453750f325)
    SetControllerToHostFlowControl(CONTROL_BASEBAND, 0x0031) {
        Params = ControllerToHostFlowControl;
        Return = ();
    }
}

cmd! {
    /// Host Buffer Size command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-99748527-de87-bf9c-935d-baf7e3f35b12)
    HostBufferSize(CONTROL_BASEBAND, 0x0033) {
        HostBufferSizeParams {
            host_acl_data_packet_len: u16,
            host_sync_data_packet_len: u8,
            host_total_acl_data_packets: u16,
            host_total_sync_data_packets: u16,
        }
        Return = ();
    }
}

cmd! {
    /// Host Number Of Completed Packets command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-14569cb0-16b0-7dc0-a1d4-e5a4ef44d81a)
    ///
    /// *Note:* This command only returns a [`CommandComplete`](crate::event::CommandComplete) event on error. No event is generated on success.
    HostNumberOfCompletedPackets(CONTROL_BASEBAND, 0x0035)  {
        Params<'a> = &'a [ConnHandleCompletedPackets];
        Return = ();
    }
}

cmd! {
    /// Set Event Mask Page 2 command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-4e91f200-b802-45d6-9282-fd03c0dfefbe)
    SetEventMaskPage2(CONTROL_BASEBAND, 0x0063) {
        Params = EventMaskPage2;
        Return = ();
    }
}

cmd! {
    /// Read Authenticated Payload Timeout command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-36c940e9-a654-f07f-75cd-2cbcf2d6adf6)
    ReadAuthenticatedPayloadTimeout(CONTROL_BASEBAND, 0x007b) {
        Params = ConnHandle;
        ReadAuthenticatedPayloadTimeoutReturn {
            timeout: Duration<10_000>,
        }
        Handle = handle: ConnHandle;
    }
}

cmd! {
    /// Write Authenticated Payload Timeout command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-9bb1bb66-1857-83d8-8954-677f773225f9)
    WriteAuthenticatedPayloadTimeout(CONTROL_BASEBAND, 0x007c) {
        WriteAuthenticatedPayloadTimeoutParams {
            timeout: Duration<10_000>,
        }
        Return = ConnHandle;
        Handle = handle: ConnHandle;
    }
}

cmd! {
    /// Read Stored Link Key command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-9c3e0da9-138b-7641-6e84-d2d17c1f082e)
    ///
    /// Reads stored link keys for one or more devices, or all devices.
    ///
    /// * `bd_addr`: Bluetooth device address to read the key for, or all devices if set to all 0xFF.
    /// * `read_all_flag`: 0x00 = read for specified device, 0x01 = read for all devices.
    ReadStoredLinkKey(CONTROL_BASEBAND, 0x000d) {
        Params = ReadStoredLinkKeyParams;
        Return = ReadStoredLinkKeyReturn;
    }
}
