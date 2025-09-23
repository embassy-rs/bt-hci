//! HCI events [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-d21276b6-83d0-cbc3-8295-6ff23b70a0c5)

use crate::cmd::{Opcode, SyncCmd};
use crate::param::{
    param, AuthenticationRequirements, BdAddr, ClockOffset, ClockType, ConnHandle, ConnHandleCompletedPackets,
    ConnectionLinkType, CoreSpecificationVersion, EncryptionEnabledLevel, Error, FlowDirection, IoCapability, KeyFlag,
    KeypressNotificationType, LinkKeyType, LinkType, LmpFeatureMask, MaxSlots, Mode, OobDataPresent, PacketType,
    PageScanRepetitionMode, RemainingBytes, Role, ServiceType, Status,
};
use crate::{AsHciBytes, FromHciBytes, FromHciBytesError, ReadHci, ReadHciError};

pub mod le;

use le::LeEvent;

/// A trait for objects which contain the parameters for a specific HCI event
pub trait EventParams<'a>: FromHciBytes<'a> {
    /// The event code these parameters are for
    const EVENT_CODE: u8;
}

param! {
    /// The header of an HCI event packet.
    struct EventPacketHeader {
        code: u8,
        params_len: u8,
    }
}

macro_rules! events {
    (
        $(
            $(#[$attrs:meta])*
            struct $name:ident$(<$life:lifetime>)?($code:expr) {
                $(
                    $(#[$field_attrs:meta])*
                    $field:ident: $ty:ty
                ),*
                $(,)?
            }
        )+
    ) => {
        /// An Event HCI packet
        #[non_exhaustive]
        #[derive(Debug, Clone, Hash)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Event<'a> {
            $(
                #[allow(missing_docs)]
                $name($name$(<$life>)?),
            )+
            #[allow(missing_docs)]
            Le(LeEvent<'a>),
            /// An event with an unknown code value
            Unknown {
                /// The event code
                code: u8,
                /// The bytes of the event parameters
                params: &'a [u8]
            },
        }


        /// An Event kind
        #[non_exhaustive]
        #[derive(Debug, Clone, Copy, Hash, PartialEq)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub struct EventKind(pub u8);

        #[allow(non_upper_case_globals)]
        impl EventKind {
            $(
                #[allow(missing_docs)]
                pub const $name: EventKind = EventKind($code);
            )+
            #[allow(missing_docs)]
            pub const Le: EventKind = EventKind(0x3F);
        }

        /// An Event HCI packet
        #[derive(Debug, Clone, Hash)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub struct EventPacket<'a> {
            /// Which kind of event.
            pub kind: EventKind,
            /// Event data.
            pub data: &'a [u8],
        }

        impl<'a> EventPacket<'a> {
            fn from_header_hci_bytes(header: EventPacketHeader, data: &'a [u8]) -> Result<Self, FromHciBytesError> {
                let (kind, data) = EventKind::from_header_hci_bytes(header, data)?;
                Ok(Self {
                    kind,
                    data,
                })
            }
        }

        impl EventKind {
            fn from_header_hci_bytes(header: EventPacketHeader, data: &[u8]) -> Result<(Self, &[u8]), FromHciBytesError> {
                let (data, _) = if data.len() < usize::from(header.params_len) {
                    return Err(FromHciBytesError::InvalidSize);
                } else {
                    data.split_at(usize::from(header.params_len))
                };

                match header.code {
                    $($code => Ok((Self::$name, data)),)+
                    0x3e => Ok((Self::Le, data)),
                    _ => {
                        Ok((EventKind(header.code), data))
                    }
                }
            }
        }

        impl<'a> TryFrom<EventPacket<'a>> for Event<'a> {
            type Error = FromHciBytesError;
            fn try_from(packet: EventPacket<'a>) -> Result<Self, Self::Error> {
                match packet.kind {
                    $(EventKind::$name => Ok(Self::$name($name::from_hci_bytes_complete(packet.data)?)),)+
                    EventKind::Le => {
                        Ok(Self::Le(LeEvent::from_hci_bytes_complete(packet.data)?))
                    }
                    EventKind(code) => Ok(Self::Unknown { code, params: packet.data }),
                }
            }
        }

        impl<'a> Event<'a> {
            fn from_header_hci_bytes(header: EventPacketHeader, data: &'a [u8]) -> Result<(Self, &'a [u8]), FromHciBytesError> {
                let (data, rest) = if data.len() < usize::from(header.params_len) {
                    return Err(FromHciBytesError::InvalidSize);
                } else {
                    data.split_at(usize::from(header.params_len))
                };

                match header.code {
                    $($code => $name::from_hci_bytes_complete(data).map(|x| (Self::$name(x), rest)),)+
                    0x3e => LeEvent::from_hci_bytes_complete(data).map(|x| (Self::Le(x), rest)),
                    _ => {
                        Ok((Self::Unknown { code: header.code, params: data }, rest))
                    }
                }
            }
        }

        $(
            $(#[$attrs])*
            #[derive(Debug, Clone, Copy, Hash)]
            #[cfg_attr(feature = "defmt", derive(defmt::Format))]
            pub struct $name$(<$life>)? {
                $(
                    #[doc = stringify!($field)]
                    $(#[$field_attrs])*
                    pub $field: $ty,
                )*
            }

            #[automatically_derived]
            impl<'a> $crate::FromHciBytes<'a> for $name$(<$life>)? {
                #[allow(unused_variables)]
                fn from_hci_bytes(data: &'a [u8]) -> Result<(Self, &'a [u8]), $crate::FromHciBytesError> {
                    let total = 0;
                    $(
                        let ($field, data) = <$ty as $crate::FromHciBytes>::from_hci_bytes(data)?;
                    )*
                    Ok((Self {
                        $($field,)*
                    }, data))
                }
            }

            #[automatically_derived]
            impl<'a> $crate::event::EventParams<'a> for $name$(<$life>)? {
                const EVENT_CODE: u8 = $code;
            }
        )+
    };
}

events! {
    // 0x00 - 0x0f

    /// Inquiry Complete event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-cde759f8-6c4d-2dd4-7053-1657125ded74)
    struct InquiryComplete(0x01) {
        status: Status,
    }

    /// Inquiry Result event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-3467df70-1d7a-73c5-5a3e-8689dba5523f)
    struct InquiryResult<'a>(0x02) {
        num_responses: u8,
        /// All remaining bytes for this event (contains all fields for all responses)
        bytes: RemainingBytes<'a>,
    }

    /// Connection Complete event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-ebb06dd7-356e-605c-cbc1-d06dc00f1d2b)
    struct ConnectionComplete(0x03) {
        status: Status,
        handle: ConnHandle,
        bd_addr: BdAddr,
        link_type: ConnectionLinkType,
        encryption_enabled: bool,
    }

    /// Connection Request event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-3115f164-ffcd-9451-09ef-0ed3809889eb)
    struct ConnectionRequest(0x04) {
        bd_addr: BdAddr,
        class_of_device: [u8; 3],
        link_type: ConnectionLinkType,
    }

    /// Disconnection Complete event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-332adb1f-b5ac-5289-82a2-c51a59d533e7)
    struct DisconnectionComplete(0x05) {
        status: Status,
        handle: ConnHandle,
        reason: Status,
    }

    /// Authentication Complete event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-00ede464-d351-7076-e82a-d8f4b30ee594)
    struct AuthenticationComplete(0x06) {
        status: Status,
        handle: ConnHandle,
    }

    /// Remote Name Request Complete event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-1e2ccd32-b73f-7f00-f4ff-25b2235aaf02)
    struct RemoteNameRequestComplete<'a>(0x07) {
        status: Status,
        bd_addr: BdAddr,
        remote_name: RemainingBytes<'a>, // 248 bytes max, null-terminated string
    }

    /// Encryption Change (v1) event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-7b7d27f0-1a33-ff57-5b97-7d49a04cea26)
    struct EncryptionChangeV1(0x08) {
        status: Status,
        handle: ConnHandle,
        enabled: EncryptionEnabledLevel,
    }

    /// Change Connection Link Key Complete event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-8d639c74-ec4f-24e3-4e39-952ce11fba57)
    struct ChangeConnectionLinkKeyComplete(0x09) {
        status: Status,
        handle: ConnHandle,
    }

    /// Link Key Type Changed event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-9eb2cea6-248a-e017-2c09-2797aba08cbf)
    struct LinkKeyTypeChanged(0x0a) {
        status: Status,
        handle: ConnHandle,
        key_flag: KeyFlag,
    }

    /// Read Remote Supported Features Complete event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-e191dc19-453a-d0e0-2317-2406ffc4d512)
    struct ReadRemoteSupportedFeaturesComplete(0x0b) {
        status: Status,
        handle: ConnHandle,
        lmp_features: LmpFeatureMask,
    }

    /// Read Remote Version Information Complete event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-81ed98a1-98b1-dae5-a3f5-bb7bc69d39b7)
    struct ReadRemoteVersionInformationComplete(0x0c) {
        status: Status,
        handle: ConnHandle,
        version: CoreSpecificationVersion,
        company_id: u16,
        subversion: u16,
    }

    /// QoS Setup Complete event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-a4ff46db-e472-dbd3-eaa6-810300dcd1b6)
    struct QosSetupComplete(0x0d) {
        status: Status,
        handle: ConnHandle,
        unused: u8, // Always 0
        service_type: ServiceType,
        token_rate: u32,
        peak_bandwidth: u32,
        latency: u32,
        delay_variation: u32,
    }

    /// Command Complete event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-76d31a33-1a9e-07bc-87c4-8ebffee065fd)
    struct CommandComplete<'a>(0x0e) {
        num_hci_cmd_pkts: u8,
        cmd_opcode: Opcode,
        bytes: RemainingBytes<'a>,
    }

    /// Command Status event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-4d87067c-be74-d2ff-d5c4-86416bf7af91)
    struct CommandStatus(0x0f) {
        status: Status,
        num_hci_cmd_pkts: u8,
        cmd_opcode: Opcode,
    }

    // 0x10 - 0x1f

    /// Hardware Error event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-2479a101-ae3b-5b5d-f3d4-4776af39a377)
    struct HardwareError(0x10) {
        hardware_code: u8,
    }

    /// Flush Occurred event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-26be4e2f-f6c3-eb68-ebf4-d11255eb9ec6)
    struct FlushOccurred(0x11) {
        handle: ConnHandle,
    }

    /// Role Change event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-ed33ee5b-970f-9664-0c15-3db2b884961a)
    struct RoleChange(0x12) {
        status: Status,
        bd_addr: BdAddr,
        new_role: Role,
    }

    /// Number Of Completed Packets event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-9ccbff85-45ce-9c0d-6d0c-2e6e5af52b0e)
    struct NumberOfCompletedPackets<'a>(0x13) {
        completed_packets: &'a [ConnHandleCompletedPackets],
    }

    /// Mode Change event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-b277da2e-804e-d956-9395-37eabc7e7b41)
    struct ModeChange(0x14) {
        status: Status,
        handle: ConnHandle,
        mode: Mode,
        interval: u16,
    }

    /// Return Link Keys event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-e0d451f2-4b53-cdf0-efb5-926e08b27cd2)
    struct ReturnLinkKeys<'a>(0x15) {
        num_keys: u8,
        bytes: RemainingBytes<'a>, // bd_addr: Num_Keys × 6 octets, link_key: Num_Keys × 16 octets, always zero
    }

    /// PIN Code Request event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-7666987c-9040-9aaa-cad6-96941b46d2b5)
    struct PinCodeRequest(0x16) {
        bd_addr: BdAddr,
    }

    /// Link Key Request event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-58400663-f69d-a482-13af-ec558a3f4c03)
    struct LinkKeyRequest(0x17) {
        bd_addr: BdAddr,
    }

    /// Link Key Notification event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-ef6b7301-ab4b-cce6-1fa4-c053c3cd1585)
    struct LinkKeyNotification(0x18) {
        bd_addr: BdAddr,
        link_key: [u8; 16],
        key_type: LinkKeyType,
    }

    /// Loopback Command event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-5f4e8f2c-7b9a-1c8e-8f6d-2e7a4b9c5f8e)
    struct LoopbackCommand<'a>(0x19) {
        command_packet: RemainingBytes<'a>,
    }

    /// Data Buffer Overflow event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-e15e12c7-d29a-8c25-349f-af6206c2ae57)
    struct DataBufferOverflow(0x1a) {
        link_type: LinkType,
    }

    /// Max Slots Change event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-1561feb4-2f2e-4ec1-1db7-57ec1dc436e2)
    struct MaxSlotsChange(0x1b) {
        handle: ConnHandle,
        lmp_max_slots: MaxSlots,
    }

    /// Read Clock Offset Complete event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-3ab4cf46-7f13-7901-4abb-af026ebee703)
    struct ReadClockOffsetComplete(0x1c) {
        status: Status,
        handle: ConnHandle,
        clock_offset: ClockOffset,
    }

    /// Connection Packet Type Changed event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-864f38f0-afde-bd09-09ff-c5bb5fefca62)
    struct ConnectionPacketTypeChanged(0x1d) {
        status: Status,
        handle: ConnHandle,
        packet_type: PacketType,
    }

    /// QoS Violation event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-cc019a24-4654-aa9f-dcd3-2a286e7cdd55)
    struct QosViolation(0x1e) {
        handle: ConnHandle,
    }

    // 0x20 - 0x2f

    /// Page Scan Repetition Mode Change event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-a535391f-0ecf-ff2d-dfad-1e61b021c0d6)
    struct PageScanRepetitionModeChange(0x20) {
        bd_addr: BdAddr,
        page_scan_repetition_mode: PageScanRepetitionMode,
    }

    /// Flow Specification Complete event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-65dffc5b-4c38-ce5c-a618-0d7a3e145012)
    struct FlowSpecificationComplete(0x21) {
        status: Status,
        handle: ConnHandle,
        unused: u8, // Always 0
        flow_direction: FlowDirection,
        service_type: ServiceType,
        token_rate: u32,
        token_bucket_size: u32,
        peak_bandwidth: u32,
        access_latency: u32,
    }

    /// Inquiry Result with RSSI event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-c2550565-1c65-a514-6cf0-3d55c8943dab)
    struct InquiryResultWithRssi<'a>(0x22) {
        num_responses: u8,
        /// All remaining bytes for this event (contains all fields for all responses)
        bytes: RemainingBytes<'a>,
    }

    /// Read Remote Extended Features Complete event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-6a78850c-d310-761a-74cb-abe8b2ddddd8)
    struct ReadRemoteExtendedFeaturesComplete(0x23) {
        status: Status,
        handle: ConnHandle,
        page_number: u8,
        maximum_page_number: u8,
        extended_lmp_features: LmpFeatureMask,
    }

    /// Synchronous Connection Complete event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-a625aef5-c3d7-12e9-39e4-b8f3386150bb)
    struct SynchronousConnectionComplete(0x2c) {
        status: Status,
        handle: ConnHandle,
        bd_addr: BdAddr,
        link_type: ConnectionLinkType,
        transmission_interval: u8,
        retransmission_window: u8,
        rx_packet_length: u16,
        tx_packet_length: u16,
        air_mode: u8,
    }

    /// Synchronous Connection Changed event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-5774f3c9-81e9-24e6-86a6-6f6935ee8998)
    struct SynchronousConnectionChanged(0x2d) {
        status: Status,
        handle: ConnHandle,
        transmission_interval: u8,
        retransmission_window: u8,
        rx_packet_length: u16,
        tx_packet_length: u16,
    }


    /// Sniff Subrating event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-4c17aedf-cddf-0844-39cc-3dcfddd34ec0)
    struct SniffSubrating(0x2e) {
        status: Status,
        handle: ConnHandle,
        max_tx_latency: u16,
        max_rx_latency: u16,
        min_remote_timeout: u16,
        min_local_timeout: u16,
    }

    /// Extended Inquiry Result event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-e3e8c7bc-2262-14f4-b6f5-2eeb0b25aa4f)
    struct ExtendedInquiryResult<'a>(0x2f) {
        num_responses: u8,
        bd_addr: BdAddr,
        page_scan_repetition_mode: PageScanRepetitionMode,
        reserved: u8,
        class_of_device: [u8; 3],
        clock_offset: ClockOffset,
        rssi: i8,
        eir_data: RemainingBytes<'a>,
    }

    // 0x30 - 0x3f

    /// Encryption Key Refresh Complete event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-a321123c-83a5-7baf-6971-05edd1241357)
    struct EncryptionKeyRefreshComplete(0x30) {
        status: Status,
        handle: ConnHandle,
    }

    /// IO Capability Request event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-343681e1-ca08-8d4c-79c3-e4b2c86ecba1)
    struct IoCapabilityRequest(0x31) {
        bd_addr: BdAddr,
    }

    /// IO Capability Response event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-7f2cf1ee-49ba-de05-5f26-f54925363197)
    struct IoCapabilityResponse(0x32) {
        bd_addr: BdAddr,
        io_capability: IoCapability,
        oob_data_present: OobDataPresent,
        authentication_requirements: AuthenticationRequirements,
    }

    /// User Confirmation Request event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-e7014a9e-718f-6aa8-d657-35b547e9c5d6)
    struct UserConfirmationRequest(0x33) {
        bd_addr: BdAddr,
        numeric_value: u32,
    }

    /// User Passkey Request event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-2f7a60c5-e17f-28f2-699c-e5943e488ec9)
    struct UserPasskeyRequest(0x34) {
        bd_addr: BdAddr,
    }

    /// Remote OOB Data Request event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-c5c5d906-fdde-b062-2bc6-6f5b5de25066)
    struct RemoteOobDataRequest(0x35) {
        bd_addr: BdAddr,
    }

    /// Simple Pairing Complete event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-e07a2674-e7bf-c963-0a41-9a997c940d26)
    struct SimplePairingComplete(0x36) {
        status: Status,
        bd_addr: BdAddr,
    }

    /// Link Supervision Timeout Changed event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-4d4758a0-eab4-25b8-f05e-2bbdcc6384cc)
    struct LinkSupervisionTimeoutChanged(0x38) {
        handle: ConnHandle,
        link_supervision_timeout: u16,
    }

    /// Enhanced Flush Complete event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-0102cc4f-0f30-c3cb-fd0a-f70ac2d5ed08)
    struct EnhancedFlushComplete(0x39) {
        handle: ConnHandle,
    }

    /// User Passkey Notification event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-39f61a47-537a-e0d9-4aa4-ccab280ebc99)
    struct UserPasskeyNotification(0x3b) {
        bd_addr: BdAddr,
        passkey: u32,
    }

    /// Keypress Notification event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-a96ae6b4-2259-c16d-0098-34a5e551284e)
    struct KeypressNotification(0x3c) {
        bd_addr: BdAddr,
        notification_type: KeypressNotificationType,
    }

    /// Remote Host Supported Features Notification event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-ba740ba0-44d8-d028-0a67-1abab648f6dd)
    struct RemoteHostSupportedFeaturesNotification(0x3d) {
        bd_addr: BdAddr,
        features: LmpFeatureMask,
    }

    // 0x40 - 0x4f

    /// Number Of Completed Data Blocks event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-f8e9efbb-ad3e-9f37-ee61-02a17eb0ba48)
    struct NumberOfCompletedDataBlocks<'a>(0x48) {
        total_num_data_blocks: u16,
        num_of_handles: u8,
        bytes: RemainingBytes<'a>, // Contains handle + num_completed_packets + num_completed_blocks for each handle
    }

    /// Triggered Clock Capture event  [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-0f10aa12-6d70-245a-84d9-342508615b28)
    struct TriggeredClockCapture(0x4e) {
        handle: ConnHandle,
        which_clock: ClockType,
        clock: u32,
        slot_offset: u16,
    }

    /// Synchronization Train Complete event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-57849409-0331-57bf-7c23-ed4a14d4ac3d)
    struct SynchronizationTrainComplete(0x4f) {
        status: Status,
    }

    // 0x50 - 0x5f

    /// Synchronization Train Received event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-38436ae8-4bf0-c6cf-9b64-a6d17fac279f)
    struct SynchronizationTrainReceived(0x50) {
        status: Status,
        bd_addr: BdAddr,
        clock_offset: u32,
        afh_channel_map: [u8; 10],
        lt_addr: u8,
        next_broadcast_instant: u32,
        connectionless_peripheral_broadcast_interval: u16,
        service_data: u8,
    }

    /// Connectionless Peripheral Broadcast Receive event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-3ea779f7-980c-f253-7845-2e9df03eaf3d)
    struct ConnectionlessPeripheralBroadcastReceive<'a>(0x51) {
        bd_addr: BdAddr,
        lt_addr: u8,
        clock: u32,
        offset: u32,
        rx_status: u8,
        fragment: u8,
        data_length: u8,
        data: RemainingBytes<'a>,
    }

    /// Connectionless Peripheral Broadcast Timeout event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-783a9fcb-1c5c-fcb2-2169-445c9cac9f91)
    struct ConnectionlessPeripheralBroadcastTimeout(0x52) {
        bd_addr: BdAddr,
        lt_addr: u8,
    }

    /// Truncated Page Complete event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-99bf982f-4155-cdc5-eef4-3a198afd5d09)
    struct TruncatedPageComplete(0x53) {
        status: Status,
        bd_addr: BdAddr,
    }

    /// Peripheral Page Response Timeout event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-928cc95f-6680-8ee0-b48b-31e97ff7ec4e)
    struct PeripheralPageResponseTimeout(0x54) {
    }

    /// Connectionless Peripheral Broadcast Channel Map Change event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-2d2c3fab-5387-79bf-03bf-6a3e3ad410a5)
    struct ConnectionlessPeripheralBroadcastChannelMapChange(0x55) {
        channel_map: [u8; 10],
    }

    /// Inquiry Response Notification event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-4d5e8f2c-7b9a-3c8e-8f6d-2e7a4b9c5f8e)
    struct InquiryResponseNotification(0x56) {
        lap: [u8; 3],
        rssi: i8,
    }

    /// Authenticated Payload Timeout Expired event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-6cfdff94-ace8-294c-6af9-d90d94653e19)
    struct AuthenticatedPayloadTimeoutExpired(0x57) {
        handle: ConnHandle,
    }

    /// SAM Status Change event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-57dba68a-5205-8c2e-1e2a-9c4d858eb93b)
    struct SamStatusChange(0x58) {
        handle: ConnHandle,
        local_sam_index: u8,
        local_sam_tx_availability: u8,
        local_sam_rx_availability: u8,
        remote_sam_index: u8,
        remote_sam_tx_availability: u8,
        remote_sam_rx_availability: u8,
    }

    /// Encryption Change (v2) event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-7b7d27f0-1a33-ff57-5b97-7d49a04cea26)
    struct EncryptionChangeV2(0x59) {
        status: Status,
        handle: ConnHandle,
        encryption_enabled: EncryptionEnabledLevel,
        encryption_key_size: u8,
    }

    // 0xf0 - 0xff

    /// HCI Event packet [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-f209cdf7-0496-8bcd-b7e1-500831511378)
    struct Vendor<'a>(0xff) {
        params: RemainingBytes<'a>,
    }
}

