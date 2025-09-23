//! Parameter types for HCI command and event packets [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-8af7a4d8-7a08-0895-b041-fdf9e27d6508)

use crate::{AsHciBytes, ByteAlignedValue, FixedSizeValue, FromHciBytes, FromHciBytesError, WriteHci};

mod classic;
mod cmd_mask;
mod event_masks;
mod feature_masks;
mod le;
mod macros;
mod primitives;
mod status;

pub use classic::*;
pub use cmd_mask::*;
pub use event_masks::*;
pub use feature_masks::*;
pub use le::*;
pub(crate) use macros::{param, param_slice};
pub use status::*;

/// A special parameter which takes all remaining bytes in the buffer
#[repr(transparent)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct RemainingBytes<'a>(&'a [u8]);

impl core::ops::Deref for RemainingBytes<'_> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl WriteHci for RemainingBytes<'_> {
    #[inline(always)]
    fn size(&self) -> usize {
        self.0.len()
    }

    #[inline(always)]
    fn write_hci<W: embedded_io::Write>(&self, mut writer: W) -> Result<(), W::Error> {
        writer.write_all(self.0)
    }

    #[inline(always)]
    async fn write_hci_async<W: embedded_io_async::Write>(&self, mut writer: W) -> Result<(), W::Error> {
        writer.write_all(self.0).await
    }
}

impl AsHciBytes for RemainingBytes<'_> {
    fn as_hci_bytes(&self) -> &[u8] {
        self.0
    }
}

impl<'a> FromHciBytes<'a> for RemainingBytes<'a> {
    fn from_hci_bytes(data: &'a [u8]) -> Result<(Self, &'a [u8]), FromHciBytesError> {
        Ok((RemainingBytes(data), &[]))
    }
}

impl<'a> RemainingBytes<'a> {
    pub(crate) fn into_inner(self) -> &'a [u8] {
        self.0
    }
}

param!(struct BdAddr([u8; 6]));

impl BdAddr {
    /// Create a new instance.
    pub fn new(val: [u8; 6]) -> Self {
        Self(val)
    }

    /// Get the byte representation.
    pub fn raw(&self) -> &[u8] {
        &self.0[..]
    }
}

unsafe impl ByteAlignedValue for BdAddr {}

impl<'de> crate::FromHciBytes<'de> for &'de BdAddr {
    #[inline(always)]
    fn from_hci_bytes(data: &'de [u8]) -> Result<(Self, &'de [u8]), crate::FromHciBytesError> {
        <BdAddr as crate::ByteAlignedValue>::ref_from_hci_bytes(data)
    }
}

param!(struct ConnHandle(u16));

impl ConnHandle {
    /// Create a new instance.
    pub fn new(val: u16) -> Self {
        assert!(val <= 0xeff);
        Self(val)
    }

    /// Get the underlying representation.
    pub fn raw(&self) -> u16 {
        self.0
    }
}

/// A 16-bit duration. The `US` generic parameter indicates the timebase in µs.
#[repr(transparent)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Duration<const US: u32 = 625>(u16);

unsafe impl<const US: u32> FixedSizeValue for Duration<US> {
    #[inline(always)]
    fn is_valid(_data: &[u8]) -> bool {
        true
    }
}

impl<const US: u32> Duration<US> {
    #[inline(always)]
    /// Create a new instance from raw value.
    pub const fn from_u16(val: u16) -> Self {
        Self(val)
    }

    /// Create an instance from microseconds.
    #[inline(always)]
    pub fn from_micros(val: u64) -> Self {
        Self::from_u16(unwrap!((val / u64::from(US)).try_into()))
    }

    /// Create an instance from milliseconds.
    #[inline(always)]
    pub fn from_millis(val: u32) -> Self {
        Self::from_micros(u64::from(val) * 1000)
    }

    /// Create an instance from seconds.
    #[inline(always)]
    pub fn from_secs(val: u32) -> Self {
        Self::from_micros(u64::from(val) * 1_000_000)
    }

    /// Get the underlying representation.
    #[inline(always)]
    pub fn as_u16(&self) -> u16 {
        self.0
    }

