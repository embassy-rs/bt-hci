//! Bluetooth Core Specification Vol 4, Part E, §7.4

use super::cmd;
use crate::param::{BdAddr, CmdMask, CoreSpecificationVersion, LmpFeatureMask};

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.4.1
    ReadLocalVersionInformation(INFO_PARAMS, 0x0001) {
        Params = ();
        /// Bluetooth Core Specification Vol 4, Part E, §7.4.1
        ReadLocalVersionInformationReturn {
            hci_version: CoreSpecificationVersion,
            hci_subversion: u16,
            lmp_version: CoreSpecificationVersion,
            company_identifier: u16,
            lmp_subversion: u16,
        }
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.4.2
    ReadLocalSupportedCmds(INFO_PARAMS, 0x0002) {
        Params = ();
        Return = CmdMask;
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.4.3
    ReadLocalSupportedFeatures(INFO_PARAMS, 0x0003) {
        Params = ();
        Return = LmpFeatureMask;
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.4.6
    ReadBdAddr(INFO_PARAMS, 0x0009) {
        Params = ();
        Return = BdAddr;
    }
}