impl<'de> FromHciBytes<'de> for Event<'de> {
    fn from_hci_bytes(data: &'de [u8]) -> Result<(Self, &'de [u8]), FromHciBytesError> {
        let (header, data) = EventPacketHeader::from_hci_bytes(data)?;
        Self::from_header_hci_bytes(header, data)
    }
}

impl<'de> FromHciBytes<'de> for EventPacket<'de> {
    fn from_hci_bytes(data: &'de [u8]) -> Result<(Self, &'de [u8]), FromHciBytesError> {
        let (header, data) = EventPacketHeader::from_hci_bytes(data)?;
        let pkt = Self::from_header_hci_bytes(header, data)?;
        Ok((pkt, &[]))
    }
}

impl<'de> ReadHci<'de> for EventPacket<'de> {
    const MAX_LEN: usize = 257;

    fn read_hci<R: embedded_io::Read>(mut reader: R, buf: &'de mut [u8]) -> Result<Self, ReadHciError<R::Error>> {
        let mut header = [0; 2];
        reader.read_exact(&mut header)?;
        let (header, _) = EventPacketHeader::from_hci_bytes(&header)?;
        let params_len = usize::from(header.params_len);
        if buf.len() < params_len {
            Err(ReadHciError::BufferTooSmall)
        } else {
            let (buf, _) = buf.split_at_mut(params_len);
            reader.read_exact(buf)?;
            let pkt = Self::from_header_hci_bytes(header, buf)?;
            Ok(pkt)
        }
    }

    async fn read_hci_async<R: embedded_io_async::Read>(
        mut reader: R,
        buf: &'de mut [u8],
    ) -> Result<Self, ReadHciError<R::Error>> {
        let mut header = [0; 2];
        reader.read_exact(&mut header).await?;
        let (header, _) = EventPacketHeader::from_hci_bytes(&header)?;
        let params_len = usize::from(header.params_len);
        if buf.len() < params_len {
            Err(ReadHciError::BufferTooSmall)
        } else {
            let (buf, _) = buf.split_at_mut(params_len);
            reader.read_exact(buf).await?;
            let pkt = Self::from_header_hci_bytes(header, buf)?;
            Ok(pkt)
        }
    }
}

