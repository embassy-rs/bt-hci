//! Blocking controller types and traits.
use crate::controller::ErrorType;
use crate::{data, ControllerToHostPacket, FromHciBytesError};

/// Trait representing a HCI controller which supports blocking and non-blocking operations.
pub trait Controller: ErrorType {
    /// Write ACL data to the controller. Blocks until done.
    fn write_acl_data(&self, packet: &data::AclPacket) -> Result<(), Self::Error>;

    /// Write Sync data to the controller. Blocks until done.
    fn write_sync_data(&self, packet: &data::SyncPacket) -> Result<(), Self::Error>;

    /// Write Iso data to the controller. Blocks until done.
    fn write_iso_data(&self, packet: &data::IsoPacket) -> Result<(), Self::Error>;

    /// Attempt to write ACL data to the controller.
    ///
    /// Returns a TryError if the operation would block.
    fn try_write_acl_data(&self, packet: &data::AclPacket) -> Result<(), TryError<Self::Error>>;

    /// Attempt to write Sync data to the controller.
    ///
    /// Returns a TryError if the operation would block.
    fn try_write_sync_data(&self, packet: &data::SyncPacket) -> Result<(), TryError<Self::Error>>;

    /// Attempt to write Iso data to the controller.
    ///
    /// Returns a TryError if the operation would block.
    fn try_write_iso_data(&self, packet: &data::IsoPacket) -> Result<(), TryError<Self::Error>>;

    /// Read a valid HCI packet from the controller. Blocks until done.
    fn read<'a>(&self, buf: &'a mut [u8]) -> Result<ControllerToHostPacket<'a>, Self::Error>;

    /// Read a valid HCI packet from the controller.
    ///
    /// Returns a TryError if the operation would block.
    fn try_read<'a>(&self, buf: &'a mut [u8]) -> Result<ControllerToHostPacket<'a>, TryError<Self::Error>>;
}

/// Error for representing an operation that blocks or fails
/// with an error.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TryError<E> {
    /// Underlying controller error.
    Error(E),
    /// Operation would block.
    Busy,
}

impl<E: From<FromHciBytesError>> From<FromHciBytesError> for TryError<E> {
    fn from(value: FromHciBytesError) -> Self {
        TryError::Error(E::from(value))
    }
}
