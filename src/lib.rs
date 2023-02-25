#![no_std]
#![cfg_attr(feature = "async", feature(async_fn_in_trait))]
#![cfg_attr(feature = "async", allow(incomplete_features))]

mod fmt;

pub mod cmd;
pub mod event;
pub mod param;
pub mod transport;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PacketKind {
    Cmd = 1,
    AclData = 2,
    SyncData = 3,
    Event = 4,
    IsoData = 5,
}

/// Abbreviations:
/// - command -> cmd
/// - properties -> props
/// - advertising -> adv
/// - advertiser -> adv
/// - address -> addr
/// - connection -> conn
/// - extended -> ext
/// - type -> kind
const _FOO: () = ();