impl<'de> ReadHci<'de> for Event<'de> {
    const MAX_LEN: usize = 257;

    fn read_hci<R: embedded_io::Read>(mut reader: R, buf: &'de mut [u8]) -> Result<Self, ReadHciError<R::Error>> {
        let mut header = [0; 2];
        reader.read_exact(&mut header)?;
        let (header, _) = EventPacketHeader::from_hci_bytes(&header)?;
        let params_len = usize::from(header.params_len);
        if buf.len() < params_len {
            Err(ReadHciError::BufferTooSmall)
        } else {
            let (buf, _) = buf.split_at_mut(params_len);
            reader.read_exact(buf)?;
            let (pkt, _) = Self::from_header_hci_bytes(header, buf)?;
            Ok(pkt)
        }
    }

    async fn read_hci_async<R: embedded_io_async::Read>(
        mut reader: R,
        buf: &'de mut [u8],
    ) -> Result<Self, ReadHciError<R::Error>> {
        let mut header = [0; 2];
        reader.read_exact(&mut header).await?;
        let (header, _) = EventPacketHeader::from_hci_bytes(&header)?;
        let params_len = usize::from(header.params_len);
        if buf.len() < params_len {
            Err(ReadHciError::BufferTooSmall)
        } else {
            let (buf, _) = buf.split_at_mut(params_len);
            reader.read_exact(buf).await?;
            let (pkt, _) = Self::from_header_hci_bytes(header, buf)?;
            Ok(pkt)
        }
    }
}

