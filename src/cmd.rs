//! HCI commands [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-ee8bbec6-ebdd-b47d-41d5-a7e655cad979)

use core::future::Future;

use embedded_io::ErrorType;

use crate::controller::{ControllerCmdAsync, ControllerCmdSync};
use crate::{param, FixedSizeValue, FromHciBytes, HostToControllerPacket, PacketKind, WriteHci};

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
    /// Link Control commands [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-fe2a33d3-28f4-9fd1-4d08-62286985c05e)
    pub const LINK_CONTROL: OpcodeGroup = OpcodeGroup(1);
    /// Link Policy commands [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-a593fa1a-89f3-8042-5ebe-6da6174e2cf9)
    pub const LINK_POLICY: OpcodeGroup = OpcodeGroup(2);
    /// Controller & Baseband commands [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-5ced811b-a6ce-701a-16b2-70f2d9795c05)
    pub const CONTROL_BASEBAND: OpcodeGroup = OpcodeGroup(3);
    /// Informational parameters [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-42372304-c9ef-dcab-6905-4e5b64703d45)
    pub const INFO_PARAMS: OpcodeGroup = OpcodeGroup(4);
    /// Status parameters [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-40e8a930-65b3-c409-007e-388fd48e1041)
    pub const STATUS_PARAMS: OpcodeGroup = OpcodeGroup(5);
    /// Testing commands [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-ec2ddbf2-ae4c-ec45-7a06-94f8b3327220)
    pub const TESTING: OpcodeGroup = OpcodeGroup(6);
    /// LE Controller commands [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-0f07d2b9-81e3-6508-ee08-8c808e468fed)
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
    /// Special opcode for command events with no associated command
    pub const UNSOLICITED: Opcode = Opcode::new(OpcodeGroup(0), 0);

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

/// An error type for HCI commands
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error<E> {
    /// HCI error.
    Hci(param::Error),
    /// I/O error.
    Io(E),
}

impl<E> From<param::Error> for Error<E> {
    fn from(e: param::Error) -> Self {
        Self::Hci(e)
    }
}

/// An object representing an HCI Command
pub trait Cmd: WriteHci {
    /// The opcode identifying this kind of HCI Command
    const OPCODE: Opcode;

    /// Parameters type for this command.
    type Params: WriteHci;

    /// Parameters expected for this command.
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

/// A marker trait for objects representing HCI Commands that generate [`CommandStatus`](crate::event::CommandStatus)
/// events
pub trait AsyncCmd: Cmd {
    /// Run the command on the provided controller.
    fn exec<C: ControllerCmdAsync<Self>>(
        &self,
        controller: &C,
    ) -> impl Future<Output = Result<(), Error<<C as ErrorType>::Error>>> {
        controller.exec(self)
    }
}

/// Type representing the buffer for a command response.
pub trait CmdReturnBuf: Copy + AsRef<[u8]> + AsMut<[u8]> {
    /// Length of buffer.
    const LEN: usize;

    /// Create a new instance of the buffer.
    fn new() -> Self;
}

impl<const N: usize> CmdReturnBuf for [u8; N] {
    const LEN: usize = N;

    #[inline(always)]
    fn new() -> Self {
        [0; N]
    }
}

/// A trait for objects representing HCI Commands that generate [`CommandComplete`](crate::event::CommandComplete)
/// events
pub trait SyncCmd: Cmd {
    /// The type of the parameters for the [`CommandComplete`](crate::event::CommandComplete) event
    type Return: for<'a> FromHciBytes<'a> + Copy;
    /// Handle returned by this command.
    type Handle: FixedSizeValue;
    /// Return buffer used by this command.
    type ReturnBuf: CmdReturnBuf;

    /// Handle parameter for this command.
    fn param_handle(&self) -> Self::Handle;

    /// Extracts the [`Self::Handle`] from the return parameters for commands that return a handle.
    ///
    /// If the command takes a handle or BdAddr and returns it as the first parameter of the associated
    /// [`CommandComplete`](crate::event::CommandComplete) event, this method will extract that handle from the return
    /// parameters. This is needed to identify which command the `CommandComplete` event was for in the event that the
    /// status of the command was an error.
    ///
    /// See Bluetooth Core Specification Vol 4, Part E, §4.5
    fn return_handle(_data: &[u8]) -> Result<Self::Handle, crate::FromHciBytesError>;

