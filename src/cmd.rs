use crate::param::param;
use crate::{FromHciBytes, HostToControllerPacket, PacketKind, WriteHci};

pub mod controller_baseband;
pub mod info;
pub mod le;
pub mod link_control;
pub mod status;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct OpcodeGroup(u8);

impl OpcodeGroup {
    pub const LINK_CONTROL: OpcodeGroup = OpcodeGroup(1);
    pub const LINK_POLICY: OpcodeGroup = OpcodeGroup(2);
    pub const CONTROL_BASEBAND: OpcodeGroup = OpcodeGroup(3);
    pub const INFO_PARAMS: OpcodeGroup = OpcodeGroup(4);
    pub const STATUS_PARAMS: OpcodeGroup = OpcodeGroup(5);
    pub const TESTING: OpcodeGroup = OpcodeGroup(6);
    pub const LE: OpcodeGroup = OpcodeGroup(8);
    pub const VENDOR_SPECIFIC: OpcodeGroup = OpcodeGroup(0x3f);

    pub const fn new(val: u8) -> Self {
        Self(val)
    }
}

param!(struct Opcode(u16));

impl Opcode {
    pub const fn new(ogf: OpcodeGroup, ocf: u16) -> Self {
        Self(((ogf.0 as u16) << 10) | ocf)
    }

    pub const fn group(self) -> OpcodeGroup {
        OpcodeGroup((self.0 >> 10) as u8)
    }

    pub const fn cmd(self) -> u16 {
        self.0 & 0x03ff
    }

    pub const fn to_raw(self) -> u16 {
        self.0
    }
}

pub trait Cmd: WriteHci {
    const OPCODE: Opcode;

    /// The command packet header for this command
    fn header(&self) -> [u8; 3] {
        let opcode_bytes = Self::OPCODE.0.to_le_bytes();
        [opcode_bytes[0], opcode_bytes[1], (self.size() - 3) as u8]
    }
}

impl<T: Cmd> HostToControllerPacket for T {
    const KIND: PacketKind = PacketKind::Cmd;
}

pub trait SyncCmd: Cmd {
    type Return<'de>: FromHciBytes<'de>;
}

macro_rules! cmd {
    (
        $name:ident($group:ident, $cmd:expr) {
            Params$(<$life:lifetime>)? {
                $($param_name:ident: $param_ty:ty,)*
            }
            $ret:ident {
                $($ret_name:ident: $ret_ty:ty,)+
            }
        }
    ) => {
        $crate::cmd::cmd! {
            $name($group, $cmd) {
                Params$(<$life:lifetime>)? { $($param_name: $param_ty,)* }
                Return = $ret;
            }
        }

        $crate::param::param! {
            #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
            struct $ret {
                $($ret_name: $ret_ty,)*
            }
        }
    };
    (
        $name:ident($group:ident, $cmd:expr) {
            Params$(<$life:lifetime>)? {
                $($param_name:ident: $param_ty:ty,)*
            }
            Return = $ret_ty:ty;
        }
    ) => {
        $crate::cmd::cmd! {
            $name($group, $cmd) {
                Params$(<$life>)? { $($param_name: $param_ty,)* }
            }
        }

        impl$(<$life>)? $crate::cmd::SyncCmd for $name$(<$life>)? {
            type Return<'ret> = $ret_ty;
        }
    };
    (
        $name:ident($group:ident, $cmd:expr) {
            Params$(<$life:lifetime>)? {
                $($param_name:ident: $param_ty:ty,)*
            }
        }
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub struct $name$(<$life>)? {
            $(
                pub $param_name: $param_ty,
            )*
        }

        #[automatically_derived]
        #[allow(unused_mut, unused_variables, unused_imports)]
        impl$(<$life>)? $crate::WriteHci for $name$(<$life>)? {
            fn size(&self) -> usize {
                $(core::mem::size_of::<$param_ty>() +)* 3
            }

            fn write_hci<W: embedded_io::blocking::Write>(&self, mut writer: W) -> Result<(), W::Error> {
                use $crate::cmd::Cmd;
                writer.write_all(&self.header())?;
                $(
                    <$param_ty as $crate::WriteHci>::write_hci(&self.$param_name, &mut writer)?;
                )*
                Ok(())
            }

            #[cfg(feature = "async")]
            async fn write_hci_async<W: embedded_io::asynch::Write>(&self, mut writer: W) -> Result<(), W::Error> {
                use $crate::cmd::Cmd;
                writer.write_all(&self.header()).await?;
                $(
                    <$param_ty as $crate::WriteHci>::write_hci_async(&self.$param_name, &mut writer).await?;
                )*
                Ok(())
            }
        }

        #[automatically_derived]
        #[allow(unused_mut, unused_variables, unused_imports)]
        impl$(<$life>)? $crate::cmd::Cmd for $name$(<$life>)? {
            const OPCODE: $crate::cmd::Opcode = $crate::cmd::Opcode::new($crate::cmd::OpcodeGroup::$group, $cmd);
        }
    };
}

pub(crate) use cmd;
