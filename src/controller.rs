//! HCI controller

use core::cell::RefCell;
use core::future::{poll_fn, Future};
use core::mem::MaybeUninit;
use core::task::Poll;

use cmd::controller_baseband::Reset;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::signal::Signal;
use embassy_sync::waitqueue::AtomicWaker;
use embedded_io::ErrorType;
use futures_intrusive::sync::LocalSemaphore;

use crate::cmd::{Cmd, CmdReturnBuf};
use crate::event::{CommandComplete, CommandCompleteWithStatus, CommandStatus, EventKind};
use crate::param::{RemainingBytes, Status};
use crate::transport::Transport;
use crate::{cmd, data, ControllerToHostPacket, FixedSizeValue, FromHciBytes, FromHciBytesError};

pub mod blocking;

/// Trait representing a HCI controller which supports async operations.
pub trait Controller: ErrorType {
    /// Write ACL data to the controller.
    fn write_acl_data(&self, packet: &data::AclPacket) -> impl Future<Output = Result<(), Self::Error>>;
    /// Write Sync data to the controller.
    fn write_sync_data(&self, packet: &data::SyncPacket) -> impl Future<Output = Result<(), Self::Error>>;
    /// Write Iso data to the controller.
    fn write_iso_data(&self, packet: &data::IsoPacket) -> impl Future<Output = Result<(), Self::Error>>;

    /// Read a valid HCI packet from the controller.
    fn read<'a>(&self, buf: &'a mut [u8]) -> impl Future<Output = Result<ControllerToHostPacket<'a>, Self::Error>>;
}

/// Marker trait for declaring that a controller supports a given HCI command.
pub trait ControllerCmdSync<C: cmd::SyncCmd + ?Sized>: Controller {
    /// Note: Some implementations may require [`Controller::read()`] to be polled for this to return.
    fn exec(&self, cmd: &C) -> impl Future<Output = Result<C::Return, cmd::Error<Self::Error>>>;
}

/// Marker trait for declaring that a controller supports a given async HCI command.
pub trait ControllerCmdAsync<C: cmd::AsyncCmd + ?Sized>: Controller {
    /// Note: Some implementations may require [`Controller::read()`] to be polled for this to return.
    fn exec(&self, cmd: &C) -> impl Future<Output = Result<(), cmd::Error<Self::Error>>>;
}

/// An external Bluetooth controller with communication via [`Transport`] type `T`.
///
/// The controller state holds a number of command slots that can be used
/// to issue commands and await responses from an underlying controller.
///
/// The contract is that before sending a command, a slot is reserved, which
/// returns a signal handle that can be used to await a response.
pub struct ExternalController<T, const SLOTS: usize> {
    transport: T,
    slots: ControllerState<SLOTS>,
}

impl<T, const SLOTS: usize> ExternalController<T, SLOTS> {
    /// Create a new instance.
    pub fn new(transport: T) -> Self {
        Self {
            slots: ControllerState::new(),
            transport,
        }
    }
}

impl<T, const SLOTS: usize> ErrorType for ExternalController<T, SLOTS>
where
    T: ErrorType,
{
    type Error = T::Error;
}

impl<T, const SLOTS: usize> Controller for ExternalController<T, SLOTS>
where
    T: Transport,
    T::Error: From<FromHciBytesError>,
{
    async fn write_acl_data(&self, packet: &data::AclPacket<'_>) -> Result<(), Self::Error> {
        self.transport.write(packet).await?;
        Ok(())
    }

    async fn write_sync_data(&self, packet: &data::SyncPacket<'_>) -> Result<(), Self::Error> {
        self.transport.write(packet).await?;
        Ok(())
    }

    async fn write_iso_data(&self, packet: &data::IsoPacket<'_>) -> Result<(), Self::Error> {
        self.transport.write(packet).await?;
        Ok(())
    }

    async fn read<'a>(&self, buf: &'a mut [u8]) -> Result<ControllerToHostPacket<'a>, Self::Error> {
        loop {
            {
                // Safety: we will not hold references across loop iterations.
                let buf = unsafe { core::slice::from_raw_parts_mut(buf.as_mut_ptr(), buf.len()) };
                let value = self.transport.read(&mut buf[..]).await?;
                match value {
                    ControllerToHostPacket::Event(ref event) => match event.kind {
                        EventKind::CommandComplete => {
                            let e = CommandComplete::from_hci_bytes_complete(event.data)?;
                            if !e.has_status() {
                                return Ok(value);
                            }
                            let e: CommandCompleteWithStatus = e.try_into()?;
                            self.slots.complete(
                                e.cmd_opcode,
                                e.status,
                                e.num_hci_cmd_pkts as usize,
                                e.return_param_bytes.as_ref(),
                            );
                            continue;
                        }
                        EventKind::CommandStatus => {
                            let e = CommandStatus::from_hci_bytes_complete(event.data)?;
                            self.slots
                                .complete(e.cmd_opcode, e.status, e.num_hci_cmd_pkts as usize, &[]);
                            continue;
                        }
                        _ => return Ok(value),
                    },
                    _ => return Ok(value),
                }
            }
        }
    }
}