    /// Run the command on the provided controller.
    fn exec<C: ControllerCmdSync<Self>>(
        &self,
        controller: &C,
    ) -> impl Future<Output = Result<Self::Return, Error<<C as ErrorType>::Error>>> {
        controller.exec(self)
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! cmd {
    (
        $(#[$attrs:meta])*
        $name:ident($group:ident, $cmd:expr) {
            $params:ident$(<$life:lifetime>)? {
                $($param_name:ident: $param_ty:ty,)+
            }
            $ret:ident {
                $($ret_name:ident: $ret_ty:ty,)+
            }
            $(Handle = $handle_name:ident: $handle:ty;)?
        }
    ) => {
        $crate::cmd! {
            $(#[$attrs])*
            $name($group, $cmd) {
                $params$(<$life>)? {
                    $($param_name: $param_ty,)+
                }
                Return = $ret;
                $(Handle = $handle_name: $handle;)?
            }
        }

        $crate::param! {
            #[doc = "Return parameters for"]
            $(#[$attrs])*
            struct $ret {
                $($handle_name: $handle,)?
                $($ret_name: $ret_ty,)*
            }
        }
    };
    (
        $(#[$attrs:meta])*
        $name:ident($group:ident, $cmd:expr) {
            Params = ();
            $ret:ident {
                $($ret_name:ident: $ret_ty:ty,)+
            }
        }
    ) => {
        $crate::cmd! {
            $(#[$attrs])*
            $name($group, $cmd) {
                Params = ();
                Return = $ret;
            }
        }

        $crate::param! {
            #[doc = "Return parameters for"]
            $(#[$attrs])*
            struct $ret {
                $($ret_name: $ret_ty,)*
            }
        }
    };
    (
        $(#[$attrs:meta])*
        $name:ident($group:ident, $cmd:expr) {
            Params$(<$life:lifetime>)? = $param:ty;
            $ret:ident {
                $($ret_name:ident: $ret_ty:ty,)+
            }
            $(Handle = $handle_name:ident: $handle:ty;)?
        }
    ) => {
        $crate::cmd! {
            $(#[$attrs])*
            $name($group, $cmd) {
                Params$(<$life>)? = $param;
                Return = $ret;
                $(Handle = $handle;)?
            }
        }

        $crate::param! {
            #[doc = "Return parameters for"]
            $(#[$attrs])*
            struct $ret {
                $($handle_name: $handle,)?
                $($ret_name: $ret_ty,)*
            }
        }
    };
    (
        $(#[$attrs:meta])*
        $name:ident($group:ident, $cmd:expr) {
            $params:ident$(<$life:lifetime>)? {
                $($param_name:ident: $param_ty:ty,)+
            }
            $(
                Return = $ret:ty;
                $(Handle = $handle_name:ident: $handle:ty;)?
            )?
        }
    ) => {
        $crate::cmd! {
            BASE
            $(#[$attrs])*
            $name($group, $cmd) {
                Params$(<$life>)? = $params$(<$life>)?;
                $(
                    Return = $ret;
                    $(Handle = $handle;)?
                )?
            }
        }

        impl$(<$life>)? $name$(<$life>)? {
            #[allow(clippy::too_many_arguments)]
            /// Create a new instance of a command.
            pub fn new($($($handle_name: $handle,)?)? $($param_name: $param_ty),+) -> Self {
                Self($params {
                    $($($handle_name,)?)?
                    $($param_name,)*
                })
            }

            $(
                $(
                    fn handle(&self) -> $handle {
                        self.0.$handle_name
                    }
                )?
            )?
        }

        $crate::param! {
            #[doc = "Parameters for"]
            $(#[$attrs])*
            struct $params$(<$life>)? {
                $($($handle_name: $handle,)?)?
                $($param_name: $param_ty,)*
            }
        }
    };
    (
        $(#[$attrs:meta])*
        $name:ident($group:ident, $cmd:expr) {
            Params = ();
            $(Return = $ret:ty;)?
        }
    ) => {
        $crate::cmd! {
            BASE
            $(#[$attrs])*
            $name($group, $cmd) {
                Params = ();
                $(Return = $ret;)?
            }
        }

        impl $name {
            /// Create a new instance of this command.
            pub fn new() -> Self {
                Self(())
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self(())
            }
        }
    };
    (
        $(#[$attrs:meta])*
        $name:ident($group:ident, $cmd:expr) {
            Params$(<$life:lifetime>)? = $params:ty;
            $(
                Return = $ret:ty;
                $(Handle = $handle:ty;)?
            )?
        }
    ) => {
        $crate::cmd! {
            BASE
            $(#[$attrs])*
            $name($group, $cmd) {
                Params$(<$life>)? = $params;
                $(
                    Return = $ret;
                    $(Handle = $handle;)?
                )?
            }
        }

        impl$(<$life>)? $name$(<$life>)? {
            /// Create a new instance of the command with the provided parameters.
            pub fn new(param: $params) -> Self {
                Self(param)
            }

            $(
                $(
                    fn handle(&self) -> $handle {
                        self.0
                    }
                )?
            )?
        }
    };
    (
        BASE
        $(#[$attrs:meta])*
        $name:ident($group:ident, $cmd:expr) {
            Params$(<$life:lifetime>)? = $params:ty;
            $(
                Return = $ret:ty;
                $(Handle = $handle:ty;)?
            )?
        }
    ) => {
        $(#[$attrs])*
        #[repr(transparent)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub struct $name$(<$life>)?($params);

        #[automatically_derived]
        #[allow(unused_mut, unused_variables, unused_imports)]
        impl$(<$life>)? $crate::cmd::Cmd for $name$(<$life>)? {
            const OPCODE: $crate::cmd::Opcode = $crate::cmd::Opcode::new($crate::cmd::OpcodeGroup::$group, $cmd);
            type Params = $params;

            fn params(&self) -> &$params {
                &self.0
            }
        }

        #[automatically_derived]
        impl$(<$life>)? From<$params> for $name$(<$life>)? {
            fn from(params: $params) -> Self {
                Self(params)
            }
        }

        impl$(<$life>)? $crate::WriteHci for $name$(<$life>)? {
            #[inline(always)]
            fn size(&self) -> usize {
                <$params as $crate::WriteHci>::size(&self.0) + 3
            }

            fn write_hci<W: embedded_io::Write>(&self, mut writer: W) -> Result<(), W::Error> {
                writer.write_all(&<Self as $crate::cmd::Cmd>::header(self))?;
                <$params as $crate::WriteHci>::write_hci(&self.0, writer)
            }

            async fn write_hci_async<W: embedded_io_async::Write>(&self, mut writer: W) -> Result<(), W::Error> {
                writer.write_all(&<Self as $crate::cmd::Cmd>::header(self)).await?;
                <$params as $crate::WriteHci>::write_hci_async(&self.0, writer).await
            }
        }

        $crate::cmd! {
            RETURN
            $name$(<$life>)? {
                $(
                    Return = $ret;
                    $(Handle = $handle;)?
                )?
            }
        }
    };
    (
        RETURN
        $name:ident$(<$life:lifetime>)? {
            Return = $ret:ty;
            Handle = $handle:ty;
        }
    ) => {
        impl$(<$life>)? $crate::cmd::SyncCmd for $name$(<$life>)? {
            type Return = $ret;
            type Handle = $handle;
            type ReturnBuf = [u8; <$ret as $crate::ReadHci>::MAX_LEN];

            fn param_handle(&self) -> Self::Handle {
                self.handle()
            }

            fn return_handle(data: &[u8]) -> Result<Self::Handle, $crate::FromHciBytesError> {
                <$handle as $crate::FromHciBytes>::from_hci_bytes(data).map(|(x, _)| x)
            }
        }
    };
    (
        RETURN
        $name:ident$(<$life:lifetime>)? {
            Return = $ret:ty;
        }
    ) => {
        impl$(<$life>)? $crate::cmd::SyncCmd for $name$(<$life>)? {
            type Return = $ret;
            type Handle = ();
            type ReturnBuf = [u8; <$ret as $crate::ReadHci>::MAX_LEN];

            fn param_handle(&self) {}

            fn return_handle(_data: &[u8]) -> Result<Self::Handle, $crate::FromHciBytesError> {
                Ok(())
            }
        }
    };
    (
        RETURN
        $name:ident$(<$life:lifetime>)? {
        }
    ) => {
        impl$(<$life>)? $crate::cmd::AsyncCmd for $name$(<$life>)? {}
    };
}

pub use cmd;
