use crate::param::RemainingBytes;
use crate::param::Status;
use crate::FromHciBytes;
use crate::WriteHci;
use crate::{
    cmd::{self},
    data,
    event::{CommandComplete, Event},
    param::{self},
    CmdError, Controller, ControllerCmdAsync, ControllerCmdSync, ControllerToHostPacket, FixedSizeValue, WithIndicator,
};
use core::mem;
use core::mem::MaybeUninit;
use core::{cell::RefCell, future::Future};
use embassy_sync::{blocking_mutex::raw::NoopRawMutex, signal::Signal};

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
                                self.slots.complete(e);
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
            // TODO: Something more appropriately sized matching the expected return lenght
            let mut retval: [u8; 255] = [0u8; 255];

            //info!("Executing command with opcode {}", C::OPCODE);
            let (slot, idx) = self
                .slots
                .acquire_slot(C::OPCODE, retval.as_mut_ptr())
                .ok_or(CmdError::Param(param::Error::CONN_REJECTED_LIMITED_RESOURCES))?;
            let _d = OnDrop::new(|| {
                self.slots.release_slot(idx);
            });

            self.write(WithIndicator::new(cmd))
                .await
                .map_err(CmdError::Controller)?;

            let result = slot.wait().await;
            let return_param_bytes = RemainingBytes::from_hci_bytes_complete(&retval[..result.len]).unwrap();
            let e = CommandComplete {
                num_hci_cmd_pkts: result.num_hci_cmd_pkts,
                status: result.status,
                cmd_opcode: C::OPCODE,
                return_param_bytes,
            };
            let r = e.to_result::<C>().map_err(CmdError::Param)?;
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
            self.write(WithIndicator::new(cmd))
                .await
                .map_err(CmdError::Controller)?;
            Ok(())
        }
    }
}

pub struct ControllerState<const SLOTS: usize> {
    slots: RefCell<[CommandSlot; SLOTS]>,
    signals: [Signal<NoopRawMutex, CommandResponse>; SLOTS],
}

pub struct CommandResponse {
    num_hci_cmd_pkts: u8,
    status: Status,
    len: usize,
}

pub enum CommandSlot {
    Empty,
    Pending { opcode: u16, event: *mut u8 },
}

impl<const SLOTS: usize> ControllerState<SLOTS> {
    const EMPTY_SLOT: CommandSlot = CommandSlot::Empty;
    const EMPTY_SIGNAL: Signal<NoopRawMutex, CommandResponse> = Signal::new();

    pub fn new() -> Self {
        Self {
            slots: RefCell::new([Self::EMPTY_SLOT; SLOTS]),
            signals: [Self::EMPTY_SIGNAL; SLOTS],
        }
    }

    pub fn complete(&self, evt: &CommandComplete<'_>) {
        let mut slots = self.slots.borrow_mut();
        for (idx, slot) in slots.iter_mut().enumerate() {
            match slot {
                CommandSlot::Pending { opcode, event } if *opcode == evt.cmd_opcode.to_raw() => {
                    let data = evt.return_param_bytes.as_ref();

                    // Safety: since the slot is in pending, the caller stack will be valid.
                    unsafe { event.copy_from(data.as_ptr(), data.len()) };
                    self.signals[idx].signal(CommandResponse {
                        num_hci_cmd_pkts: evt.num_hci_cmd_pkts,
                        status: evt.status,
                        len: evt.return_param_bytes.len(),
                    });
                    break;
                }
                _ => {}
            }
        }
    }

    fn release_slot(&self, idx: usize) {
        let mut slots = self.slots.borrow_mut();
        slots[idx] = CommandSlot::Empty;
    }

    fn acquire_slot(&self, op: cmd::Opcode, event: *mut u8) -> Option<(&Signal<NoopRawMutex, CommandResponse>, usize)> {
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
