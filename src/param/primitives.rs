use crate::{FromHciBytes, FromHciBytesError, WriteHci};

impl WriteHci for bool {
    #[inline(always)]
    fn size(&self) -> usize {
        1
    }

    #[inline(always)]
    fn write_hci<W: ::embedded_io::Write>(&self, mut writer: W) -> Result<(), W::Error> {
        writer.write_all(&(*self as u8).to_le_bytes())
    }

    #[inline(always)]
    async fn write_hci_async<W: ::embedded_io_async::Write>(&self, mut writer: W) -> Result<(), W::Error> {
        writer.write_all(&(*self as u8).to_le_bytes()).await
    }
}

impl<'de> FromHciBytes<'de> for bool {
    fn from_hci_bytes(data: &'de [u8]) -> Result<(Self, &'de [u8]), FromHciBytesError> {
        match data.split_first() {
            Some((0, data)) => Ok((false, data)),
            Some((1, data)) => Ok((true, data)),
            Some(_) => Err(FromHciBytesError::InvalidValue),
            None => Err(FromHciBytesError::InvalidSize),
        }
    }
}

impl<'a> WriteHci for &'a [u8] {
    #[inline(always)]
    fn size(&self) -> usize {
        self.len()
    }

    #[inline(always)]
    fn write_hci<W: embedded_io::Write>(&self, mut writer: W) -> Result<(), W::Error> {
        writer.write_all(&[self.size() as u8])?;
        writer.write_all(self)
    }

    #[inline(always)]
    async fn write_hci_async<W: embedded_io_async::Write>(&self, mut writer: W) -> Result<(), W::Error> {
        writer.write_all(&[self.size() as u8]).await?;
        writer.write_all(self).await
    }
}

impl<'de> FromHciBytes<'de> for &'de [u8] {
    fn from_hci_bytes(data: &'de [u8]) -> Result<(Self, &'de [u8]), FromHciBytesError> {
        match data.split_first() {
            Some((&len, data)) => {
                let len = usize::from(len);
                if data.len() >= len {
                    Ok(data.split_at(len))
                } else {
                    Err(FromHciBytesError::InvalidSize)
                }
            }
            None => Err(FromHciBytesError::InvalidSize),
        }
    }
}

impl<const N: usize> WriteHci for [u8; N] {
    #[inline(always)]
    fn size(&self) -> usize {
        N
    }

    #[inline(always)]
    fn write_hci<W: embedded_io::Write>(&self, mut writer: W) -> Result<(), W::Error> {
        writer.write_all(self)
    }

    #[inline(always)]
    async fn write_hci_async<W: embedded_io_async::Write>(&self, mut writer: W) -> Result<(), W::Error> {
        writer.write_all(self).await
    }
}

impl<'de, const N: usize> FromHciBytes<'de> for [u8; N] {
    fn from_hci_bytes(data: &'de [u8]) -> Result<(Self, &'de [u8]), FromHciBytesError> {
        if data.len() >= N {
            let (data, rest) = data.split_at(N);
            Ok((unsafe { data.try_into().unwrap_unchecked() }, rest))
        } else {
            Err(FromHciBytesError::InvalidSize)
        }
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
