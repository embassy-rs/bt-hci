#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![no_std]

use bt_hci_transport::blocking::TryError;
use bt_hci_transport::{PacketKind, PacketToController, PacketToHost, ReadHciError, Transport, WithIndicator};
use embassy_sync::blocking_mutex::raw::RawMutex;
use embassy_sync::mutex::Mutex;
use embedded_io::{ErrorType, ReadExactError};

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

impl<E: embedded_io::Error> From<ReadExactError<E>> for Error<E> {
    fn from(e: ReadExactError<E>) -> Self {
        Self::Read(e.into())
    }
}

// Required by `ExternalController`, which bounds its transport error type on
// `From<ReadHciError<Infallible>>`. A blanket `From<ReadHciError<E>>` would
// overlap with this impl at `E = Infallible`, so the conversion is widening
// instead: an infallible read error maps into the equivalent `ReadHciError<E>`.
impl<E: embedded_io::Error> From<ReadHciError<core::convert::Infallible>> for Error<E> {
    fn from(e: ReadHciError<core::convert::Infallible>) -> Self {
        Self::Read(match e {
            ReadHciError::BufferTooSmall => ReadHciError::BufferTooSmall,
            ReadHciError::InvalidValue => ReadHciError::InvalidValue,
            ReadHciError::Read(ReadExactError::UnexpectedEof) => ReadHciError::Read(ReadExactError::UnexpectedEof),
            ReadHciError::Read(ReadExactError::Other(e)) => match e {},
        })
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
        let kind = PacketKind::read_async(&mut *r).await.map_err(Error::Read)?;
        P::read_hci_async(kind, &mut *r, rx).await.map_err(Error::Read)
    }

    async fn write<P: PacketToController>(&self, tx: &P) -> Result<(), Self::Error> {
        let mut w = self.writer.lock().await;
        WithIndicator::new(tx)
            .write_hci_async(&mut *w)
            .await
            .map_err(|e| Error::Write(e))
    }
}

impl<M: RawMutex, R: embedded_io::Read<Error = E>, W: embedded_io::Write<Error = E>, E: embedded_io::Error>
    blocking::Transport for SerialTransport<M, R, W>
{
    fn read<'a, P: PacketToHost<'a>>(&self, rx: &'a mut [u8]) -> Result<P, TryError<Self::Error>> {
        let mut r = self.reader.try_lock().map_err(|_| TryError::Busy)?;
        let kind = PacketKind::read(&mut *r)
            .map_err(Error::Read)
            .map_err(TryError::Error)?;
        P::read_hci(kind, &mut *r, rx)
            .map_err(Error::Read)
            .map_err(TryError::Error)
    }

    fn write<P: PacketToController>(&self, tx: &P) -> Result<(), TryError<Self::Error>> {
        let mut w = self.writer.try_lock().map_err(|_| TryError::Busy)?;
        WithIndicator::new(tx)
            .write_hci(&mut *w)
            .map_err(|e| Error::Write(e))
            .map_err(TryError::Error)
    }
}

pub mod blocking {
    //! Blocking transport trait.
    pub use bt_hci_transport::blocking::Transport;
}

#[cfg(test)]
mod tests {
    use embassy_sync::blocking_mutex::raw::NoopRawMutex;

    use super::*;

    struct FakeCmd;

    impl PacketToController for FakeCmd {
        const KIND: PacketKind = PacketKind::Cmd;

        fn size(&self) -> usize {
            3
        }

        fn write_hci<W: embedded_io::Write>(&self, mut writer: W) -> Result<(), W::Error> {
            writer.write_all(&[0x03, 0x0c, 0x00])
        }

        async fn write_hci_async<W: embedded_io_async::Write>(&self, mut writer: W) -> Result<(), W::Error> {
            writer.write_all(&[0x03, 0x0c, 0x00]).await
        }
    }

    struct SliceWriter<'a> {
        buf: &'a mut [u8],
        pos: usize,
    }

    impl embedded_io::ErrorType for SliceWriter<'_> {
        type Error = core::convert::Infallible;
    }

    impl embedded_io::Write for SliceWriter<'_> {
        fn write(&mut self, data: &[u8]) -> Result<usize, Self::Error> {
            let n = data.len().min(self.buf.len() - self.pos);
            self.buf[self.pos..self.pos + n].copy_from_slice(&data[..n]);
            self.pos += n;
            Ok(n)
        }

        fn flush(&mut self) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    impl embedded_io_async::Write for SliceWriter<'_> {
        async fn write(&mut self, data: &[u8]) -> Result<usize, Self::Error> {
            embedded_io::Write::write(self, data)
        }

        async fn flush(&mut self) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    #[test]
    fn write_prepends_packet_kind_indicator() {
        let mut buf = [0u8; 4];
        {
            let transport: SerialTransport<NoopRawMutex, &[u8], SliceWriter<'_>> =
                SerialTransport::new(&[][..], SliceWriter { buf: &mut buf, pos: 0 });
            blocking::Transport::write(&transport, &FakeCmd).unwrap();
        }
        assert_eq!(buf, [0x01, 0x03, 0x0c, 0x00]);
    }
}
