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
pub mod transport;

pub enum FromHciBytesError {
    InvalidSize,
    InvalidValue,
}

pub trait FromHciBytes<'de>: Sized {
    fn from_hci_bytes(data: &'de [u8]) -> Result<(Self, usize), FromHciBytesError>;
}

pub enum ReadHciError<E: embedded_io::Error> {
    BufTooSmall,
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

pub trait ReadHci<'de>: Sized {
    fn read_hci<R: embedded_io::blocking::Read>(reader: R, buf: &'de mut [u8]) -> Result<Self, ReadHciError<R::Error>>;

    #[cfg(feature = "async")]
    async fn read_hci_async<R: embedded_io::asynch::Read>(
        reader: R,
        buf: &'de mut [u8],
    ) -> Result<Self, ReadHciError<R::Error>>;
}

pub trait WriteHci {
    /// The number of bytes this value will write
    fn size(&self) -> usize;

    fn write_hci<W: embedded_io::blocking::Write>(&self, writer: W) -> Result<(), W::Error>;

    #[cfg(feature = "async")]
    async fn write_hci_async<W: embedded_io::asynch::Write>(&self, writer: W) -> Result<(), W::Error>;
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
    fn from_hci_bytes(data: &'de [u8]) -> Result<(Self, usize), FromHciBytesError> {
        if data.is_empty() {
            Err(FromHciBytesError::InvalidSize)
        } else {
            match data[0] {
                1 => Ok((PacketKind::Cmd, 1)),
                2 => Ok((PacketKind::AclData, 1)),
                3 => Ok((PacketKind::SyncData, 1)),
                4 => Ok((PacketKind::Event, 1)),
                5 => Ok((PacketKind::IsoData, 1)),
                _ => Err(FromHciBytesError::InvalidValue),
            }
        }
    }
}

impl<'de> ReadHci<'de> for PacketKind {
    fn read_hci<R: embedded_io::blocking::Read>(
        mut reader: R,
        _buf: &'de mut [u8],
    ) -> Result<Self, ReadHciError<R::Error>> {
        let mut buf = [0];
        reader.read_exact(&mut buf)?;
        Self::from_hci_bytes(&buf).map(|(x, _)| x).map_err(Into::into)
    }

    #[cfg(feature = "async")]
    async fn read_hci_async<R: embedded_io::asynch::Read>(
        mut reader: R,
        _buf: &'de mut [u8],
    ) -> Result<Self, ReadHciError<R::Error>> {
        let mut buf = [0];
        reader.read_exact(&mut buf).await?;
        Self::from_hci_bytes(&buf).map(|(x, _)| x).map_err(Into::into)
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

pub enum Packet<'a> {
    // Cmd(CmdPacket),
    Acl(data::AclPacket<'a>),
    Sync(data::SyncPacket<'a>),
    // Event(EventPacket),
    Iso(data::IsoPacket<'a>),
}

impl<'a> Packet<'a> {
    pub fn kind(&self) -> PacketKind {
        match self {
            Packet::Acl(_) => PacketKind::AclData,
            Packet::Sync(_) => PacketKind::SyncData,
            Packet::Iso(_) => PacketKind::IsoData,
        }
    }
}

impl<'de> FromHciBytes<'de> for Packet<'de> {
    fn from_hci_bytes(data: &'de [u8]) -> Result<(Self, usize), FromHciBytesError> {
        let (kind, kind_len) = PacketKind::from_hci_bytes(data)?;
        let data = &data[kind_len..];
        match kind {
            PacketKind::Cmd => todo!(),
            PacketKind::AclData => data::AclPacket::from_hci_bytes(data).map(|(x, y)| (Packet::Acl(x), kind_len + y)),
            PacketKind::SyncData => {
                data::SyncPacket::from_hci_bytes(data).map(|(x, y)| (Packet::Sync(x), kind_len + y))
            }
            PacketKind::Event => todo!(),
            PacketKind::IsoData => data::IsoPacket::from_hci_bytes(data).map(|(x, y)| (Packet::Iso(x), kind_len + y)),
        }
    }
}

impl<'de> ReadHci<'de> for Packet<'de> {
    fn read_hci<R: embedded_io::blocking::Read>(
        mut reader: R,
        buf: &'de mut [u8],
    ) -> Result<Self, ReadHciError<R::Error>> {
        match PacketKind::read_hci(&mut reader, buf)? {
            PacketKind::Cmd => todo!(),
            PacketKind::AclData => data::AclPacket::read_hci(reader, buf).map(Packet::Acl),
            PacketKind::SyncData => data::SyncPacket::read_hci(reader, buf).map(Packet::Sync),
            PacketKind::Event => todo!(),
            PacketKind::IsoData => data::IsoPacket::read_hci(reader, buf).map(Packet::Iso),
        }
    }

    #[cfg(feature = "async")]
    async fn read_hci_async<R: embedded_io::asynch::Read>(
        mut reader: R,
        buf: &'de mut [u8],
    ) -> Result<Self, ReadHciError<R::Error>> {
        match PacketKind::read_hci_async(&mut reader, buf).await? {
            PacketKind::Cmd => todo!(),
            PacketKind::AclData => data::AclPacket::read_hci_async(reader, buf).await.map(Packet::Acl),
            PacketKind::SyncData => data::SyncPacket::read_hci_async(reader, buf).await.map(Packet::Sync),
            PacketKind::Event => todo!(),
            PacketKind::IsoData => data::IsoPacket::read_hci_async(reader, buf).await.map(Packet::Iso),
        }
    }
}

impl<'a> WriteHci for Packet<'a> {
    fn size(&self) -> usize {
        1 + match self {
            Packet::Acl(pkt) => pkt.size(),
            Packet::Sync(pkt) => pkt.size(),
            Packet::Iso(pkt) => pkt.size(),
        }
    }

    fn write_hci<W: embedded_io::blocking::Write>(&self, mut writer: W) -> Result<(), W::Error> {
        self.kind().write_hci(&mut writer)?;
        match self {
            Packet::Acl(pkt) => pkt.write_hci(writer),
            Packet::Sync(pkt) => pkt.write_hci(writer),
            Packet::Iso(pkt) => pkt.write_hci(writer),
        }
    }

    #[cfg(feature = "async")]
    async fn write_hci_async<W: embedded_io::asynch::Write>(&self, mut writer: W) -> Result<(), W::Error> {
        self.kind().write_hci_async(&mut writer).await?;
        match self {
            Packet::Acl(pkt) => pkt.write_hci_async(writer).await,
            Packet::Sync(pkt) => pkt.write_hci_async(writer).await,
            Packet::Iso(pkt) => pkt.write_hci_async(writer).await,
        }
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
