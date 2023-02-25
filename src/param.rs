pub mod cmds;
pub mod events;
pub mod features;
pub mod info;
pub mod le;
pub mod status;

pub trait CmdParam {
    /// The size (in bytes) of the parameter
    fn size(&self) -> usize;

    fn write<W: embedded_io::blocking::Write>(&self, writer: W) -> Result<(), W::Error>;

    #[cfg(feature = "async")]
    async fn write_async<W: embedded_io::asynch::Write>(&self, writer: W) -> Result<(), W::Error>;
}

pub enum FromBytesError {
    InvalidSize,
    InvalidValue,
}

pub trait EventParam<'de>: Sized {
    fn from_bytes(data: &'de [u8]) -> Result<(Self, usize), FromBytesError>;
}

macro_rules! impl_param_int {
    ($($ty:ty),+) => {
        $(
            impl CmdParam for $ty {
                fn size(&self) -> usize {
                    ::core::mem::size_of::<Self>()
                }

                fn write<W: ::embedded_io::blocking::Write>(&self, mut writer: W) -> Result<(), W::Error> {
                    writer.write_all(&self.to_le_bytes())
                }

                #[cfg(feature = "async")]
                async fn write_async<W: ::embedded_io::asynch::Write>(&self, mut writer: W) -> Result<(), W::Error> {
                    writer.write_all(&self.to_le_bytes()).await
                }
            }

            impl<'de> EventParam<'de> for $ty {
                fn from_bytes(data: &'de [u8]) -> Result<(Self, usize), FromBytesError> {
                    let size = ::core::mem::size_of::<Self>();
                    if data.len() >= size {
                        Ok((Self::from_le_bytes(unsafe { data[..size].try_into().unwrap_unchecked() }), size))
                    } else {
                        Err($crate::param::FromBytesError::InvalidSize)
                    }

                }
            }
        )+
    };
}

impl_param_int!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128);

impl CmdParam for bool {
    fn size(&self) -> usize {
        ::core::mem::size_of::<Self>()
    }
    fn write<W: ::embedded_io::blocking::Write>(&self, mut writer: W) -> Result<(), W::Error> {
        writer.write_all(&(*self as u8).to_le_bytes())
    }
    #[cfg(feature = "async")]
    async fn write_async<W: ::embedded_io::asynch::Write>(&self, mut writer: W) -> Result<(), W::Error> {
        writer.write_all(&(*self as u8).to_le_bytes()).await
    }
}

impl<'de> EventParam<'de> for bool {
    fn from_bytes(data: &'de [u8]) -> Result<(Self, usize), FromBytesError> {
        match data.first() {
            Some(0) => Ok((false, 1)),
            Some(1) => Ok((true, 1)),
            Some(_) => Err(FromBytesError::InvalidValue),
            None => Err(FromBytesError::InvalidSize),
        }
    }
}

impl<'a> CmdParam for &'a [u8] {
    fn size(&self) -> usize {
        self.len()
    }

    fn write<W: embedded_io::blocking::Write>(&self, mut writer: W) -> Result<(), W::Error> {
        writer.write_all(&[self.size() as u8])?;
        writer.write_all(self)
    }

    #[cfg(feature = "async")]
    async fn write_async<W: embedded_io::asynch::Write>(&self, mut writer: W) -> Result<(), W::Error> {
        writer.write_all(&[self.size() as u8]).await?;
        writer.write_all(self).await
    }
}

impl<'de: 'a, 'a> EventParam<'de> for &'a [u8] {
    fn from_bytes(data: &'de [u8]) -> Result<(Self, usize), FromBytesError> {
        match data.split_first() {
            Some((len, rest)) if usize::from(*len) <= rest.len() => Ok((rest, usize::from(*len))),
            _ => Err(FromBytesError::InvalidSize),
        }
    }
}

impl<'de: 'a, 'a, T: EventParam<'de>, const N: usize> EventParam<'de> for heapless::Vec<T, N> {
    fn from_bytes(data: &'de [u8]) -> Result<(Self, usize), FromBytesError> {
        let mut vec = heapless::Vec::new();
        match data.split_first() {
            Some((&count, mut data)) => {
                let mut total = 1;
                for _ in 0..count {
                    let (val, len) = T::from_bytes(data)?;
                    vec.push(val).or(Err(FromBytesError::InvalidValue))?;
                    data = &data[len..];
                    total += len;
                }
                Ok((vec, total))
            }
            _ => Err(FromBytesError::InvalidSize),
        }
    }
}

impl<const N: usize> CmdParam for [u8; N] {
    fn size(&self) -> usize {
        N
    }

