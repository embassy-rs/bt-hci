use core::num::NonZeroU8;

use crate::{FromHciBytes, FromHciBytesError, WriteHci};

#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Status(u8);

impl Status {
    pub fn into_inner(self) -> u8 {
        self.0
    }
}

impl WriteHci for Status {
    #[inline(always)]
    fn size(&self) -> usize {
        WriteHci::size(&self.0)
    }

    #[inline(always)]
    fn write_hci<W: ::embedded_io::Write>(&self, writer: W) -> Result<(), W::Error> {
        <u8 as WriteHci>::write_hci(&self.0, writer)
    }

    #[inline(always)]
    async fn write_hci_async<W: ::embedded_io_async::Write>(&self, writer: W) -> Result<(), W::Error> {
        <u8 as WriteHci>::write_hci_async(&self.0, writer).await
    }
}

impl<'de> FromHciBytes<'de> for Status {
    fn from_hci_bytes(data: &'de [u8]) -> Result<(Self, &'de [u8]), FromHciBytesError> {
        <u8 as FromHciBytes>::from_hci_bytes(data).map(|(x, y)| (Self(x), y))
    }
}

impl Status {
    pub const SUCCESS: Status = Status(0);

    pub const fn new(n: u8) -> Self {
        Status(n)
    }

    pub const fn to_result(self) -> Result<(), Error> {
        if self.0 == Self::SUCCESS.0 {
            Ok(())
        } else {
            Err(Error(unsafe { NonZeroU8::new_unchecked(self.0) }))
        }
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for Status {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::Format::format(&self.to_result(), fmt)
    }
}

impl core::fmt::Debug for Status {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.to_result(), f)
    }
}

impl From<u8> for Status {
    fn from(value: u8) -> Self {
        Status(value)
    }
}

impl From<Status> for u8 {
    fn from(value: Status) -> Self {
        value.0
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Error(NonZeroU8);

impl Error {
    const unsafe fn from_u8(err: u8) -> Error {
        Error(NonZeroU8::new_unchecked(err))
    }

    pub const fn to_status(self) -> Status {
        Status(self.0.get())
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, f)
    }
}

impl From<Error> for u8 {
    fn from(value: Error) -> Self {
        value.0.get()
    }
}

macro_rules! errnos {
        (
            $(
                ($val:expr, $konst:ident, $desc:expr);
            )+
        ) => {
            impl Error {
            $(
                #[doc = $desc]
                pub const $konst: Error = unsafe { Error::from_u8($val) };
            )+
            }

            impl Status {
            $(
                #[doc = $desc]
                pub const $konst: Status = Error::$konst.to_status();
            )+
            }

            #[cfg(feature = "defmt")]
            impl defmt::Format for Error {
                fn format(&self, fmt: defmt::Formatter) {
                    match *self {
                        $(
                        Self::$konst => defmt::write!(fmt, $desc),
                        )+
                        _ => defmt::write!(fmt, "Unknown error: {}", self.0),
                    }
                }
            }

            impl core::fmt::Debug for Error {
                fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    match *self {
                        $(
                        Self::$konst => core::write!(fmt, $desc),
                        )+
                        _ => core::write!(fmt, "Unknown errno: {}", self.0),
                    }
                }
            }
        }
    }

