use crate::param::macros::param;

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
    bitfield SyncPacketType[2]{
        // Basic synchronous packet types
        (0, hv1_may_be_used, set_hv1_may_be_used);
        (1, hv2_may_be_used, set_hv2_may_be_used);
        (2, hv3_may_be_used, set_hv3_may_be_used);
        // Extended synchronous packet types
        (3, ev3_may_be_used, set_ev3_may_be_used);
        (4, ev4_may_be_used, set_ev4_may_be_used);
        (5, ev5_may_be_used, set_ev5_may_be_used);
        // EDR synchronous packet types that shall not be used
        (6, shall_not_be_used_2ev3, set_shall_not_be_used_2ev3);
        (7, shall_not_be_used_3ev3, set_shall_not_be_used_3ev3);
        (8, shall_not_be_used_2ev5, set_shall_not_be_used_2ev5);
        (9, shall_not_be_used_3ev5, set_shall_not_be_used_3ev5);
        // All other bits reserved for future use
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
    bitfield VoiceSetting[2]{
        // Bits 0-1: Air coding format
        (0, air_coding_format_0, set_air_coding_format_0);
        (1, air_coding_format_1, set_air_coding_format_1);
        // Bits 2-4: Linear PCM bit position
        (2, linear_pcm_bit_position_0, set_linear_pcm_bit_position_0);
        (3, linear_pcm_bit_position_1, set_linear_pcm_bit_position_1);
        (4, linear_pcm_bit_position_2, set_linear_pcm_bit_position_2);
        // Bit 5: Input sample size (only for linear PCM)
        (5, input_sample_size, set_input_sample_size);
        // Bits 6-7: Input data format
        (6, input_data_format_0, set_input_data_format_0);
        (7, input_data_format_1, set_input_data_format_1);
        // Bits 8-9: Input coding format
        (8, input_coding_format_0, set_input_coding_format_0);
        (9, input_coding_format_1, set_input_coding_format_1);
        // Bits 10-15: Reserved for future use
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
    enum IoCapability{
        /// Display Only
        DisplayOnly = 0x00,
        /// Display Yes/No
        DisplayYesNo = 0x01,
        /// Keyboard Only
        KeyboardOnly = 0x02,
        /// No Input No Output
        NoInputNoOutput = 0x03,
        // All other values reserved for future use
    }
}

param! {
    enum OobDataPresent {
        /// OOB authentication data not present
        NotPresent = 0x00,
        /// P-192 OOB authentication data from remote device present
        P192Present = 0x01,
        /// P-256 OOB authentication data from remote device present
        P256Present = 0x02,
        /// P-192 and P-256 OOB authentication data from remote device present
        P192AndP256Present = 0x03,
        // All other values reserved for future use
    }
}

param! {
    enum AuthenticationRequirements {
        /// MITM Protection Not Required – No Bonding. Numeric comparison with automatic accept allowed.
        MitmNotRequiredNoBonding = 0x00,
        /// MITM Protection Required – No Bonding. Use IO Capabilities to determine authentication procedure
        MitmRequiredNoBonding = 0x01,
        /// MITM Protection Not Required – Dedicated Bonding. Numeric comparison with automatic accept allowed.
        MitmNotRequiredDedicatedBonding = 0x02,
        /// MITM Protection Required – Dedicated Bonding. Use IO Capabilities to determine authentication procedure
        MitmRequiredDedicatedBonding = 0x03,
        /// MITM Protection Not Required – General Bonding. Numeric Comparison with automatic accept allowed.
        MitmNotRequiredGeneralBonding = 0x04,
        /// MITM Protection Required – General Bonding. Use IO capabilities to determine authentication procedure.
        MitmRequiredGeneralBonding = 0x05,
        // All other values reserved for future use
    }
}

param! {
    enum Role {
        /// Become the Central for this connection. The LM will perform the role switch.
        Central = 0x00,
        /// Remain the Peripheral for this connection. The LM will NOT perform the role switch.
        Peripheral = 0x01,
    }
}

param! {
    enum RejectReason {
        /// Connection Rejected due to Limited Resources
        LimitedResources = 0x0D,
        /// Connection Rejected Due To Security Reasons
        SecurityReasons = 0x0E,
        /// Connection Rejected due to Unacceptable BD_ADDR
        UnacceptableBdAddr = 0x0F,
    }
}

param! {
    enum KeyFlag{
        /// Use semi-permanent Link Keys.
        SemiPermanent = 0x00,
        /// Use Temporary Link Key.
        Temporary = 0x01,
    }
}

param! {
    enum RetransmissionEffort{
        /// No retransmissions (SCO or eSCO connection allowed)
        NoRetransmissions = 0x00,
        /// At least one retransmission, optimize for power consumption (eSCO connection required)
        OptimizePowerConsumption = 0x01,
        /// At least one retransmission, optimize for link quality (eSCO connection required)
        OptimizeLinkQuality = 0x02,
        /// Don't care (SCO or eSCO connection allowed)
        DontCare = 0xFF,
        // All other values reserved for future use
    }
}