    fn write<W: embedded_io::blocking::Write>(&self, mut writer: W) -> Result<(), W::Error> {
        writer.write_all(self)
    }

    #[cfg(feature = "async")]
    async fn write_async<W: embedded_io::asynch::Write>(&self, mut writer: W) -> Result<(), W::Error> {
        writer.write_all(self).await
    }
}

impl<'de, const N: usize> EventParam<'de> for [u8; N] {
    fn from_bytes(data: &'de [u8]) -> Result<(Self, usize), FromBytesError> {
        if data.len() >= N {
            Ok((unsafe { data[..N].try_into().unwrap_unchecked() }, N))
        } else {
            Err(FromBytesError::InvalidSize)
        }
    }
}

macro_rules! impl_param_tuple {
    ($($a:ident)*) => {
        #[automatically_derived]
        #[allow(non_snake_case)]
        impl<$($a: CmdParam,)*> CmdParam for ($($a,)*) {
            fn size(&self) -> usize {
                let ($(ref $a,)*) = *self;
                $($a.size() +)* 0
            }

            #[allow(unused_mut, unused_variables)]
            fn write<W: ::embedded_io::blocking::Write>(&self, mut writer: W) -> Result<(), W::Error> {
                let ($(ref $a,)*) = *self;
                $($a.write(&mut writer)?;)*
                Ok(())
            }

            #[cfg(feature = "async")]
            #[allow(unused_mut, unused_variables)]
            async fn write_async<W: ::embedded_io::asynch::Write>(&self, mut writer: W) -> Result<(), W::Error> {
                let ($(ref $a,)*) = *self;
                $($a.write_async(&mut writer).await?;)*
                Ok(())
            }
        }

        #[automatically_derived]
        #[allow(non_snake_case)]
        impl<'de, $($a: EventParam<'de>,)*> EventParam<'de> for ($($a,)*) {
            #[allow(unused_mut, unused_variables)]
            fn from_bytes(data: &'de [u8]) -> Result<(Self, usize), FromBytesError> {
                let total = 0;
                $(
                    let ($a, len) = $a::from_bytes(data)?;
                    let total = total + len;
                    let data = &data[len..];
                )*
                Ok((($($a,)*), total))
            }
        }
    };
}

impl_param_tuple! {}
impl_param_tuple! { A }
impl_param_tuple! { A B }
impl_param_tuple! { A B C }
impl_param_tuple! { A B C D }
impl_param_tuple! { A B C D E }
impl_param_tuple! { A B C D E F }
impl_param_tuple! { A B C D E F G }
impl_param_tuple! { A B C D E F G H }
impl_param_tuple! { A B C D E F G H I }
impl_param_tuple! { A B C D E F G H I J }
impl_param_tuple! { A B C D E F G H I J K }
impl_param_tuple! { A B C D E F G H I J K L }

