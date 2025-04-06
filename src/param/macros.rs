macro_rules! impl_param_int {
    ($($ty:ty),+) => {
        $(
            #[automatically_derived]
            unsafe impl $crate::FixedSizeValue for $ty {
                #[inline(always)]
                fn is_valid(data: &[u8]) -> bool {
                    true
                }
            }
        )+
    };
}

const _IS_LITTLE_ENDIAN: [u8; 0] = [0; (u32::from_le_bytes(0x01020304u32.to_ne_bytes()) != 0x01020304u32) as usize];

impl_param_int!(u8, i8, u16, i16, u32, u64, u128);

unsafe impl crate::ByteAlignedValue for u8 {}
unsafe impl crate::ByteAlignedValue for i8 {}

impl<'de> crate::FromHciBytes<'de> for &'de u8 {
    #[inline(always)]
    fn from_hci_bytes(data: &'de [u8]) -> Result<(Self, &'de [u8]), crate::FromHciBytesError> {
        <u8 as crate::ByteAlignedValue>::ref_from_hci_bytes(data)
    }
}

impl<'de> crate::FromHciBytes<'de> for &'de i8 {
    #[inline(always)]
    fn from_hci_bytes(data: &'de [u8]) -> Result<(Self, &'de [u8]), crate::FromHciBytesError> {
        <i8 as crate::ByteAlignedValue>::ref_from_hci_bytes(data)
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! param {
    (
        $(#[$attrs:meta])*
        struct $name:ident($wrapped:ty)
    ) => {
        $(#[$attrs])*
        #[repr(transparent)]
        #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
        /// $name
        pub struct $name($wrapped);

        impl $name {
            /// Get inner representation
            pub fn into_inner(self) -> $wrapped {
                self.0
            }
        }

        unsafe impl $crate::FixedSizeValue for $name {
            #[inline(always)]
            fn is_valid(data: &[u8]) -> bool {
                <$wrapped as $crate::FixedSizeValue>::is_valid(data)
            }
        }
    };

    (
        $(#[$attrs:meta])*
        struct $name:ident {
            $($field:ident: $ty:ty),*
            $(,)?
        }
    ) => {
        $(#[$attrs])*
        #[repr(C, packed)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        /// $name parameter
        #[allow(missing_docs)]
        pub struct $name {
            $(pub $field: $ty,)*
        }

        #[cfg(feature = "defmt")]
        impl defmt::Format for $name {
            fn format(&self, f: defmt::Formatter) {
                // Copy out the field values since we can't take references to packed fields
                let Self { $($field),* } = *self;

                defmt::write!(f, "{} {{ ", stringify!($name));
                $(defmt::write!(f, "{}: {}, ", stringify!($field), $field);)*
                defmt::write!(f, "}}");
            }
        }

        #[automatically_derived]
        unsafe impl $crate::FixedSizeValue for $name {
            #[inline(always)]
            fn is_valid(data: &[u8]) -> bool {
                true
                $(
                    && <$ty as $crate::FixedSizeValue>::is_valid(&data[core::mem::offset_of!(Self, $field)..])
                )*
            }
        }

        unsafe impl $crate::ByteAlignedValue for $name {}

        impl<'de> $crate::FromHciBytes<'de> for &'de $name {
            #[inline(always)]
            fn from_hci_bytes(data: &'de [u8]) -> Result<(Self, &'de [u8]), $crate::FromHciBytesError> {
                <$name as $crate::ByteAlignedValue>::ref_from_hci_bytes(data)
            }
        }
    };

    (
        $(#[$attrs:meta])*
        struct $name:ident$(<$life:lifetime>)? {
            $($field:ident: $ty:ty),*
            $(,)?
        }
    ) => {
        $(#[$attrs])*
        #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        /// $name parameter
        #[allow(missing_docs)]
        pub struct $name$(<$life>)? {
            $(pub $field: $ty,)*
        }

        impl$(<$life>)? $crate::WriteHci for $name$(<$life>)? {
            #[inline(always)]
            fn size(&self) -> usize {
                $(<$ty as $crate::WriteHci>::size(&self.$field) +)* 0
            }

            #[inline(always)]
            fn write_hci<W: ::embedded_io::Write>(&self, mut writer: W) -> Result<(), W::Error> {
                $(<$ty as $crate::WriteHci>::write_hci(&self.$field, &mut writer)?;)*
                Ok(())
            }

            #[inline(always)]
            async fn write_hci_async<W: ::embedded_io_async::Write>(&self, mut writer: W) -> Result<(), W::Error> {
                $(<$ty as $crate::WriteHci>::write_hci_async(&self.$field, &mut writer).await?;)*
                Ok(())
            }
        }

        impl<$($life, )?'de> $crate::FromHciBytes<'de> for $name$(<$life> where 'de: $life, $life: 'de)? {
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
    };

    (
        $(#[$attrs:meta])*
        enum $name:ident {
            $(
                $(#[$variant_attrs:meta])*
                $variant:ident = $value:expr,
            )+
        }
    ) => {
        $(#[$attrs])*
        #[repr(u8)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        #[allow(missing_docs)]
        /// $name.
        pub enum $name {
            $(
                $(#[$variant_attrs])*
                $variant = $value,
            )+
        }

        unsafe impl $crate::FixedSizeValue for $name {
            #[inline(always)]
            fn is_valid(data: &[u8]) -> bool {
                $(data[0] == $value ||)* false
            }
        }

        unsafe impl $crate::ByteAlignedValue for $name {}

        impl<'de> $crate::FromHciBytes<'de> for &'de $name {
            #[inline(always)]
            fn from_hci_bytes(data: &'de [u8]) -> Result<(Self, &'de [u8]), $crate::FromHciBytesError> {
                <$name as $crate::ByteAlignedValue>::ref_from_hci_bytes(data)
            }
        }
    };

    (
        $(#[$attrs:meta])*
        bitfield $name:ident[1] {
            $(($bit:expr, $get:ident, $set:ident);)+
        }
    ) => {
        $(#[$attrs])*
        #[repr(transparent)]
        #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        /// $name.
        pub struct $name(u8);

        impl $name {
            /// Create a new instance.
            pub fn new() -> Self {
                Self::default()
            }

            /// Get the inner representation.
            pub fn into_inner(self) -> u8 {
                self.0
            }

            $(
                #[allow(missing_docs)]
                pub const fn $get(&self) -> bool {
                    (self.0 & (1 << $bit)) != 0
                }

                #[allow(missing_docs)]
                pub const fn $set(self, val: bool) -> Self {
                    Self((self.0 & !(1 << $bit)) | ((val as u8) << $bit))
                }
            )+
        }

        unsafe impl $crate::FixedSizeValue for $name {
            #[inline(always)]
            fn is_valid(_data: &[u8]) -> bool {
                true
            }
        }

        unsafe impl $crate::ByteAlignedValue for $name {}

        impl<'de> $crate::FromHciBytes<'de> for &'de $name {
            #[inline(always)]
            fn from_hci_bytes(data: &'de [u8]) -> Result<(Self, &'de [u8]), $crate::FromHciBytesError> {
                <$name as $crate::ByteAlignedValue>::ref_from_hci_bytes(data)
            }
        }
    };
    (
        $(#[$attrs:meta])*
        bitfield $name:ident[$octets:expr] {
            $(($bit:expr, $get:ident, $set:ident);)+
        }
    ) => {
        $(#[$attrs])*
        #[repr(transparent)]
        #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        /// $name
        pub struct $name([u8; $octets]);

        impl $name {
            /// Create a new instance.
            pub fn new() -> Self {
                Self::default()
            }

            /// Get the inner representation.
            pub fn into_inner(self) -> [u8; $octets] {
                self.0
            }

            $(
                #[allow(missing_docs)]
                pub const fn $get(&self) -> bool {
                    const OCTET: usize = $bit / 8;
                    const BIT: usize = $bit % 8;
                    (self.0[OCTET] & (1 << BIT)) != 0
                }

                #[allow(missing_docs)]
                pub const fn $set(mut self, val: bool) -> Self {
                    const OCTET: usize = $bit / 8;
                    const BIT: usize = $bit % 8;
                    self.0[OCTET] = (self.0[OCTET] & !(1 << BIT)) | ((val as u8) << BIT);
                    self
                }
            )+
        }

        unsafe impl $crate::FixedSizeValue for $name {
            #[inline(always)]
            fn is_valid(_data: &[u8]) -> bool {
                true
            }
        }

        unsafe impl $crate::ByteAlignedValue for $name {}

        impl<'de> $crate::FromHciBytes<'de> for &'de $name {
            #[inline(always)]
            fn from_hci_bytes(data: &'de [u8]) -> Result<(Self, &'de [u8]), $crate::FromHciBytesError> {
                <$name as $crate::ByteAlignedValue>::ref_from_hci_bytes(data)
            }
        }
    };
}

macro_rules! param_slice {
    (&$life:lifetime [$el:ty]) => {
        impl<$life> $crate::WriteHci for &$life [$el] {
            #[inline(always)]
            fn size(&self) -> usize {
                1 + self.iter().map($crate::WriteHci::size).sum::<usize>()
            }

            #[inline(always)]
            fn write_hci<W: ::embedded_io::Write>(&self, mut writer: W) -> Result<(), W::Error> {
                writer.write_all(&[self.len() as u8])?;
                for x in self.iter() {
                    <$el as $crate::WriteHci>::write_hci(x, &mut writer)?;
                }
                Ok(())
            }

            #[inline(always)]
            async fn write_hci_async<W: ::embedded_io_async::Write>(&self, mut writer: W) -> Result<(), W::Error> {
                writer.write_all(&[self.len() as u8]).await?;
                for x in self.iter() {
                    <$el as $crate::WriteHci>::write_hci_async(x, &mut writer).await?;
                }
                Ok(())
            }
        }
    };

    (
        $(#[$attrs:meta])*
        [$name:ident; $octets:expr] {
            $($field:ident[$off:expr]: $ty:ty),*
            $(,)?
        }
    ) => {
        $(#[$attrs])*
        #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        /// $name
        pub struct $name([u8; $octets]);

        impl $name {
            $(
                /// Get value of $field
                pub fn $field(&self) -> Result<$ty, $crate::FromHciBytesError> {
                    <$ty as $crate::FromHciBytes>::from_hci_bytes(&self.0[$off..]).map(|(x, _)| x)
                }
            )+
        }

        impl<'a> $crate::WriteHci for &'a [$name] {
            #[inline(always)]
            fn size(&self) -> usize {
                1 + self.len() * $octets
            }

            #[inline(always)]
            fn write_hci<W: ::embedded_io::Write>(&self, mut writer: W) -> Result<(), W::Error> {
                writer.write_all(&[self.len() as u8])?;
                for x in self.iter() {
                    <[u8; $octets] as $crate::WriteHci>::write_hci(&x.0, &mut writer)?;
                }
                Ok(())
            }

            #[inline(always)]
            async fn write_hci_async<W: ::embedded_io_async::Write>(&self, mut writer: W) -> Result<(), W::Error> {
                writer.write_all(&[self.len() as u8]).await?;
                for x in self.iter() {
                    <[u8; $octets] as $crate::WriteHci>::write_hci_async(&x.0, &mut writer).await?;
                }
                Ok(())
            }
        }

        impl<'a, 'de: 'a> $crate::FromHciBytes<'de> for &'a [$name] {
            #[allow(unused_variables)]
            fn from_hci_bytes(data: &'de [u8]) -> Result<(Self, &'de [u8]), $crate::FromHciBytesError> {
                match data.split_first() {
                    Some((&len, data)) => {
                        let len = usize::from(len);
                        let size = $octets * len;
                        if data.len() >= size {
                            let (data, rest) = data.split_at(size);
                            // Safety: $name has align of 1, no padding, and all bit patterns are valid
                            let slice = unsafe { core::slice::from_raw_parts(data.as_ptr() as *const _, len) };
                            Ok((slice, rest))
                        } else {
                            Err($crate::FromHciBytesError::InvalidSize)
                        }
                    }
                    None => Err($crate::FromHciBytesError::InvalidSize),
                }
            }
        }
    };
}

pub use param;
pub(crate) use param_slice;