impl<T, const SLOTS: usize> blocking::Controller for ExternalController<T, SLOTS>
where
    T: crate::transport::blocking::Transport,
    T::Error: From<FromHciBytesError>,
{
    fn write_acl_data(&self, packet: &data::AclPacket<'_>) -> Result<(), Self::Error> {
        loop {
            match self.try_write_acl_data(packet) {
                Err(blocking::TryError::Busy) => {}
                Err(blocking::TryError::Error(e)) => return Err(e),
                Ok(r) => return Ok(r),
            }
        }
    }

    fn write_sync_data(&self, packet: &data::SyncPacket<'_>) -> Result<(), Self::Error> {
        loop {
            match self.try_write_sync_data(packet) {
                Err(blocking::TryError::Busy) => {}
                Err(blocking::TryError::Error(e)) => return Err(e),
                Ok(r) => return Ok(r),
            }
        }
    }

    fn write_iso_data(&self, packet: &data::IsoPacket<'_>) -> Result<(), Self::Error> {
        loop {
            match self.try_write_iso_data(packet) {
                Err(blocking::TryError::Busy) => {}
                Err(blocking::TryError::Error(e)) => return Err(e),
                Ok(r) => return Ok(r),
            }
        }
    }

    fn read<'a>(&self, buf: &'a mut [u8]) -> Result<ControllerToHostPacket<'a>, Self::Error> {
        loop {
            // Safety: we will not hold references across loop iterations.
            let buf = unsafe { core::slice::from_raw_parts_mut(buf.as_mut_ptr(), buf.len()) };
            match self.try_read(buf) {
                Err(blocking::TryError::Busy) => {}
                Err(blocking::TryError::Error(e)) => return Err(e),
                Ok(r) => return Ok(r),
            }
        }
    }

    fn try_write_acl_data(&self, packet: &data::AclPacket<'_>) -> Result<(), blocking::TryError<Self::Error>> {
        self.transport.write(packet)?;
        Ok(())
    }

    fn try_write_sync_data(&self, packet: &data::SyncPacket<'_>) -> Result<(), blocking::TryError<Self::Error>> {
        self.transport.write(packet)?;
        Ok(())
    }

    fn try_write_iso_data(&self, packet: &data::IsoPacket<'_>) -> Result<(), blocking::TryError<Self::Error>> {
        self.transport.write(packet)?;
        Ok(())
    }

    fn try_read<'a>(&self, buf: &'a mut [u8]) -> Result<ControllerToHostPacket<'a>, blocking::TryError<Self::Error>> {
        loop {
            {
                // Safety: we will not hold references across loop iterations.
                let buf = unsafe { core::slice::from_raw_parts_mut(buf.as_mut_ptr(), buf.len()) };
                let value = self.transport.read(&mut buf[..])?;
                match value {
                    ControllerToHostPacket::Event(ref event) => match event.kind {
                        EventKind::CommandComplete => {
                            let e = CommandComplete::from_hci_bytes_complete(event.data)?;
                            if !e.has_status() {
                                return Ok(value);
                            }
                            let e: CommandCompleteWithStatus = e.try_into()?;
                            self.slots.complete(
                                e.cmd_opcode,
                                e.status,
                                e.num_hci_cmd_pkts as usize,
                                e.return_param_bytes.as_ref(),
                            );
                            continue;
                        }
                        EventKind::CommandStatus => {
                            let e = CommandStatus::from_hci_bytes_complete(event.data)?;
                            self.slots
                                .complete(e.cmd_opcode, e.status, e.num_hci_cmd_pkts as usize, &[]);
                            continue;
                        }
                        _ => return Ok(value),
                    },
                    _ => return Ok(value),
                }
            }
        }
    }
}

