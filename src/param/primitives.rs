use crate::{FromHciBytes, FromHciBytesError, WriteHci};

impl WriteHci for bool {
    fn size(&self) -> usize {
        ::core::mem::size_of::<Self>()
    }
    fn write_hci<W: ::embedded_io::blocking::Write>(&self, mut writer: W) -> Result<(), W::Error> {
        writer.write_all(&(*self as u8).to_le_bytes())
    }
    #[cfg(feature = "async")]
    async fn write_hci_async<W: ::embedded_io::asynch::Write>(&self, mut writer: W) -> Result<(), W::Error> {
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
    fn size(&self) -> usize {
        self.len()
    }

    fn write_hci<W: embedded_io::blocking::Write>(&self, mut writer: W) -> Result<(), W::Error> {
        writer.write_all(&[self.size() as u8])?;
        writer.write_all(self)
    }

    #[cfg(feature = "async")]
    async fn write_hci_async<W: embedded_io::asynch::Write>(&self, mut writer: W) -> Result<(), W::Error> {
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

impl<'de, T: FromHciBytes<'de>, const N: usize> FromHciBytes<'de> for heapless::Vec<T, N> {
    fn from_hci_bytes(data: &'de [u8]) -> Result<(Self, &'de [u8]), FromHciBytesError> {
        match data.split_first() {
            Some((&len, mut data)) => {
                let mut vec = heapless::Vec::new();
                for _ in 0..len {
                    let (val, rest) = T::from_hci_bytes(data)?;
                    vec.push(val).or(Err(FromHciBytesError::InvalidValue))?;
                    data = rest;
                }
                Ok((vec, data))
            }
            None => Err(FromHciBytesError::InvalidSize),
        }
    }
}

impl<const N: usize> WriteHci for [u8; N] {
    fn size(&self) -> usize {
        N
    }

    fn write_hci<W: embedded_io::blocking::Write>(&self, mut writer: W) -> Result<(), W::Error> {
        writer.write_all(self)
    }

    #[cfg(feature = "async")]
    async fn write_hci_async<W: embedded_io::asynch::Write>(&self, mut writer: W) -> Result<(), W::Error> {
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
    fn size(&self) -> usize {
        self.as_ref().map(|x| x.size()).unwrap_or_default()
    }

    fn write_hci<W: embedded_io::blocking::Write>(&self, writer: W) -> Result<(), W::Error> {
        match self {
            Some(val) => val.write_hci(writer),
            None => Ok(()),
        }
    }

    async fn write_hci_async<W: embedded_io::asynch::Write>(&self, writer: W) -> Result<(), W::Error> {
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
