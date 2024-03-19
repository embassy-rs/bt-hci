use embassy_sync::{blocking_mutex::raw::{NoopRawMutex, RawMutex}, mutex::Mutex, signal::Signal};
use core::{cell::RefCell, future::Future};
use crate::{cmd::{self, Opcode}, data, event::{CommandComplete, CommandStatus, Event}, param::{self, ConnHandle, Status}, CmdError, Controller, ControllerCmdAsync, ControllerCmdSync, ControllerToHostPacket, FixedSizeValue, WithIndicator};
use crate::WriteHci as _;
use crate::FromHciBytes;
use crate::ReadHci as _;
use core::ops::DerefMut;
use heapless::Vec;


pub struct CommandResponse {
    status: Status,
    num_hci_cmd_pkts: u8,
    cmd_opcode: Opcode,
    extra: Vec<u8, 32>, // TODO: Adjust to realistic max
}

pub struct ControllerState<const SLOTS: usize> {
    slots: RefCell<[CommandSlot; SLOTS]>,
    signals: [Signal<NoopRawMutex, CommandResponse>; SLOTS],
}

pub enum CommandSlot {
    Empty,
    Pending {
        opcode: u16,
        handle: u16,
    }
}

impl<const SLOTS: usize> ControllerState<SLOTS> {
    const EMPTY_SLOT: CommandSlot = CommandSlot::Empty;
    const EMPTY_SIGNAL: Signal<NoopRawMutex, CommandResponse>= Signal::new();

    pub fn new() -> Self {
        Self {
            slots: RefCell::new([Self::EMPTY_SLOT; SLOTS]),
            signals: [Self::EMPTY_SIGNAL; SLOTS],
        }
    }

    pub fn complete(&self, complete: &CommandComplete<'_>) {
        let mut slots = self.slots.borrow_mut();
        for (idx, slot) in slots.iter_mut().enumerate() {
            match slot {
                CommandSlot::Pending { opcode, handle } if *opcode == complete.cmd_opcode.to_raw() => {
                    let extra = Vec::from_slice(&complete.return_param_bytes).unwrap();
                    self.signals[idx].signal(CommandResponse { status: complete.status, num_hci_cmd_pkts: complete.num_hci_cmd_pkts, cmd_opcode: complete.cmd_opcode, extra });
                    break;
                }
                _ => {}
            }
        }
    }

    pub fn status(&self, status: &CommandStatus) {
        let mut slots = self.slots.borrow_mut();
        for (idx, slot) in slots.iter_mut().enumerate() {
            match slot {
                CommandSlot::Pending { opcode, handle } if *opcode == status.cmd_opcode.to_raw() => {
                    self.signals[idx].signal(CommandResponse { status: status.status, num_hci_cmd_pkts: status.num_hci_cmd_pkts, cmd_opcode: status.cmd_opcode, extra: Vec::new() });
                    break;
                }
                _ => {}
            }
        }
    }

    pub fn release_slot(&self, idx: usize) {
        let mut slots = self.slots.borrow_mut();
        slots[idx] = CommandSlot::Empty;
    }

    pub fn acquire_slot(&self, opcode: cmd::Opcode, handle: u16) -> Option<&Signal<NoopRawMutex, CommandResponse>> {
        let mut slots = self.slots.borrow_mut();
        for (idx, slot) in slots.iter_mut().enumerate() {
            match slot {
                CommandSlot::Empty => {
                    *slot = CommandSlot::Pending { opcode: opcode.to_raw(), handle };
                    return Some(&self.signals[idx])
                }
                _ => {}
            }
        }
        None
    }
}

pub struct SerialController<M, T, const SLOTS: usize>
where
    M: RawMutex,
    T: embedded_io_async::Read + embedded_io_async::Write,
{
    io: Mutex<M, T>,
    slots: ControllerState<SLOTS>,
}

