use crate::param::{param, Status};
use crate::{FromHciBytes, FromHciBytesError, ReadHci, ReadHciError};

pub trait EventParams<'a>: FromHciBytes<'a> {
    const CODE: u8;
}

param! {
    struct EventPacketHeader {
        code: u8,
        params_len: u8,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct EventPacket<'a> {
    code: u8,
    params: &'a [u8],
}

impl<'a> EventPacket<'a> {
    pub fn from_header_hci_bytes(header: EventPacketHeader, data: &'a [u8]) -> Result<Self, FromHciBytesError> {
        let params_len = usize::from(header.params_len);
        if data.len() != params_len {
            Err(FromHciBytesError::InvalidSize)
        } else {
            Ok(Self {
                code: header.code,
                params: data,
            })
        }
    }

    pub fn code(&self) -> u8 {
        self.code
    }

    pub fn param_bytes(&self) -> &[u8] {
        self.params
    }

    pub fn params<P: EventParams<'a>>(&self) -> Result<P, FromHciBytesError> {
        assert_eq!(self.code, P::CODE);
        match P::from_hci_bytes(self.params) {
            Ok((val, rest)) if rest.is_empty() => Ok(val),
            Ok(_) => Err(FromHciBytesError::InvalidSize),
            Err(err) => Err(err),
        }
    }
}

impl<'de> FromHciBytes<'de> for EventPacket<'de> {
    fn from_hci_bytes(data: &'de [u8]) -> Result<(Self, &'de [u8]), FromHciBytesError> {
        let (header, data) = EventPacketHeader::from_hci_bytes(data)?;
        Self::from_header_hci_bytes(header, data).map(|x| (x, &[] as &[u8]))
    }
}

impl<'de> ReadHci<'de> for EventPacket<'de> {
    fn read_hci<R: embedded_io::blocking::Read>(
        mut reader: R,
        buf: &'de mut [u8],
    ) -> Result<Self, ReadHciError<R::Error>> {
        let mut header = [0; 4];
        reader.read_exact(&mut header)?;
        let (header, _) = EventPacketHeader::from_hci_bytes(&header)?;
        let params_len = usize::from(header.params_len);
        if buf.len() < params_len {
            Err(ReadHciError::BufferTooSmall)
        } else {
            let (buf, _) = buf.split_at_mut(params_len);
            reader.read_exact(buf)?;
            Self::from_header_hci_bytes(header, buf).map_err(Into::into)
        }
    }

    #[cfg(feature = "async")]
    async fn read_hci_async<R: embedded_io::asynch::Read>(
        mut reader: R,
        buf: &'de mut [u8],
    ) -> Result<Self, ReadHciError<R::Error>> {
        let mut header = [0; 4];
        reader.read_exact(&mut header).await?;
        let (header, _) = EventPacketHeader::from_hci_bytes(&header)?;
        let params_len = usize::from(header.params_len);
        if buf.len() < params_len {
            Err(ReadHciError::BufferTooSmall)
        } else {
            let (buf, _) = buf.split_at_mut(params_len);
            reader.read_exact(buf).await?;
            Self::from_header_hci_bytes(header, buf).map_err(Into::into)
        }
    }
}

macro_rules! event {
    (struct $name:ident$(<$life:lifetime>)?($code:expr) {
        $($field:ident: $ty:ty),*
        $(,)?
    }) => {
        $crate::event::event! {
            #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
            struct $name$(<$life>)?($code) {
                $($field: $ty,)*
            }
        }
    };
    (
        #[derive($($derive:ty),*)]
        struct $name:ident$(<$life:lifetime>)?($code:expr) {
            $($field:ident: $ty:ty),*
            $(,)?
        }
    ) => {
        #[derive($($derive,)*)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub struct $name$(<$life>)? {
            pub $($field: $ty,)*
        }

        impl<$($life,)? 'de> $crate::FromHciBytes<'de> for $name $(where 'de: $life)? {
            #[allow(unused_variables)]
            fn from_hci_bytes(data: &'de [u8]) -> Result<(Self, &'de [u8]), $crate::FromHciBytesError> {
                let total = 0;
                $(
                    let ($field, data) = <$ty as $crate::FromHciBytes>::from_hci_bytes(data)?;
                )*
                Ok((Self {
                    $($field,)*
                }, data))
            }
        }

        #[automatically_derived]
        #[allow(unused_mut, unused_variables, unused_imports)]
        impl<$($life,)? 'de> $crate::event::EventParams<'de> for $name$(<$life>)? $(where 'de: $life)? {
            const CODE: u8 = $code;
        }
    };
}

pub(crate) use event;

event! {
    struct InquiryComplete(0x01) {
        status: Status,
    }
}
