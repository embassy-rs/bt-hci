//! HCI events [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-d21276b6-83d0-cbc3-8295-6ff23b70a0c5)

use crate::cmd::{Opcode, SyncCmd};
use crate::param::{
    param, BdAddr, ClockOffset, ConnHandle, ConnHandleCompletedPackets, ConnectionLinkType, CoreSpecificationVersion,
    Error, LinkKeyType, LinkType, LmpFeatureMask, PageScanRepetitionMode, RemainingBytes, Status,
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
        enabled: bool,
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

    // 0x10 - 0x1f

    /// Hardware Error event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-2479a101-ae3b-5b5d-f3d4-4776af39a377)
    struct HardwareError(0x10) {
        hardware_code: u8,
    }

    /// Number Of Completed Packets event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-9ccbff85-45ce-9c0d-6d0c-2e6e5af52b0e)
    struct NumberOfCompletedPackets<'a>(0x13) {
        completed_packets: &'a [ConnHandleCompletedPackets],
    }

    /// Return Link Keys event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-e0d451f2-4b53-cdf0-efb5-926e08b27cd2)
    struct ReturnLinkKeys<'a>(0x15) {
        num_keys: u8,
        bd_addr: RemainingBytes<'a>, // Num_Keys × 6 octets
        link_key: RemainingBytes<'a>, // Num_Keys × 16 octets, always zero
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

    /// Data Buffer Overflow event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-e15e12c7-d29a-8c25-349f-af6206c2ae57)
    struct DataBufferOverflow(0x1a) {
        link_type: LinkType,
    }

    // 0x20 - 0x2f

    /// Inquiry Result with RSSI event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-c2550565-1c65-a514-6cf0-3d55c8943dab)
    struct InquiryResultWithRssi<'a>(0x22) {
        num_responses: u8,
        /// All remaining bytes for this event (contains all fields for all responses)
        bytes: RemainingBytes<'a>,
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

    /// User Confirmation Request event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-e7014a9e-718f-6aa8-d657-35b547e9c5d6)
    struct UserConfirmationRequest(0x33) {
        bd_addr: BdAddr,
        numeric_value: u32,
    }

    /// Remote Host Supported Features Notification event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-ba740ba0-44d8-d028-0a67-1abab648f6dd)
    struct RemoteHostSupportedFeaturesNotification(0x3d) {
        bd_addr: BdAddr,
        features: LmpFeatureMask,
    }

    // 0x40 - 0x4f

    // 0x50 - 0x5f

    /// Authenticated Payload Timeout Expired event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-6cfdff94-ace8-294c-6af9-d90d94653e19)
    struct AuthenticatedPayloadTimeoutExpired(0x57) {
        handle: ConnHandle,
    }

    /// Encryption Change (v2) event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-7b7d27f0-1a33-ff57-5b97-7d49a04cea26)
    struct EncryptionChangeV2(0x59) {
        status: Status,
        handle: ConnHandle,
        encryption_enabled: bool,
        encryption_key_size: u8,
    }

    // 0x60 - 0x6f

    // 0x70 - 0x7f

    // 0x80 - 0x8f

    // 0x90 - 0x9f

    // 0xa0 - 0xaf

    // 0xb0 - 0xbf

    // 0xc0 - 0xcf

    // 0xd0 - 0xdf

    // 0xe0 - 0xef

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
    pub fn iter(&self) -> InquiryResultIter {
        let bytes = self.bytes.as_hci_bytes();
        let n = self.num_responses as usize;
        InquiryResultIter::new_standard(bytes, n)
    }
}

/// Inquiry result event containing multiple responses with RSSI
impl InquiryResultWithRssi<'_> {
    /// Returns an iterator over all valid inquiry result items.
    pub fn iter(&self) -> InquiryResultIter {
        let bytes = self.bytes.as_hci_bytes();
        let n = self.num_responses as usize;
        InquiryResultIter::new_with_rssi(bytes, n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::param::PageScanRepetitionMode;

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
}