impl CommandComplete<'_> {
    /// Whether or not this event has a status
    pub fn has_status(&self) -> bool {
        self.cmd_opcode != Opcode::UNSOLICITED
    }
}

impl<'d> TryFrom<CommandComplete<'d>> for CommandCompleteWithStatus<'d> {
    type Error = FromHciBytesError;
    fn try_from(e: CommandComplete<'d>) -> Result<CommandCompleteWithStatus<'d>, Self::Error> {
        if e.cmd_opcode == Opcode::UNSOLICITED {
            return Err(FromHciBytesError::InvalidSize);
        }
        let bytes = e.bytes.into_inner();
        let (status, remaining) = Status::from_hci_bytes(bytes)?;
        let return_param_bytes: RemainingBytes<'d> = RemainingBytes::from_hci_bytes_complete(remaining)?;
        Ok(Self {
            num_hci_cmd_pkts: e.num_hci_cmd_pkts,
            cmd_opcode: e.cmd_opcode,
            status,
            return_param_bytes,
        })
    }
}

/// Struct representing a command complete event with status
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandCompleteWithStatus<'d> {
    /// Number of packets complete.
    pub num_hci_cmd_pkts: u8,
    /// Command opcode.
    pub cmd_opcode: Opcode,
    /// Command status.
    pub status: Status,
    /// Return parameters
    pub return_param_bytes: RemainingBytes<'d>,
}