errnos! {
    (0x01, UNKNOWN_CMD, "Unknown HCI Command");
    (0x02, UNKNOWN_CONN_IDENTIFIER, "Unknown Connection Identifier");
    (0x03, HARDWARE_FAILURE, "Hardware Failure");
    (0x04, PAGE_TIMEOUT, "Page Timeout");
    (0x05, AUTHENTICATION_FAILURE, "Authentication Failure");
    (0x06, PIN_OR_KEY_MISSING, "PIN or Key Missing");
    (0x07, MEMORY_CAPACITY_EXCEEDED, "Memory Capacity Exceeded");
    (0x08, CONN_TIMEOUT, "Connection Timeout");
    (0x09, CONN_LIMIT_EXCEEDED, "Connection Limit Exceeded");
    (0x0A, SYNCHRONOUS_CONN_LIMIT_EXCEEDED, "Synchronous Connection Limit To A Device Exceeded");
    (0x0B, CONN_ALREADY_EXISTS, "Connection Already Exists");
    (0x0C, CMD_DISALLOWED, "Command Disallowed");
    (0x0D, CONN_REJECTED_LIMITED_RESOURCES, "Connection Rejected due to Limited Resources");
    (0x0E, CONN_REJECTED_SECURITY_REASONS, "Connection Rejected Due To Security Reasons");
    (0x0F, CONN_REJECTED_UNACCEPTABLE_BD_ADDR, "Connection Rejected due to Unacceptable BD_ADDR");
    (0x10, CONN_ACCEPT_TIMEOUT_EXCEEDED, "Connection Accept Timeout Exceeded");
    (0x11, UNSUPPORTED, "Unsupported Feature or Parameter Value");
    (0x12, INVALID_HCI_PARAMETERS, "Invalid HCI Command Parameters");
    (0x13, REMOTE_USER_TERMINATED_CONN, "Remote User Terminated Connection");
    (0x14, REMOTE_DEVICE_TERMINATED_CONN_LOW_RESOURCES, "Remote Device Terminated Connection due to Low Resources");
    (0x15, REMOTE_DEVICE_TERMINATED_CONN_POWER_OFF, "Remote Device Terminated Connection due to Power Off");
    (0x16, CONN_TERMINATED_BY_LOCAL_HOST, "Connection Terminated By Local Host");
    (0x17, REPEATED_ATTEMPTS, "Repeated Attempts");
    (0x18, PAIRING_NOT_ALLOWED, "Pairing Not Allowed");
    (0x19, UNKNOWN_LMP_PDU, "Unknown LMP PDU");
    (0x1A, UNSUPPORTED_REMOTE_FEATURE, "Unsupported Remote Feature");
    (0x1B, SCO_OFFSET_REJECTED, "SCO Offset Rejected");
    (0x1C, SCO_INTERVAL_REJECTED, "SCO Interval Rejected");
    (0x1D, SCO_AIR_MODE_REJECTED, "SCO Air Mode Rejected");
    (0x1E, INVALID_LMP_LL_PARAMETERS, "Invalid LMP Parameters / Invalid LL Parameters");
    (0x1F, UNSPECIFIED, "Unspecified Error");
    (0x20, UNSUPPORTED_LMP_LL_PARAMETER_VALUE, "Unsupported LMP Parameter Value / Unsupported LL Parameter Value");
    (0x21, ROLE_CHANGE_NOT_ALLOWED, "Role Change Not Allowed");
    (0x22, LMP_LL_RESPONSE_TIMEOUT, "LMP Response Timeout / LL Response Timeout");
    (0x23, LMP_LL_COLLISION, "LMP Error Transaction Collision / LL Procedure Collision");
    (0x24, LMP_PDU_NOT_ALLOWED, "LMP PDU Not Allowed");
    (0x25, ENCRYPTION_MODE_NOT_ACCEPTABLE, "Encryption Mode Not Acceptable");
    (0x26, LINK_KEY_CANNOT_BE_CHANGED, "Link Key cannot be Changed");
    (0x27, REQUESTED_QOS_NOT_SUPPORTED, "Requested QoS Not Supported");
    (0x28, INSTANT_PASSED, "Instant Passed");
    (0x29, PAIRING_WITH_UNIT_KEY_NOT_SUPPORTED, "Pairing With Unit Key Not Supported");
    (0x2A, DIFFERENT_TRANSACTION_COLLISION, "Different Transaction Collision");
    (0x2C, QOS_UNACCEPTABLE_PARAMETER, "QoS Unacceptable Parameter");
    (0x2D, QOS_REJECTED, "QoS Rejected");
    (0x2E, CHANNEL_CLASSIFICATION_NOT_SUPPORTED, "Channel Classification Not Supported");
    (0x2F, INSUFFICIENT_SECURITY, "Insufficient Security");
    (0x30, PARAMETER_OUT_OF_RANGE, "Parameter Out Of Mandatory Range");
    (0x32, ROLE_SWITCH_PENDING, "Role Switch Pending");
    (0x34, RESERVED_SLOT_VIOLATION, "Reserved Slot Violation");
    (0x35, ROLE_SWITCH_FAILED, "Role Switch Failed");
    (0x36, EXT_INQUIRY_RESPONSE_TOO_LARGE, "Extended Inquiry Response Too Large");
    (0x37, SECURE_SIMPLE_PAIRING_NOT_SUPPORTED_BY_HOST, "Secure Simple Pairing Not Supported By Host");
    (0x38, HOST_BUSY_PAIRING, "Host Busy - Pairing");
    (0x39, CONN_REJECTED_NO_SUITABLE_CHANNEL_FOUND, "Connection Rejected due to No Suitable Channel Found");
    (0x3A, CONTROLLER_BUSY, "Controller Busy");
    (0x3B, UNACCEPTABLE_CONN_PARAMETERS, "Unacceptable Connection Parameters");
    (0x3C, ADV_TIMEOUT, "Advertising Timeout");
    (0x3D, CONN_TERMINATED_DUE_TO_MIC_FAILURE, "Connection Terminated due to MIC Failure");
    (0x3E, CONN_FAILED_SYNCHRONIZATION_TIMEOUT, "Connection Failed to be Established / Synchronization Timeout");
    (0x40, COARSE_CLOCK_ADJUSTMENT_REJECTED, "Coarse Clock Adjustment Rejected but Will Try to Adjust Using Clock Dragging");
    (0x41, TYPE0_SUBMAP_NOT_DEFINED, "Type0 Submap Not Defined");
    (0x42, UNKNOWN_ADV_IDENTIFIER, "Unknown Advertising Identifier");
    (0x43, LIMIT_REACHED, "Limit Reached");
    (0x44, OPERATION_CANCELLED_BY_HOST, "Operation Cancelled by Host");
    (0x45, PACKET_TOO_LONG, "Packet Too Long");
}
