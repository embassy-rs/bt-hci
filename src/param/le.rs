use super::{param, Duration};

param!(struct AddrKind(u8));

impl AddrKind {
    pub const PUBLIC: AddrKind = AddrKind(0);
    pub const RANDOM: AddrKind = AddrKind(1);
    pub const RESOLVABLE_PRIVATE_OR_PUBLIC: AddrKind = AddrKind(2);
    pub const RESOLVABLE_PRIVATE_OR_RANDOM: AddrKind = AddrKind(3);
    pub const ANONYMOUS_ADV: AddrKind = AddrKind(0xff);
}

param! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    bitfield AdvChannelMap[1] {
        (0, is_channel_37_enabled, enable_channel_37);
        (1, is_channel_38_enabled, enable_channel_38);
        (2, is_channel_39_enabled, enable_channel_39);
    }
}

impl AdvChannelMap {
    pub const ALL: AdvChannelMap = AdvChannelMap(0x07);
    pub const CHANNEL_37: AdvChannelMap = AdvChannelMap(0x01);
    pub const CHANNEL_38: AdvChannelMap = AdvChannelMap(0x02);
    pub const CHANNEL_39: AdvChannelMap = AdvChannelMap(0x04);
}

impl Default for AdvChannelMap {
    fn default() -> Self {
        Self::ALL
    }
}

param!(struct ChannelMap([u8; 5]));

impl ChannelMap {
    pub fn is_channel_bad(&self, channel: u8) -> bool {
        let byte = usize::from(channel / 8);
        let bit = channel % 8;
        (self.0[byte] & (1 << bit)) != 0
    }

    pub fn set_channel_bad(&mut self, channel: u8, bad: bool) {
        let byte = usize::from(channel / 8);
        let bit = channel % 8;
        self.0[byte] = (self.0[byte] & !(1 << bit)) | (u8::from(bad) << bit);
    }
}

param! {
    enum AdvKind {
        AdvInd = 0,
        AdvDirectIndHigh = 1,
        AdvScanInd = 2,
        AdvNonconnInd = 3,
        AdvDirectIndLow = 4,
    }
}

param! {
    enum AdvFilterPolicy {
        Unfiltered = 0,
        FilterScan = 1,
        FilterConn = 2,
        FilterConnAndScan = 3,
    }
}

param! {
    enum LeScanKind {
        Passive = 0,
        Active = 1,
    }
}

param! {
    enum ScanningFilterPolicy {
        BasicUnfiltered = 0,
        BasicFiltered = 1,
        ExtUnfiltered = 2,
        ExtFiltered = 3,
    }
}

param! {
    enum PhyKind {
        Le1M = 1,
        Le2M = 2,
        LeCodedS8 = 3,
        LeCodedS2 = 4,
    }
}

param! {
    bitfield AllPhys[1] {
        (0, has_no_tx_phy_preference, set_has_no_tx_phy_preference);
        (1, has_no_rx_phy_preference, set_has_no_rx_phy_preference);
    }
}

param! {
    bitfield PhyMask[1] {
        (0, is_le_1m_preferred, set_le_1m_preferred);
        (1, is_le_2m_preferred, set_le_2m_preferred);
        (2, is_le_coded_preferred, set_le_coded_preferred);
    }
}

param! {
    enum PhyOptions {
        NoPreferredCoding = 0,
        S2CodingPreferred = 1,
        S8CodingPreferred = 2,
    }
}

param!(struct AdvHandle(u8));

param! {
    bitfield AdvEventProps[1] {
        (0, connectable_adv, set_connectable_adv);
        (1, scannable_adv, set_scannable_adv);
        (2, directed_adv, set_directed_adv);
        (3, high_duty_cycle_directed_connectable_adv, set_high_duty_cycle_directed_connectable_adv);
        (4, legacy_adv, set_legacy_adv);
        (5, anonymous_adv, set_anonymous_adv);
        (6, include_tx_power, set_include_tx_power);
    }
}

param! {
    enum Operation {
        IntermediateFragment = 0,
        FirstFragment = 1,
        LastFragment = 2,
        Complete = 3,
        Unchanged = 4,
    }
}

param! {
    struct AdvSet {
        adv_handle: AdvHandle,
        duration: Duration<16>,
        max_ext_adv_events: u8,
    }
}

param!(&'a [AdvSet]);

param! {
    bitfield PeriodicAdvProps[2] {
        (6, is_tx_power_included, include_tx_power);
    }
}

param! {
    enum FilterDuplicates {
        Disabled = 0,
        Enabled = 1,
        EnabledPerScanPeriod = 2,
    }
}

param! {
    bitfield LePeriodicAdvCreateSyncOptions[1] {
        (0, is_using_periodic_adv_list, use_periodic_adv_list);
        (1, is_reporting_initially_disabled, disable_initial_reporting);
        (2, is_duplicate_filtering_enabled, enable_duplicate_filtering);
    }
}

param! {
    bitfield CteMask[1] {
        (0, is_aoa_cte, set_aoa_cte);
        (1, is_aod_1us_cte, set_aod_1us_cte);
        (2, is_aod_2us_cte, set_aod_2us_cte);
        (3, is_type_3_cte, set_type_3_cte);
        (4, is_non_cte, set_non_cte);
    }
}

param!(struct SyncHandle(u16));

param! {
    enum PrivacyMode {
        Network = 0,
        Device = 1,
    }
}

param! {
    enum CteKind {
        AoA = 0,
        AoD1Us = 1,
        AoD2Us = 2,
    }
}

param! {
    bitfield SwitchingSamplingRates[1] {
        (0, is_1us_aod_tx, set_1us_aod_tx);
        (1, is_1us_aod_rx, set_1us_aod_rx);
        (2, is_1us_aoa_rx, set_1us_aoa_rx);
    }
}

param! {
    bitfield LePeriodicAdvReceiveEnable[1] {
        (0, is_reporting, set_reporting);
        (1, is_duplicate_filtering, set_duplicate_filtering);
    }
}

param! {
    enum LePeriodicAdvSyncTransferMode {
        NoSync = 0,
        SyncRx = 1,
        SyncRxReport = 2,
        SyncRxReportFilterDuplicates = 3,
    }
}

param! {
    bitfield LeDataRelatedAddrChangeReasons[1] {
        (0, change_on_adv_data_change, set_change_addr_on_adv_data_changes);
        (1, change_on_scan_repsonse_data_change, set_change_addr_on_scan_response_data_changes);
    }
}
