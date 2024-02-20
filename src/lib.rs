#![no_std]

use core::future::Future;

use embedded_io::ReadExactError;

mod fmt;

pub mod cmd;
pub mod data;
pub mod event;
pub mod param;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FromHciBytesError {
    InvalidSize,
    InvalidValue,
}

pub trait AsHciBytes {
    fn as_hci_bytes(&self) -> &[u8];
}

pub trait FromHciBytes<'de>: Sized {
    fn from_hci_bytes(data: &'de [u8]) -> Result<(Self, &'de [u8]), FromHciBytesError>;

    fn from_hci_bytes_complete(data: &'de [u8]) -> Result<Self, FromHciBytesError> {
        let (val, buf) = Self::from_hci_bytes(data)?;
        if buf.is_empty() {
            Ok(val)
        } else {
            Err(FromHciBytesError::InvalidSize)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
    fn read_hci<R: embedded_io::Read>(reader: R, buf: &'de mut [u8]) -> Result<Self, ReadHciError<R::Error>>;

    fn read_hci_async<R: embedded_io_async::Read>(
        reader: R,
        buf: &'de mut [u8],
    ) -> impl Future<Output = Result<Self, ReadHciError<R::Error>>>;
}

pub trait WriteHci {
    /// The number of bytes this value will write
    fn size(&self) -> usize;

    fn write_hci<W: embedded_io::Write>(&self, writer: W) -> Result<(), W::Error>;

    fn write_hci_async<W: embedded_io_async::Write>(&self, writer: W) -> impl Future<Output = Result<(), W::Error>>;
}

pub trait HostToControllerPacket: WriteHci {
    const KIND: PacketKind;
}

/// # Safety
/// - Must not contain any padding (uninitialized) bytes (recursively)
/// - structs must be `#[repr(C)]` or `#[repr(transparent)]`
/// - enums must be `#[repr(<int>)]`
/// - Must not contain any references, pointers, atomics, or interior mutability
/// - `is_valid()` must return true only if `data` is a valid bit representation of `Self`
pub unsafe trait FixedSizeValue: Copy {
    /// Checks if the bit representation in data is valid for Self.
    ///
    /// May panic if `data.len() != core::mem::size_of::<Self>()`
    fn is_valid(data: &[u8]) -> bool;
}

impl<T: FixedSizeValue> AsHciBytes for T {
    fn as_hci_bytes(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self as *const _ as *const u8, core::mem::size_of::<Self>()) }
    }
}

impl<'de, T: FixedSizeValue> FromHciBytes<'de> for T {
    fn from_hci_bytes(data: &'de [u8]) -> Result<(Self, &'de [u8]), FromHciBytesError> {
        if data.len() < core::mem::size_of::<Self>() {
            Err(FromHciBytesError::InvalidSize)
        } else if !Self::is_valid(data) {
            Err(FromHciBytesError::InvalidValue)
        } else {
            let (data, rest) = data.split_at(core::mem::size_of::<Self>());
            Ok((unsafe { core::ptr::read_unaligned(data.as_ptr() as *const Self) }, rest))
        }
    }
}

impl<'de, T: FixedSizeValue> ReadHci<'de> for T {
    fn read_hci<R: embedded_io::Read>(mut reader: R, buf: &'de mut [u8]) -> Result<Self, ReadHciError<R::Error>> {
        if buf.len() < core::mem::size_of::<Self>() {
            Err(ReadHciError::BufferTooSmall)
        } else {
            let (buf, _) = buf.split_at_mut(core::mem::size_of::<Self>());
            reader.read_exact(buf)?;
            Self::from_hci_bytes(buf).map(|(x, _)| x).map_err(Into::into)
        }
    }

    async fn read_hci_async<R: embedded_io_async::Read>(
        mut reader: R,
        buf: &'de mut [u8],
    ) -> Result<Self, ReadHciError<R::Error>> {
        if buf.len() < core::mem::size_of::<Self>() {
            Err(ReadHciError::BufferTooSmall)
        } else {
            let (buf, _) = buf.split_at_mut(core::mem::size_of::<Self>());
            reader.read_exact(buf).await?;
            Self::from_hci_bytes(buf).map(|(x, _)| x).map_err(Into::into)
        }
    }
}

impl<T: FixedSizeValue> WriteHci for T {
    #[inline(always)]
    fn size(&self) -> usize {
        core::mem::size_of::<Self>()
    }

    fn write_hci<W: embedded_io::Write>(&self, mut writer: W) -> Result<(), W::Error> {
        writer.write_all(self.as_hci_bytes())
    }

    async fn write_hci_async<W: embedded_io_async::Write>(&self, mut writer: W) -> Result<(), W::Error> {
        writer.write_all(self.as_hci_bytes()).await
    }
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

impl WriteHci for PacketKind {
    #[inline(always)]
    fn size(&self) -> usize {
        1
    }

    #[inline(always)]
    fn write_hci<W: embedded_io::Write>(&self, mut writer: W) -> Result<(), W::Error> {
        writer.write_all(&(*self as u8).to_le_bytes())
    }