    /// Get value as microseconds.
    #[inline(always)]
    pub fn as_micros(&self) -> u64 {
        u64::from(self.as_u16()) * u64::from(US)
    }

    /// Get value as milliseconds.
    #[inline(always)]
    pub fn as_millis(&self) -> u32 {
        unwrap!((self.as_micros() / 1000).try_into())
    }

    /// Get value as seconds.
    #[inline(always)]
    pub fn as_secs(&self) -> u32 {
        // (u16::MAX * u32::MAX / 1_000_000) < u32::MAX so this is safe
        (self.as_micros() / 1_000_000) as u32
    }
}

#[cfg(feature = "embassy-time")]
impl<const US: u32> From<embassy_time::Duration> for Duration<US> {
    fn from(duration: embassy_time::Duration) -> Self {
        Self::from_micros(duration.as_micros())
    }
}

/// A 24-bit isochronous duration (in microseconds)
#[repr(transparent)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ExtDuration<const US: u16 = 1>([u8; 3]);

unsafe impl<const US: u16> FixedSizeValue for ExtDuration<US> {
    #[inline(always)]
    fn is_valid(_data: &[u8]) -> bool {
        true
    }
}

unsafe impl<const US: u16> ByteAlignedValue for ExtDuration<US> {}

impl<'de, const US: u16> FromHciBytes<'de> for &'de ExtDuration<US> {
    #[inline(always)]
    fn from_hci_bytes(data: &'de [u8]) -> Result<(Self, &'de [u8]), crate::FromHciBytesError> {
        <ExtDuration<US> as crate::ByteAlignedValue>::ref_from_hci_bytes(data)
    }
}

impl<const US: u16> ExtDuration<US> {
    /// Create a new instance from raw value.
    #[inline(always)]
    pub fn from_u32(val: u32) -> Self {
        assert!(val < (1 << 24));
        Self(*unwrap!(val.to_le_bytes().first_chunk()))
    }

    /// Create an instance from microseconds.
    #[inline(always)]
    pub fn from_micros(val: u64) -> Self {
        Self::from_u32(unwrap!((val / u64::from(US)).try_into()))
    }

    /// Create an instance from milliseconds.
    #[inline(always)]
    pub fn from_millis(val: u32) -> Self {
        Self::from_micros(u64::from(val) * 1000)
    }

    /// Create an instance from seconds.
    #[inline(always)]
    pub fn from_secs(val: u32) -> Self {
        Self::from_micros(u64::from(val) * 1_000_000)
    }

    /// Get value as microseconds.
    #[inline(always)]
    pub fn as_micros(&self) -> u64 {
        u64::from_le_bytes([self.0[0], self.0[1], self.0[2], 0, 0, 0, 0, 0]) * u64::from(US)
    }

    /// Get value as milliseconds.
    #[inline(always)]
    pub fn as_millis(&self) -> u32 {
        // ((1 << 24 - 1) * u16::MAX / 1_000) < u32::MAX so this is safe
        (self.as_micros() / 1000) as u32
    }

    /// Get value as seconds.
    #[inline(always)]
    pub fn as_secs(&self) -> u32 {
        // ((1 << 24 - 1) * u16::MAX / 1_000_000) < u32::MAX so this is safe
        (self.as_micros() / 1_000_000) as u32
    }
}

#[cfg(feature = "embassy-time")]
impl<const US: u16> From<embassy_time::Duration> for ExtDuration<US> {
    fn from(duration: embassy_time::Duration) -> Self {
        Self::from_micros(duration.as_micros())
    }
}

param!(
    enum DisconnectReason {
        AuthenticationFailure = 0x05,
        RemoteUserTerminatedConn = 0x13,
        RemoteDeviceTerminatedConnLowResources = 0x14,
        RemoteDeviceTerminatedConnPowerOff = 0x15,
        UnsupportedRemoteFeature = 0x1A,
        PairingWithUnitKeyNotSupported = 0x29,
        UnacceptableConnParameters = 0x3b,
    }
);

param!(
    enum RemoteConnectionParamsRejectReason {
        UnacceptableConnParameters = 0x3b,
    }
);

