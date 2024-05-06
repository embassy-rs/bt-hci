use crate::{controller::ErrorType, data, ControllerToHostPacket};

pub trait Controller: ErrorType {
    fn write_acl_data(&self, packet: &data::AclPacket) -> Result<(), Self::Error>;
    fn write_sync_data(&self, packet: &data::SyncPacket) -> Result<(), Self::Error>;
    fn write_iso_data(&self, packet: &data::IsoPacket) -> Result<(), Self::Error>;

    fn try_write_acl_data(&self, packet: &data::AclPacket) -> Result<(), TryError<Self::Error>>;
    fn try_write_sync_data(&self, packet: &data::SyncPacket) -> Result<(), TryError<Self::Error>>;
    fn try_write_iso_data(&self, packet: &data::IsoPacket) -> Result<(), TryError<Self::Error>>;

    fn read<'a>(&self, buf: &'a mut [u8]) -> Result<ControllerToHostPacket<'a>, Self::Error>;
    fn try_read<'a>(&self, buf: &'a mut [u8]) -> Result<ControllerToHostPacket<'a>, TryError<Self::Error>>;
}

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TryError<E> {
    Error(E),
    Busy,
}