impl<T, C, const SLOTS: usize> ControllerCmdSync<C> for ExternalController<T, SLOTS>
where
    T: Transport,
    C: cmd::SyncCmd,
    C::Return: FixedSizeValue,
    T::Error: From<FromHciBytesError>,
{
    async fn exec(&self, cmd: &C) -> Result<C::Return, cmd::Error<Self::Error>> {
        let mut retval: C::ReturnBuf = C::ReturnBuf::new();

        //info!("Executing command with opcode {}", C::OPCODE);
        let (slot, idx) = self.slots.acquire(C::OPCODE, retval.as_mut()).await;
        let _d = OnDrop::new(|| {
            self.slots.release_slot(idx);
        });

        self.transport.write(cmd).await.map_err(cmd::Error::Io)?;

        let result = slot.wait().await;
        let return_param_bytes = RemainingBytes::from_hci_bytes_complete(&retval.as_ref()[..result.len]).unwrap();
        let e = CommandCompleteWithStatus {
            num_hci_cmd_pkts: 0,
            status: result.status,
            cmd_opcode: C::OPCODE,
            return_param_bytes,
        };
        let r = e.to_result::<C>().map_err(cmd::Error::Hci)?;
        // info!("Done executing command with opcode {}", C::OPCODE);
        Ok(r)
    }
}

impl<T, C, const SLOTS: usize> ControllerCmdAsync<C> for ExternalController<T, SLOTS>
where
    T: Transport,
    C: cmd::AsyncCmd,
    T::Error: From<FromHciBytesError>,
{
    async fn exec(&self, cmd: &C) -> Result<(), cmd::Error<Self::Error>> {
        let (slot, idx) = self.slots.acquire(C::OPCODE, &mut []).await;
        let _d = OnDrop::new(|| {
            self.slots.release_slot(idx);
        });

        self.transport.write(cmd).await.map_err(cmd::Error::Io)?;

        let result = slot.wait().await;
        result.status.to_result()?;
        Ok(())
    }
}

struct ControllerState<const SLOTS: usize> {
    permits: LocalSemaphore,
    slots: RefCell<[CommandSlot; SLOTS]>,
    signals: [Signal<NoopRawMutex, CommandResponse>; SLOTS],
    waker: AtomicWaker,
}

struct CommandResponse {
    status: Status,
    len: usize,
}

enum CommandSlot {
    Empty,
    Pending { opcode: u16, event: *mut [u8] },
}

impl<const SLOTS: usize> Default for ControllerState<SLOTS> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const SLOTS: usize> ControllerState<SLOTS> {
    const EMPTY_SLOT: CommandSlot = CommandSlot::Empty;
    #[allow(clippy::declare_interior_mutable_const)]
    const EMPTY_SIGNAL: Signal<NoopRawMutex, CommandResponse> = Signal::new();

    fn new() -> Self {
        Self {
            permits: LocalSemaphore::new(true, 1),
            slots: RefCell::new([Self::EMPTY_SLOT; SLOTS]),
            signals: [Self::EMPTY_SIGNAL; SLOTS],
            waker: AtomicWaker::new(),
        }
    }

    fn complete(&self, op: cmd::Opcode, status: Status, num_hci_command_packets: usize, data: &[u8]) {
        let mut slots = self.slots.borrow_mut();
        for (idx, slot) in slots.iter_mut().enumerate() {
            match slot {
                CommandSlot::Pending { opcode, event } if *opcode == op.to_raw() => {
                    if !data.is_empty() {
                        assert!(!event.is_null());
                        // Safety: since the slot is in pending, the caller stack will be valid.
                        unsafe { (&mut (**event))[..data.len()].copy_from_slice(data) };
                    }
                    self.signals[idx].signal(CommandResponse {
                        status,
                        len: data.len(),
                    });
                    if op != Reset::OPCODE {
                        break;
                    }
                }
                CommandSlot::Pending { opcode: _, event: _ } if op == Reset::OPCODE => {
                    // Signal other commands
                    self.signals[idx].signal(CommandResponse {
                        status: Status::CONTROLLER_BUSY,
                        len: 0,
                    });
                }
                _ => {}
            }
        }

        // Adjust the semaphore permits ensuring we don't grant more than num_hci_cmd_pkts
        self.permits
            .release(num_hci_command_packets.saturating_sub(self.permits.permits()));
    }

