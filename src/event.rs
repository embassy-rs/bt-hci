//! HCI events [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-d21276b6-83d0-cbc3-8295-6ff23b70a0c5)

use crate::cmd::{Opcode, SyncCmd};
use crate::param::{
    param, BdAddr, ConnHandle, ConnHandleCompletedPackets, CoreSpecificationVersion, Error, LinkType, RemainingBytes,
    Status,
};
use crate::{FromHciBytes, FromHciBytesError, ReadHci, ReadHciError};

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
            /// $name
            pub struct $name$(<$life>)? {
                $(
                    /// $field
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
    /// Disconnection Complete event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-332adb1f-b5ac-5289-82a2-c51a59d533e7)
    struct DisconnectionComplete(0x05) {
        status: Status,
        handle: ConnHandle,
        reason: Status,
    }

    /// Inquiry Result event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-3467df70-1d7a-73c5-5a3e-8689dba5523f)
    struct InquiryResult<'a>(0x02) {
        num_responses: u8,
        bd_addr: RemainingBytes<'a>,
        page_scan_repetition_mode: RemainingBytes<'a>,
        reserved: RemainingBytes<'a>,
        class_of_device: RemainingBytes<'a>,
        clock_offset: RemainingBytes<'a>,
    }

    /// Extended Inquiry Result event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-e3e8c7bc-2262-14f4-b6f5-2eeb0b25aa4f)
    struct ExtendedInquiryResult<'a>(0x2f) {
        num_responses: u8,
        bd_addr: [u8; 6],
        page_scan_repetition_mode: u8,
        reserved: u8,
        class_of_device: [u8; 3],
        clock_offset: u16,
        rssi: i8,
        eir_data: RemainingBytes<'a>,
    }

    /// Inquiry Result with RSSI event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-c2550565-1c65-a514-6cf0-3d55c8943dab)
    struct InquiryResultWithRssi<'a>(0x22) {
        num_responses: u8,
        bd_addr: RemainingBytes<'a>,
        page_scan_repetition_mode: RemainingBytes<'a>,
        reserved: RemainingBytes<'a>,
        class_of_device: RemainingBytes<'a>,
        clock_offset: RemainingBytes<'a>,
        rssi: RemainingBytes<'a>,
    }

    /// Encryption Change (v1) event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-7b7d27f0-1a33-ff57-5b97-7d49a04cea26)
    struct EncryptionChangeV1(0x08) {
        status: Status,
        handle: ConnHandle,
        enabled: bool,
    }

    /// Encryption Change (v2) event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-7b7d27f0-1a33-ff57-5b97-7d49a04cea26)
    struct EncryptionChangeV2(0x59) {
        status: Status,
        handle: ConnHandle,
        encryption_enabled: bool,
        encryption_key_size: u8,
    }

    /// Read Remote Version Information Complete event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-81ed98a1-98b1-dae5-a3f5-bb7bc69d39b7)
    struct ReadRemoteVersionInformationComplete(0x0c) {
        status: Status,
        handle: ConnHandle,
        version: CoreSpecificationVersion,
        company_id: u16,
        subversion: u16,
    }

    /// Command Complete event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-76d31a33-1a9e-07bc-87c4-8ebffee065fd)
    struct CommandComplete<'a>(0x0e) {
        num_hci_cmd_pkts: u8,
        cmd_opcode: Opcode,
        status: Status, // All return parameters have status as the first field
        return_param_bytes: RemainingBytes<'a>,
    }

    /// Command Status event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-4d87067c-be74-d2ff-d5c4-86416bf7af91)
    struct CommandStatus(0x0f) {
        status: Status,
        num_hci_cmd_pkts: u8,
        cmd_opcode: Opcode,
    }

    /// Hardware Error event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-2479a101-ae3b-5b5d-f3d4-4776af39a377)
    struct HardwareError(0x10) {
        hardware_code: u8,
    }

    /// Number Of Completed Packets event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-9ccbff85-45ce-9c0d-6d0c-2e6e5af52b0e)
    struct NumberOfCompletedPackets<'a>(0x13) {
        completed_packets: &'a [ConnHandleCompletedPackets],
    }

    /// Data Buffer Overflow event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-e15e12c7-d29a-8c25-349f-af6206c2ae57)
    struct DataBufferOverflow(0x1a) {
        link_type: LinkType,
    }

    /// Encryption Key Refresh Complete event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-a321123c-83a5-7baf-6971-05edd1241357)
    struct EncryptionKeyRefreshComplete(0x30) {
        status: Status,
        handle: ConnHandle,
    }

    /// Authenticated Payload Timeout Expired event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-6cfdff94-ace8-294c-6af9-d90d94653e19)
    struct AuthenticatedPayloadTimeoutExpired(0x57) {
        handle: ConnHandle,
    }

    /// Return Link Keys event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-e0d451f2-4b53-cdf0-efb5-926e08b27cd2)
    struct ReturnLinkKeys<'a>(0x15) {
        num_keys: u8,
        bd_addr: RemainingBytes<'a>, // Num_Keys × 6 octets
        link_key: RemainingBytes<'a>, // Num_Keys × 16 octets, always zero
    }

    /// HCI Event packet [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-f209cdf7-0496-8bcd-b7e1-500831511378)
    struct Vendor<'a>(0xff) {
        params: RemainingBytes<'a>,
    }

    /// Remote Name Request Complete event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-1e2ccd32-b73f-7f00-f4ff-25b2235aaf02)
    struct RemoteNameRequestComplete<'a>(0x07) {
        status: Status,
        bd_addr: BdAddr,
        remote_name: RemainingBytes<'a>, // 248 bytes max, null-terminated string
    }

    /// Remote Host Supported Features Notification event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-ba740ba0-44d8-d028-0a67-1abab648f6dd)
    struct RemoteHostSupportedFeaturesNotification(0x3d) {
        bd_addr: BdAddr,
        features: [u8; 8],
    }

    /// Connection Complete event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-ebb06dd7-356e-605c-cbc1-d06dc00f1d2b)
    struct ConnectionComplete(0x03) {
        status: Status,
        handle: ConnHandle,
        bd_addr: BdAddr,
        link_type: u8,
        encryption_enabled: u8,
    }

    /// Link Key Request event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-58400663-f69d-a482-13af-ec558a3f4c03)
    struct LinkKeyRequest(0x17) {
        bd_addr: BdAddr,
    }

    /// PIN Code Request event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-7666987c-9040-9aaa-cad6-96941b46d2b5)
    struct PinCodeRequest(0x16) {
        bd_addr: BdAddr,
    }

    /// Link Key Notification event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-ef6b7301-ab4b-cce6-1fa4-c053c3cd1585)
    struct LinkKeyNotification(0x18) {
        bd_addr: BdAddr,
        link_key: [u8; 16],
        key_type: u8,
    }

    /// Authentication Complete event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-00ede464-d351-7076-e82a-d8f4b30ee594)
    struct AuthenticationComplete(0x06) {
        status: Status,
        handle: ConnHandle,
    }
}

impl<'de> FromHciBytes<'de> for Event<'de> {
    fn from_hci_bytes(data: &'de [u8]) -> Result<(Self, &'de [u8]), FromHciBytesError> {
        let (header, data) = EventPacketHeader::from_hci_bytes(data)?;
        Self::from_header_hci_bytes(header, data)
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
