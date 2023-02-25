use super::cmd;
use crate::param::{BdAddr, CmdMask, CoreSpecificationVersion, LmpFeatureMask};

cmd! {
    ReadLocalVersionInformation(INFO_PARAMS, 0x0001) {
        Params {}
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
    ReadLocalSupportedCmds(INFO_PARAMS, 0x0002) {
        Params {}
        Return = CmdMask;
    }
}

cmd! {
    ReadLocalSupportedFeatures(INFO_PARAMS, 0x0003) {
        Params {}
        Return = LmpFeatureMask;
    }
}

cmd! {
    ReadBdAddr(INFO_PARAMS, 0x0009) {
        Params {}
        Return = BdAddr;
    }
}
