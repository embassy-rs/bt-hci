//! Bluetooth Core Specification Vol 4, Part E, §7.8

use super::cmd;
use crate::param::{
    AddrKind, AdvChannelMap, AdvEventProps, AdvFilterPolicy, AdvHandle, AdvKind, AdvSet, AllPhys, BdAddr, ChannelMap,
    ConnHandle, CteKind, CteMask, Duration, FilterDuplicates, InitiatingPhy, LeDataRelatedAddrChangeReasons,
    LeEventMask, LeFeatureMask, LePeriodicAdvCreateSyncOptions, LePeriodicAdvReceiveEnable,
    LePeriodicAdvSyncTransferMode, LeScanKind, Operation, PeriodicAdvProps, PhyKind, PhyMask, PhyOptions, PhyParams,
    PrivacyMode, ScanningFilterPolicy, ScanningPhy, SwitchingSamplingRates, SyncHandle,
};

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.1
    LeSetEventMask(LE, 0x0001) {
        Params {
            mask: LeEventMask,
        }
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.2
    LeReadBufferSize(LE, 0x0002) {
        Params {}
        /// Bluetooth Core Specification Vol 4, Part E, §7.8.2
        LeReadBufferSizeReturn {
            le_acl_data_packet_length: u16,
            total_num_le_acl_data_packets: u8,
        }
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.3
    LeReadLocalSupportedFeatures(LE, 0x0003) {
        Params {}
        Return = LeFeatureMask;
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.4
    LeSetRandomAddr(LE, 0x0005) {
        Params {
            random_addr: BdAddr,
        }
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.5
    LeSetAdvParams(LE, 0x0006) {
        Params {
            adv_interval_min: Duration<625>,
            adv_interval_max: Duration<625>,
            adv_kind: AdvKind,
            own_addr_kind: AddrKind,
            peer_addr_kind: AddrKind,
            peer_addr: BdAddr,
            adv_channel_map: AdvChannelMap,
            adv_filter_policy: AdvFilterPolicy,
        }
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.6
    LeReadAdvPhysicalChannelTxPower(LE, 0x0007) {
        Params {}
        Return = i8;
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.7
    LeSetAdvData(LE, 0x0008) {
        Params {
            data_len: u8,
            data: [u8; 31],
        }
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.8
    LeSetScanResponseData(LE, 0x0009) {
        Params {
            data_len: u8,
            data: [u8; 31],
        }
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.9
    LeSetAdvEnable(LE, 0x000a) {
        Params {
            enable: bool,
        }
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.10
    LeSetScanParams(LE, 0x000b) {
        Params {
            le_scan_kind: LeScanKind,
            le_scan_interval: Duration<10_000>,
            le_scan_window: Duration<10_000>,
            own_addr_kind: AddrKind,
            scanning_filter_policy: ScanningFilterPolicy,
        }
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.11
    LeSetScanEnable(LE, 0x000c) {
        Params {
            enable: bool,
            filter_duplicates: bool,
        }
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.12
    LeCreateConn(LE, 0x000d) {
        Params {
            le_scan_interval: Duration<10_000>,
            le_scan_window: Duration<10_000>,
            use_filter_accept_list: bool,
            peer_addr_kind: AddrKind,
            peer_addr: BdAddr,
            own_addr_kind: AddrKind,
            conn_interval_min: Duration<1_250>,
            conn_interval_max: Duration<1_250>,
            max_latency: u16,
            supervision_timeout: Duration<10_000>,
            min_ce_length: Duration<625>,
            max_ce_length: Duration<625>,
        }
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.13
    LeCreateConnCancel(LE, 0x000e) {
        Params {}
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.14
    LeReadFilterAcceptListSize(LE, 0x000f) {
        Params {}
        Return = u8;
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.15
    LeClearFilterAcceptList(LE, 0x0010) {
        Params {}
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.16
    LeAddDeviceToFilterAcceptList(LE, 0x0011) {
        Params {
            addr_kind: AddrKind,
            addr: BdAddr,
        }
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.17
    LeRemoveDeviceFromFilterAcceptList(LE, 0x0012) {
        Params {
            addr_kind: AddrKind,
            addr: BdAddr,
        }
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.18
    LeConnUpdate(LE, 0x0013) {
        Params {
            handle: ConnHandle,
            conn_interval_min: Duration<1_250>,
            conn_interval_max: Duration<1_250>,
            max_latency: u16,
            supervision_timeout: Duration<10_000>,
            min_ce_length: Duration<625>,
            max_ce_length: Duration<625>,
        }
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.19
    LeSetHostChannelClassification(LE, 0x0014) {
        Params {
            channel_map: ChannelMap,
        }
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.20
    LeReadChannelMap(LE, 0x0015) {
        Params {
            handle: ConnHandle,
        }
        /// Bluetooth Core Specification Vol 4, Part E, §7.8.20
        LeReadChannelMapReturn {
            handle: ConnHandle,
            channel_map: ChannelMap,
        }
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.21
    LeReadRemoteFeatures(LE, 0x0016) {
        Params {
            handle: ConnHandle,
        }
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.22
    LeEncrypt(LE, 0x0017) {
        Params {
            key: [u8; 16],
            plaintext: [u8; 16],
        }
        Return = [u8; 16];
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.23
    LeRand(LE, 0x0018) {
        Params {}
        Return = [u8; 8];
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.24
    LeEnableEncryption(LE, 0x0019) {
        Params {
            handle: ConnHandle,
            random: [u8; 8],
            encrypted_diversifier: u16,
            long_term_key: [u8; 16],
        }
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.25
    LeLongTermKeyRequestReply(LE, 0x001a) {
        Params {
            handle: ConnHandle,
            long_term_key: [u8; 16],
        }
        Return = ConnHandle;
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.26
    LeLongTermKeyRequestNegativeReply(LE, 0x001b) {
        Params {
            handle: ConnHandle,
        }
        Return = ConnHandle;
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.27
    LeReadSupportedStates(LE, 0x001c) {
        Params {}
        Return = [u8; 8];
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.30
    LeTestEnd(LE, 0x001f) {
        Params {}
        Return = u16;
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.33
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
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.34
    LeReadSuggestedDefaultDataLength(LE, 0x0023) {
        Params {}
        /// Bluetooth Core Specification Vol 4, Part E, §7.8.34
        LeReadSuggestedDefaultDataLengthReturn {
            suggested_max_tx_octets: u16,
            suggested_max_tx_time: u16,
        }
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.35
    LeWriteSuggestedDefaultDataLength(LE, 0x0024) {
        Params {
            suggested_max_tx_octets: u16,
            suggested_max_tx_time: u16,
        }
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.38
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
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.39
    LeRemoveDeviceFromResolvingList(LE, 0x0028) {
        Params {
            peer_id_addr_kind: AddrKind,
            peer_id_addr: BdAddr,
        }
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.40
    LeClearResolvingList(LE, 0x0029) {
        Params {}
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.41
    LeReadResolvingListSize(LE, 0x002a) {
        Params {}
        Return = u8;
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.44
    LeSetAddrResolutionEnable(LE, 0x002d) {
        Params {
            enable: bool,
        }
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.45
    LeSetResolvablePrivateAddrTimeout(LE, 0x002e) {
        Params {
            rpa_timeout: Duration<1_000_000>,
        }
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.46
    LeReadMaxDataLength(LE, 0x002f) {
        Params {}
        /// Bluetooth Core Specification Vol 4, Part E, §7.8.46
        LeReadMaxDataLengthReturn {
            supported_max_tx_octets: u16,
            supported_max_tx_time: u16,
            supported_max_rx_octets: u16,
            supported_max_rx_time: u16,
        }
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.47
    LeReadPhy(LE, 0x0030) {
        Params {
                handle: ConnHandle,
        }
        /// Bluetooth Core Specification Vol 4, Part E, §7.8.47
        LeReadPhyReturn {
                handle: ConnHandle,
                tx_phy: PhyKind,
                rx_phy: PhyKind,
        }
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.48
    LeSetDefaultPhy(LE, 0x0031) {
        Params {
            all_phys: AllPhys,
            tx_phys: PhyMask,
            rx_phys: PhyMask,
        }
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.49
    LeSetPhy(LE, 0x0032) {
        Params {
            handle: ConnHandle,
            all_phys: AllPhys,
            tx_phys: PhyMask,
            rx_phys: PhyMask,
            phy_options: PhyOptions,
        }
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.52
    LeSetAdvSetRandomAddr(LE, 0x0035) {
        Params {
            adv_handle: AdvHandle,
            random_addr: BdAddr,
        }
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.53
    LeSetExtAdvParams(LE, 0x0036) {
        Params {
                adv_handle: AdvHandle,
                adv_event_props: AdvEventProps,
                primary_adv_interval_min: Duration<625>,
                primary_adv_interval_max: Duration<625>,
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

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.54
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
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.55
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

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.56
    LeSetExtAdvEnable(LE, 0x0039) {
        Params<'a> {
            enable: bool,
            sets: &'a [AdvSet],
        }
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.57
    LeReadMaxAdvDataLength(LE, 0x003a) {
        Params {}
        Return = u16;
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.58
    LeReadNumberOfSupportedAdvSets(LE, 0x003b) {
        Params {}
        Return = u8;
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.59
    LeRemoveAdvSet(LE, 0x003c) {
        Params {
            adv_handle: AdvHandle,
        }
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.60
    LeClearAdvSets(LE, 0x003d) {
        Params {}
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.61
    LeSetPeriodicAdvParams(LE, 0x003e) {
        Params {
            adv_handle: AdvHandle,
            periodic_adv_interval_min: Duration<1_250>,
            periodic_adv_interval_max: Duration<1_250>,
            periodic_adv_props: PeriodicAdvProps,
        }
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.62
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
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.63
    LeSetPeriodicAdvEnable(LE, 0x0040) {
        Params {
            enable: bool,
            adv_handle: AdvHandle,
        }
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.64
    LeSetExtScanParams(LE, 0x0041) {
        Params {
            own_addr_kind: AddrKind,
            scanning_filter_policy: ScanningFilterPolicy,
            scanning_phys: PhyParams<ScanningPhy>,
        }
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.65
    LeSetExtScanEnable(LE, 0x0042) {
        Params {
            enable: bool,
            filter_duplicates: FilterDuplicates,
            duration: Duration<10_000>,
            period: Duration<1_280_000>,
        }
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.66
    LeExtCreateConn(LE, 0x0043) {
        Params {
            initiator_filter_policy: bool,
            own_addr_kind: AddrKind,
            peer_addr_kind: AddrKind,
            peer_addr: BdAddr,
            initiating_phys: PhyParams<InitiatingPhy>,
        }
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.67
    LePeriodicAdvCreateSync(LE, 0x0044) {
        Params {
            options: LePeriodicAdvCreateSyncOptions,
            adv_sid: u8,
            adv_addr_kind: AddrKind,
            adv_addr: BdAddr,
            skip: u16,
            sync_timeout: Duration<10_000>,
            sync_cte_kind: CteMask,
        }
    }
}
cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.68
    LePeriodicAdvCreateSyncCancel(LE, 0x0045) {
        Params {}
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.69
    LePeriodicAdvTerminateSync(LE, 0x0046) {
        Params {
            sync_handle: SyncHandle,
        }
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.70
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
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.71
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
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.72
    LeClearPeriodicAdvList(LE, 0x0049) {
        Params {}
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.73
    LeReadPeriodicAdvListSize(LE, 0x004a) {
        Params {}
        Return = u8;
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.74
    LeReadTransmitPower(LE, 0x004b) {
        Params {}
        Return = (i8, i8);
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.75
    LeReadRfPathCompensation(LE, 0x004c) {
        Params {}
        Return = (i16, i16);
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.76
    LeWriteRfPathCompensation(LE, 0x004d) {
        Params {
            rf_tx_path_compensation_value: i16,
            rf_rx_path_compensation_value: i16,
        }
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.77
    LeSetPrivacyMode(LE, 0x004e) {
        Params {
            peer_id_addr_kind: AddrKind,
            peer_id_addr: BdAddr,
            privacy_mode: PrivacyMode,
        }
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.80
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
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.81
    LeSetConnectionlessCteTransmitEnable(LE, 0x0052) {
        Params {
            adv_handle: AdvHandle,
            cte_enable: bool,
        }
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.84
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
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.86
    LeConnCteResponseEnable(LE, 0x0057) {
        Params {
            handle: ConnHandle,
            enable: bool,
        }
        Return = ConnHandle;
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.87
    LeReadAntennaInformation(LE, 0x0058) {
        Params {}
        /// Bluetooth Core Specification Vol 4, Part E, §7.8.87
        LeReadAntennaInformationReturn {
            supported_switching_sampling_rates: SwitchingSamplingRates,
            num_antennae: u8,
            max_switching_pattern_len: u8,
            max_cte_len: u8,
        }
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.88
    LeSetPeriodicAdvReceiveEnable(LE, 0x0059) {
        Params {
            sync_handle: SyncHandle,
            enable: LePeriodicAdvReceiveEnable,
        }
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.89
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
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.90
    LePeriodicAdvSetInfoTransfer(LE, 0x005b) {
        Params {
            handle: ConnHandle,
            service_data: u16,
            adv_handle: AdvHandle,
        }
        Return = ConnHandle;
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.91
    LeSetPeriodicAdvSyncTransferParams(LE, 0x005c) {
        Params {
            handle: ConnHandle,
            mode: LePeriodicAdvSyncTransferMode,
            skip: u16,
            sync_timeout: Duration<10_000>,
            cte_kind: CteMask,
        }
        Return = ConnHandle;
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.92
    LeSetDefaultPeriodicAdvSyncTransferParams(LE, 0x005d) {
        Params {
            mode: LePeriodicAdvSyncTransferMode,
            skip: u16,
            sync_timeout: Duration<10_000>,
            cte_kind: CteMask,
        }
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.108
    LeRequestPeerSca(LE, 0x006d) {
        Params {
            handle: ConnHandle,
        }
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.117
    LeEnhancedReadTransmitPowerLevel(LE, 0x0076) {
        Params {
            handle: ConnHandle,
            phy: PhyKind,
        }
        /// Bluetooth Core Specification Vol 4, Part E, §7.8.117
        LeEnhancedReadTransmitPowerLevelReturn {
            handle: ConnHandle,
            phy: PhyKind,
            current_tx_power_level: i8,
            max_tx_power_level: i8,
        }
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.118
    LeReadRemoteTransmitPowerLevel(LE, 0x0077) {
        Params {
            handle: ConnHandle,
            phy: PhyKind,
        }
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.119
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
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.120
    LeSetPathLossReportingEnable(LE, 0x0079) {
        Params {
            handle: ConnHandle,
            enable: bool,
        }
        Return = ConnHandle;
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.121
    LeSetTransmitPowerReportingEnable(LE, 0x007a) {
        Params {
            handle: ConnHandle,
            local_enable: bool,
            remote_enable: bool,
        }
        Return = ConnHandle;
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.122
    LeSetDataRelatedAddrChanges(LE, 0x007c) {
        Params {
            adv_handle: AdvHandle,
            change_reasons: LeDataRelatedAddrChangeReasons,
        }
        Return = ();
    }
}