param! {
    #[derive(Default)]
    enum PowerLevelKind {
        #[default]
        Current = 0,
        Maximum = 1,
    }
}

param! {
    #[derive(Default)]
    enum ControllerToHostFlowControl {
        #[default]
        Off = 0,
        AclOnSyncOff = 1,
        AclOffSyncOn = 2,
        BothOn = 3,
    }
}

param!(struct CoreSpecificationVersion(u8));

#[allow(missing_docs)]
impl CoreSpecificationVersion {
    pub const VERSION_1_0B: CoreSpecificationVersion = CoreSpecificationVersion(0x00);
    pub const VERSION_1_1: CoreSpecificationVersion = CoreSpecificationVersion(0x01);
    pub const VERSION_1_2: CoreSpecificationVersion = CoreSpecificationVersion(0x02);
    pub const VERSION_2_0_EDR: CoreSpecificationVersion = CoreSpecificationVersion(0x03);
    pub const VERSION_2_1_EDR: CoreSpecificationVersion = CoreSpecificationVersion(0x04);
    pub const VERSION_3_0_HS: CoreSpecificationVersion = CoreSpecificationVersion(0x05);
    pub const VERSION_4_0: CoreSpecificationVersion = CoreSpecificationVersion(0x06);
    pub const VERSION_4_1: CoreSpecificationVersion = CoreSpecificationVersion(0x07);
    pub const VERSION_4_2: CoreSpecificationVersion = CoreSpecificationVersion(0x08);
    pub const VERSION_5_0: CoreSpecificationVersion = CoreSpecificationVersion(0x09);
    pub const VERSION_5_1: CoreSpecificationVersion = CoreSpecificationVersion(0x0A);
    pub const VERSION_5_2: CoreSpecificationVersion = CoreSpecificationVersion(0x0B);
    pub const VERSION_5_3: CoreSpecificationVersion = CoreSpecificationVersion(0x0C);
    pub const VERSION_5_4: CoreSpecificationVersion = CoreSpecificationVersion(0x0D);
}

unsafe impl ByteAlignedValue for CoreSpecificationVersion {}

impl<'de> crate::FromHciBytes<'de> for &'de CoreSpecificationVersion {
    #[inline(always)]
    fn from_hci_bytes(data: &'de [u8]) -> Result<(Self, &'de [u8]), crate::FromHciBytesError> {
        <CoreSpecificationVersion as crate::ByteAlignedValue>::ref_from_hci_bytes(data)
    }
}

param! {
    #[derive(Default)]
    enum LinkType {
        #[default]
        SyncData = 0,
        AclData = 1,
        IsoData = 2,
    }
}

param_slice! {
    [ConnHandleCompletedPackets; 4] {
        handle[0]: ConnHandle,
        num_completed_packets[2]: u16,
    }
}

impl ConnHandleCompletedPackets {
    /// Create a new instance.
    pub fn new(handle: ConnHandle, num_completed_packets: u16) -> Self {
        let mut dest = [0; 4];
        handle.write_hci(&mut dest[0..2]).unwrap();
        num_completed_packets.write_hci(&mut dest[2..4]).unwrap();
        Self(dest)
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "serde")]
    use postcard;

    use super::*;

    #[test]
    fn test_encode_decode_conn_handle_completed_packets() {
        let completed = ConnHandleCompletedPackets::new(ConnHandle::new(42), 2334);

        assert_eq!(completed.handle().unwrap(), ConnHandle::new(42));
        assert_eq!(completed.num_completed_packets().unwrap(), 2334);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_bdaddr() {
        let bytes = [0x01, 0xaa, 0x55, 0x04, 0x05, 0xfe];

        let address = BdAddr::new(bytes);

        let mut buffer = [0u8; 32];
        let vykort = postcard::to_slice(&address, &mut buffer).unwrap();

        assert_eq!(vykort, &bytes);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_deserialize_bdaddr() {
        let bytes = [0xff, 0x5a, 0xa5, 0x00, 0x05, 0xfe];

        let address = postcard::from_bytes::<BdAddr>(&bytes).unwrap();

        let expected = BdAddr::new(bytes);

        assert_eq!(address, expected);
    }
}
