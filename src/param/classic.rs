use crate::param::macros::param;
use crate::param::{BdAddr, Status};

param! {
    bitfield PacketType[2] {
        // ACL Link_Type bits
        (1, shall_not_be_used_2dh1, set_shall_not_be_used_2dh1);
        (2, shall_not_be_used_3dh1, set_shall_not_be_used_3dh1);
        // Bit 3: Ignored; DM1 may be used whether or not this bit is set
        (4, dh1_may_be_used, set_dh1_may_be_used);
        // SCO Link_Type bits
        (5, hv1_may_be_used, set_hv1_may_be_used);
        (6, hv2_may_be_used, set_hv2_may_be_used);
        (7, hv3_may_be_used, set_hv3_may_be_used);
        // ACL Link_Type bits (continued)
        (8, shall_not_be_used_2dh3, set_shall_not_be_used_2dh3);
        (9, shall_not_be_used_3dh3, set_shall_not_be_used_3dh3);
        (10, dm3_may_be_used, set_dm3_may_be_used);
        (11, dh3_may_be_used, set_dh3_may_be_used);
        (12, shall_not_be_used_2dh5, set_shall_not_be_used_2dh5);
        (13, shall_not_be_used_3dh5, set_shall_not_be_used_3dh5);
        (14, dm5_may_be_used, set_dm5_may_be_used);
        (15, dh5_may_be_used, set_dh5_may_be_used);
    }
}

param! {
    bitfield ClockOffset[2] {
        // Bits 0-14: Bits 2 to 16 of CLKNPeripheral - CLK
        (0, clock_offset_0, set_clock_offset_0);
        (1, clock_offset_1, set_clock_offset_1);
        (2, clock_offset_2, set_clock_offset_2);
        (3, clock_offset_3, set_clock_offset_3);
        (4, clock_offset_4, set_clock_offset_4);
        (5, clock_offset_5, set_clock_offset_5);
        (6, clock_offset_6, set_clock_offset_6);
        (7, clock_offset_7, set_clock_offset_7);
        (8, clock_offset_8, set_clock_offset_8);
        (9, clock_offset_9, set_clock_offset_9);
        (10, clock_offset_10, set_clock_offset_10);
        (11, clock_offset_11, set_clock_offset_11);
        (12, clock_offset_12, set_clock_offset_12);
        (13, clock_offset_13, set_clock_offset_13);
        (14, clock_offset_14, set_clock_offset_14);
        // Bit 15: Clock_Offset_Valid_Flag (0 = Invalid, 1 = Valid)
        (15, clock_offset_valid_flag, set_clock_offset_valid_flag);
    }
}

param! {
    enum PageScanRepetitionMode{
        /// Page scan repetition mode 0
        R0 = 0x00,
        /// Page scan repetition mode 1
        R1 = 0x01,
        /// Page scan repetition mode 2
        R2 = 0x02,
        // Reserved for future use
    }
}

param! {
    enum AllowRoleSwitch{
        /// The local device will be a Central, and will not accept a role switch requested by
        /// the remote device at the connection setup.
        NotAllowed = 0x00,
        /// The local device may be a Central, or may become a Peripheral after accepting a role switch
        /// requested by the remote device at the connection setup.
        Allowed = 0x01,
        // Reserved for future use
    }
}

param! {
    enum ConnectionLinkType {
        /// SCO connection
        Sco = 0x00,
        /// ACL connection (Data Channels)
        Acl = 0x01,
        /// eSCO connection (Enhanced Data Channels)
        EnhancedSco = 0x02,
        // All other values reserved for future use
    }
}

param! {
    enum LinkKeyType {
        /// Combination Key
        CombinationKey = 0x00,
        /// Debug Combination Key
        DebugCombinationKey = 0x03,
        /// Unauthenticated Combination Key generated from P-192
        UnauthenticatedCombinationKeyP192 = 0x04,
        /// Authenticated Combination Key generated from P-192
        AuthenticatedCombinationKeyP192 = 0x05,
        /// Changed Combination Key
        ChangedCombinationKey = 0x06,
        /// Unauthenticated Combination Key generated from P-256
        UnauthenticatedCombinationKeyP256 = 0x07,
        /// Authenticated Combination Key generated from P-256
        AuthenticatedCombinationKeyP256 = 0x08,
        // All other values reserved for future use
    }
}

param! {
    /// Common return parameters for commands that return a status and a Bluetooth device address
    struct StatusBdAddrReturn {
        status: Status,
        bd_addr: BdAddr,
    }
}
