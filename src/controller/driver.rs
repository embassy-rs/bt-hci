use core::cell::RefCell;
use core::future::{poll_fn, Future};
use core::mem;
use core::mem::MaybeUninit;
use core::task::Poll;

use cmd::controller_baseband::Reset;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::signal::Signal;
use embassy_sync::waitqueue::AtomicWaker;
use futures_intrusive::sync::LocalSemaphore;

use super::{CmdError, Controller, ControllerCmdAsync, ControllerCmdSync};
use crate::cmd::{
    Cmd, CmdReturnBuf, {self},
};
use crate::event::{CommandComplete, Event};
use crate::param::{RemainingBytes, Status};
use crate::{data, ControllerToHostPacket, FixedSizeValue, FromHciBytes, WithIndicator, WriteHci};

/// A packet-oriented HCI trait.
pub trait HciDriver {
    type Error: embedded_io::Error;
    /// Read a complete HCI packet into the rx buffer
    fn read(&self, rx: &mut [u8]) -> impl Future<Output = Result<usize, Self::Error>>;
    /// Write a complete HCI packet from the tx buffer
    fn write(&self, tx: &[u8]) -> impl Future<Output = Result<(), Self::Error>>;
}

/// The controller state holds a number of command slots that can be used
/// to issue commands and await responses from an underlying controller.
///
/// The contract is that before sending a command, a slot is reserved, which
/// returns a signal handle that can be used to await a response.
pub struct HciController<D, const SLOTS: usize>
where
    D: HciDriver,
{
    driver: D,
    slots: ControllerState<SLOTS>,
}

impl<D, const SLOTS: usize> HciController<D, SLOTS>
where
    D: HciDriver,
{
    pub fn new(driver: D) -> Self {
        Self {
            slots: ControllerState::new(),
            driver,
        }
    }

    async fn write<W: WriteHci>(&self, data: W) -> Result<(), D::Error> {
        let mut packet: [u8; 512] = [0; 512];
        let len = data.size();
        data.write_hci(&mut packet[..]).unwrap();
        self.driver.write(&packet[..len]).await?;
        Ok(())
    }
}

