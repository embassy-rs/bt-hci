use super::cmd;
use crate::param::events::LeEventMask;
use crate::param::features::LeFeatureMask;
use crate::param::le::{AddrKind, AdvChannelMap, ChannelMap};
use crate::param::{param, BdAddr, ConnHandle, Duration};

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

cmd! {
    LeSetEventMask(LE, 0x0001) {
        Params {
            mask: LeEventMask,
        }
        Return = ();
    }
}

cmd! {
    LeReadBufferSize(LE, 0x0002) {
        Params {}
        LeReadBufferSizeReturn {
            le_acl_data_packet_length: u16,
            total_num_le_acl_data_packets: u8,
        }
    }
}

cmd! {
    LeReadLocalSupportedFeatures(LE, 0x0003) {
        Params {}
        Return = LeFeatureMask;
    }
}

cmd! {
    LeSetRandomAddr(LE, 0x0005) {
        Params {
            random_addr: BdAddr,
        }
        Return = ();
    }
}

cmd! {
    LeSetAdvParams(LE, 0x0006) {
        Params {
            adv_interval_min: Duration<1>,
            adv_interval_max: Duration<1>,
            adv_kind: AdvKind,
            own_addr_kind: AddrKind,
            peer_addr_kind: AddrKind,
            adv_channel_map: AdvChannelMap,
            adv_filter_policy: AdvFilterPolicy,
        }
        Return = ();
    }
}

cmd! {
    LeReadAdvPhysicalChannelTxPower(LE, 0x0007) {
        Params {}
        Return = i8;
    }
}

cmd! {
    LeSetAdvData(LE, 0x0008) {
        Params {
            data_len: u8,
            data: [u8; 31],
        }
        Return = ();
    }
}

cmd! {
    LeSetScanResponseData(LE, 0x0009) {
        Params {
            data_len: u8,
            data: [u8; 31],
        }
        Return = ();
    }
}

cmd! {
    LeSetAdvEnable(LE, 0x000a) {
        Params {
            enable: bool,
        }
        Return = ();
    }
}

cmd! {
    LeSetScanParams(LE, 0x000b) {
        Params {
            le_scan_kind: LeScanKind,
            le_scan_interval: Duration<16>,
            le_scan_window: Duration<16>,
            own_addr_kind: AddrKind,
            scanning_filter_policy: ScanningFilterPolicy,
        }
        Return = ();
    }
}

cmd! {
    LeSetScanEnable(LE, 0x000c) {
        Params {
            enable: bool,
            filter_duplicates: bool,
        }
        Return = ();
    }
}

cmd! {
    LeCreateConn(LE, 0x000d) {
        Params {
            le_scan_interval: Duration<16>,
            le_scan_window: Duration<16>,
            use_filter_accept_list: bool,
            peer_addr_kind: AddrKind,
            peer_addr: BdAddr,
            own_addr_kind: AddrKind,
            conn_interval_min: Duration<2>,
            conn_interval_max: Duration<2>,
            max_latency: u16,
            supervision_timeout: Duration<16>,
            min_ce_length: Duration<1>,
            max_ce_length: Duration<1>,
        }
    }
}

cmd! {
    LeCreateConnCancel(LE, 0x000e) {
        Params {}
        Return = ();
    }
}

cmd! {
    LeReadFilterAcceptListSize(LE, 0x000f) {
        Params {}
        Return = u8;
    }
}

cmd! {
    LeClearFilterAcceptList(LE, 0x0010) {
        Params {}
        Return = ();
    }
}

cmd! {
    LeAddDeviceToFilterAcceptList(LE, 0x0011) {
        Params {
            addr_kind: AddrKind,
            addr: BdAddr,
        }
        Return = ();
    }
}

cmd! {
    LeRemoveDeviceFromFilterAcceptList(LE, 0x0012) {
        Params {
            addr_kind: AddrKind,
            addr: BdAddr,
        }
        Return = ();
    }
}

cmd! {
    LeConnUpdate(LE, 0x0013) {
        Params {
            handle: ConnHandle,
            conn_interval_min: Duration<2>,
            conn_interval_max: Duration<2>,
            max_latency: u16,
            supervision_timeout: Duration<16>,
            min_ce_length: Duration<1>,
            max_ce_length: Duration<1>,
        }
    }
}