impl CommandCompleteWithStatus<'_> {
    /// Gets the connection handle associated with the command that has completed.
    ///
    /// For commands that return the connection handle provided as a parameter as
    /// their first return parameter, this will be valid even if `status` is an error.
    pub fn handle<C: SyncCmd>(&self) -> Result<C::Handle, FromHciBytesError> {
        C::return_handle(&self.return_param_bytes)
    }

    /// Gets a result with the return parameters for `C` or an `Error` if `status` is
    /// an error.
    ///
    /// # Panics
    ///
    /// May panic if `C::OPCODE` is not equal to `self.cmd_opcode`.
    pub fn to_result<C: SyncCmd>(&self) -> Result<C::Return, Error> {
        self.status
            .to_result()
            .and_then(|_| self.return_params::<C>().or(Err(Error::INVALID_HCI_PARAMETERS)))
    }

    /// Parses the return parameters for `C` from this event. This may fail if `status`
    /// is an error.
    ///
    /// # Panics
    ///
    /// May panic if `C::OPCODE` is not equal to `self.cmd_opcode`.
    pub fn return_params<C: SyncCmd>(&self) -> Result<C::Return, FromHciBytesError> {
        assert_eq!(self.cmd_opcode, C::OPCODE);
        C::Return::from_hci_bytes(&self.return_param_bytes).and_then(|(params, rest)| {
            if rest.is_empty() {
                Ok(params)
            } else {
                Err(FromHciBytesError::InvalidSize)
            }
        })
    }
}

/// Struct representing a single parsed inquiry result item
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InquiryResultItem {
    /// Bluetooth Device Address (BD_ADDR) of the device found
    pub bd_addr: BdAddr,
    /// Page scan repetition mode of the device
    pub page_scan_repetition_mode: Option<PageScanRepetitionMode>,
    /// Class of device (CoD) of the device found
    pub class_of_device: Option<[u8; 3]>,
    /// Clock offset of the device found
    pub clock_offset: Option<ClockOffset>,
    /// Received Signal Strength Indicator (RSSI) of the device found
    /// This field is only present in `InquiryResultWithRssi`
    pub rssi: Option<i8>,
}

/// Iterator over inquiry result items
pub struct InquiryResultIter<'a> {
    bytes: &'a [u8],
    num_responses: usize,
    idx: usize,
    kind: InquiryResultKind,
}

/// Kind of inquiry result, indicating whether it includes RSSI or not
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum InquiryResultKind {
    /// Standard inquiry result without RSSI
    Standard,
    /// Inquiry result with RSSI
    WithRssi,
}

impl<'a> InquiryResultIter<'a> {
    /// Creates a new iterator for standard inquiry results
    pub fn new_standard(bytes: &'a [u8], num_responses: usize) -> Self {
        InquiryResultIter {
            bytes,
            num_responses,
            idx: 0,
            kind: InquiryResultKind::Standard,
        }
    }

    /// Creates a new iterator for inquiry results with RSSI
    pub fn new_with_rssi(bytes: &'a [u8], num_responses: usize) -> Self {
        InquiryResultIter {
            bytes,
            num_responses,
            idx: 0,
            kind: InquiryResultKind::WithRssi,
        }
    }
}

