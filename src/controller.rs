use core::future::Future;

use crate::{cmd, data, param, ControllerToHostPacket};

pub mod driver;
pub mod serial;

pub trait Controller {
    type Error: embedded_io::Error;

    fn write_acl_data(&self, packet: &data::AclPacket) -> impl Future<Output = Result<(), Self::Error>>;
    fn write_sync_data(&self, packet: &data::SyncPacket) -> impl Future<Output = Result<(), Self::Error>>;
    fn write_iso_data(&self, packet: &data::IsoPacket) -> impl Future<Output = Result<(), Self::Error>>;

    fn read<'a>(&self, buf: &'a mut [u8]) -> impl Future<Output = Result<ControllerToHostPacket<'a>, Self::Error>>;
}

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CmdError<E> {
    Hci(param::Error),
    Io(E),
}

impl<E> From<param::Error> for CmdError<E> {
    fn from(e: param::Error) -> Self {
        Self::Hci(e)
    }
}

pub trait ControllerCmdSync<C: cmd::SyncCmd + ?Sized>: Controller {
    /// Note: Some implementations may require [`Controller::read()`] to be polled for this to return.
    fn exec(&self, cmd: &C) -> impl Future<Output = Result<C::Return, CmdError<Self::Error>>>;
}

pub trait ControllerCmdAsync<C: cmd::AsyncCmd + ?Sized>: Controller {
    /// Note: Some implementations may require [`Controller::read()`] to be polled for this to return.
    fn exec(&self, cmd: &C) -> impl Future<Output = Result<(), CmdError<Self::Error>>>;
}
