//! Bluetooth HCI command packets.
//!
//! See Bluetooth Core Specification Vol 4, Part E, §7.

use crate::param::{param, ConnHandle};
use crate::{FromHciBytes, HostToControllerPacket, PacketKind, WriteHci};

pub mod controller_baseband;
pub mod info;
pub mod le;
pub mod link_control;
pub mod status;

/// The 6-bit Opcode Group Field (OGF)
///
/// See Bluetooth Core Specification Vol 4, Part E, §5.4.1
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct OpcodeGroup(u8);

impl OpcodeGroup {
    /// Bluetooth Core Specification Vol 4, Part E, §7.1
    pub const LINK_CONTROL: OpcodeGroup = OpcodeGroup(1);
    /// Bluetooth Core Specification Vol 4, Part E, §7.2
    pub const LINK_POLICY: OpcodeGroup = OpcodeGroup(2);
    /// Bluetooth Core Specification Vol 4, Part E, §7.3
    pub const CONTROL_BASEBAND: OpcodeGroup = OpcodeGroup(3);
    /// Bluetooth Core Specification Vol 4, Part E, §7.4
    pub const INFO_PARAMS: OpcodeGroup = OpcodeGroup(4);
    /// Bluetooth Core Specification Vol 4, Part E, §7.5
    pub const STATUS_PARAMS: OpcodeGroup = OpcodeGroup(5);
    /// Bluetooth Core Specification Vol 4, Part E, §7.6
    pub const TESTING: OpcodeGroup = OpcodeGroup(6);
    /// Bluetooth Core Specification Vol 4, Part E, §7.8
    pub const LE: OpcodeGroup = OpcodeGroup(8);
    /// Vendor Specific Debug commands
    pub const VENDOR_SPECIFIC: OpcodeGroup = OpcodeGroup(0x3f);

    /// Create a new `OpcodeGroup` with the given value
    pub const fn new(val: u8) -> Self {
        Self(val)
    }
}

param!(
    /// The 2 byte Opcode uniquely identifying the type of a command
    ///
    /// See Bluetooth Core Specification Vol 4, Part E, §5.4.1
    struct Opcode(u16)
);

impl Opcode {
    /// Create an `Opcode` with the given OGF and OCF values
    pub const fn new(ogf: OpcodeGroup, ocf: u16) -> Self {
        Self(((ogf.0 as u16) << 10) | ocf)
    }

    /// Get the OGF value of this Opcode
    pub const fn group(self) -> OpcodeGroup {
        OpcodeGroup((self.0 >> 10) as u8)
    }

    /// Get the OCF value of this Opcode
    pub const fn cmd(self) -> u16 {
        self.0 & 0x03ff
    }

    /// Get the raw 16-bit value for this Opcode
    pub const fn to_raw(self) -> u16 {
        self.0
    }
}

/// An object representing an HCI Command
pub trait Cmd: WriteHci {
    /// The opcode identifying this kind of HCI Command
    const OPCODE: Opcode;

    type Params: WriteHci;

    fn params(&self) -> &Self::Params;

    /// The command packet header for this command
    fn header(&self) -> [u8; 3] {
        let opcode_bytes = Self::OPCODE.0.to_le_bytes();
        [opcode_bytes[0], opcode_bytes[1], self.params().size() as u8]
    }
}

impl<T: Cmd> HostToControllerPacket for T {
    const KIND: PacketKind = PacketKind::Cmd;
}

/// A trait for objects representing HCI Commands that generate [`CommandComplete`](crate::event::CommandComplete)
/// events
pub trait SyncCmd: Cmd {
    /// The type of the parameters for the [`CommandComplete`](crate::event::CommandComplete) event
    type Return: for<'a> FromHciBytes<'a> + Copy;

    /// Extracts the [`ConnHandle`] from the return parameters for commands that return a `ConnHandle`
    ///
    /// If the command takes a connection handle and returns it as the first parameter of the associated
    /// [`CommandComplete`](crate::event::CommandComplete) event, this method will extract that handle from the return
    /// parameters. This is needed to identify which command the `CommandComplete` event was for in the event that the
    /// status of the command was an error.
    ///
    /// See Bluetooth Core Specification Vol 4, Part E, §4.5
    fn return_handle(_data: &[u8]) -> Option<ConnHandle> {
        None
    }
}

