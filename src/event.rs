//! HCI events [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-d21276b6-83d0-cbc3-8295-6ff23b70a0c5)

use crate::cmd::{Opcode, SyncCmd};
use crate::param::{param, BdAddr, ClockOffset, Error, PageScanRepetitionMode, RemainingBytes, Status};
use crate::{FromHciBytes, FromHciBytesError, ReadHci, ReadHciError};

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
            pub const Le: EventKind = EventKind(0x3e);
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

                Ok((EventKind(header.code), data))
            }
        }

        impl<'a> TryFrom<EventPacket<'a>> for Event<'a> {
            type Error = FromHciBytesError;
            fn try_from(packet: EventPacket<'a>) -> Result<Self, Self::Error> {
                match packet.kind {
                    $(EventKind::$name => Ok(Self::$name($name::from_hci_bytes_complete(packet.data)?)),)+
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