cmd! {
    LeSetHostChannelClassification(LE, 0x0014) {
        Params {
            channel_map: ChannelMap,
        }
        Return = ();
    }
}

cmd! {
    LeReadChannelMap(LE, 0x0015) {
        Params {
            handle: ConnHandle,
        }
        LeReadChannelMapReturn {
            handle: ConnHandle,
            channel_map: ChannelMap,
        }
    }
}

cmd! {
    LeReadRemoteFeatures(LE, 0x0016) {
        Params {
            handle: ConnHandle,
        }
    }
}

cmd! {
    LeEncrypt(LE, 0x0017) {
        Params {
            key: [u8; 16],
            plaintext: [u8; 16],
        }
        Return = [u8; 16];
    }
}

cmd! {
    LeRand(LE, 0x0018) {
        Params {}
        Return = [u8; 8];
    }
}

cmd! {
    LeEnableEncryption(LE, 0x0019) {
        Params {
            handle: ConnHandle,
            random: [u8; 8],
            encrypted_diversifier: u16,
            long_term_key: [u8; 16],
        }
        Return = ();
    }
}

cmd! {
    LeLongTermKeyRequestReply(LE, 0x001a) {
        Params {
            handle: ConnHandle,
            long_term_key: [u8; 16],
        }
        Return = ConnHandle;
    }
}

cmd! {
    LeLongTermKeyRequestNegativeReply(LE, 0x001b) {
        Params {
            handle: ConnHandle,
        }
        Return = ConnHandle;
    }
}

cmd! {
    LeReadSupportedStates(LE, 0x001c) {
        Params {}
        Return = [u8; 8];
    }
}

cmd! {
    LeTestEnd(LE, 0x001f) {
        Params {}
        Return = u16;
    }
}

cmd! {
    LeSetDataLength(LE, 0x0022) {
        Params {
            handle: ConnHandle,
            tx_octets: u16,
            tx_time: u16,
        }
        Return = ConnHandle;
    }
}

cmd! {
    LeReadSuggestedDefaultDataLength(LE, 0x0023) {
        Params {}
        LeReadSuggestedDefaultDataLengthReturn {
            suggested_max_tx_octets: u16,
            suggested_max_tx_time: u16,
        }
    }
}

cmd! {
    LeWriteSuggestedDefaultDataLength(LE, 0x0024) {
        Params {
            suggested_max_tx_octets: u16,
            suggested_max_tx_time: u16,
        }
        Return = ();
    }
}

cmd! {
    LeAddDeviceToResolvingList(LE, 0x0027) {
        Params {
            peer_id_addr_kind: AddrKind,
            peer_id_addr: BdAddr,
            peer_irk: [u8; 16],
            local_irk: [u8; 16],
        }
        Return = ();
    }
}

cmd! {
    LeRemoveDeviceFromResolvingList(LE, 0x0028) {
        Params {
            peer_id_addr_kind: AddrKind,
            peer_id_addr: BdAddr,
        }
        Return = ();
    }
}

cmd! {
    LeClearResolvingList(LE, 0x0029) {
        Params {}
        Return = ();
    }
}

cmd! {
    LeReadResolvingListSize(LE, 0x002a) {
        Params {}
        Return = u8;
    }
}

cmd! {
    LeSetAddrResolutionEnable(LE, 0x002d) {
        Params {
            enable: bool,
        }
        Return = ();
    }
}

cmd! {
    LeSetResolvablePrivateAddrTimeout(LE, 0x002e) {
        Params {
            rpa_timeout: Duration<1600>,
        }
        Return = ();
    }
}

