use embassy_sync::blocking_mutex::raw::RawMutex;
use embassy_sync::mutex::Mutex;
use embedded_io::ReadExactError;

use super::driver::HciDriver;
use crate::data::AclPacketHeader;
use crate::event::EventPacketHeader;
use crate::{FromHciBytes, FromHciBytesError, PacketKind, ReadHciError};

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
    async fn read(&self, rx: &mut [u8]) -> Result<usize, Self::Error> {
        let mut r = self.reader.lock().await;

        r.read_exact(&mut rx[0..1]).await?;

        match PacketKind::from_hci_bytes(&rx[0..1])?.0 {
            PacketKind::Cmd => Err(Error::Read(ReadHciError::InvalidValue)),
            PacketKind::AclData => {
                r.read_exact(&mut rx[1..5]).await?;
                let header = AclPacketHeader::from_hci_bytes_complete(&rx[1..5])?;
                let data_len = header.data_len();
                r.read_exact(&mut rx[5..5 + data_len]).await?;
                Ok(5 + data_len)
            }
            PacketKind::SyncData => unimplemented!(),
            PacketKind::IsoData => unimplemented!(),
            PacketKind::Event => {
                r.read_exact(&mut rx[1..3]).await?;
                let header = EventPacketHeader::from_hci_bytes_complete(&rx[1..3])?;
                let params_len = usize::from(header.params_len);
                r.read_exact(&mut rx[3..3 + params_len]).await?;
                Ok(3 + params_len)
            }
        }
    }

    async fn write(&self, tx: &[u8]) -> Result<(), Self::Error> {
        let mut w = self.writer.lock().await;
        w.write_all(tx)
            .await
            .map_err(|e| Error::Write(WriteHciError::Write(e)))?;
        Ok(())
    }
}
