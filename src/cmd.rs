use crate::param::EventParam;

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

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Opcode(u16);

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

pub trait Cmd {
    const OPCODE: Opcode;

    /// The size (in bytes) of the params for this command
    fn size(&self) -> u8;

    /// The command packet header for this command
    fn header(&self) -> [u8; 3] {
        let opcode_bytes = Self::OPCODE.0.to_le_bytes();
        [opcode_bytes[0], opcode_bytes[1], self.size()]
    }

    fn write<W: embedded_io::blocking::Write>(&self, writer: W) -> Result<(), W::Error>;

    #[cfg(feature = "async")]
    async fn write_async<W: embedded_io::asynch::Write>(&self, writer: W) -> Result<(), W::Error>;
}

pub trait Synchronous: Cmd {
    type Return<'de>: EventParam<'de>;
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

        impl$(<$life>)? $crate::cmd::Synchronous for $name$(<$life>)? {
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
        impl$(<$life>)? $crate::cmd::Cmd for $name$(<$life>)? {
            const OPCODE: $crate::cmd::Opcode = $crate::cmd::Opcode::new($crate::cmd::OpcodeGroup::$group, $cmd);

            fn size(&self) -> u8 {
                ($(core::mem::size_of::<$param_ty>() +)* 0) as u8
            }

            fn write<W: embedded_io::blocking::Write>(&self, mut writer: W) -> Result<(), W::Error> {
                writer.write_all(&self.header())?;
                $(
                    <$param_ty as $crate::param::CmdParam>::write(&self.$param_name, &mut writer)?;
                )*
                Ok(())
            }

            #[cfg(feature = "async")]
            async fn write_async<W: embedded_io::asynch::Write>(&self, mut writer: W) -> Result<(), W::Error> {
                writer.write_all(&self.header()).await?;
                $(
                    <$param_ty as $crate::param::CmdParam>::write_async(&self.$param_name, &mut writer).await?;
                )*
                Ok(())
            }
        }
    };
}

pub(crate) use cmd;