impl<D, const SLOTS: usize> Controller for HciController<D, SLOTS>
where
    D: HciDriver,
{
    type Error = D::Error;
    fn write_acl_data(&self, packet: &data::AclPacket) -> impl Future<Output = Result<(), Self::Error>> {
        async {
            self.write(WithIndicator::new(packet)).await?;
            Ok(())
        }
    }

    fn write_sync_data(&self, packet: &data::SyncPacket) -> impl Future<Output = Result<(), Self::Error>> {
        async {
            self.write(WithIndicator::new(packet)).await?;
            Ok(())
        }
    }

    fn write_iso_data(&self, packet: &data::IsoPacket) -> impl Future<Output = Result<(), Self::Error>> {
        async {
            self.write(WithIndicator::new(packet)).await?;
            Ok(())
        }
    }

    fn read<'a>(&self, buf: &'a mut [u8]) -> impl Future<Output = Result<ControllerToHostPacket<'a>, Self::Error>> {
        async {
            loop {
                {
                    // Safety: we will not hold references across loop iterations.
                    let buf = unsafe { core::slice::from_raw_parts_mut(buf.as_mut_ptr(), buf.len()) };
                    let len = self.driver.read(&mut buf[..]).await?;
                    let (value, _): (ControllerToHostPacket<'a>, _) =
                        ControllerToHostPacket::from_hci_bytes(&buf[..len]).unwrap();
                    match value {
                        ControllerToHostPacket::Event(ref event) => match &event {
                            Event::CommandComplete(e) => {
                                self.slots.complete(
                                    e.cmd_opcode,
                                    e.status,
                                    e.num_hci_cmd_pkts as usize,
                                    e.return_param_bytes.as_ref(),
                                );
                                continue;
                            }
                            Event::CommandStatus(e) => {
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
}

impl<D, C, const SLOTS: usize> ControllerCmdSync<C> for HciController<D, SLOTS>
where
    D: HciDriver,
    C: cmd::SyncCmd,
    C::Return: FixedSizeValue,
{
    fn exec(&self, cmd: &C) -> impl Future<Output = Result<C::Return, CmdError<Self::Error>>> {
        async {
            let mut retval: C::ReturnBuf = C::ReturnBuf::new();

            //info!("Executing command with opcode {}", C::OPCODE);
            let (slot, idx) = self.slots.acquire(C::OPCODE, retval.as_mut()).await;
            let _d = OnDrop::new(|| {
                self.slots.release_slot(idx);
            });

            self.write(WithIndicator::new(cmd)).await.map_err(CmdError::Io)?;

            let result = slot.wait().await;
            let return_param_bytes = RemainingBytes::from_hci_bytes_complete(&retval.as_ref()[..result.len]).unwrap();
            let e = CommandComplete {
                num_hci_cmd_pkts: 0,
                status: result.status,
                cmd_opcode: C::OPCODE,
                return_param_bytes,
            };
            let r = e.to_result::<C>().map_err(CmdError::Hci)?;
            // info!("Done executing command with opcode {}", C::OPCODE);
            Ok(r)
        }
    }
}

impl<D, C, const SLOTS: usize> ControllerCmdAsync<C> for HciController<D, SLOTS>
where
    D: HciDriver,
    C: cmd::AsyncCmd,
{
    fn exec(&self, cmd: &C) -> impl Future<Output = Result<(), CmdError<Self::Error>>> {
        async {
            let (slot, idx) = self.slots.acquire(C::OPCODE, &mut []).await;
            let _d = OnDrop::new(|| {
                self.slots.release_slot(idx);
            });

            self.write(WithIndicator::new(cmd)).await.map_err(CmdError::Io)?;

            let result = slot.wait().await;
            result.status.to_result()?;
            Ok(())
        }
    }
}

pub struct ControllerState<const SLOTS: usize> {
    permits: LocalSemaphore,
    slots: RefCell<[CommandSlot; SLOTS]>,
    signals: [Signal<NoopRawMutex, CommandResponse>; SLOTS],
    waker: AtomicWaker,
}

pub struct CommandResponse {
    status: Status,
    len: usize,
}

pub enum CommandSlot {
    Empty,
    Pending { opcode: u16, event: *mut [u8] },
}

impl<const SLOTS: usize> ControllerState<SLOTS> {
    const EMPTY_SLOT: CommandSlot = CommandSlot::Empty;
    const EMPTY_SIGNAL: Signal<NoopRawMutex, CommandResponse> = Signal::new();

    pub fn new() -> Self {
        Self {
            permits: LocalSemaphore::new(true, 1),
            slots: RefCell::new([Self::EMPTY_SLOT; SLOTS]),
            signals: [Self::EMPTY_SIGNAL; SLOTS],
            waker: AtomicWaker::new(),
        }
    }

    pub fn complete(&self, op: cmd::Opcode, status: Status, num_hci_command_packets: usize, data: &[u8]) {
        let mut slots = self.slots.borrow_mut();
        for (idx, slot) in slots.iter_mut().enumerate() {
            match slot {
                CommandSlot::Pending { opcode, event } if *opcode == op.to_raw() => {
                    if !data.is_empty() {
                        assert!(!event.is_null());
                        // Safety: since the slot is in pending, the caller stack will be valid.
                        unsafe { (**event)[..data.len()].copy_from_slice(data) };
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
            match slot {
                CommandSlot::Empty => {
                    *slot = CommandSlot::Pending {
                        opcode: op.to_raw(),
                        event,
                    };
                    self.signals[idx].reset();
                    return Some((&self.signals[idx], idx));
                }
                _ => {}
            }
        }
        None
    }
}

/// A type to delay the drop handler invocation.
#[must_use = "to delay the drop handler invocation to the end of the scope"]
pub struct OnDrop<F: FnOnce()> {
    f: MaybeUninit<F>,
}

impl<F: FnOnce()> OnDrop<F> {
    /// Create a new instance.
    pub fn new(f: F) -> Self {
        Self { f: MaybeUninit::new(f) }
    }

    /// Prevent drop handler from running.
    pub fn defuse(self) {
        mem::forget(self)
    }
}

impl<F: FnOnce()> Drop for OnDrop<F> {
    fn drop(&mut self) {
        unsafe { self.f.as_ptr().read()() }
    }
}

/// An explosive ordinance that panics if it is improperly disposed of.
///
/// This is to forbid dropping futures, when there is absolutely no other choice.
///
/// To correctly dispose of this device, call the [defuse](struct.DropBomb.html#method.defuse)
/// method before this object is dropped.
#[must_use = "to delay the drop bomb invokation to the end of the scope"]
pub struct DropBomb {
    _private: (),
}

impl DropBomb {
    /// Create a new instance.
    pub fn new() -> Self {
        Self { _private: () }
    }

    /// Defuses the bomb, rendering it safe to drop.
    pub fn defuse(self) {
        mem::forget(self)
    }
}

impl Drop for DropBomb {
    fn drop(&mut self) {
        panic!("boom")
    }
}