impl<M, T, const SLOTS: usize> SerialController<M, T, SLOTS>
where
    M: RawMutex,
    T: embedded_io_async::Read + embedded_io_async::Write,
{
    pub fn new(io: T) -> Self {
        Self { slots: ControllerState::new(), io: Mutex::new(io) }
    }
}

impl<M, T, const SLOTS: usize> Controller for SerialController<M, T, SLOTS>
where
    M: RawMutex,
    T: embedded_io_async::Read + embedded_io_async::Write,
{
    type Error = T::Error;
    fn write_acl_data(&self, packet: &data::AclPacket) -> impl Future<Output = Result<(), Self::Error>> {
        async {
            let mut io = self.io.lock().await;
            WithIndicator::new(packet)
                .write_hci_async(io.deref_mut()).await?;
            Ok(())
        }
    }

    fn write_sync_data(&self, packet: &data::SyncPacket) -> impl Future<Output = Result<(), Self::Error>> {
        async {
            let mut io = self.io.lock().await;
            WithIndicator::new(packet)
                .write_hci_async(io.deref_mut()).await?;
            Ok(())
        }
    }

    fn write_iso_data(&self, packet: &data::IsoPacket) -> impl Future<Output = Result<(), Self::Error>> {
        async {
            let mut io = self.io.lock().await;
            WithIndicator::new(packet)
                .write_hci_async(io.deref_mut()).await?;
            Ok(())
        }
    }

    fn read<'a>(&self, buf: &'a mut [u8]) -> impl Future<Output = Result<ControllerToHostPacket<'a>, Self::Error>> {
        async {
            loop {
                // TODO
                let buf = unsafe {core::slice::from_raw_parts_mut(buf.as_mut_ptr(), buf.len())};
                {
                    let value: ControllerToHostPacket<'a> = {
                        let mut io = self.io.lock().await;
                        ControllerToHostPacket::read_hci_async(io.deref_mut(), buf)
                        .await.unwrap()
                    };

                    match value {
                        ControllerToHostPacket::Event(ref event) => match &event {
                            Event::CommandComplete(e) => {
                                self.slots.complete(&e);
                                continue;
                            }
                            Event::CommandStatus(e) => {
                                self.slots.status(&e);
                                continue;
                            }
                            _ => {
                                return Ok(value)
                            }
                        }
                        _ => return Ok(value)
                    }
                }
            }
        }
    }
}

impl<M, T, C, const SLOTS: usize> ControllerCmdSync<C> for SerialController<M, T, SLOTS>
where
    M: RawMutex,
    T: embedded_io_async::Read + embedded_io_async::Write,
    C: cmd::SyncCmd,
    C::Return: FixedSizeValue,
{
    fn exec(&self, cmd: &C) -> impl Future<Output = Result<C::Return, CmdError<Self::Error>>> {
        async {
            let slot = self.slots.acquire_slot(C::OPCODE, 0).ok_or(CmdError::Param(param::Error::CONN_REJECTED_LIMITED_RESOURCES))?;
            let mut io = self.io.lock().await;
            WithIndicator::new(cmd).write_hci_async(io.deref_mut()).await.map_err(CmdError::Controller)?;
            let result = slot.wait().await;
            let (r, _) = C::Return::from_hci_bytes(&result.extra[..]).unwrap();
            Ok(r)
        }
    }
}

impl<M, T, C, const SLOTS: usize> ControllerCmdAsync<C> for SerialController<M, T, SLOTS>
where
    M: RawMutex,
    T: embedded_io_async::Read + embedded_io_async::Write,
    C: cmd::AsyncCmd,
{
    fn exec(&self, cmd: &C) -> impl Future<Output = Result<(), CmdError<Self::Error>>> {
        async {
            let mut io = self.io.lock().await;
            WithIndicator::new(cmd).write_hci_async(io.deref_mut()).await.map_err(CmdError::Controller)?;
            Ok(())
        }
    }
}
