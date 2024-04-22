//! Informational parameters [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-42372304-c9ef-dcab-6905-4e5b64703d45)

use super::cmd;
use crate::param::{BdAddr, CmdMask, CoreSpecificationVersion, LmpFeatureMask};

cmd! {
    /// Read Local Version Information command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-cf7fef88-faa4-fd2e-7c00-ab1ec7985a19)
    ReadLocalVersionInformation(INFO_PARAMS, 0x0001) {
        Params = ();
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
    /// Read Local Supported Commands command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-d9df0f48-030f-0567-ecf3-8304df5c3eb0)
    ReadLocalSupportedCmds(INFO_PARAMS, 0x0002) {
        Params = ();
        Return = CmdMask;
    }
}

cmd! {
    /// Read Local Supported Features command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-17c54ce9-1772-096f-c512-ba080bd11d04)
    ReadLocalSupportedFeatures(INFO_PARAMS, 0x0003) {
        Params = ();
        Return = LmpFeatureMask;
    }
}

cmd! {
    /// Read BD_ADDR command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-151a8bec-71be-df54-2043-92d366376c53)
    ReadBdAddr(INFO_PARAMS, 0x0009) {
        Params = ();
        Return = BdAddr;
    }
}