impl Iterator for InquiryResultIter<'_> {
    type Item = InquiryResultItem;
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.num_responses {
            return None;
        }

        let i = self.idx;
        let n = self.num_responses;

        let bd_addr_size = n * 6;
        let page_scan_size = n;
        let class_size = n * 3;
        let clock_size = n * 2;

        let reserved_size = match self.kind {
            InquiryResultKind::Standard => n * 2,
            InquiryResultKind::WithRssi => n,
        };

        let bd_addr_off = i * 6;
        let page_scan_off = bd_addr_size + i;
        let class_off = bd_addr_size + page_scan_size + reserved_size + i * 3;
        let clock_off = bd_addr_size + page_scan_size + reserved_size + class_size + i * 2;

        if self.bytes.len() < bd_addr_off + 6 {
            return None;
        }

        let bd_addr = BdAddr::new([
            self.bytes[bd_addr_off],
            self.bytes[bd_addr_off + 1],
            self.bytes[bd_addr_off + 2],
            self.bytes[bd_addr_off + 3],
            self.bytes[bd_addr_off + 4],
            self.bytes[bd_addr_off + 5],
        ]);

        let page_scan_repetition_mode = self
            .bytes
            .get(page_scan_off)
            .and_then(|b| PageScanRepetitionMode::from_hci_bytes(&[*b]).ok().map(|(m, _)| m));

        let class_of_device = self.bytes.get(class_off..class_off + 3).map(|s| [s[0], s[1], s[2]]);

        let clock_offset = self
            .bytes
            .get(clock_off..clock_off + 2)
            .and_then(|s| ClockOffset::from_hci_bytes(s).ok().map(|(c, _)| c));

        let rssi = if self.kind == InquiryResultKind::WithRssi {
            let rssi_off = bd_addr_size + page_scan_size + reserved_size + class_size + clock_size + i;
            self.bytes.get(rssi_off).map(|b| *b as i8)
        } else {
            None
        };

        self.idx += 1;

        Some(InquiryResultItem {
            bd_addr,
            page_scan_repetition_mode,
            class_of_device,
            clock_offset,
            rssi,
        })
    }
}

/// Inquiry result event containing multiple responses
impl InquiryResult<'_> {
    /// Returns an iterator over all valid inquiry result items.
    pub fn iter(&self) -> InquiryResultIter<'_> {
        let bytes = self.bytes.as_hci_bytes();
        let n = self.num_responses as usize;
        InquiryResultIter::new_standard(bytes, n)
    }
}

