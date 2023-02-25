#![no_std]
#![cfg_attr(feature = "async", feature(async_fn_in_trait))]
#![cfg_attr(feature = "async", feature(impl_trait_projections))]
#![cfg_attr(feature = "async", allow(incomplete_features))]

use embedded_io::blocking::ReadExactError;

mod fmt;

pub mod cmd;
pub mod data;
pub mod event;
pub mod param;

pub enum FromHciBytesError {
    InvalidSize,
    InvalidValue,
}

pub trait FromHciBytes<'de>: Sized {
    fn from_hci_bytes(data: &'de [u8]) -> Result<(Self, &'de [u8]), FromHciBytesError>;
}

pub enum ReadHciError<E: embedded_io::Error> {
    BufferTooSmall,
    InvalidValue,
    Read(ReadExactError<E>),
}

impl<E: embedded_io::Error> From<ReadExactError<E>> for ReadHciError<E> {
    fn from(value: ReadExactError<E>) -> Self {
        ReadHciError::Read(value)
    }
}

impl<E: embedded_io::Error> From<FromHciBytesError> for ReadHciError<E> {
    fn from(value: FromHciBytesError) -> Self {
        match value {
            FromHciBytesError::InvalidSize => ReadHciError::Read(ReadExactError::UnexpectedEof),
            FromHciBytesError::InvalidValue => ReadHciError::InvalidValue,
        }
    }
}

