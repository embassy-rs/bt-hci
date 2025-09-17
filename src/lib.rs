#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![no_std]

use core::future::Future;

use embedded_io::ReadExactError;

mod fmt;

pub mod cmd;
pub mod controller;
pub mod data;
pub mod event;
pub mod param;
pub mod transport;
pub use btuuid as uuid;

/// Errors from parsing HCI data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FromHciBytesError {
    /// Size of input did not match valid size.
    InvalidSize,
    /// Value of input did not match valid values.
    InvalidValue,
}

/// A HCI type which can be represented as bytes.
pub trait AsHciBytes {
    /// Get the byte representation of this type.
    fn as_hci_bytes(&self) -> &[u8];
}

/// A fixed size HCI type that can be deserialized from bytes.
pub trait FromHciBytes<'de>: Sized {
    /// Deserialize bytes into a HCI type, return additional bytes.
    fn from_hci_bytes(data: &'de [u8]) -> Result<(Self, &'de [u8]), FromHciBytesError>;

    /// Deserialize bytes into a HCI type, must consume all bytes.
    fn from_hci_bytes_complete(data: &'de [u8]) -> Result<Self, FromHciBytesError> {
        let (val, buf) = Self::from_hci_bytes(data)?;
        if buf.is_empty() {
            Ok(val)
        } else {
            Err(FromHciBytesError::InvalidSize)
        }
    }
}

/// Errors from reading HCI data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ReadHciError<E: embedded_io::Error> {
    /// Not enough bytes in buffer for reading.
    BufferTooSmall,
    /// Value of input did not match valid values.
    InvalidValue,
    /// Error from underlying embedded-io type.
    Read(ReadExactError<E>),
}

impl<E: embedded_io::Error> embedded_io::Error for ReadHciError<E> {
    fn kind(&self) -> embedded_io::ErrorKind {
        match self {
            Self::BufferTooSmall => embedded_io::ErrorKind::OutOfMemory,
            Self::InvalidValue => embedded_io::ErrorKind::InvalidInput,
            Self::Read(ReadExactError::Other(e)) => e.kind(),
            Self::Read(ReadExactError::UnexpectedEof) => embedded_io::ErrorKind::BrokenPipe,
        }
    }
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

/// Adapter trait for deserializing HCI types from embedded-io implementations.
pub trait ReadHci<'de>: FromHciBytes<'de> {
    /// Max length read by this type.
    const MAX_LEN: usize;

    /// Read this type from the provided reader.
    fn read_hci<R: embedded_io::Read>(reader: R, buf: &'de mut [u8]) -> Result<Self, ReadHciError<R::Error>>;

    /// Read this type from the provided reader, async version.
    fn read_hci_async<R: embedded_io_async::Read>(
        reader: R,
        buf: &'de mut [u8],
    ) -> impl Future<Output = Result<Self, ReadHciError<R::Error>>>;
}

/// Adapter trait for serializing HCI types to embedded-io implementations.
pub trait WriteHci {
    /// The number of bytes this value will write
    fn size(&self) -> usize;

    /// Write this value to the provided writer.
    fn write_hci<W: embedded_io::Write>(&self, writer: W) -> Result<(), W::Error>;

