//! HCI transport layers [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface.html)

use core::future::Future;

use embassy_sync::blocking_mutex::raw::RawMutex;
use embassy_sync::mutex::Mutex;
use embedded_io::ReadExactError;

use crate::{ControllerToHostPacket, FromHciBytesError, HostToControllerPacket, ReadHci, ReadHciError, WriteHci};

/// A packet-oriented HCI Transport Layer
pub trait Transport {
    type Error: embedded_io::Error;
    /// Read a complete HCI packet into the rx buffer
    fn read<'a>(&self, rx: &'a mut [u8]) -> impl Future<Output = Result<ControllerToHostPacket<'a>, Self::Error>>;
    /// Write a complete HCI packet from the tx buffer
    fn write<T: HostToControllerPacket>(&self, val: &T) -> impl Future<Output = Result<(), Self::Error>>;
}

/// HCI transport layer for a split serial bus using the UART transport layer protocol [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/uart-transport-layer.html)
pub struct SerialTransport<M: RawMutex, R, W> {
    reader: Mutex<M, R>,
    writer: Mutex<M, W>,
}

/// Error type for HCI transport layer communication errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error<E: embedded_io::Error> {
    Read(ReadHciError<E>),
    Write(E),
}

impl<E: embedded_io::Error> embedded_io::Error for Error<E> {
    fn kind(&self) -> embedded_io::ErrorKind {
        match self {
            Self::Read(e) => e.kind(),
            Self::Write(e) => e.kind(),
        }
    }
}

impl<E: embedded_io::Error> From<E> for Error<E> {
    fn from(e: E) -> Self {
        Self::Write(e)
    }
}

impl<E: embedded_io::Error> From<ReadHciError<E>> for Error<E> {
    fn from(e: ReadHciError<E>) -> Self {
        Self::Read(e)
    }
}

impl<E: embedded_io::Error> From<ReadExactError<E>> for Error<E> {
    fn from(e: ReadExactError<E>) -> Self {
        Self::Read(e.into())
    }
}

impl<E: embedded_io::Error> From<FromHciBytesError> for Error<E> {
    fn from(e: FromHciBytesError) -> Self {
        Self::Read(e.into())
    }
}

impl<M: RawMutex, R: embedded_io_async::Read, W: embedded_io_async::Write> SerialTransport<M, R, W> {
    pub fn new(reader: R, writer: W) -> Self {
        Self {
            reader: Mutex::new(reader),
            writer: Mutex::new(writer),
        }
    }
}

impl<
        M: RawMutex,
        R: embedded_io_async::Read<Error = E>,
        W: embedded_io_async::Write<Error = E>,
        E: embedded_io::Error,
    > Transport for SerialTransport<M, R, W>
{
    type Error = Error<E>;
    async fn read<'a>(&self, rx: &'a mut [u8]) -> Result<ControllerToHostPacket<'a>, Self::Error> {
        let mut r = self.reader.lock().await;
        ControllerToHostPacket::read_hci_async(&mut *r, rx)
            .await
            .map_err(Error::Read)
    }

    async fn write<T: HostToControllerPacket>(&self, tx: &T) -> Result<(), Self::Error> {
        let mut w = self.writer.lock().await;
        WithIndicator(tx)
            .write_hci_async(&mut *w)
            .await
            .map_err(|e| Error::Write(e))
    }
}

/// Wrapper for a [`HostToControllerPacket`] that will write the [`PacketKind`](crate::PacketKind) indicator byte before the packet itself
/// when serialized with [`WriteHci`].
///
/// This is used for transports where all packets are sent over a common channel, such as the UART transport.
pub struct WithIndicator<'a, T: HostToControllerPacket>(&'a T);

impl<'a, T: HostToControllerPacket> WithIndicator<'a, T> {
    pub fn new(pkt: &'a T) -> Self {
        Self(pkt)
    }
}

impl<'a, T: HostToControllerPacket> WriteHci for WithIndicator<'a, T> {
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
