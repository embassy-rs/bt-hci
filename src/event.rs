//! Bluetooth HCI event packets.
//!
//! See Bluetooth Core Specification Vol 4, Part E, §7.7.

use crate::cmd::{Opcode, SyncCmd};
use crate::param::{
    param, ConnHandle, ConnHandleCompletedPackets, CoreSpecificationVersion, DisconnectReason, Error, LinkType,
    RemainingBytes, Status,
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
            fn from_header_hci_bytes(header: EventPacketHeader, data: &'a [u8]) -> Result<Self, FromHciBytesError> {
                match header.code {
                    $($code => $name::from_hci_bytes_complete(data).map(Self::$name),)+
                    0x3e => LeEvent::from_hci_bytes_complete(data).map(Self::Le),
                    _ => Ok(Self::Unknown { code: header.code, params: data }),
                }
            }
        }

        $(
            $(#[$attrs])*
            #[derive(Debug, Clone, Copy, Hash)]
            #[cfg_attr(feature = "defmt", derive(defmt::Format))]
            pub struct $name$(<$life>)? {
                $(
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
    /// Bluetooth Core Specification Vol 4, Part E, §7.7.5
    struct DisconnectionComplete(0x05) {
        status: Status,
        handle: ConnHandle,
        reasdon: DisconnectReason,
    }

    /// Bluetooth Core Specification Vol 4, Part E, §7.7.8
    struct EncryptionChangeV1(0x08) {
        status: Status,
        handle: ConnHandle,
        enabled: bool,
    }

    /// Bluetooth Core Specification Vol 4, Part E, §7.7.8
    struct EncryptionChangeV2(0x59) {
        status: Status,
        handle: ConnHandle,
        encryption_enabled: bool,
        encryption_key_size: u8,
    }

    /// Bluetooth Core Specification Vol 4, Part E, §7.7.12
    struct ReadRemoteVersionInformationComplete(0x0c) {
        status: Status,
        handle: ConnHandle,
        version: CoreSpecificationVersion,
        company_id: u16,
        subversion: u16,
    }

    /// Bluetooth Core Specification Vol 4, Part E, §7.7.14
    struct CommandComplete<'a>(0x0e) {
        num_hci_cmd_pkts: u8,
        cmd_opcode: Opcode,
        status: Status, // All return parameters have status as the first field
        return_param_bytes: RemainingBytes<'a>,
    }

    /// Bluetooth Core Specification Vol 4, Part E, §7.7.15
    struct CommandStatus(0x0f) {
        status: Status,
        num_hci_cmd_pkts: u8,
        cmd_opcode: Opcode,
    }

    /// Bluetooth Core Specification Vol 4, Part E, §7.7.16
    struct HardwareError(0x10) {
        hardware_code: u8,
    }

    /// Bluetooth Core Specification Vol 4, Part E, §7.7.19
    struct NumberOfCompletedPackets<'a>(0x13) {
        completed_packets: &'a [ConnHandleCompletedPackets],
    }

    /// Bluetooth Core Specification Vol 4, Part E, §7.7.26
    struct DataBufferOverflow(0x1a) {
        link_type: LinkType,
    }

    /// Bluetooth Core Specification Vol 4, Part E, §7.7.39
    struct EncryptionKeyRefreshComplete(0x30) {
        status: Status,
        handle: ConnHandle,
    }

    /// Bluetooth Core Specification Vol 4, Part E, §7.7.75
    struct AuthenticatedPayloadTimeoutExpired(0x57) {
        handle: ConnHandle,
    }

    /// Bluetooth Core Specification Vol 4, Part E, §5.4.4
    struct Vendor<'a>(0xff) {
        params: RemainingBytes<'a>,
    }
}

impl<'de> FromHciBytes<'de> for Event<'de> {
    fn from_hci_bytes(data: &'de [u8]) -> Result<(Self, &'de [u8]), FromHciBytesError> {
        let (header, data) = EventPacketHeader::from_hci_bytes(data)?;
        Self::from_header_hci_bytes(header, data).map(|x| (x, &[] as &[u8]))
    }
}

impl<'de> ReadHci<'de> for Event<'de> {
    fn read_hci<R: embedded_io::Read>(mut reader: R, buf: &'de mut [u8]) -> Result<Self, ReadHciError<R::Error>> {
        let mut header = [0; 4];
        reader.read_exact(&mut header)?;
        let (header, _) = EventPacketHeader::from_hci_bytes(&header)?;
        let params_len = usize::from(header.params_len);
        if buf.len() < params_len {
            Err(ReadHciError::BufferTooSmall)
        } else {
            let (buf, _) = buf.split_at_mut(params_len);
            reader.read_exact(buf)?;
            Self::from_header_hci_bytes(header, buf).map_err(Into::into)
        }
    }

    async fn read_hci_async<R: embedded_io_async::Read>(
        mut reader: R,
        buf: &'de mut [u8],
    ) -> Result<Self, ReadHciError<R::Error>> {
        let mut header = [0; 4];
        reader.read_exact(&mut header).await?;
        let (header, _) = EventPacketHeader::from_hci_bytes(&header)?;
        let params_len = usize::from(header.params_len);
        if buf.len() < params_len {
            Err(ReadHciError::BufferTooSmall)
        } else {
            let (buf, _) = buf.split_at_mut(params_len);
            reader.read_exact(buf).await?;
            Self::from_header_hci_bytes(header, buf).map_err(Into::into)
        }
    }
}

impl<'a> CommandComplete<'a> {
    /// Gets the connection handle associated with the command that has completed.
    ///
    /// For commands that return the connection handle provided as a parameter as
    /// their first return parameter, this will be valid even if `status` is an error.
    pub fn handle<C: SyncCmd>(&self) -> Option<ConnHandle> {
        C::return_handle(&self.return_param_bytes)
    }

    /// Gets a result with the return parameters for `C` or an `Error` if `status` is
    /// an error.
    ///
    /// # Panics
    ///
    /// May panic if `C::OPCODE` is not equal to `self.cmd_opcode`.
    pub fn to_result<C: SyncCmd>(&self) -> Result<C::Return<'_>, Error> {
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
    pub fn return_params<C: SyncCmd>(&self) -> Result<C::Return<'_>, FromHciBytesError> {
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
