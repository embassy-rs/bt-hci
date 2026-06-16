#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![no_std]

use core::future::Future;

use embedded_io::{Read, ReadExactError, Write};
use embedded_io_async::{Read as AsyncRead, Write as AsyncWrite};

/// An HCI packet from the controller to the host.
pub trait PacketToHost<'d>: Sized {
    /// Deserialize bytes into a HCI type, return additional bytes.
    fn read_hci<R: Read>(data: &mut R, buf: &'d mut [u8]) -> Result<Self, ReadHciError<R::Error>>;

    /// Deserialize bytes into a HCI type, return the value and any remaining bytes.
    fn read_hci_async<R: AsyncRead>(
        data: &mut R,
        buf: &'d mut [u8],
    ) -> impl Future<Output = Result<Self, ReadHciError<R::Error>>>;
}

/// An HCI packet from the host to the controller.
pub trait PacketToController: Sized {
    /// Write this value to the provided writer.
    fn write_hci<W: Write>(&self, writer: W) -> Result<(), W::Error>;

    /// Write this value to the provided writer, async version.
    fn write_hci_async<W: AsyncWrite>(&self, writer: W) -> impl Future<Output = Result<(), W::Error>>;
}

/// A packet-oriented HCI Transport Layer
pub trait Transport: embedded_io::ErrorType {
    /// Read a complete HCI packet into the rx buffer
    fn read<'a, P: PacketToHost<'a>>(&self, rx: &'a mut [u8]) -> impl Future<Output = Result<P, Self::Error>>;

    /// Write a complete HCI packet from the tx buffer
    fn write<P: PacketToController>(&self, tx: &P) -> impl Future<Output = Result<(), Self::Error>>;
}

/// Blocking transport trait.
pub mod blocking {
    use super::*;

    /// A packet-oriented HCI Transport Layer
    pub trait Transport: embedded_io::ErrorType {
        /// Read a complete HCI packet into the rx buffer
        fn read<'a, P: PacketToHost<'a>>(&self, rx: &'a mut [u8]) -> Result<P, TryError<Self::Error>>;

        /// Write a complete HCI packet from the tx buffer
        fn write<P: PacketToController>(&self, tx: &P) -> Result<(), TryError<Self::Error>>;
    }

    /// Error for representing an operation that blocks or fails
    /// with an error.
    #[derive(Debug)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum TryError<E> {
        /// Underlying controller error.
        Error(E),
        /// Operation would block.
        Busy,
    }

    impl<I: embedded_io::Error, E: From<ReadHciError<I>>> From<ReadHciError<I>> for TryError<E> {
        fn from(value: ReadHciError<I>) -> Self {
            TryError::Error(E::from(value))
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

impl<E: embedded_io::Error> core::fmt::Display for ReadHciError<E> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<E: embedded_io::Error> core::error::Error for ReadHciError<E> {}

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