    #[inline(always)]
    async fn write_hci_async<W: embedded_io_async::Write>(&self, mut writer: W) -> Result<(), W::Error> {
        writer.write_all(&(*self as u8).to_le_bytes()).await
    }
}

pub enum ControllerToHostPacket<'a> {
    Acl(data::AclPacket<'a>),
    Sync(data::SyncPacket<'a>),
    Event(event::Event<'a>),
    Iso(data::IsoPacket<'a>),
}

impl<'a> ControllerToHostPacket<'a> {
    pub fn kind(&self) -> PacketKind {
        match self {
            Self::Acl(_) => PacketKind::AclData,
            Self::Sync(_) => PacketKind::SyncData,
            Self::Event(_) => PacketKind::Event,
            Self::Iso(_) => PacketKind::IsoData,
        }
    }

    pub fn from_hci_bytes_with_kind(kind: PacketKind, data: &'a [u8]) -> Result<(ControllerToHostPacket<'a>, &'a [u8]), FromHciBytesError> {
        match kind {
            PacketKind::Cmd => Err(FromHciBytesError::InvalidValue),
            PacketKind::AclData => data::AclPacket::from_hci_bytes(data).map(|(x, y)| (Self::Acl(x), y)),
            PacketKind::SyncData => data::SyncPacket::from_hci_bytes(data).map(|(x, y)| (Self::Sync(x), y)),
            PacketKind::Event => event::Event::from_hci_bytes(data).map(|(x, y)| (Self::Event(x), y)),
            PacketKind::IsoData => data::IsoPacket::from_hci_bytes(data).map(|(x, y)| (Self::Iso(x), y)),
        }
    }
}

impl<'de> FromHciBytes<'de> for ControllerToHostPacket<'de> {
    fn from_hci_bytes(data: &'de [u8]) -> Result<(Self, &'de [u8]), FromHciBytesError> {
        let (kind, data) = PacketKind::from_hci_bytes(data)?;
        match kind {
            PacketKind::Cmd => Err(FromHciBytesError::InvalidValue),
            PacketKind::AclData => data::AclPacket::from_hci_bytes(data).map(|(x, y)| (Self::Acl(x), y)),
            PacketKind::SyncData => data::SyncPacket::from_hci_bytes(data).map(|(x, y)| (Self::Sync(x), y)),
            PacketKind::Event => event::Event::from_hci_bytes(data).map(|(x, y)| (Self::Event(x), y)),
            PacketKind::IsoData => data::IsoPacket::from_hci_bytes(data).map(|(x, y)| (Self::Iso(x), y)),
        }
    }
}

impl<'de> ReadHci<'de> for ControllerToHostPacket<'de> {
    fn read_hci<R: embedded_io::Read>(mut reader: R, buf: &'de mut [u8]) -> Result<Self, ReadHciError<R::Error>> {
        let mut kind = [0];
        reader.read_exact(&mut kind)?;
        match PacketKind::from_hci_bytes(&kind)?.0 {
            PacketKind::Cmd => Err(ReadHciError::InvalidValue),
            PacketKind::AclData => data::AclPacket::read_hci(reader, buf).map(Self::Acl),
            PacketKind::SyncData => data::SyncPacket::read_hci(reader, buf).map(Self::Sync),
            PacketKind::Event => event::Event::read_hci(reader, buf).map(Self::Event),
            PacketKind::IsoData => data::IsoPacket::read_hci(reader, buf).map(Self::Iso),
        }
    }

    async fn read_hci_async<R: embedded_io_async::Read>(
        mut reader: R,
        buf: &'de mut [u8],
    ) -> Result<Self, ReadHciError<R::Error>> {
        let mut kind = [0];
        reader.read_exact(&mut kind).await?;
        match PacketKind::from_hci_bytes(&kind)?.0 {
            PacketKind::Cmd => Err(ReadHciError::InvalidValue),
            PacketKind::AclData => data::AclPacket::read_hci_async(reader, buf).await.map(Self::Acl),
            PacketKind::SyncData => data::SyncPacket::read_hci_async(reader, buf).await.map(Self::Sync),
            PacketKind::Event => event::Event::read_hci_async(reader, buf).await.map(Self::Event),
            PacketKind::IsoData => data::IsoPacket::read_hci_async(reader, buf).await.map(Self::Iso),
        }
    }
}

/// Wrapper for a [`HostToControllerPacket`] that will write the [`PacketKind`] indicator byte before the packet itself
/// when serialized with [`WriteHci`].
///
/// This is used for transports where all packets are sent over a common channel, such as the UART transport.
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
    #[inline(always)]
    fn size(&self) -> usize {
        1 + self.0.size()
    }

    #[inline(always)]
    fn write_hci<W: embedded_io::Write>(&self, mut writer: W) -> Result<(), W::Error> {
        T::KIND.write_hci(&mut writer)?;
        self.0.write_hci(writer)
    }

    #[inline(always)]
    async fn write_hci_async<W: embedded_io_async::Write>(&self, mut writer: W) -> Result<(), W::Error> {
        T::KIND.write_hci_async(&mut writer).await?;
        self.0.write_hci_async(writer).await
    }
}

/// Abbreviations:
/// - address -> addr
/// - advertiser -> adv
/// - advertising -> adv
/// - command -> cmd
/// - connection -> conn
/// - event -> evt
/// - extended -> ext
/// - extension -> ext
/// - identifier -> id
/// - length -> len
/// - packet -> pkt
/// - properties -> props
/// - receive -> rx
/// - transmit -> tx
/// - type -> kind
const _FOO: () = ();
