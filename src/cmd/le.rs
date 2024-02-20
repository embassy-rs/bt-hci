//! Bluetooth Core Specification Vol 4, Part E, §7.8

use crate::param::{
    AddrKind, AdvChannelMap, AdvEventProps, AdvFilterPolicy, AdvHandle, AdvKind, AdvSet, AllPhys, BdAddr, ChannelMap,
    ConnHandle, CteKind, CteMask, Duration, FilterDuplicates, InitiatingPhy, LeDataRelatedAddrChangeReasons,
    LeEventMask, LeFeatureMask, LePeriodicAdvCreateSyncOptions, LePeriodicAdvReceiveEnable, LePeriodicAdvSubeventData,
    LePeriodicAdvSyncTransferMode, LeScanKind, Operation, PeriodicAdvProps, PhyKind, PhyMask, PhyOptions, PhyParams,
    PrivacyMode, ScanningFilterPolicy, ScanningPhy, SwitchingSamplingRates, SyncHandle,
};
use crate::{cmd, param, WriteHci};

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.1
    LeSetEventMask(LE, 0x0001) {
        Params = LeEventMask;
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.2
    LeReadBufferSize(LE, 0x0002) {
        Params = ();
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
        Params = ();
        Return = LeFeatureMask;
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.4
    LeSetRandomAddr(LE, 0x0005) {
        Params = BdAddr;
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.5
    LeSetAdvParams(LE, 0x0006) {
        Params = LeSetAdvParamsParams;
        Return = ();
    }
}

param! {
    struct LeSetAdvParamsParams {
        adv_interval_min: Duration<625>,
        adv_interval_max: Duration<625>,
        adv_kind: AdvKind,
        own_addr_kind: AddrKind,
        peer_addr_kind: AddrKind,
        peer_addr: BdAddr,
        adv_channel_map: AdvChannelMap,
        adv_filter_policy: AdvFilterPolicy,
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.6
    LeReadAdvPhysicalChannelTxPower(LE, 0x0007) {
        Params = ();
        Return = i8;
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.7
    LeSetAdvData(LE, 0x0008) {
        Params = LeSetAdvDataParams;
        Return = ();
    }
}

param! {
    struct LeSetAdvDataParams {
        data_len: u8,
        data: [u8; 31],
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.8
    LeSetScanResponseData(LE, 0x0009) {
        Params = LeSetScanResponseDataParams;
        Return = ();
    }
}

param! {
    struct LeSetScanResponseDataParams {
        data_len: u8,
        data: [u8; 31],
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.9
    LeSetAdvEnable(LE, 0x000a) {
        Params = bool;
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.10
    LeSetScanParams(LE, 0x000b) {
        Params = LeSetScanParamsParams;
        Return = ();
    }
}

param! {
    struct LeSetScanParamsParams {
        le_scan_kind: LeScanKind,
        le_scan_interval: Duration<10_000>,
        le_scan_window: Duration<10_000>,
        own_addr_kind: AddrKind,
        scanning_filter_policy: ScanningFilterPolicy,
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.11
    LeSetScanEnable(LE, 0x000c) {
        Params = LeSetScanEnableParams;
        Return = ();
    }
}

param! {
    struct LeSetScanEnableParams {
        enable: bool,
        filter_duplicates: bool,
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.12
    LeCreateConn(LE, 0x000d) {
        Params = LeCreateConnParams;
    }
}

param! {
    struct LeCreateConnParams {
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

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.13
    LeCreateConnCancel(LE, 0x000e) {
        Params = ();
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.14
    LeReadFilterAcceptListSize(LE, 0x000f) {
        Params = ();
        Return = u8;
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.15
    LeClearFilterAcceptList(LE, 0x0010) {
        Params = ();
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.16
    LeAddDeviceToFilterAcceptList(LE, 0x0011) {
        Params = LeAddDeviceToFilterAcceptListParams;
        Return = ();
    }
}

param! {
    struct LeAddDeviceToFilterAcceptListParams {
        addr_kind: AddrKind,
        addr: BdAddr,
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.17
    LeRemoveDeviceFromFilterAcceptList(LE, 0x0012) {
        Params = LeRemoveDeviceFromFilterAcceptListParams;
        Return = ();
    }
}

param! {
    struct LeRemoveDeviceFromFilterAcceptListParams {
        addr_kind: AddrKind,
        addr: BdAddr,
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.18
    LeConnUpdate(LE, 0x0013) {
        Params = LeConnUpdateParams;
    }
}

param! {
    struct LeConnUpdateParams {
        handle: ConnHandle,
        conn_interval_min: Duration<1_250>,
        conn_interval_max: Duration<1_250>,
        max_latency: u16,
        supervision_timeout: Duration<10_000>,
        min_ce_length: Duration<625>,
        max_ce_length: Duration<625>,
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.19
    LeSetHostChannelClassification(LE, 0x0014) {
        Params = ChannelMap;
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.20
    LeReadChannelMap(LE, 0x0015) {
        Params = ConnHandle;
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
        Params = ConnHandle;
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.22
    LeEncrypt(LE, 0x0017) {
        Params = LeEncryptParams;
        Return = [u8; 16];
    }
}

param! {
    struct LeEncryptParams {
        key: [u8; 16],
        plaintext: [u8; 16],
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.23
    LeRand(LE, 0x0018) {
        Params = ();
        Return = [u8; 8];
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.24
    LeEnableEncryption(LE, 0x0019) {
        Params = LeEnableEncryptionParams;
    }
}

param! {
    struct LeEnableEncryptionParams {
        handle: ConnHandle,
        random: [u8; 8],
        encrypted_diversifier: u16,
        long_term_key: [u8; 16],
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.25
    LeLongTermKeyRequestReply(LE, 0x001a) {
        Params = LeLongTermKeyRequestReplyParams;
        Return = ConnHandle;
    }
}

param! {
    struct LeLongTermKeyRequestReplyParams {
        handle: ConnHandle,
        long_term_key: [u8; 16],
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.26
    LeLongTermKeyRequestNegativeReply(LE, 0x001b) {
        Params = ConnHandle;
        Return = ConnHandle;
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.27
    LeReadSupportedStates(LE, 0x001c) {
        Params = ();
        Return = [u8; 8];
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.30
    LeTestEnd(LE, 0x001f) {
        Params = ();
        Return = u16;
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.33
    LeSetDataLength(LE, 0x0022) {
        Params = LeSetDataLengthParams;
        Return = ConnHandle;
    }
}

param! {
    struct LeSetDataLengthParams {
        handle: ConnHandle,
        tx_octets: u16,
        tx_time: u16,
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.34
    LeReadSuggestedDefaultDataLength(LE, 0x0023) {
        Params = ();
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
        Params = LeWriteSuggestedDefaultDataLengthParams;
        Return = ();
    }
}

param! {
    struct LeWriteSuggestedDefaultDataLengthParams {
        suggested_max_tx_octets: u16,
        suggested_max_tx_time: u16,
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.38
    LeAddDeviceToResolvingList(LE, 0x0027) {
        Params = LeAddDeviceToResolvingListParams;
        Return = ();
    }
}

param! {
    struct LeAddDeviceToResolvingListParams {
        peer_id_addr_kind: AddrKind,
        peer_id_addr: BdAddr,
        peer_irk: [u8; 16],
        local_irk: [u8; 16],
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.39
    LeRemoveDeviceFromResolvingList(LE, 0x0028) {
        Params = LeRemoveDeviceFromResolvingListParams;
        Return = ();
    }
}

param! {
    struct LeRemoveDeviceFromResolvingListParams {
        peer_id_addr_kind: AddrKind,
        peer_id_addr: BdAddr,
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.40
    LeClearResolvingList(LE, 0x0029) {
        Params = ();
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.41
    LeReadResolvingListSize(LE, 0x002a) {
        Params = ();
        Return = u8;
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.44
    LeSetAddrResolutionEnable(LE, 0x002d) {
        Params = bool;
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.45
    LeSetResolvablePrivateAddrTimeout(LE, 0x002e) {
        Params = Duration<1_000_000>;
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.46
    LeReadMaxDataLength(LE, 0x002f) {
        Params = ();
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
        Params = ConnHandle;
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
        Params = LeSetDefaultPhyParams;
        Return = ();
    }
}

param! {
    struct LeSetDefaultPhyParams {
        all_phys: AllPhys,
        tx_phys: PhyMask,
        rx_phys: PhyMask,
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.49
    LeSetPhy(LE, 0x0032) {
        Params = LeSetPhyParams;
    }
}

param! {
    struct LeSetPhyParams {
        handle: ConnHandle,
        all_phys: AllPhys,
        tx_phys: PhyMask,
        rx_phys: PhyMask,
        phy_options: PhyOptions,
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.52
    LeSetAdvSetRandomAddr(LE, 0x0035) {
        Params = LeSetAdvSetRandomAddrParams;
        Return = ();
    }
}

param! {
    struct LeSetAdvSetRandomAddrParams {
        adv_handle: AdvHandle,
        random_addr: BdAddr,
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.53
    LeSetExtAdvParams(LE, 0x0036) {
        Params = LeSetExtAdvParamsParams;
        Return = i8;
    }
}

param! {
    struct LeSetExtAdvParamsParams {
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
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.54
    LeSetExtAdvData(LE, 0x0037) {
        Params<'d> = LeSetExtAdvDataParams<'d>;
        Return = ();
    }
}

param! {
    struct LeSetExtAdvDataParams<'d> {
        adv_handle: AdvHandle,
        operation: Operation,
        fragment_preference: bool,
        adv_data: &'d [u8],
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.55
    LeSetExtScanResponseData(LE, 0x0038) {
        Params<'d> = LeSetExtScanResponseDataParams<'d>;
        Return = ();
    }
}

param! {
    struct LeSetExtScanResponseDataParams<'d> {
        adv_handle: AdvHandle,
        operation: Operation,
        fragment_preference: bool,
        scan_response_data: &'d [u8],
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.56
    LeSetExtAdvEnable(LE, 0x0039) {
        Params<'a> = LeSetExtAdvEnableParams<'a>;
        Return = ();
    }
}

param! {
    struct LeSetExtAdvEnableParams<'a> {
        enable: bool,
        sets: &'a [AdvSet],
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.57
    LeReadMaxAdvDataLength(LE, 0x003a) {
        Params = ();
        Return = u16;
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.58
    LeReadNumberOfSupportedAdvSets(LE, 0x003b) {
        Params = ();
        Return = u8;
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.59
    LeRemoveAdvSet(LE, 0x003c) {
        Params = AdvHandle;
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.60
    LeClearAdvSets(LE, 0x003d) {
        Params = ();
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.61
    LeSetPeriodicAdvParams(LE, 0x003e) {
        Params = LeSetPeriodicAdvParamsParams;
        Return = ();
    }
}

param! {
    struct LeSetPeriodicAdvParamsParams {
        adv_handle: AdvHandle,
        periodic_adv_interval_min: Duration<1_250>,
        periodic_adv_interval_max: Duration<1_250>,
        periodic_adv_props: PeriodicAdvProps,
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.61
    LeSetPeriodicAdvParamsV2(LE, 0x003e) {
        Params = LeSetPeriodicAdvParamsV2Params;
        Return = AdvHandle;
    }
}

param! {
    struct LeSetPeriodicAdvParamsV2Params {
        adv_handle: AdvHandle,
        periodic_adv_interval_min: Duration<1_250>,
        periodic_adv_interval_max: Duration<1_250>,
        periodic_adv_props: PeriodicAdvProps,
        num_subevents: u8,
        subevent_interval: u8, // * 1.25ms
        response_slot_delay: u8, // * 1.25ms
        response_slot_spacing: u8, // * 0.125ms
        num_response_slots: u8,
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.62
    LeSetPeriodicAdvData(LE, 0x003f) {
        Params<'a> = LeSetPeriodicAdvDataParams<'a>;
        Return = ();
    }
}

param! {
    struct LeSetPeriodicAdvDataParams<'a> {
        adv_handle: AdvHandle,
        operation: Operation,
        adv_data: &'a [u8],
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.63
    LeSetPeriodicAdvEnable(LE, 0x0040) {
        Params = LeSetPeriodicAdvEnableParams;
        Return = ();
    }
}

param! {
    struct LeSetPeriodicAdvEnableParams {
        enable: bool,
        adv_handle: AdvHandle,
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.64
    LeSetExtScanParams(LE, 0x0041) {
        Params = LeSetExtScanParamsParams;
        Return = ();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct LeSetExtScanParamsParams {
    pub own_addr_kind: AddrKind,
    pub scanning_filter_policy: ScanningFilterPolicy,
    pub scanning_phys: PhyParams<ScanningPhy>,
}

impl WriteHci for LeSetExtScanParamsParams {
    #[inline(always)]
    fn size(&self) -> usize {
        self.own_addr_kind.size() + self.scanning_filter_policy.size() + self.scanning_phys.size()
    }

    fn write_hci<W: embedded_io::Write>(&self, mut writer: W) -> Result<(), W::Error> {
        self.own_addr_kind.write_hci(&mut writer)?;
        self.scanning_filter_policy.write_hci(&mut writer)?;
        self.scanning_phys.write_hci(writer)
    }

    async fn write_hci_async<W: embedded_io_async::Write>(&self, mut writer: W) -> Result<(), W::Error> {
        self.own_addr_kind.write_hci_async(&mut writer).await?;
        self.scanning_filter_policy.write_hci_async(&mut writer).await?;
        self.scanning_phys.write_hci_async(writer).await
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.65
    LeSetExtScanEnable(LE, 0x0042) {
        Params = LeSetExtScanEnableParams;
        Return = ();
    }
}

param! {
    struct LeSetExtScanEnableParams {
        enable: bool,
        filter_duplicates: FilterDuplicates,
        duration: Duration<10_000>,
        period: Duration<1_280_000>,
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.66
    LeExtCreateConn(LE, 0x0043) {
        Params = LeExtCreateConnParams;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct LeExtCreateConnParams {
    pub initiator_filter_policy: bool,
    pub own_addr_kind: AddrKind,
    pub peer_addr_kind: AddrKind,
    pub peer_addr: BdAddr,
    pub initiating_phys: PhyParams<InitiatingPhy>,
}

impl WriteHci for LeExtCreateConnParams {
    #[inline(always)]
    fn size(&self) -> usize {
        self.initiator_filter_policy.size()
            + self.own_addr_kind.size()
            + self.peer_addr_kind.size()
            + self.peer_addr.size()
            + self.initiating_phys.size()
    }

    fn write_hci<W: embedded_io::Write>(&self, mut writer: W) -> Result<(), W::Error> {
        self.initiator_filter_policy.write_hci(&mut writer)?;
        self.own_addr_kind.write_hci(&mut writer)?;
        self.peer_addr_kind.write_hci(&mut writer)?;
        self.peer_addr.write_hci(&mut writer)?;
        self.initiating_phys.write_hci(writer)
    }

    async fn write_hci_async<W: embedded_io_async::Write>(&self, mut writer: W) -> Result<(), W::Error> {
        self.initiator_filter_policy.write_hci_async(&mut writer).await?;
        self.own_addr_kind.write_hci_async(&mut writer).await?;
        self.peer_addr_kind.write_hci_async(&mut writer).await?;
        self.peer_addr.write_hci_async(&mut writer).await?;
        self.initiating_phys.write_hci_async(writer).await
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.66
    LeExtCreateConnV2(LE, 0x0085) {
        Params = LeExtCreateConnV2Params;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct LeExtCreateConnV2Params {
    pub adv_handle: AdvHandle,
    pub subevent: u8,
    pub initiator_filter_policy: bool,
    pub own_addr_kind: AddrKind,
    pub peer_addr_kind: AddrKind,
    pub peer_addr: BdAddr,
    pub initiating_phys: PhyParams<InitiatingPhy>,
}

impl WriteHci for LeExtCreateConnV2Params {
    #[inline(always)]
    fn size(&self) -> usize {
        self.adv_handle.size()
            + self.subevent.size()
            + self.initiator_filter_policy.size()
            + self.own_addr_kind.size()
            + self.peer_addr_kind.size()
            + self.peer_addr.size()
            + self.initiating_phys.size()
    }

    fn write_hci<W: embedded_io::Write>(&self, mut writer: W) -> Result<(), W::Error> {
        self.adv_handle.write_hci(&mut writer)?;
        self.subevent.write_hci(&mut writer)?;
        self.initiator_filter_policy.write_hci(&mut writer)?;
        self.own_addr_kind.write_hci(&mut writer)?;
        self.peer_addr_kind.write_hci(&mut writer)?;
        self.peer_addr.write_hci(&mut writer)?;
        self.initiating_phys.write_hci(writer)
    }

    async fn write_hci_async<W: embedded_io_async::Write>(&self, mut writer: W) -> Result<(), W::Error> {
        self.adv_handle.write_hci_async(&mut writer).await?;
        self.subevent.write_hci_async(&mut writer).await?;
        self.initiator_filter_policy.write_hci_async(&mut writer).await?;
        self.own_addr_kind.write_hci_async(&mut writer).await?;
        self.peer_addr_kind.write_hci_async(&mut writer).await?;
        self.peer_addr.write_hci_async(&mut writer).await?;
        self.initiating_phys.write_hci_async(writer).await
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.67
    LePeriodicAdvCreateSync(LE, 0x0044) {
        Params = LePeriodicAdvCreateSyncParams;
    }
}

param! {
    struct LePeriodicAdvCreateSyncParams {
        options: LePeriodicAdvCreateSyncOptions,
        adv_sid: u8,
        adv_addr_kind: AddrKind,
        adv_addr: BdAddr,
        skip: u16,
        sync_timeout: Duration<10_000>,
        sync_cte_kind: CteMask,
    }
}
cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.68
    LePeriodicAdvCreateSyncCancel(LE, 0x0045) {
        Params = ();
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.69
    LePeriodicAdvTerminateSync(LE, 0x0046) {
        Params = SyncHandle;
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.70
    LeAddDeviceToPeriodicAdvList(LE, 0x0047) {
        Params = LeAddDeviceToPeriodicAdvListParams;
        Return = ();
    }
}

param! {
    struct LeAddDeviceToPeriodicAdvListParams {
        adv_addr_kind: AddrKind,
        adv_addr: BdAddr,
        adv_sid: u8,
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.71
    LeRemoveDeviceFromPeriodicAdvList(LE, 0x0048) {
        Params = LeRemoveDeviceFromPeriodicAdvListParams;
        Return = ();
    }
}

param! {
    struct LeRemoveDeviceFromPeriodicAdvListParams {
        adv_addr_kind: AddrKind,
        adv_addr: BdAddr,
        adv_sid: u8,
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.72
    LeClearPeriodicAdvList(LE, 0x0049) {
        Params = ();
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.73
    LeReadPeriodicAdvListSize(LE, 0x004a) {
        Params = ();
        Return = u8;
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.74
    LeReadTransmitPower(LE, 0x004b) {
        Params = ();
        LeReadTransmitPowerReturn {
            min_tx_power: i8,
            max_tx_power: i8,
        }
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.75
    LeReadRfPathCompensation(LE, 0x004c) {
        Params = ();
        LeReadRfPathCompensationReturn {
            rf_tx_path_compensation_value: i16,
            rf_rx_path_compenstaion_value: i16,
        }
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.76
    LeWriteRfPathCompensation(LE, 0x004d) {
        Params = LeWriteRfPathCompensationParams;
        Return = ();
    }
}

param! {
    struct LeWriteRfPathCompensationParams {
        rf_tx_path_compensation_value: i16,
        rf_rx_path_compensation_value: i16,
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.77
    LeSetPrivacyMode(LE, 0x004e) {
        Params = LeSetPrivacyModeParams;
        Return = ();
    }
}

param! {
    struct LeSetPrivacyModeParams {
        peer_id_addr_kind: AddrKind,
        peer_id_addr: BdAddr,
        privacy_mode: PrivacyMode,
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.80
    LeSetConnectionlessCteTransmitParams(LE, 0x0051) {
        Params<'a> = LeSetConnectionlessCteTransmitParamsParams<'a>;
        Return = ();
    }
}

param! {
    struct LeSetConnectionlessCteTransmitParamsParams<'a> {
        adv_handle: AdvHandle,
        cte_length: u8,
        cte_kind: CteKind,
        cte_count: u8,
        switching_pattern: &'a [u8],
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.81
    LeSetConnectionlessCteTransmitEnable(LE, 0x0052) {
        Params = LeSetConnectionlessCteTransmitEnableParams;
        Return = ();
    }
}

param! {
    struct LeSetConnectionlessCteTransmitEnableParams {
        adv_handle: AdvHandle,
        cte_enable: bool,
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.84
    LeSetConnCteTransmitParams(LE, 0x0055) {
        Params<'a> = LeSetConnCteTransmitParamsParams<'a>;
        Return = ConnHandle;
    }
}

param! {
    struct LeSetConnCteTransmitParamsParams<'a> {
        handle: ConnHandle,
        cte_kinds: CteMask,
        switching_pattern: &'a [u8],
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.86
    LeConnCteResponseEnable(LE, 0x0057) {
        Params = LeConnCteResponseEnableParams;
        Return = ConnHandle;
    }
}

param! {
    struct LeConnCteResponseEnableParams {
        handle: ConnHandle,
        enable: bool,
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.87
    LeReadAntennaInformation(LE, 0x0058) {
        Params = ();
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
        Params = LeSetPeriodicAdvReceiveEnableParams;
        Return = ();
    }
}

param! {
    struct LeSetPeriodicAdvReceiveEnableParams {
        sync_handle: SyncHandle,
        enable: LePeriodicAdvReceiveEnable,
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.89
    LePeriodicAdvSyncTransfer(LE, 0x005a) {
        Params = LePeriodicAdvSyncTransferParams;
        Return = ConnHandle;
    }
}

param! {
    struct LePeriodicAdvSyncTransferParams {
        handle: ConnHandle,
        service_data: u16,
        sync_handle: SyncHandle,
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.90
    LePeriodicAdvSetInfoTransfer(LE, 0x005b) {
        Params = LePeriodicAdvSetInfoTransferParams;
        Return = ConnHandle;
    }
}

param! {
    struct LePeriodicAdvSetInfoTransferParams {
        handle: ConnHandle,
        service_data: u16,
        adv_handle: AdvHandle,
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.91
    LeSetPeriodicAdvSyncTransferParams(LE, 0x005c) {
        Params = LeSetPeriodicAdvSyncTransferParamsParams;
        Return = ConnHandle;
    }
}

param! {
    struct LeSetPeriodicAdvSyncTransferParamsParams {
        handle: ConnHandle,
        mode: LePeriodicAdvSyncTransferMode,
        skip: u16,
        sync_timeout: Duration<10_000>,
        cte_kind: CteMask,
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.92
    LeSetDefaultPeriodicAdvSyncTransferParams(LE, 0x005d) {
        Params = LeSetDefaultPeriodicAdvSyncTransferParamsParams;
        Return = ();
    }
}

param! {
    struct LeSetDefaultPeriodicAdvSyncTransferParamsParams {
        mode: LePeriodicAdvSyncTransferMode,
        skip: u16,
        sync_timeout: Duration<10_000>,
        cte_kind: CteMask,
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.108
    LeRequestPeerSca(LE, 0x006d) {
        Params = ConnHandle;
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.117
    LeEnhancedReadTransmitPowerLevel(LE, 0x0076) {
        Params = LeEnhancedReadTransmitPowerLevelParams;
        /// Bluetooth Core Specification Vol 4, Part E, §7.8.117
        LeEnhancedReadTransmitPowerLevelReturn {
            handle: ConnHandle,
            phy: PhyKind,
            current_tx_power_level: i8,
            max_tx_power_level: i8,
        }
    }
}

param! {
    struct LeEnhancedReadTransmitPowerLevelParams {
        handle: ConnHandle,
        phy: PhyKind,
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.118
    LeReadRemoteTransmitPowerLevel(LE, 0x0077) {
        Params = LeReadRemoteTransmitPowerLevelParams;
    }
}

param! {
    struct LeReadRemoteTransmitPowerLevelParams {
        handle: ConnHandle,
        phy: PhyKind,
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.119
    LeSetPathLossReportingParams(LE, 0x0078) {
        Params = LeSetPathLossReportingParamsParams;
        Return = ConnHandle;
    }
}

param! {
    struct LeSetPathLossReportingParamsParams {
        handle: ConnHandle,
        high_threshold: i8,
        high_hysteresis: i8,
        low_threshold: i8,
        low_hysteresis: i8,
        min_time_spent: u16,
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.120
    LeSetPathLossReportingEnable(LE, 0x0079) {
        Params = LeSetPathLossReportingEnableParams;
        Return = ConnHandle;
    }
}

param! {
    struct LeSetPathLossReportingEnableParams {
        handle: ConnHandle,
        enable: bool,
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.121
    LeSetTransmitPowerReportingEnable(LE, 0x007a) {
        Params = LeSetTransmitPowerReportingEnableParams;
        Return = ConnHandle;
    }
}

param! {
    struct LeSetTransmitPowerReportingEnableParams {
        handle: ConnHandle,
        local_enable: bool,
        remote_enable: bool,
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.122
    LeSetDataRelatedAddrChanges(LE, 0x007c) {
        Params = LeSetDataRelatedAddrChangesParams;
        Return = ();
    }
}

param! {
    struct LeSetDataRelatedAddrChangesParams {
        adv_handle: AdvHandle,
        change_reasons: LeDataRelatedAddrChangeReasons,
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.125
    LeSetPeriodicAdvSubeventData(LE, 0x0082) {
        Params<'a> = LeSetPeriodicAdvSubeventDataParams<'a>;
        Return = AdvHandle;
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct LeSetPeriodicAdvSubeventDataParams<'a> {
    pub adv_handle: AdvHandle,
    pub subevent: &'a [LePeriodicAdvSubeventData<'a>],
}

impl<'a> WriteHci for LeSetPeriodicAdvSubeventDataParams<'a> {
    #[inline(always)]
    fn size(&self) -> usize {
        self.adv_handle.size() + self.subevent.size()
    }

    #[inline(always)]
    fn write_hci<W: ::embedded_io::Write>(&self, mut writer: W) -> Result<(), W::Error> {
        self.adv_handle.write_hci(&mut writer)?;
        self.subevent.write_hci(&mut writer)?;
        Ok(())
    }

    #[inline(always)]
    async fn write_hci_async<W: ::embedded_io_async::Write>(&self, mut writer: W) -> Result<(), W::Error> {
        self.adv_handle.write_hci_async(&mut writer).await?;
        self.subevent.write_hci_async(&mut writer).await?;
        Ok(())
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.126
    LeSetPeriodicAdvResponseData(LE, 0x007c) {
        Params<'a> = LeSetPeriodicAdvResponseDataParams<'a>;
        Return = SyncHandle;
    }
}

param! {
    struct LeSetPeriodicAdvResponseDataParams<'a> {
        adv_handle: SyncHandle,
        request_event: u16,
        request_subevent: u8,
        response_subevent: u8,
        response_slot: u8,
        response_data: &'a [u8],
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.127
    LeSetPeriodicSyncSubevent(LE, 0x007c) {
        Params<'a> = LeSetPeriodicSyncSubeventParams<'a>;
        Return = SyncHandle;
    }
}

param! {
    struct LeSetPeriodicSyncSubeventParams<'a> {
        adv_handle: SyncHandle,
        periodic_adv_props: PeriodicAdvProps,
        subevents: &'a [u8],
    }
}
