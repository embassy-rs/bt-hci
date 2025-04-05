use crate::{ByteAlignedValue, FixedSizeValue, FromHciBytes, FromHciBytesError, WriteHci};

unsafe impl FixedSizeValue for () {
    #[inline(always)]
    fn is_valid(_data: &[u8]) -> bool {
        true
    }
}

unsafe impl ByteAlignedValue for () {}

impl<'de> FromHciBytes<'de> for &'static () {
    #[inline(always)]
    fn from_hci_bytes(data: &'de [u8]) -> Result<(Self, &'de [u8]), crate::FromHciBytesError> {
        Ok((&(), data))
    }
}

unsafe impl FixedSizeValue for bool {
    #[inline(always)]
    fn is_valid(data: &[u8]) -> bool {
        !data.is_empty() && data[0] < 2
    }
}

unsafe impl ByteAlignedValue for bool {}

impl<'de> FromHciBytes<'de> for &'de bool {
    #[inline(always)]
    fn from_hci_bytes(data: &'de [u8]) -> Result<(Self, &'de [u8]), crate::FromHciBytesError> {
        <bool as crate::ByteAlignedValue>::ref_from_hci_bytes(data)
    }
}

impl WriteHci for &[u8] {
    #[inline(always)]
    fn size(&self) -> usize {
        self.len() + 1
    }

    #[inline(always)]
    fn write_hci<W: embedded_io::Write>(&self, mut writer: W) -> Result<(), W::Error> {
        writer.write_all(&[self.len() as u8])?;
        writer.write_all(self)
    }

    #[inline(always)]
    async fn write_hci_async<W: embedded_io_async::Write>(&self, mut writer: W) -> Result<(), W::Error> {
        writer.write_all(&[self.size() as u8]).await?;
        writer.write_all(self).await
    }
}

unsafe impl<T: FixedSizeValue, const N: usize> FixedSizeValue for [T; N] {
    #[inline(always)]
    fn is_valid(_data: &[u8]) -> bool {
        true
    }
}

unsafe impl<T: ByteAlignedValue, const N: usize> ByteAlignedValue for [T; N] {}

impl<'de, T: ByteAlignedValue, const N: usize> FromHciBytes<'de> for &'de [T; N] {
    #[inline(always)]
    fn from_hci_bytes(data: &'de [u8]) -> Result<(Self, &'de [u8]), crate::FromHciBytesError> {
        <[T; N] as crate::ByteAlignedValue>::ref_from_hci_bytes(data)
    }
}

impl<T: WriteHci> WriteHci for Option<T> {
    #[inline(always)]
    fn size(&self) -> usize {
        self.as_ref().map(|x| x.size()).unwrap_or_default()
    }

    #[inline(always)]
    fn write_hci<W: embedded_io::Write>(&self, writer: W) -> Result<(), W::Error> {
        match self {
            Some(val) => val.write_hci(writer),
            None => Ok(()),
        }
    }

    #[inline(always)]
    async fn write_hci_async<W: embedded_io_async::Write>(&self, writer: W) -> Result<(), W::Error> {
        match self {
            Some(val) => val.write_hci_async(writer).await,
            None => Ok(()),
        }
    }
}

impl<'de, T: FromHciBytes<'de>> FromHciBytes<'de> for Option<T> {
    fn from_hci_bytes(data: &'de [u8]) -> Result<(Self, &'de [u8]), FromHciBytesError> {
        if data.is_empty() {
            Ok((None, data))
        } else {
            T::from_hci_bytes(data).map(|(x, y)| (Some(x), y))
        }
    }
}
