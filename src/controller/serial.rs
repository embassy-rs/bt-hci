use embassy_sync::blocking_mutex::raw::RawMutex;
use embassy_sync::mutex::Mutex;
use embedded_io::ReadExactError;

use super::driver::HciDriver;
use crate::{
    ControllerToHostPacket, FromHciBytesError, HostToControllerPacket, ReadHci, ReadHciError, WithIndicator, WriteHci,
};

/// A HCI driver implementation for a split serial
pub struct SerialHciDriver<M: RawMutex, R, W> {
    reader: Mutex<M, R>,
    writer: Mutex<M, W>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WriteHciError<E: embedded_io::Error> {
    Write(E),
}

impl<E: embedded_io::Error> embedded_io::Error for WriteHciError<E> {
    fn kind(&self) -> embedded_io::ErrorKind {
        match self {
            Self::Write(e) => e.kind(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error<E: embedded_io::Error> {
    Read(ReadHciError<E>),
    Write(WriteHciError<E>),
}

impl<E: embedded_io::Error> embedded_io::Error for Error<E> {
    fn kind(&self) -> embedded_io::ErrorKind {
        match self {
            Self::Read(e) => e.kind(),
            Self::Write(e) => e.kind(),
        }
    }
}

impl<E: embedded_io::Error> From<WriteHciError<E>> for Error<E> {
    fn from(e: WriteHciError<E>) -> Self {
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

impl<M: RawMutex, R: embedded_io_async::Read, W: embedded_io_async::Write> SerialHciDriver<M, R, W> {
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
    > HciDriver for SerialHciDriver<M, R, W>
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
            .map_err(|e| Error::Write(WriteHciError::Write(e)))
    }
}