pub trait ReadHci<'de>: FromHciBytes<'de> {
    fn read_hci<R: embedded_io::blocking::Read>(
        reader: R,
        buf: &'de mut [u8],
    ) -> Result<(Self, &'de mut [u8]), ReadHciError<R::Error>>;

    #[cfg(feature = "async")]
    async fn read_hci_async<R: embedded_io::asynch::Read>(
        reader: R,
        buf: &'de mut [u8],
    ) -> Result<(Self, &'de mut [u8]), ReadHciError<R::Error>>;
}

pub trait WriteHci {
    /// The number of bytes this value will write
    fn size(&self) -> usize;

    fn write_hci<W: embedded_io::blocking::Write>(&self, writer: W) -> Result<(), W::Error>;

    #[cfg(feature = "async")]
    async fn write_hci_async<W: embedded_io::asynch::Write>(&self, writer: W) -> Result<(), W::Error>;
}

pub trait HostToControllerPacket: WriteHci {
    const KIND: PacketKind;
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PacketKind {
    Cmd = 1,
    AclData = 2,
    SyncData = 3,
    Event = 4,
    IsoData = 5,
}

impl<'de> FromHciBytes<'de> for PacketKind {
    fn from_hci_bytes(data: &'de [u8]) -> Result<(Self, &'de [u8]), FromHciBytesError> {
        if data.is_empty() {
            Err(FromHciBytesError::InvalidSize)
        } else {
            let (data, rest) = data.split_at(1);
            match data[0] {
                1 => Ok((PacketKind::Cmd, rest)),
                2 => Ok((PacketKind::AclData, rest)),
                3 => Ok((PacketKind::SyncData, rest)),
                4 => Ok((PacketKind::Event, rest)),
                5 => Ok((PacketKind::IsoData, rest)),
                _ => Err(FromHciBytesError::InvalidValue),
            }
        }
    }
}

impl<'de> ReadHci<'de> for PacketKind {
    fn read_hci<R: embedded_io::blocking::Read>(
        mut reader: R,
        buf: &'de mut [u8],
    ) -> Result<(Self, &'de mut [u8]), ReadHciError<R::Error>> {
        let mut val = [0];
        reader.read_exact(&mut val)?;
        Self::from_hci_bytes(&val).map(|(x, _)| (x, buf)).map_err(Into::into)
    }

    #[cfg(feature = "async")]
    async fn read_hci_async<R: embedded_io::asynch::Read>(
        mut reader: R,
        buf: &'de mut [u8],
    ) -> Result<(Self, &'de mut [u8]), ReadHciError<R::Error>> {
        let mut val = [0];
        reader.read_exact(&mut val).await?;
        Self::from_hci_bytes(&val).map(|(x, _)| (x, buf)).map_err(Into::into)
    }
}

impl WriteHci for PacketKind {
    fn size(&self) -> usize {
        1
    }

    fn write_hci<W: embedded_io::blocking::Write>(&self, mut writer: W) -> Result<(), W::Error> {
        writer.write_all(&(*self as u8).to_le_bytes())
    }

    #[cfg(feature = "async")]
    async fn write_hci_async<W: embedded_io::asynch::Write>(&self, mut writer: W) -> Result<(), W::Error> {
        writer.write_all(&(*self as u8).to_le_bytes()).await
    }
}

pub enum ControllerToHostPacket<'a> {
    Acl(data::AclPacket<'a>),
    Sync(data::SyncPacket<'a>),
    // Event(EventPacket),
    Iso(data::IsoPacket<'a>),
}

impl<'a> ControllerToHostPacket<'a> {
    pub fn kind(&self) -> PacketKind {
        match self {
            ControllerToHostPacket::Acl(_) => PacketKind::AclData,
            ControllerToHostPacket::Sync(_) => PacketKind::SyncData,
            ControllerToHostPacket::Iso(_) => PacketKind::IsoData,
        }
    }
}

impl<'de> FromHciBytes<'de> for ControllerToHostPacket<'de> {
    fn from_hci_bytes(data: &'de [u8]) -> Result<(Self, &'de [u8]), FromHciBytesError> {
        let (kind, data) = PacketKind::from_hci_bytes(data)?;
        match kind {
            PacketKind::Cmd => Err(FromHciBytesError::InvalidValue),
            PacketKind::AclData => {
                data::AclPacket::from_hci_bytes(data).map(|(x, y)| (ControllerToHostPacket::Acl(x), y))
            }
            PacketKind::SyncData => {
                data::SyncPacket::from_hci_bytes(data).map(|(x, y)| (ControllerToHostPacket::Sync(x), y))
            }
            PacketKind::Event => todo!(),
            PacketKind::IsoData => {
                data::IsoPacket::from_hci_bytes(data).map(|(x, y)| (ControllerToHostPacket::Iso(x), y))
            }
        }
    }
}

impl<'de> ReadHci<'de> for ControllerToHostPacket<'de> {
    fn read_hci<R: embedded_io::blocking::Read>(
        mut reader: R,
        buf: &'de mut [u8],
    ) -> Result<(Self, &'de mut [u8]), ReadHciError<R::Error>> {
        match PacketKind::read_hci(&mut reader, buf)? {
            (PacketKind::Cmd, _buf) => Err(ReadHciError::InvalidValue),
            (PacketKind::AclData, buf) => {
                data::AclPacket::read_hci(reader, buf).map(|(x, y)| (ControllerToHostPacket::Acl(x), y))
            }
            (PacketKind::SyncData, buf) => {
                data::SyncPacket::read_hci(reader, buf).map(|(x, y)| (ControllerToHostPacket::Sync(x), y))
            }
            (PacketKind::Event, _buf) => todo!(),
            (PacketKind::IsoData, buf) => {
                data::IsoPacket::read_hci(reader, buf).map(|(x, y)| (ControllerToHostPacket::Iso(x), y))
            }
        }
    }

    #[cfg(feature = "async")]
    async fn read_hci_async<R: embedded_io::asynch::Read>(
        mut reader: R,
        buf: &'de mut [u8],
    ) -> Result<(Self, &'de mut [u8]), ReadHciError<R::Error>> {
        match PacketKind::read_hci_async(&mut reader, buf).await? {
            (PacketKind::Cmd, _buf) => Err(ReadHciError::InvalidValue),
            (PacketKind::AclData, buf) => data::AclPacket::read_hci_async(reader, buf)
                .await
                .map(|(x, y)| (ControllerToHostPacket::Acl(x), y)),
            (PacketKind::SyncData, buf) => data::SyncPacket::read_hci_async(reader, buf)
                .await
                .map(|(x, y)| (ControllerToHostPacket::Sync(x), y)),
            (PacketKind::Event, _buf) => todo!(),
            (PacketKind::IsoData, buf) => data::IsoPacket::read_hci_async(reader, buf)
                .await
                .map(|(x, y)| (ControllerToHostPacket::Iso(x), y)),
        }
    }
}

impl<'a> WriteHci for ControllerToHostPacket<'a> {
    fn size(&self) -> usize {
        1 + match self {
            ControllerToHostPacket::Acl(pkt) => pkt.size(),
            ControllerToHostPacket::Sync(pkt) => pkt.size(),
            ControllerToHostPacket::Iso(pkt) => pkt.size(),
        }
    }

    fn write_hci<W: embedded_io::blocking::Write>(&self, mut writer: W) -> Result<(), W::Error> {
        self.kind().write_hci(&mut writer)?;
        match self {
            ControllerToHostPacket::Acl(pkt) => pkt.write_hci(writer),
            ControllerToHostPacket::Sync(pkt) => pkt.write_hci(writer),
            ControllerToHostPacket::Iso(pkt) => pkt.write_hci(writer),
        }
    }

    #[cfg(feature = "async")]
    async fn write_hci_async<W: embedded_io::asynch::Write>(&self, mut writer: W) -> Result<(), W::Error> {
        self.kind().write_hci_async(&mut writer).await?;
        match self {
            ControllerToHostPacket::Acl(pkt) => pkt.write_hci_async(writer).await,
            ControllerToHostPacket::Sync(pkt) => pkt.write_hci_async(writer).await,
            ControllerToHostPacket::Iso(pkt) => pkt.write_hci_async(writer).await,
        }
    }
}

pub struct WithIndicator<T: HostToControllerPacket>(T);

impl<T: HostToControllerPacket> WithIndicator<T> {
    pub fn new(pkt: T) -> Self {
        Self(pkt)
    }

    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T: HostToControllerPacket> WriteHci for WithIndicator<T> {
    fn size(&self) -> usize {
        1 + self.0.size()
    }

    fn write_hci<W: embedded_io::blocking::Write>(&self, mut writer: W) -> Result<(), W::Error> {
        T::KIND.write_hci(&mut writer)?;
        self.0.write_hci(writer)
    }

    #[cfg(feature = "async")]
    async fn write_hci_async<W: embedded_io::asynch::Write>(&self, mut writer: W) -> Result<(), W::Error> {
        T::KIND.write_hci_async(&mut writer).await?;
        self.0.write_hci_async(writer).await
    }
}

/// Abbreviations:
/// - command -> cmd
/// - properties -> props
/// - advertising -> adv
/// - advertiser -> adv
/// - address -> addr
/// - connection -> conn
/// - extended -> ext
/// - type -> kind
const _FOO: () = ();