macro_rules! param {
    (struct $name:ident($wrapped:ty)) => {
        $crate::param::param! {
            #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
            struct $name($wrapped);
        }
    };
    (
        #[derive($($derive:ty),*)]
        struct $name:ident($wrapped:ty);
    ) => {
        #[repr(transparent)]
        #[derive($($derive,)*)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub struct $name($wrapped);

        impl $name {
            pub fn into_inner(self) -> $wrapped {
                self.0
            }
        }

        impl $crate::param::CmdParam for $name {
            fn size(&self) -> usize {
                $crate::param::CmdParam::size(&self.0)
            }

            fn write<W: ::embedded_io::blocking::Write>(&self, writer: W) -> Result<(), W::Error> {
                <$wrapped as $crate::param::CmdParam>::write(&self.0, writer)
            }

            #[cfg(feature = "async")]
            async fn write_async<W: ::embedded_io::asynch::Write>(&self, writer: W) -> Result<(), W::Error> {
                <$wrapped as $crate::param::CmdParam>::write_async(&self.0, writer).await
            }
        }

        impl<'de> $crate::param::EventParam<'de> for $name {
            fn from_bytes(data: &'de [u8]) -> Result<(Self, usize), $crate::param::FromBytesError> {
                <$wrapped as $crate::param::EventParam>::from_bytes(data).map(|(x, y)| (Self(x), y))
            }
        }
    };

    (struct $name:ident {
        $($field:ident: $ty:ty),*
        $(,)?
    }) => {
        $crate::param::param! {
            #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
            struct $name {
                $($field: $ty,)*
            }
        }
    };
    (
        #[derive($($derive:ty),*)]
        struct $name:ident {
            $($field:ident: $ty:ty),*
            $(,)?
        }
    ) => {
        #[derive($($derive,)*)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub struct $name {
            pub $($field: $ty,)*
        }

        impl $crate::param::CmdParam for $name {
            fn size(&self) -> usize {
                $(<$ty as $crate::param::CmdParam>::size(&self.$field) +)* 0
            }

            fn write<W: ::embedded_io::blocking::Write>(&self, mut writer: W) -> Result<(), W::Error> {
                $(<$ty as $crate::param::CmdParam>::write(&self.$field, &mut writer)?;)*
                Ok(())
            }

            #[cfg(feature = "async")]
            async fn write_async<W: ::embedded_io::asynch::Write>(&self, mut writer: W) -> Result<(), W::Error> {
                $(<$ty as $crate::param::CmdParam>::write_async(&self.$field, &mut writer).await?;)*
                Ok(())
            }
        }

        impl<'de> $crate::param::EventParam<'de> for $name {
            #[allow(unused_variables)]
            fn from_bytes(data: &'de [u8]) -> Result<(Self, usize), $crate::param::FromBytesError> {
                let total = 0;
                $(
                    let ($field, len) = <$ty as $crate::param::EventParam>::from_bytes(data)?;
                    let total = total + len;
                    let data = &data[len..];
                )*
                Ok((Self {
                    $($field,)*
                }, total))
            }
        }
    };

    (
        enum $name:ident {
            $(
                $variant:ident = $value:expr,
            )+
        }
    ) => {
        $crate::param::param! {
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
            enum $name {
                $($variant = $value,)+
            }
        }
    };
    (
        #[derive($($derive:ty),* $(,)?)]
        enum $name:ident {
            $(
                $variant:ident = $value:expr,
            )+
        }
    ) => {
        #[repr(u8)]
        #[derive($($derive,)*)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum $name {
            $(
                $variant = $value,
            )+
        }

        impl $crate::param::CmdParam for $name {
            fn size(&self) -> usize {
                1
            }

            fn write<W: ::embedded_io::blocking::Write>(&self, writer: W) -> Result<(), W::Error> {
                <u8 as $crate::param::CmdParam>::write(&(*self as u8), writer)
            }

            #[cfg(feature = "async")]
            async fn write_async<W: ::embedded_io::asynch::Write>(&self, writer: W) -> Result<(), W::Error> {
                <u8 as $crate::param::CmdParam>::write_async(&(*self as u8), writer).await
            }
        }

        impl<'de> $crate::param::EventParam<'de> for $name {
            #[allow(unused_variables)]
            fn from_bytes(data: &'de [u8]) -> Result<(Self, usize), $crate::param::FromBytesError> {
                match data.first() {
                    Some(byte) => match byte {
                        $($value => Ok((Self::$variant, 1)),)+
                        _ => Err($crate::param::FromBytesError::InvalidValue),
                    }
                    None => Err($crate::param::FromBytesError::InvalidSize),
                }
            }
        }
    };

    (
        bitfield $name:ident[$octets:expr] {
            $(($bit:expr, $get:ident, $set:ident);)+
        }
    ) => {
        $crate::param::param! {
            #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
            bitfield $name[$octets] {
                $(($bit, $get, $set);)+
            }
        }
    };
    (
        #[derive($($derive:ty),*)]
        bitfield $name:ident[1] {
            $(($bit:expr, $get:ident, $set:ident);)+
        }
    ) => {
        #[repr(transparent)]
        #[derive($($derive,)*)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub struct $name(u8);

        impl $name {
            pub fn into_inner(self) -> u8 {
                self.0
            }

            $(
                pub const fn $get(&self) -> bool {
                    (self.0 & (1 << $bit)) != 0
                }

                pub const fn $set(self, val: bool) -> Self {
                    Self((self.0 & !(1 << $bit)) | ((val as u8) << $bit))
                }
            )+
        }

        impl $crate::param::CmdParam for $name {
            fn size(&self) -> usize {
                1
            }

            fn write<W: ::embedded_io::blocking::Write>(&self, writer: W) -> Result<(), W::Error> {
                <u8 as $crate::param::CmdParam>::write(&self.0, writer)
            }

            #[cfg(feature = "async")]
            #[allow(unused_mut)]
            async fn write_async<W: ::embedded_io::asynch::Write>(&self, mut writer: W) -> Result<(), W::Error> {
                <u8 as $crate::param::CmdParam>::write_async(&self.0, writer).await
            }
        }
    };
    (
        #[derive($($derive:ty),*)]
        bitfield $name:ident[$octets:expr] {
            $(($bit:expr, $get:ident, $set:ident);)+
        }
    ) => {
        #[repr(transparent)]
        #[derive($($derive,)*)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub struct $name([u8; $octets]);

        impl $name {
            pub fn into_inner(self) -> [u8; $octets] {
                self.0
            }

            $(
                pub const fn $get(&self) -> bool {
                    const OCTET: usize = $bit / 8;
                    const BIT: usize = $bit % 8;
                    (self.0[OCTET] & (1 << BIT)) != 0
                }

                pub const fn $set(mut self, val: bool) -> Self {
                    const OCTET: usize = $bit / 8;
                    const BIT: usize = $bit % 8;
                    self.0[OCTET] = (self.0[OCTET] & !(1 << BIT)) | ((val as u8) << BIT);
                    self
                }
            )+
        }

        impl $crate::param::CmdParam for $name {
            fn size(&self) -> usize {
                $octets
            }

            fn write<W: ::embedded_io::blocking::Write>(&self, writer: W) -> Result<(), W::Error> {
                <[u8; $octets] as $crate::param::CmdParam>::write(&self.0, writer)
            }

            #[cfg(feature = "async")]
            #[allow(unused_mut)]
            async fn write_async<W: ::embedded_io::asynch::Write>(&self, mut writer: W) -> Result<(), W::Error> {
                <[u8; $octets] as $crate::param::CmdParam>::write_async(&self.0, writer).await
            }
        }

        impl<'de> $crate::param::EventParam<'de> for $name {
            fn from_bytes(data: &'de [u8]) -> Result<(Self, usize), $crate::param::FromBytesError> {
                <[u8; $octets] as $crate::param::EventParam>::from_bytes(data).map(|(x,y)| (Self(x), y))
            }
        }
    };

    (&$life:lifetime [$el:ty]) => {
        impl<$life> $crate::param::CmdParam for &$life [$el] {
            fn size(&self) -> usize {
                1 + self.iter().map($crate::param::CmdParam::size).sum::<usize>()
            }

            fn write<W: ::embedded_io::blocking::Write>(&self, mut writer: W) -> Result<(), W::Error> {
                writer.write_all(&[self.len() as u8])?;
                for x in self.iter() {
                    <$el as $crate::param::CmdParam>::write(x, &mut writer)?;
                }
                Ok(())
            }

            #[cfg(feature = "async")]
            async fn write_async<W: ::embedded_io::asynch::Write>(&self, mut writer: W) -> Result<(), W::Error> {
                writer.write_all(&[self.len() as u8]).await?;
                for x in self.iter() {
                    <$el as $crate::param::CmdParam>::write_async(x, &mut writer).await?;
                }
                Ok(())
            }
        }
    };
}