/// Inquiry result event containing multiple responses with RSSI
impl InquiryResultWithRssi<'_> {
    /// Returns an iterator over all valid inquiry result items.
    pub fn iter(&self) -> InquiryResultIter<'_> {
        let bytes = self.bytes.as_hci_bytes();
        let n = self.num_responses as usize;
        InquiryResultIter::new_with_rssi(bytes, n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cmd::OpcodeGroup;
    use crate::event::le::LeEventPacket;
    use crate::param::*;

    #[test]
    fn test_inquiry_result() {
        let data = [
            0x02, // num_responses = 2
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, // addr 1
            0x11, 0x12, 0x13, 0x14, 0x15, 0x16, // addr 2
            0x01, 0x02, // R1, R2
            0x00, 0x00, 0x00, 0x00, // reserved
            0x20, 0x04, 0x00, 0x30, 0x05, 0x01, // class of device
            0x34, 0x12, 0x78, 0x56, // clock offsets
        ];
        let (inquiry_result, _) = InquiryResult::from_hci_bytes(&data).unwrap();

        let mut iter = inquiry_result.iter();

        let item1 = iter.next().unwrap();
        assert_eq!(item1.bd_addr.as_hci_bytes(), &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06]);
        assert_eq!(item1.page_scan_repetition_mode, Some(PageScanRepetitionMode::R1));
        assert_eq!(item1.class_of_device, Some([0x20, 0x04, 0x00]));
        assert_eq!(item1.clock_offset.unwrap().as_hci_bytes(), &[0x34, 0x12]);
        assert_eq!(item1.rssi, None);

        let item2 = iter.next().unwrap();
        assert_eq!(item2.bd_addr.as_hci_bytes(), &[0x11, 0x12, 0x13, 0x14, 0x15, 0x16]);
        assert_eq!(item2.page_scan_repetition_mode, Some(PageScanRepetitionMode::R2));
        assert_eq!(item2.class_of_device, Some([0x30, 0x05, 0x01]));
        assert_eq!(item2.clock_offset.unwrap().as_hci_bytes(), &[0x78, 0x56]);
        assert_eq!(item2.rssi, None);
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_inquiry_result_with_rssi() {
        let data = [
            0x02, // num_responses = 2
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, // addr 1
            0x11, 0x12, 0x13, 0x14, 0x15, 0x16, // addr 2
            0x01, 0x02, // R1, R2
            0x00, 0x00, // reserved
            0x20, 0x04, 0x00, 0x30, 0x05, 0x01, // class of device
            0x34, 0x12, 0x78, 0x56, // clock offsets
            0xF0, 0xE8, // RSSI
        ];
        let (inquiry_result, _) = InquiryResultWithRssi::from_hci_bytes(&data).unwrap();

        let mut iter = inquiry_result.iter();

        let item1 = iter.next().unwrap();
        assert_eq!(item1.bd_addr.as_hci_bytes(), &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06]);
        assert_eq!(item1.page_scan_repetition_mode, Some(PageScanRepetitionMode::R1));
        assert_eq!(item1.class_of_device, Some([0x20, 0x04, 0x00]));
        assert_eq!(item1.clock_offset.unwrap().as_hci_bytes(), &[0x34, 0x12]);
        assert_eq!(item1.rssi, Some(-16));

        let item2 = iter.next().unwrap();
        assert_eq!(item2.bd_addr.as_hci_bytes(), &[0x11, 0x12, 0x13, 0x14, 0x15, 0x16]);
        assert_eq!(item2.page_scan_repetition_mode, Some(PageScanRepetitionMode::R2));
        assert_eq!(item2.class_of_device, Some([0x30, 0x05, 0x01]));
        assert_eq!(item2.clock_offset.unwrap().as_hci_bytes(), &[0x78, 0x56]);
        assert_eq!(item2.rssi, Some(-24));
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_extended_inquiry_result() {
        let data = [
            0x01, // num_responses = 1
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, // bd_addr
            0x01, // page_scan_repetition_mode (R1)
            0x00, // reserved
            0x20, 0x04, 0x00, // class_of_device
            0x34, 0x12, // clock_offset
            0xF0, // rssi (-16)
            0x09, 0x08, 0x54, 0x65, 0x73, 0x74, 0x20, 0x44, 0x65, 0x76, // eir_data (sample EIR data)
        ];
        let (eir_result, _) = ExtendedInquiryResult::from_hci_bytes(&data).unwrap();

        assert_eq!(eir_result.num_responses, 1);
        assert_eq!(eir_result.bd_addr.as_hci_bytes(), &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06]);
        assert_eq!(eir_result.page_scan_repetition_mode, PageScanRepetitionMode::R1);
        assert_eq!(eir_result.reserved, 0x00);
        assert_eq!(eir_result.class_of_device, [0x20, 0x04, 0x00]);
        assert_eq!(eir_result.clock_offset.as_hci_bytes(), &[0x34, 0x12]);
        assert_eq!(eir_result.rssi, -16);
        assert_eq!(
            eir_result.eir_data.as_hci_bytes(),
            &[0x09, 0x08, 0x54, 0x65, 0x73, 0x74, 0x20, 0x44, 0x65, 0x76]
        );
    }

    #[test]
    fn test_io_capability_request() {
        let data = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
        let (evt, rest) = IoCapabilityRequest::from_hci_bytes(&data).unwrap();
        assert_eq!(evt.bd_addr.raw(), [1, 2, 3, 4, 5, 6]);
        assert!(rest.is_empty());
    }

    #[test]
    fn test_user_confirmation_request() {
        let data = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x78, 0x56, 0x34, 0x12];
        let (evt, rest) = UserConfirmationRequest::from_hci_bytes(&data).unwrap();
        assert_eq!(evt.bd_addr.raw(), [1, 2, 3, 4, 5, 6]);
        assert_eq!(evt.numeric_value, 0x1234_5678);
        assert!(rest.is_empty());
    }

    #[test]
    fn test_connection_request() {
        let data = [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, // bd_addr
            0x20, 0x04, 0x00, // class_of_device
            0x01, // link_type (ACL)
        ];
        let (evt, rest) = ConnectionRequest::from_hci_bytes(&data).unwrap();
        assert_eq!(evt.bd_addr.raw(), [1, 2, 3, 4, 5, 6]);
        assert_eq!(evt.class_of_device, [0x20, 0x04, 0x00]);
        assert_eq!(evt.link_type, ConnectionLinkType::Acl);
        assert!(rest.is_empty());
    }

    #[test]
    fn test_role_change() {
        let data = [
            0x00, // status (success)
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, // bd_addr
            0x00, // new_role (Central)
        ];
        let (evt, rest) = RoleChange::from_hci_bytes(&data).unwrap();
        assert_eq!(evt.status, Status::SUCCESS);
        assert_eq!(evt.bd_addr.raw(), [1, 2, 3, 4, 5, 6]);
        assert_eq!(evt.new_role, Role::Central);
        assert!(rest.is_empty());
    }

    #[test]
    fn test_simple_pairing_complete() {
        let data = [
            0x00, // status (success)
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, // bd_addr
        ];
        let (evt, rest) = SimplePairingComplete::from_hci_bytes(&data).unwrap();
        assert_eq!(evt.status, Status::SUCCESS);
        assert_eq!(evt.bd_addr.raw(), [1, 2, 3, 4, 5, 6]);
        assert!(rest.is_empty());
    }

    #[test]
    fn test_io_capability_response() {
        let data = [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, // bd_addr
            0x01, // io_capability (DisplayYesNo)
            0x00, // oob_data_present (not present)
            0x03, // authentication_requirements (MITM Protection Required - General Bonding)
        ];
        let (evt, rest) = IoCapabilityResponse::from_hci_bytes(&data).unwrap();
        assert_eq!(evt.bd_addr.raw(), [1, 2, 3, 4, 5, 6]);
        assert_eq!(evt.io_capability, IoCapability::DisplayYesNo);
        assert_eq!(evt.oob_data_present, OobDataPresent::NotPresent);
        assert_eq!(
            evt.authentication_requirements,
            AuthenticationRequirements::MitmRequiredDedicatedBonding
        );
        assert!(rest.is_empty());
    }

    #[test]
    fn test_number_of_completed_data_blocks() {
        let data = [
            0x00, 0x10, // total_num_data_blocks = 4096
            0x02, // num_of_handles = 2
            0x01, 0x00, // handle 1
            0x02, 0x00, // num_completed_packets for handle 1
            0x04, 0x00, // num_completed_blocks for handle 1
            0x02, 0x00, // handle 2
            0x01, 0x00, // num_completed_packets for handle 2
            0x02, 0x00, // num_completed_blocks for handle 2
        ];
        let (evt, rest) = NumberOfCompletedDataBlocks::from_hci_bytes(&data).unwrap();
        assert_eq!(evt.total_num_data_blocks, 4096);
        assert_eq!(evt.num_of_handles, 2);
        assert_eq!(evt.bytes.as_hci_bytes().len(), 12); // 2 handles * 6 bytes each
        assert!(rest.is_empty());
    }

    #[test]
    fn test_connectionless_peripheral_broadcast_receive() {
        let data = [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, // bd_addr
            0x07, // lt_addr
            0x12, 0x34, 0x56, 0x78, // clock
            0x9A, 0xBC, 0xDE, 0xF0, // offset
            0x00, // rx_status (success)
            0x01, // fragment
            0x04, // data_length
            0xAA, 0xBB, 0xCC, 0xDD, // data
        ];
        let (evt, rest) = ConnectionlessPeripheralBroadcastReceive::from_hci_bytes(&data).unwrap();
        assert_eq!(evt.bd_addr.raw(), [1, 2, 3, 4, 5, 6]);
        assert_eq!(evt.lt_addr, 0x07);
        assert_eq!(evt.clock, 0x78563412);
        assert_eq!(evt.offset, 0xF0DEBC9A);
        assert_eq!(evt.rx_status, 0x00);
        assert_eq!(evt.fragment, 0x01);
        assert_eq!(evt.data_length, 0x04);
        assert_eq!(evt.data.as_hci_bytes(), &[0xAA, 0xBB, 0xCC, 0xDD]);
        assert!(rest.is_empty());
    }

    #[test]
    fn test_connectionless_peripheral_broadcast_timeout() {
        let data = [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, // bd_addr
            0x07, // lt_addr
        ];
        let (evt, rest) = ConnectionlessPeripheralBroadcastTimeout::from_hci_bytes(&data).unwrap();
        assert_eq!(evt.bd_addr.raw(), [1, 2, 3, 4, 5, 6]);
        assert_eq!(evt.lt_addr, 0x07);
        assert!(rest.is_empty());
    }

    #[test]
    fn test_truncated_page_complete() {
        let data = [
            0x00, // status (success)
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, // bd_addr
        ];
        let (evt, rest) = TruncatedPageComplete::from_hci_bytes(&data).unwrap();
        assert_eq!(evt.status, Status::SUCCESS);
        assert_eq!(evt.bd_addr.raw(), [1, 2, 3, 4, 5, 6]);
        assert!(rest.is_empty());
    }

    #[test]
    fn test_peripheral_page_response_timeout() {
        let data = [];
        let (_evt, rest) = PeripheralPageResponseTimeout::from_hci_bytes(&data).unwrap();
        assert!(rest.is_empty());
    }

    #[test]
    fn test_connectionless_peripheral_broadcast_channel_map_change() {
        let data = [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, // channel_map
        ];
        let (evt, rest) = ConnectionlessPeripheralBroadcastChannelMapChange::from_hci_bytes(&data).unwrap();
        assert_eq!(
            evt.channel_map,
            [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A]
        );
        assert!(rest.is_empty());
    }

    #[test]
    fn test_sam_status_change() {
        let data = [
            0x01, 0x00, // handle
            0x02, // local_sam_index
            0x03, // local_sam_tx_availability
            0x04, // local_sam_rx_availability
            0x05, // remote_sam_index
            0x06, // remote_sam_tx_availability
            0x07, // remote_sam_rx_availability
        ];
        let (evt, rest) = SamStatusChange::from_hci_bytes(&data).unwrap();
        assert_eq!(evt.handle.raw(), 1);
        assert_eq!(evt.local_sam_index, 0x02);
        assert_eq!(evt.local_sam_tx_availability, 0x03);
        assert_eq!(evt.local_sam_rx_availability, 0x04);
        assert_eq!(evt.remote_sam_index, 0x05);
        assert_eq!(evt.remote_sam_tx_availability, 0x06);
        assert_eq!(evt.remote_sam_rx_availability, 0x07);
        assert!(rest.is_empty());
    }

    #[test]
    fn convert_error_packet() {
        let data = [
            0x04, 10, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, // bd_addr
            0x20, 0x04, 0x00, // class_of_device
            0x01, // link_type (ACL)
        ];
        let event = EventPacket::from_hci_bytes_complete(&data).unwrap();
        assert!(matches!(event.kind, EventKind::ConnectionRequest));

        let Event::ConnectionRequest(evt) = Event::try_from(event).unwrap() else {
            unreachable!()
        };

        assert_eq!(evt.bd_addr.raw(), [1, 2, 3, 4, 5, 6]);
        assert_eq!(evt.class_of_device, [0x20, 0x04, 0x00]);
        assert_eq!(evt.link_type, ConnectionLinkType::Acl);
    }

    #[test]
    fn convert_le_error_packet() {
        let data = [
            0x3e, 19, // header
            1,  // subevent
            0,  // success
            1, 0, // handle
            0, // role
            1, // kind
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, // bd_addr
            0x10, 0x10, // interval
            0x00, 0x00, // latency
            0x10, 0x10, // supervision timeout
            1,    // accuracy
        ];
        let event = EventPacket::from_hci_bytes_complete(&data).unwrap();
        assert!(matches!(event.kind, EventKind::Le));

        let Event::Le(LeEvent::LeConnectionComplete(e)) = Event::try_from(event).unwrap() else {
            unreachable!()
        };

        assert_eq!(e.status, Status::SUCCESS);
        assert_eq!(e.handle, ConnHandle::new(1));
        assert!(matches!(e.central_clock_accuracy, ClockAccuracy::Ppm250));
    }

    #[test]
    fn parse_le_packet() {
        let data = [
            0x3e, 19, // header
            1,  // subevent
            0,  // success
            1, 0, // handle
            0, // role
            1, // kind
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, // bd_addr
            0x10, 0x10, // interval
            0x00, 0x00, // latency
            0x10, 0x10, // supervision timeout
            1,    // accuracy
        ];
        let event = EventPacket::from_hci_bytes_complete(&data).unwrap();
        assert!(matches!(event.kind, EventKind::Le));
        let event = LeEventPacket::from_hci_bytes_complete(event.data).unwrap();
        assert!(matches!(
            event.kind,
            crate::event::le::LeEventKind::LeConnectionComplete
        ));
        let e = crate::event::le::LeConnectionComplete::from_hci_bytes_complete(event.data).unwrap();

        assert_eq!(e.status, Status::SUCCESS);
        assert_eq!(e.handle, ConnHandle::new(1));
        assert!(matches!(e.central_clock_accuracy, ClockAccuracy::Ppm250));
    }

    #[test]
    fn test_special_command_complete() {
        let data = [
            0x0e, 3, // header
            1, 0, 0, // special command
        ];

        let event = EventPacket::from_hci_bytes_complete(&data).unwrap();
        assert!(matches!(event.kind, EventKind::CommandComplete));
        let event = CommandComplete::from_hci_bytes_complete(event.data).unwrap();
        assert_eq!(event.cmd_opcode, Opcode::new(OpcodeGroup::new(0), 0));
    }

    #[test]
    fn test_normal_command_complete() {
        let opcode = Opcode::new(OpcodeGroup::LE, 0x000D).to_raw().to_le_bytes();
        let data = [
            0x0e, 4, // header
            1, opcode[0], opcode[1], // special command
            0,         // success
        ];

        let event = EventPacket::from_hci_bytes_complete(&data).unwrap();
        assert!(matches!(event.kind, EventKind::CommandComplete));
        let event = CommandComplete::from_hci_bytes_complete(event.data).unwrap();
        assert_eq!(event.cmd_opcode, Opcode::new(OpcodeGroup::LE, 0x000d));

        let event: CommandCompleteWithStatus = event.try_into().unwrap();
        assert_eq!(event.cmd_opcode, Opcode::new(OpcodeGroup::LE, 0x000d));
        assert_eq!(Status::SUCCESS, event.status);
    }
}