    /// Write this value to the provided writer, async version.
    fn write_hci_async<W: embedded_io_async::Write>(&self, writer: W) -> impl Future<Output = Result<(), W::Error>>;
}

/// Trait representing a HCI packet.
pub trait HostToControllerPacket: WriteHci {
    /// Packet kind associated with this HCI packet.
    const KIND: PacketKind;
}

/// Marker trait for HCI values that have a known, fixed size
///
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

/// Marker trait for [`FixedSizeValue`]s that have byte alignment.
///
/// # Safety
/// - Must have `core::mem::align_of::<T>() == 1`
pub unsafe trait ByteAlignedValue: FixedSizeValue {
    /// Obtain a reference to this type from a byte slice.
    ///
    /// # Safety
    /// - Must have `core::mem::align_of::<T>() == 1`
    fn ref_from_hci_bytes(data: &[u8]) -> Result<(&Self, &[u8]), FromHciBytesError> {
        if data.len() < core::mem::size_of::<Self>() {
            Err(FromHciBytesError::InvalidSize)
        } else if !Self::is_valid(data) {
            Err(FromHciBytesError::InvalidValue)
        } else {
            let (data, rest) = data.split_at(core::mem::size_of::<Self>());
            Ok((unsafe { &*(data.as_ptr() as *const Self) }, rest))
        }
    }
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

impl<'de, T: ByteAlignedValue> FromHciBytes<'de> for &'de [T] {
    fn from_hci_bytes(data: &'de [u8]) -> Result<(Self, &'de [u8]), FromHciBytesError> {
        let Some((len, data)) = data.split_first() else {
            return Err(FromHciBytesError::InvalidSize);
        };

        let len = usize::from(*len);
        let byte_len = len * core::mem::size_of::<T>();
        if byte_len > data.len() {
            return Err(FromHciBytesError::InvalidSize);
        }

        let (data, rest) = data.split_at(byte_len);

        if !data.chunks_exact(core::mem::size_of::<T>()).all(|x| T::is_valid(x)) {
            return Err(FromHciBytesError::InvalidValue);
        }

        Ok((
            unsafe { core::slice::from_raw_parts(data.as_ptr() as *const T, len) },
            rest,
        ))
    }
}

impl<'de, T: FixedSizeValue> ReadHci<'de> for T {
    const MAX_LEN: usize = core::mem::size_of::<Self>();

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

/// Enum of valid HCI packet types.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PacketKind {
    /// Command.
    Cmd = 1,
    /// ACL data.
    AclData = 2,
    /// Sync data.
    SyncData = 3,
    /// Event.
    Event = 4,
    /// Isochronous Data.
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

/// Type representing valid deserialized HCI packets.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ControllerToHostPacket<'a> {
    /// ACL packet.
    Acl(data::AclPacket<'a>),
    /// Sync packet.
    Sync(data::SyncPacket<'a>),
    /// Event packet.
    Event(event::EventPacket<'a>),
    /// Isochronous packet.
    Iso(data::IsoPacket<'a>),
}

impl<'a> ControllerToHostPacket<'a> {
    /// The packet kind.
    pub fn kind(&self) -> PacketKind {
        match self {
            Self::Acl(_) => PacketKind::AclData,
            Self::Sync(_) => PacketKind::SyncData,
            Self::Event(_) => PacketKind::Event,
            Self::Iso(_) => PacketKind::IsoData,
        }
    }

    /// Deserialize data assuming a specific kind of packet.
    pub fn from_hci_bytes_with_kind(
        kind: PacketKind,
        data: &'a [u8],
    ) -> Result<(ControllerToHostPacket<'a>, &'a [u8]), FromHciBytesError> {
        match kind {
            PacketKind::Cmd => Err(FromHciBytesError::InvalidValue),
            PacketKind::AclData => data::AclPacket::from_hci_bytes(data).map(|(x, y)| (Self::Acl(x), y)),
            PacketKind::SyncData => data::SyncPacket::from_hci_bytes(data).map(|(x, y)| (Self::Sync(x), y)),
            PacketKind::Event => event::EventPacket::from_hci_bytes(data).map(|(x, y)| (Self::Event(x), y)),
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
            PacketKind::Event => event::EventPacket::from_hci_bytes(data).map(|(x, y)| (Self::Event(x), y)),
            PacketKind::IsoData => data::IsoPacket::from_hci_bytes(data).map(|(x, y)| (Self::Iso(x), y)),
        }
    }
}

impl<'de> ReadHci<'de> for ControllerToHostPacket<'de> {
    const MAX_LEN: usize = 258;

    fn read_hci<R: embedded_io::Read>(mut reader: R, buf: &'de mut [u8]) -> Result<Self, ReadHciError<R::Error>> {
        let mut kind = [0];
        reader.read_exact(&mut kind)?;
        match PacketKind::from_hci_bytes(&kind)?.0 {
            PacketKind::Cmd => Err(ReadHciError::InvalidValue),
            PacketKind::AclData => data::AclPacket::read_hci(reader, buf).map(Self::Acl),
            PacketKind::SyncData => data::SyncPacket::read_hci(reader, buf).map(Self::Sync),
            PacketKind::Event => event::EventPacket::read_hci(reader, buf).map(Self::Event),
            PacketKind::IsoData => data::IsoPacket::read_hci(reader, buf).map(Self::Iso),
        }
    }

    async fn read_hci_async<R: embedded_io_async::Read>(
        mut reader: R,
        buf: &'de mut [u8],
    ) -> Result<Self, ReadHciError<R::Error>> {
        let mut kind = [0u8];
        reader.read_exact(&mut kind).await?;
        match PacketKind::from_hci_bytes(&kind)?.0 {
            PacketKind::Cmd => Err(ReadHciError::InvalidValue),
            PacketKind::AclData => data::AclPacket::read_hci_async(reader, buf).await.map(Self::Acl),
            PacketKind::SyncData => data::SyncPacket::read_hci_async(reader, buf).await.map(Self::Sync),
            PacketKind::Event => event::EventPacket::read_hci_async(reader, buf).await.map(Self::Event),
            PacketKind::IsoData => data::IsoPacket::read_hci_async(reader, buf).await.map(Self::Iso),
        }
    }
}