pub(crate) use param;

param!(struct BdAddr([u8; 6]));

impl BdAddr {
    pub fn new(val: [u8; 6]) -> Self {
        Self(val)
    }
}

param!(struct ConnHandle(u16));

impl ConnHandle {
    pub fn new(val: u16) -> Self {
        assert!(val <= 0xeff);
        Self(val)
    }
}

#[repr(transparent)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Duration<const N: u32 = 1>(u16);

impl<const N: u32> CmdParam for Duration<N> {
    fn size(&self) -> usize {
        CmdParam::size(&self.0)
    }

    fn write<W: ::embedded_io::blocking::Write>(&self, writer: W) -> Result<(), W::Error> {
        self.0.write(writer)
    }

    #[cfg(feature = "async")]
    #[allow(unused_mut)]
    async fn write_async<W: ::embedded_io::asynch::Write>(&self, writer: W) -> Result<(), W::Error> {
        self.0.write_async(writer).await
    }
}

impl<'de, const N: u32> EventParam<'de> for Duration<N> {
    fn from_bytes(data: &'de [u8]) -> Result<(Self, usize), FromBytesError> {
        u16::from_bytes(data).map(|(x, y)| (Self(x), y))
    }
}

impl<const N: u32> Duration<N> {
    pub fn from_u16(val: u16) -> Self {
        Self(val)
    }

    pub fn from_micros(val: u32) -> Self {
        Self::from_u16((val / (625 * N)) as u16)
    }

    pub fn from_millis(val: u32) -> Self {
        Self::from_u16((unwrap!(val.checked_mul(8)) / (5 * N)) as u16)
    }

    pub fn from_secs(val: u32) -> Self {
        Self::from_millis(unwrap!(val.checked_mul(1000)))
    }

    pub fn as_u16(&self) -> u16 {
        self.0
    }

    pub fn as_micros(&self) -> u32 {
        u32::from(self.as_u16()) * (625 * N)
    }

    pub fn as_millis(&self) -> u32 {
        (u32::from(self.as_u16()) * (5 * N)) / 8
    }

    pub fn as_secs(&self) -> u32 {
        self.as_millis() / 1000
    }
}
