//! HCI transport layers [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface.html)

use core::future::Future;

use bt_hci_driver::{PacketKind, PacketToController, PacketToHost};
pub use bt_hci_driver::{Transport, WithIndicator};
use embassy_sync::blocking_mutex::raw::RawMutex;
use embassy_sync::mutex::Mutex;
use embedded_io::{ErrorType, ReadExactError, Write};
use embedded_io_async::Write as AsyncWrite;

use crate::cmd::Cmd;
use crate::controller::blocking::TryError;
use crate::ReadHciError;

/// HCI transport layer for a split serial bus using the UART transport layer protocol [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/uart-transport-layer.html)
pub struct SerialTransport<M: RawMutex, R, W> {
    reader: Mutex<M, R>,
    writer: Mutex<M, W>,
}

/// Error type for HCI transport layer communication errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error<E: embedded_io::Error> {
    /// Error reading HCI data.
    Read(ReadHciError<E>),
    /// Error writing data.
    Write(E),
}

impl<E: embedded_io::Error> core::fmt::Display for Error<E> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<E: embedded_io::Error> core::error::Error for Error<E> {}

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

impl<E: embedded_io::Error> From<ReadExactError<E>> for Error<E> {
    fn from(e: ReadExactError<E>) -> Self {
        Self::Read(e.into())
    }
}

impl<E: embedded_io::Error> From<ReadHciError<E>> for Error<E> {
    fn from(e: ReadHciError<E>) -> Self {
        Self::Read(e)
    }
}

impl<M: RawMutex, R: embedded_io_async::Read, W: embedded_io_async::Write> SerialTransport<M, R, W> {
    /// Create a new instance.
    pub fn new(reader: R, writer: W) -> Self {
        Self {
            reader: Mutex::new(reader),
            writer: Mutex::new(writer),
        }
    }
}

impl<
        M: RawMutex,
        R: embedded_io::ErrorType<Error = E>,
        W: embedded_io::ErrorType<Error = E>,
        E: embedded_io::Error,
    > ErrorType for SerialTransport<M, R, W>
{
    type Error = Error<E>;
}

impl<
        M: RawMutex,
        R: embedded_io_async::Read<Error = E>,
        W: embedded_io_async::Write<Error = E>,
        E: embedded_io::Error,
    > Transport for SerialTransport<M, R, W>
{
    async fn read<'a, P: PacketToHost<'a>>(&self, rx: &'a mut [u8]) -> Result<P, Self::Error> {
        let mut r = self.reader.lock().await;
        let kind = PacketKind::read_async(&mut *r).await?;
        P::read_hci_async(kind, &mut *r, rx).await.map_err(Error::Read)
    }

    async fn write<P: PacketToController>(&self, tx: &P) -> Result<(), Self::Error> {
        let mut w = self.writer.lock().await;
        tx.write_hci_async(&mut *w).await.map_err(|e| Error::Write(e))
    }
}

impl<M: RawMutex, R: embedded_io::Read<Error = E>, W: embedded_io::Write<Error = E>, E: embedded_io::Error>
    blocking::Transport for SerialTransport<M, R, W>
{
    fn read<'a, P: PacketToHost<'a>>(&self, rx: &'a mut [u8]) -> Result<P, TryError<Self::Error>> {
        let mut r = self.reader.try_lock().map_err(|_| TryError::Busy)?;
        let kind = PacketKind::read(&mut *r)?;
        P::read_hci(kind, &mut *r, rx)
            .map_err(Error::Read)
            .map_err(TryError::Error)
    }

    fn write<P: PacketToController>(&self, tx: &P) -> Result<(), TryError<Self::Error>> {
        let mut w = self.writer.try_lock().map_err(|_| TryError::Busy)?;
        tx.write_hci(&mut *w)
            .map_err(|e| Error::Write(e))
            .map_err(TryError::Error)
    }
}

/// Wrapper for [`Cmd`] types.
pub struct CmdPacketWrapper<'a, T: Cmd>(pub &'a T);

impl<'a, T: Cmd> PacketToController for CmdPacketWrapper<'a, T> {
    const KIND: PacketKind = PacketKind::Cmd;

    #[inline(always)]
    fn write_hci<W: Write>(&self, writer: W) -> Result<(), W::Error> {
        self.0.write_hci(writer)
    }

    #[inline(always)]
    fn write_hci_async<W: AsyncWrite>(&self, writer: W) -> impl Future<Output = Result<(), W::Error>> {
        self.0.write_hci_async(writer)
    }
}

pub mod blocking {
    //! Blocking transport trait.
    pub use bt_hci_driver::blocking::Transport;
}