cmd! {
    LeReadMaxDataLength(LE, 0x002f) {
        Params {}
        LeReadMaxDataLengthReturn {
            supported_max_tx_octets: u16,
            supported_max_tx_time: u16,
            supported_max_rx_octets: u16,
            supported_max_rx_time: u16,
        }
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

cmd! {
    LeReadPhy(LE, 0x0030) {
        Params {
                handle: ConnHandle,
        }
        LeReadPhyReturn {
                handle: ConnHandle,
                tx_phy: PhyKind,
                rx_phy: PhyKind,
        }
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

cmd! {
    LeSetDefaultPhy(LE, 0x0031) {
        Params {
            all_phys: AllPhys,
            tx_phys: PhyMask,
            rx_phys: PhyMask,
        }
        Return = ();
    }
}

param! {
    enum PhyOptions {
        NoPreferredCoding = 0,
        S2CodingPreferred = 1,
        S8CodingPreferred = 2,
    }
}

cmd! {
    LeSetPhy(LE, 0x0032) {
        Params {
            handle: ConnHandle,
            all_phys: AllPhys,
            tx_phys: PhyMask,
            rx_phys: PhyMask,
            phy_options: PhyOptions,
        }
        Return = ();
    }
}

param!(struct AdvHandle(u8));

cmd! {
    LeSetAdvSetRandomAddr(LE, 0x0035) {
        Params {
            adv_handle: AdvHandle,
            random_addr: BdAddr,
        }
        Return = ();
    }
}

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

cmd! {
    LeSetExtAdvParams(LE, 0x0036) {
        Params {
                adv_handle: AdvHandle,
                adv_event_props: AdvEventProps,
                primary_adv_interval_min: Duration<1>,
                primary_adv_interval_max: Duration<1>,
                primary_adv_channel_map: AdvChannelMap,
                own_addr_kind: AddrKind,
                peer_addr_kind: AddrKind,
                peer_addr: BdAddr,
                adv_filter_policy: AdvFilterPolicy,
                adv_tx_power: i8,
                primary_adv_phy: PhyKind,
                secondary_adv_max_skip: u8,
                secondary_adv_phy: PhyKind,
                adv_sid: u8,
                scan_request_notification_enable: bool,
        }
        Return = i8;
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

cmd! {
    LeSetExtAdvData(LE, 0x0037) {
        Params<'d> {
            adv_handle: AdvHandle,
            operation: Operation,
            fragment_preference: bool,
            adv_data: &'d [u8],
        }
        Return = ();
    }
}

cmd! {
    LeSetExtScanResponseData(LE, 0x0038) {
        Params<'d> {
            adv_handle: AdvHandle,
            operation: Operation,
            fragment_preference: bool,
            scan_response_data: &'d [u8],
        }
        Return = ();
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

cmd! {
    LeSetExtAdvEnable(LE, 0x0039) {
        Params<'a> {
            enable: bool,
            sets: &'a [AdvSet],
        }
        Return = ();
    }
}

cmd! {
    LeReadMaxAdvDataLength(LE, 0x003a) {
        Params {}
        Return = u16;
    }
}

cmd! {
    LeReadNumberOfSupportedAdvSets(LE, 0x003b) {
        Params {}
        Return = u8;
    }
}

cmd! {
    LeRemoveAdvSet(LE, 0x003c) {
        Params {
            adv_handle: AdvHandle,
        }
        Return = ();
    }
}

cmd! {
    LeClearAdvSets(LE, 0x003d) {
        Params {}
        Return = ();
    }
}

param! {
    bitfield PeriodicAdvProps[2] {
        (6, is_tx_power_included, include_tx_power);
    }
}

cmd! {
    LeSetPeriodicAdvParams(LE, 0x003e) {
        Params {
            adv_handle: AdvHandle,
            periodic_adv_interval_min: Duration<4>,
            periodic_adv_interval_max: Duration<4>,
            periodic_adv_props: PeriodicAdvProps,
        }
        Return = ();
    }
}

cmd! {
    LeSetPeriodicAdvData(LE, 0x003f) {
        Params<'a> {
            adv_handle: AdvHandle,
            operation: Operation,
            adv_data: &'a [u8],
        }
        Return = ();
    }
}

cmd! {
    LeSetPeriodicAdvEnable(LE, 0x0040) {
        Params {
            enable: bool,
            adv_handle: AdvHandle,
        }
        Return = ();
    }
}

// TODO!
cmd! {
    LeSetExtScanParams(LE, 0x0041) {
        Params {}
        Return = ();
    }
}

param! {
    enum FilterDuplicates {
        Disabled = 0,
        Enabled = 1,
        EnabledPerScanPeriod = 2,
    }
}

cmd! {
    LeSetExtScanEnable(LE, 0x0042) {
        Params {
            enable: bool,
            filter_duplicates: FilterDuplicates,
            duration: Duration<16>,
            period: Duration<2048>,
        }
        Return = ();
    }
}

// TODO!
cmd! {
    LeExtCreateConn(LE, 0x0043) {
        Params {
            initiator_filter_policy: bool,
            own_addr_kind: AddrKind,
            peer_addr_kind: AddrKind,
            peer_addr: BdAddr,
            initiating_phys: PhyMask,
        }
        Return = ();
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

cmd! {
    LePeriodicAdvCreateSync(LE, 0x0044) {
        Params {
            options: LePeriodicAdvCreateSyncOptions,
            adv_sid: u8,
            adv_addr_kind: AddrKind,
            adv_addr: BdAddr,
            skip: u16,
            sync_timeout: Duration<16>,
            sync_cte_kind: CteMask,
        }
        Return = ();
    }
}
cmd! {
    LePeriodicAdvCreateSyncCancel(LE, 0x0045) {
        Params {}
        Return = ();
    }
}

param!(struct SyncHandle(u16));

cmd! {
    LePeriodicAdvTerminateSync(LE, 0x0046) {
        Params {
            sync_handle: SyncHandle,
        }
        Return = ();
    }
}

cmd! {
    LeAddDeviceToPeriodicAdvList(LE, 0x0047) {
        Params {
            adv_addr_kind: AddrKind,
            adv_addr: BdAddr,
            adv_sid: u8,
        }
        Return = ();
    }
}

cmd! {
    LeRemoveDeviceFromPeriodicAdvList(LE, 0x0048) {
        Params {
            adv_addr_kind: AddrKind,
            adv_addr: BdAddr,
            adv_sid: u8,
        }
        Return = ();
    }
}

cmd! {
    LeClearPeriodicAdvList(LE, 0x0049) {
        Params {}
        Return = ();
    }
}

cmd! {
    LeReadPeriodicAdvListSize(LE, 0x004a) {
        Params {}
        Return = u8;
    }
}

cmd! {
    LeReadTransmitPower(LE, 0x004b) {
        Params {}
        Return = (i8, i8);
    }
}

cmd! {
    LeReadRfPathCompensation(LE, 0x004c) {
        Params {}
        Return = (i16, i16);
    }
}

cmd! {
    LeWriteRfPathCompensation(LE, 0x004d) {
        Params {
            rf_tx_path_compensation_value: i16,
            rf_rx_path_compensation_value: i16,
        }
        Return = ();
    }
}

param! {
    enum PrivacyMode {
        Network = 0,
        Device = 1,
    }
}

cmd! {
    LeSetPrivacyMode(LE, 0x004e) {
        Params {
            peer_id_addr_kind: AddrKind,
            peer_id_addr: BdAddr,
            privacy_mode: PrivacyMode,
        }
        Return = ();
    }
}

param! {
    enum CteKind {
        AoA = 0,
        AoD1Us = 1,
        AoD2Us = 2,
    }
}

cmd! {
    LeSetConnectionlessCteTransmitParams(LE, 0x0051) {
        Params<'a> {
            adv_handle: AdvHandle,
            cte_length: u8,
            cte_kind: CteKind,
            cte_count: u8,
            switching_pattern: &'a [u8],
        }
        Return = ();
    }
}

cmd! {
    LeSetConnectionlessCteTransmitEnable(LE, 0x0052) {
        Params {
            adv_handle: AdvHandle,
            cte_enable: bool,
        }
        Return = ();
    }
}

cmd! {
    LeSetConnCteTransmitParams(LE, 0x0055) {
        Params<'a> {
            handle: ConnHandle,
            cte_kinds: CteMask,
            switching_pattern: &'a [u8],
        }
        Return = ConnHandle;
    }
}

cmd! {
    LeConnCteResponseEnable(LE, 0x0057) {
        Params {
            handle: ConnHandle,
            enable: bool,
        }
        Return = ConnHandle;
    }
}

param! {
    bitfield SwitchingSamplingRates[1] {
        (0, is_1us_aod_tx, set_1us_aod_tx);
        (1, is_1us_aod_rx, set_1us_aod_rx);
        (2, is_1us_aoa_rx, set_1us_aoa_rx);
    }
}

cmd! {
    LeReadAntennaInformation(LE, 0x0058) {
        Params {}
        LeReadAntennaInformationReturn {
            supported_switching_sampling_rates: SwitchingSamplingRates,
            num_antennae: u8,
            max_switching_pattern_len: u8,
            max_cte_len: u8,
        }
    }
}

param! {
    bitfield LePeriodicAdvReceiveEnable[1] {
        (0, is_reporting, set_reporting);
        (1, is_duplicate_filtering, set_duplicate_filtering);
    }
}

cmd! {
    LeSetPeriodicAdvReceiveEnable(LE, 0x0059) {
        Params {
            sync_handle: SyncHandle,
            enable: LePeriodicAdvReceiveEnable,
        }
        Return = ();
    }
}

cmd! {
    LePeriodicAdvSyncTransfer(LE, 0x005a) {
        Params {
            handle: ConnHandle,
            service_data: u16,
            sync_handle: SyncHandle,
        }
        Return = ConnHandle;
    }
}

cmd! {
    LePeriodicAdvSetInfoTransfer(LE, 0x005b) {
        Params {
            handle: ConnHandle,
            service_data: u16,
            adv_handle: AdvHandle,
        }
        Return = ConnHandle;
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

cmd! {
    LeSetPeriodicAdvSyncTransferParams(LE, 0x005c) {
        Params {
            handle: ConnHandle,
            mode: LePeriodicAdvSyncTransferMode,
            skip: u16,
            sync_timeout: Duration<16>,
            cte_kind: CteMask,
        }
        Return = ();
    }
}

cmd! {
    LeSetDefaultPeriodicAdvSyncTransferParams(LE, 0x005d) {
        Params {
            mode: LePeriodicAdvSyncTransferMode,
            skip: u16,
            sync_timeout: Duration<16>,
            cte_kind: CteMask,
        }
        Return = ();
    }
}

cmd! {
    LeRequestPeerSca(LE, 0x006d) {
        Params {
            handle: ConnHandle,
        }
        Return = ();
    }
}

cmd! {
    LeEnhancedReadTransmitPowerLevel(LE, 0x0076) {
        Params {
            handle: ConnHandle,
            phy: PhyKind,
        }
        LeEnhancedReadTransmitPowerLevelReturn {
            handle: ConnHandle,
            phy: PhyKind,
            current_tx_power_level: i8,
            max_tx_power_level: i8,
        }
    }
}

cmd! {
    LeReadRemoteTransmitPowerLevel(LE, 0x0077) {
        Params {
            handle: ConnHandle,
            phy: PhyKind,
        }
    }
}

cmd! {
    LeSetPathLossReportingParams(LE, 0x0078) {
        Params {
            handle: ConnHandle,
            high_threshold: i8,
            high_hysteresis: i8,
            low_threshold: i8,
            low_hysteresis: i8,
            min_time_spent: u16,
        }
        Return = ConnHandle;
    }
}

cmd! {
    LeSetPathLossReportingEnable(LE, 0x0079) {
        Params {
            handle: ConnHandle,
            enable: bool,
        }
        Return = ConnHandle;
    }
}

cmd! {
    LeSetTransmitPowerReportingEnable(LE, 0x007a) {
        Params {
            handle: ConnHandle,
            local_enable: bool,
            remote_enable: bool,
        }
        Return = ConnHandle;
    }
}

param! {
    bitfield LeDataRelatedAddrChangeReasons[1] {
        (0, change_on_adv_data_change, set_change_addr_on_adv_data_changes);
        (1, change_on_scan_repsonse_data_change, set_change_addr_on_scan_response_data_changes);
    }
}

cmd! {
    LeSetDataRelatedAddrChanges(LE, 0x007c) {
        Params {
            adv_handle: AdvHandle,
            change_reasons: LeDataRelatedAddrChangeReasons,
        }
        Return = ();
    }
}