#[macro_export]
macro_rules! cmd {
    (
        $(#[$attrs:meta])*
        $name:ident($group:ident, $cmd:expr) {
            Params$(<$life:lifetime>)? = $params:ty;
            $(#[$ret_attrs:meta])*
            $ret:ident {
                handle: ConnHandle,
                $($ret_name:ident: $ret_ty:ty,)+
            }
        }
    ) => {
        $crate::cmd! {
            $(#[$attrs])*
            $name($group, $cmd) {
                Params$(<$life>)? = $params;
            }
        }

        impl$(<$life>)? $crate::cmd::SyncCmd for $name$(<$life>)? {
            type Return = $ret;

            fn return_handle(data: &[u8]) -> Option<$crate::param::ConnHandle> {
                <$crate::param::ConnHandle as $crate::FromHciBytes>::from_hci_bytes(data).ok().map(|(x, _)| x)
            }
        }

        $crate::param! {
            $(#[$ret_attrs])*
            struct $ret {
                handle: ConnHandle,
                $($ret_name: $ret_ty,)*
            }
        }
    };
    (
        $(#[$attrs:meta])*
        $name:ident($group:ident, $cmd:expr) {
            Params$(<$life:lifetime>)? = $params:ty;
            $(#[$ret_attrs:meta])*
            $ret:ident {
                $($ret_name:ident: $ret_ty:ty,)+
            }
        }
    ) => {
        $crate::cmd! {
            $(#[$attrs])*
            $name($group, $cmd) {
                Params$(<$life>)? = $params;
                Return = $ret;
            }
        }

        $crate::param! {
            $(#[$ret_attrs])*
            struct $ret {
                $($ret_name: $ret_ty,)*
            }
        }
    };
    (
        $(#[$attrs:meta])*
        $name:ident($group:ident, $cmd:expr) {
            Params$(<$life:lifetime>)? = $params:ty;
            Return = ConnHandle;
        }
    ) => {
        $crate::cmd! {
            $(#[$attrs])*
            $name($group, $cmd) {
                Params$(<$life>)? = $params;
            }
        }

        impl$(<$life>)? $crate::cmd::SyncCmd for $name$(<$life>)? {
            type Return = ConnHandle;

            fn return_handle(data: &[u8]) -> Option<$crate::param::ConnHandle> {
                <$crate::param::ConnHandle as $crate::FromHciBytes>::from_hci_bytes(data).ok().map(|(x, _)| x)
            }
        }
    };
    (
        $(#[$attrs:meta])*
        $name:ident($group:ident, $cmd:expr) {
            Params$(<$life:lifetime>)? = $params:ty;
            Return = $ret_ty:ty;
        }
    ) => {
        $crate::cmd! {
            $(#[$attrs])*
            $name($group, $cmd) {
                Params$(<$life>)? = $params;
            }
        }

        impl$(<$life>)? $crate::cmd::SyncCmd for $name$(<$life>)? {
            type Return = $ret_ty;
        }
    };
    (
        $(#[$attrs:meta])*
        $name:ident($group:ident, $cmd:expr) {
            Params = $params:ty;
        }
    ) => {
        $(#[$attrs])*
        #[repr(transparent)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub struct $name($params);

        #[automatically_derived]
        #[allow(unused_mut, unused_variables, unused_imports)]
        impl $crate::cmd::Cmd for $name {
            const OPCODE: $crate::cmd::Opcode = $crate::cmd::Opcode::new($crate::cmd::OpcodeGroup::$group, $cmd);
            type Params = $params;
            fn params(&self) -> &$params {
                &self.0
            }
        }

        $crate::cmd! {
            WRITE_HCI
            $name
        }
    };
    (
        $(#[$attrs:meta])*
        $name:ident($group:ident, $cmd:expr) {
            Params<$life:lifetime> = $params:ty;
        }
    ) => {
        $(#[$attrs])*
        #[repr(transparent)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub struct $name<$life>($params);

        #[automatically_derived]
        #[allow(unused_mut, unused_variables, unused_imports)]
        impl<$life> $crate::cmd::Cmd for $name<$life> {
            const OPCODE: $crate::cmd::Opcode = $crate::cmd::Opcode::new($crate::cmd::OpcodeGroup::$group, $cmd);
            type Params = $params;
            fn params(&self) -> &$params {
                &self.0
            }
        }

        $crate::cmd! {
            WRITE_HCI
            $name<$life>
        }
    };
    (WRITE_HCI $name:ident$(<$life:lifetime>)?) => {
        impl$(<$life>)? $crate::WriteHci for $name$(<$life>)? {
            #[inline(always)]
            fn size(&self) -> usize {
                self.0.size() + 3
            }

            fn write_hci<W: embedded_io::Write>(&self, mut writer: W) -> Result<(), W::Error> {
                writer.write_all(&<Self as $crate::cmd::Cmd>::header(self))?;
                self.0.write_hci(writer)
            }

            async fn write_hci_async<W: embedded_io_async::Write>(&self, mut writer: W) -> Result<(), W::Error> {
                writer.write_all(&<Self as $crate::cmd::Cmd>::header(self)).await?;
                self.0.write_hci_async(writer).await
            }
        }
    };
}

pub use cmd;
