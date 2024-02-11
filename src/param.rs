//! Parameter types for Bluetooth HCI command and event packets.

use crate::{FromHciBytes, FromHciBytesError, WriteHci};

mod cmd_mask;
mod event_masks;
mod feature_masks;
mod le;
mod macros;
mod primitives;
mod status;

pub use cmd_mask::*;
pub use event_masks::*;
pub use feature_masks::*;
pub use le::*;
pub(crate) use macros::param;
pub use status::*;

/// A special parameter which takes all remaining bytes in the buffer
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct RemainingBytes<'a>(&'a [u8]);

impl<'a> core::ops::Deref for RemainingBytes<'a> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a> WriteHci for RemainingBytes<'a> {
    fn size(&self) -> usize {
        self.0.len()
    }

    fn write_hci<W: embedded_io::Write>(&self, mut writer: W) -> Result<(), W::Error> {
        writer.write_all(self.0)
    }

    async fn write_hci_async<W: embedded_io_async::Write>(&self, mut writer: W) -> Result<(), W::Error> {
        writer.write_all(self.0).await
    }
}

impl<'a> FromHciBytes<'a> for RemainingBytes<'a> {
    fn from_hci_bytes(data: &'a [u8]) -> Result<(Self, &'a [u8]), FromHciBytesError> {
        Ok((RemainingBytes(data), &[]))
    }
}

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

/// A 16-bit duration. The `US` generic paramter indicates the timebase in µs.
#[repr(transparent)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Duration<const US: u32 = 625>(u16);

impl<const US: u32> WriteHci for Duration<US> {
    fn size(&self) -> usize {
        WriteHci::size(&self.0)
    }

    fn write_hci<W: ::embedded_io::Write>(&self, writer: W) -> Result<(), W::Error> {
        self.0.write_hci(writer)
    }

    #[allow(unused_mut)]
    async fn write_hci_async<W: ::embedded_io_async::Write>(&self, writer: W) -> Result<(), W::Error> {
        self.0.write_hci_async(writer).await
    }
}

impl<'de, const US: u32> FromHciBytes<'de> for Duration<US> {
    fn from_hci_bytes(data: &'de [u8]) -> Result<(Self, &'de [u8]), FromHciBytesError> {
        u16::from_hci_bytes(data).map(|(x, y)| (Self(x), y))
    }
}

impl<const US: u32> Duration<US> {
    pub fn from_u16(val: u16) -> Self {
        Self(val)
    }

    pub fn from_micros(val: u64) -> Self {
        Self::from_u16(unwrap!((val / u64::from(US)).try_into()))
    }

    pub fn from_millis(val: u32) -> Self {
        Self::from_micros(u64::from(val) * 1000)
    }

    pub fn from_secs(val: u32) -> Self {
        Self::from_micros(u64::from(val) * 1_000_000)
    }

    pub fn as_u16(&self) -> u16 {
        self.0
    }

    pub fn as_micros(&self) -> u64 {
        u64::from(self.as_u16()) * u64::from(US)
    }

    pub fn as_millis(&self) -> u32 {
        unwrap!((self.as_micros() / 1000).try_into())
    }

    pub fn as_secs(&self) -> u32 {
        // (u16::MAX * u32::MAX / 1_000_000) < u32::MAX so this is safe
        (self.as_micros() / 1_000_000) as u32
    }
}

/// A 24-bit isochronous duration (in microseconds)
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct IsoDuration(u32);

impl WriteHci for IsoDuration {
    fn size(&self) -> usize {
        3
    }

    fn write_hci<W: ::embedded_io::Write>(&self, writer: W) -> Result<(), W::Error> {
        let bytes: [u8; 3] = unsafe { self.0.to_le_bytes()[..3].try_into().unwrap_unchecked() };
        bytes.write_hci(writer)
    }

    #[allow(unused_mut)]
    async fn write_hci_async<W: ::embedded_io_async::Write>(&self, writer: W) -> Result<(), W::Error> {
        let bytes: [u8; 3] = unsafe { self.0.to_le_bytes()[..3].try_into().unwrap_unchecked() };
        bytes.write_hci_async(writer).await
    }
}

impl<'de> FromHciBytes<'de> for IsoDuration {
    fn from_hci_bytes(data: &'de [u8]) -> Result<(Self, &'de [u8]), FromHciBytesError> {
        let (bytes, rest) = <[u8; 3]>::from_hci_bytes(data)?;
        let bytes = [bytes[0], bytes[1], bytes[2], 0];
        Ok((Self(u32::from_le_bytes(bytes)), rest))
    }
}

impl IsoDuration {
    pub fn from_micros(val: u32) -> Self {
        Self(val)
    }

    pub fn from_millis(val: u32) -> Self {
        Self(unwrap!(val.checked_mul(1000)))
    }

    pub fn from_secs(val: u32) -> Self {
        Self(unwrap!(val.checked_mul(1_000_000)))
    }

    pub fn as_micros(&self) -> u32 {
        self.0
    }

    pub fn as_millis(&self) -> u32 {
        self.as_micros() / 1000
    }

    pub fn as_secs(&self) -> u32 {
        self.as_micros() / 1_000_000
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

param! {
    enum PowerLevelKind {
        Current = 0,
        Maximum = 1,
    }
}

param! {
    enum ControllerToHostFlowControl {
        Off = 0,
        AclOnSyncOff = 1,
        AclOffSyncOn = 2,
        BothOn = 3,
    }
}

param!(struct CoreSpecificationVersion(u8));

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
}

param! {
    enum LinkType {
        SyncData = 0,
        AclData = 1,
        IsoData = 2,
    }
}

param! {
    [ConnHandleCompletedPackets; 4] {
        handle[0]: ConnHandle,
        num_completed_packets[2]: u16,
    }
}