    fn release_slot(&self, idx: usize) {
        let mut slots = self.slots.borrow_mut();
        slots[idx] = CommandSlot::Empty;
    }

    async fn acquire(&self, op: cmd::Opcode, event: *mut [u8]) -> (&Signal<NoopRawMutex, CommandResponse>, usize) {
        let to_acquire = if op == Reset::OPCODE { self.permits.permits() } else { 1 };
        let mut permit = self.permits.acquire(to_acquire).await;
        permit.disarm();
        poll_fn(|cx| match self.acquire_slot(op, event) {
            Some(ret) => Poll::Ready(ret),
            None => {
                self.waker.register(cx.waker());
                Poll::Pending
            }
        })
        .await
    }

    fn acquire_slot(
        &self,
        op: cmd::Opcode,
        event: *mut [u8],
    ) -> Option<(&Signal<NoopRawMutex, CommandResponse>, usize)> {
        let mut slots = self.slots.borrow_mut();
        // Make sure there are no existing command with this opcode
        for slot in slots.iter() {
            match slot {
                CommandSlot::Pending { opcode, event: _ } if *opcode == op.to_raw() => {
                    return None;
                }
                _ => {}
            }
        }
        // Reserve our slot
        for (idx, slot) in slots.iter_mut().enumerate() {
            if matches!(slot, CommandSlot::Empty) {
                *slot = CommandSlot::Pending {
                    opcode: op.to_raw(),
                    event,
                };
                self.signals[idx].reset();
                return Some((&self.signals[idx], idx));
            }
        }
        None
    }
}

/// A type to delay the drop handler invocation.
#[must_use = "to delay the drop handler invocation to the end of the scope"]
struct OnDrop<F: FnOnce()> {
    f: MaybeUninit<F>,
}

impl<F: FnOnce()> OnDrop<F> {
    /// Create a new instance.
    pub(crate) fn new(f: F) -> Self {
        Self { f: MaybeUninit::new(f) }
    }
}

impl<F: FnOnce()> Drop for OnDrop<F> {
    fn drop(&mut self) {
        unsafe { self.f.as_ptr().read()() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub struct TestTransport<'d> {
        pub rx: &'d [u8],
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct Error;

    impl From<FromHciBytesError> for Error {
        fn from(_: FromHciBytesError) -> Self {
            Self
        }
    }

    impl ErrorType for TestTransport<'_> {
        type Error = Error;
    }
    impl embedded_io::Error for Error {
        fn kind(&self) -> embedded_io::ErrorKind {
            embedded_io::ErrorKind::Other
        }
    }
    impl Transport for TestTransport<'_> {
        fn read<'a>(&self, rx: &'a mut [u8]) -> impl Future<Output = Result<ControllerToHostPacket<'a>, Self::Error>> {
            async {
                let to_read = rx.len().min(self.rx.len());

                rx[..to_read].copy_from_slice(&self.rx[..to_read]);
                let pkt = ControllerToHostPacket::from_hci_bytes_complete(&rx[..to_read])?;
                Ok(pkt)
            }
        }

        fn write<T: crate::HostToControllerPacket>(&self, _val: &T) -> impl Future<Output = Result<(), Self::Error>> {
            async { todo!() }
        }
    }

    #[futures_test::test]
    pub async fn test_can_handle_unsolicited_command_complete() {
        let t = TestTransport {
            rx: &[
                4, 0x0e, 3, // header
                1, 0, 0, // special command
            ],
        };
        let c: ExternalController<_, 10> = ExternalController::new(t);

        let mut rx = [0; 255];
        let pkt = c.read(&mut rx).await;
        assert!(pkt.is_ok());
    }
}
