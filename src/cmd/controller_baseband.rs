//! Bluetooth Core Specification Vol 4, Part E, §7.3

use super::{cmd, Cmd, Opcode};
use crate::{
    cmd::OpcodeGroup,
    param::{ConnHandle, ControllerToHostFlowControl, Duration, EventMask, EventMaskPage2, PowerLevelKind},
    WriteHci,
};

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.3.1
    SetEventMask(CONTROL_BASEBAND, 0x0001) {
        Params {
            mask: EventMask,
        }
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.3.2
    Reset(CONTROL_BASEBAND, 0x0003) {
        Params {}
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.3.35
    ReadTransmitPowerLevel(CONTROL_BASEBAND, 0x002d) {
        Params {
            handle: ConnHandle,
            kind: PowerLevelKind,
        }
        /// Bluetooth Core Specification Vol 4, Part E, §7.3.35
        ReadTransmitPowerLevelReturn {
            handle: ConnHandle,
            tx_power_level: i8,
        }
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.3.38
    SetControllerToHostFlowControl(CONTROL_BASEBAND, 0x0031) {
        Params {
            flow_control_enable: ControllerToHostFlowControl,
        }
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.3.39
    HostBufferSize(CONTROL_BASEBAND, 0x0033) {
        Params {
            host_acl_data_packet_len: u16,
            host_sync_data_packet_len: u8,
            host_total_acl_data_packets: u16,
            host_total_sync_data_packets: u16,
        }
        Return = ();
    }
}

/// Bluetooth Core Specification Vol 4, Part E, §7.3.40
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct HostNumberOfCompletedPackets<'a> {
    pub connection_completed_packets: &'a [(ConnHandle, u16)],
}

impl<'a> Cmd for HostNumberOfCompletedPackets<'a> {
    const OPCODE: Opcode = Opcode::new(OpcodeGroup::CONTROL_BASEBAND, 0x0035);

    #[inline(always)]
    fn params_size(&self) -> usize {
        1 + 4 * self.connection_completed_packets.len()
    }

    fn write_params<W: embedded_io::Write>(&self, mut writer: W) -> Result<(), W::Error> {
        let len = self.connection_completed_packets.len() as u8;
        len.write_hci(&mut writer)?;
        for (handle, _) in self.connection_completed_packets {
            handle.write_hci(&mut writer)?;
        }
        for (_, packets) in self.connection_completed_packets {
            packets.write_hci(&mut writer)?;
        }
        Ok(())
    }

    async fn write_params_async<W: embedded_io_async::Write>(&self, mut writer: W) -> Result<(), W::Error> {
        let len = self.connection_completed_packets.len() as u8;
        len.write_hci_async(&mut writer).await?;
        for (handle, _) in self.connection_completed_packets {
            handle.write_hci_async(&mut writer).await?;
        }
        for (_, packets) in self.connection_completed_packets {
            packets.write_hci_async(&mut writer).await?;
        }
        Ok(())
    }
}

impl<'a> crate::cmd::SyncCmd for HostNumberOfCompletedPackets<'a> {
    type Return<'ret> = ();
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.3.69
    SetEventMaskPage2(CONTROL_BASEBAND, 0x0063) {
        Params {
            mask: EventMaskPage2,
        }
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.3.93
    ReadAuthenticatedPayloadTimeout(CONTROL_BASEBAND, 0x007b) {
        Params {
            handle: ConnHandle,
        }
        /// Bluetooth Core Specification Vol 4, Part E, §7.3.93
        ReadAuthenticatedPayloadTimeoutReturn {
            handle: ConnHandle,
            timeout: Duration<10_000>,
        }
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.3.94
    WriteAuthenticatedPayloadTimeout(CONTROL_BASEBAND, 0x007c) {
        Params {
            handle: ConnHandle,
            timeout: Duration<10_000>,
        }
        Return = ConnHandle;
    }
}
