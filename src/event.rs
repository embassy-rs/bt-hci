use crate::cmd::{Opcode, SyncCmd};
use crate::param::{
    param, ConnHandle, ConnHandleCompletedPackets, CoreSpecificationVersion, DisconnectReason, LinkType,
    RemainingBytes, Status,
};
use crate::{FromHciBytes, FromHciBytesError, ReadHci, ReadHciError};

pub trait EventParams<'a>: FromHciBytes<'a> {
    const CODE: u8;
}

param! {
    struct EventPacketHeader {
        code: u8,
        params_len: u8,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct EventPacket<'a> {
    code: u8,
    params: &'a [u8],
}

impl<'a> EventPacket<'a> {
    pub fn from_header_hci_bytes(header: EventPacketHeader, data: &'a [u8]) -> Result<Self, FromHciBytesError> {
        let params_len = usize::from(header.params_len);
        if data.len() != params_len {
            Err(FromHciBytesError::InvalidSize)
        } else {
            Ok(Self {
                code: header.code,
                params: data,
            })
        }
    }

    pub fn code(&self) -> u8 {
        self.code
    }

    pub fn param_bytes(&self) -> &[u8] {
        self.params
    }

    pub fn params<P: EventParams<'a>>(&self) -> Result<P, FromHciBytesError> {
        assert_eq!(self.code, P::CODE);
        match P::from_hci_bytes(self.params) {
            Ok((val, rest)) if rest.is_empty() => Ok(val),
            Ok(_) => Err(FromHciBytesError::InvalidSize),
            Err(err) => Err(err),
        }
    }
}

impl<'de> FromHciBytes<'de> for EventPacket<'de> {
    fn from_hci_bytes(data: &'de [u8]) -> Result<(Self, &'de [u8]), FromHciBytesError> {
        let (header, data) = EventPacketHeader::from_hci_bytes(data)?;
        Self::from_header_hci_bytes(header, data).map(|x| (x, &[] as &[u8]))
    }
}

impl<'de> ReadHci<'de> for EventPacket<'de> {
    fn read_hci<R: embedded_io::blocking::Read>(
        mut reader: R,
        buf: &'de mut [u8],
    ) -> Result<Self, ReadHciError<R::Error>> {
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

    #[cfg(feature = "async")]
    async fn read_hci_async<R: embedded_io::asynch::Read>(
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

macro_rules! event {
    (
        $(#[$attrs:meta])*
        struct $name:ident$(<$life:lifetime>)?($code:expr) {
            $($field:ident: $ty:ty),*
            $(,)?
        }
    ) => {
        $(#[$attrs])*
        #[derive(Debug, Clone, Copy, Hash)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub struct $name$(<$life>)? {
            pub $($field: $ty,)*
        }

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
        #[allow(unused_mut, unused_variables, unused_imports)]
        impl<'a> $crate::event::EventParams<'a> for $name$(<$life>)? {
            const CODE: u8 = $code;
        }
    };
}

pub(crate) use event;

event! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.7.5
    struct DisconnectionComplete(0x05) {
        status: Status,
        handle: ConnHandle,
        reasdon: DisconnectReason,
    }
}

event! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.7.8
    struct EncryptionChangeV1(0x08) {
        status: Status,
        handle: ConnHandle,
        enabled: bool,
    }
}

event! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.7.8
    struct EncryptionChangeV2(0x59) {
        status: Status,
        handle: ConnHandle,
        encryption_enabled: bool,
        encryption_key_size: u8,
    }
}

event! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.7.12
    struct ReadRemoteVersionInformationComplete(0x0c) {
        status: Status,
        handle: ConnHandle,
        version: CoreSpecificationVersion,
        company_id: u16,
        subversion: u16,
    }
}

event! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.7.14
    struct CommandComplete<'a>(0x0e) {
        num_hci_cmd_pkts: u8,
        cmd_opcode: Opcode,
        return_param_bytes: RemainingBytes<'a>,
    }
}

impl<'a> CommandComplete<'a> {
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

event! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.7.15
    struct CommandStatus(0x0f) {
        status: Status,
        num_hci_cmd_pkts: u8,
        cmd_opcode: Opcode,
    }
}

event! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.7.16
    struct HardwareError(0x10) {
        hardware_code: u8,
    }
}

event! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.7.19
    struct NumberOfCompletedPackets<'a>(0x13) {
        completed_packets: &'a [ConnHandleCompletedPackets],
    }
}

event! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.7.26
    struct DataBufferOverflow(0x1a) {
        link_type: LinkType,
    }
}

event! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.7.39
    struct EncryptionKeyRefreshComplete(0x30) {
        status: Status,
        handle: ConnHandle,
    }
}

event! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.7.75
    struct AuthenticatedPayloadTimeoutExpired(0x57) {
        handle: ConnHandle,
    }
}

event! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.7.65
    struct LeMeta<'a>(0x3e) {
        subevent_code: u8,
        subevent: RemainingBytes<'a>,
    }
}

// 7.7.65.1 LeConnectionComplete
// 7.7.65.2 LeAdvertisingReport
// 7.7.65.3 LeConnectionUpdateComplete
// 7.7.65.4 LeReadRemoteFeaturesComplete
// 7.7.65.5 LeLongTermKeyRequest
// 7.7.65.6 LeRemoteConnectionParameterRequest
// 7.7.65.7 LeDataLengthChange
// 7.7.65.8 LeReadLocalP256PublicKeyComplete
// 7.7.65.9 LeGenerateDhkeyComplete
// 7.7.65.10 LeEnhancedConnectionComplete
// 7.7.65.11 LeDirectedAdvertisingReport
// 7.7.65.12 LePhyUpdateComplete
// 7.7.65.13 LeExtendedAdvertisingReport
// 7.7.65.14 LePeriodicAdvertisingSyncEstablished
// 7.7.65.15 LePeriodicAdvertisingReport
// 7.7.65.16 LePeriodicAdvertisingSyncLost
// 7.7.65.17 LeScanTimeout
// 7.7.65.18 LeAdvertisingSetTerminated
// 7.7.65.19 LeScanRequestReceived
// 7.7.65.20 LeChannelSelectionAlgorithm
// 7.7.65.21 LeConnectionlessIqReport
// 7.7.65.22 LeConnectionIqReport
// 7.7.65.23 LeCteRequestFailed
// 7.7.65.24 LePeriodicAdvertisingSyncTransferReceived
// 7.7.65.25 LeCisEstablished
// 7.7.65.26 LeCisRequest
// 7.7.65.27 LeCreateBigComplete
// 7.7.65.28 LeTerminateBigComplete
// 7.7.65.29 LeBigSyncEstablished
// 7.7.65.30 LeBigSyncLost
// 7.7.65.31 LeRequestPeerScaComplete
// 7.7.65.32 LePathLossThreshold
// 7.7.65.33 LeTransmitPowerReporting
// 7.7.65.34 LeBiginfoAdvertisingReport
// 7.7.65.35 LeSubrateChange
