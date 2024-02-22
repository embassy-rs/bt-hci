//! Bluetooth Core Specification Vol 4, Part E, §7.8

use crate::param::{
    AddrKind, AdvChannelMap, AdvEventProps, AdvFilterPolicy, AdvHandle, AdvKind, AdvSet, AllPhys, BdAddr, ChannelMap,
    ConnHandle, CteKind, CteMask, Duration, FilterDuplicates, InitiatingPhy, LeDataRelatedAddrChangeReasons,
    LeEventMask, LeFeatureMask, LePeriodicAdvCreateSyncOptions, LePeriodicAdvReceiveEnable, LePeriodicAdvSubeventData,
    LePeriodicAdvSyncTransferMode, LeScanKind, Operation, PeriodicAdvProps, PhyKind, PhyMask, PhyOptions, PhyParams,
    PrivacyMode, ScanningFilterPolicy, ScanningPhy, SwitchingSamplingRates, SyncHandle,
};
use crate::{cmd, WriteHci};

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
        LeSetAdvParamsParams {
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
        Params = ();
        Return = i8;
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.7
    LeSetAdvData(LE, 0x0008) {
        LeSetAdvDataParams {
            data_len: u8,
            data: [u8; 31],
        }
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.8
    LeSetScanResponseData(LE, 0x0009) {
        LeSetScanResponseDataParams {
            data_len: u8,
            data: [u8; 31],
        }
        Return = ();
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
        LeSetScanParamsParams {
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
        LeSetScanEnableParams {
            enable: bool,
            filter_duplicates: bool,
        }
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.12
    LeCreateConn(LE, 0x000d) {
        LeCreateConnParams {
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
        LeAddDeviceToFilterAcceptListParams {
            addr_kind: AddrKind,
            addr: BdAddr,
        }
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.17
    LeRemoveDeviceFromFilterAcceptList(LE, 0x0012) {
        LeRemoveDeviceFromFilterAcceptListParams {
            addr_kind: AddrKind,
            addr: BdAddr,
        }
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.18
    LeConnUpdate(LE, 0x0013) {
        LeConnUpdateParams {
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
            channel_map: ChannelMap,
        }
        Handle = handle: ConnHandle;
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
        LeEncryptParams {
            key: [u8; 16],
            plaintext: [u8; 16],
        }
        Return = [u8; 16];
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
        LeEnableEncryptionParams {
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
        LeLongTermKeyRequestReplyParams {
            long_term_key: [u8; 16],
        }
        Return = ConnHandle;
        Handle = handle: ConnHandle;
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.26
    LeLongTermKeyRequestNegativeReply(LE, 0x001b) {
        Params = ConnHandle;
        Return = ConnHandle;
        Handle = ConnHandle;
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
        LeSetDataLengthParams {
            tx_octets: u16,
            tx_time: u16,
        }
        Return = ConnHandle;
        Handle = handle: ConnHandle;
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
        LeWriteSuggestedDefaultDataLengthParams {
            suggested_max_tx_octets: u16,
            suggested_max_tx_time: u16,
        }
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.38
    LeAddDeviceToResolvingList(LE, 0x0027) {
        LeAddDeviceToResolvingListParams {
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
        LeRemoveDeviceFromResolvingListParams {
            peer_id_addr_kind: AddrKind,
            peer_id_addr: BdAddr,
        }
        Return = ();
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
            tx_phy: PhyKind,
            rx_phy: PhyKind,
        }
        Handle = handle: ConnHandle;
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.48
    LeSetDefaultPhy(LE, 0x0031) {
        LeSetDefaultPhyParams {
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
        LeSetPhyParams {
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
        LeSetAdvSetRandomAddrParams {
            adv_handle: AdvHandle,
            random_addr: BdAddr,
        }
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.53
    LeSetExtAdvParams(LE, 0x0036) {
        LeSetExtAdvParamsParams {
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
        LeSetExtAdvDataParams<'d> {
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
        LeSetExtScanResponseDataParams<'d> {
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
        LeSetExtAdvEnableParams<'a> {
            enable: bool,
            sets: &'a [AdvSet],
        }
        Return = ();
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
        LeSetPeriodicAdvParamsParams {
            adv_handle: AdvHandle,
            periodic_adv_interval_min: Duration<1_250>,
            periodic_adv_interval_max: Duration<1_250>,
            periodic_adv_props: PeriodicAdvProps,
        }
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.61
    LeSetPeriodicAdvParamsV2(LE, 0x003e) {
        LeSetPeriodicAdvParamsV2Params {
            periodic_adv_interval_min: Duration<1_250>,
            periodic_adv_interval_max: Duration<1_250>,
            periodic_adv_props: PeriodicAdvProps,
            num_subevents: u8,
            subevent_interval: u8, // * 1.25ms
            response_slot_delay: u8, // * 1.25ms
            response_slot_spacing: u8, // * 0.125ms
            num_response_slots: u8,
        }
        Return = AdvHandle;
        Handle = adv_handle: AdvHandle;
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.62
    LeSetPeriodicAdvData(LE, 0x003f) {
        LeSetPeriodicAdvDataParams<'a> {
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
        LeSetPeriodicAdvEnableParams {
            enable: bool,
            adv_handle: AdvHandle,
        }
        Return = ();
    }
}

crate::cmd! {
    BASE
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.64
    LeSetExtScanParams(LE, 0x0041) {
        Params = LeSetExtScanParamsParams;
        Return = ();
      }
}

impl LeSetExtScanParams {
    pub fn new(
        own_addr_kind: AddrKind,
        scanning_filter_policy: ScanningFilterPolicy,
        scanning_phys: PhyParams<ScanningPhy>,
    ) -> Self {
        Self(LeSetExtScanParamsParams {
            own_addr_kind,
            scanning_filter_policy,
            scanning_phys,
        })
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
        LeSetExtScanEnableParams {
            enable: bool,
            filter_duplicates: FilterDuplicates,
            duration: Duration<10_000>,
            period: Duration<1_280_000>,
        }
        Return = ();
    }
}

crate::cmd! {
    BASE
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.66
    LeExtCreateConn(LE, 0x0043) {
        Params = LeExtCreateConnParams;
    }
}

impl LeExtCreateConn {
    pub fn new(
        initiator_filter_policy: bool,
        own_addr_kind: AddrKind,
        peer_addr_kind: AddrKind,
        peer_addr: BdAddr,
        initiating_phys: PhyParams<InitiatingPhy>,
    ) -> Self {
        Self(LeExtCreateConnParams {
            initiator_filter_policy,
            own_addr_kind,
            peer_addr_kind,
            peer_addr,
            initiating_phys,
        })
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
    BASE
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.66
    LeExtCreateConnV2(LE, 0x0085) {
        Params = LeExtCreateConnV2Params;
    }
}

impl LeExtCreateConnV2 {
    pub fn new(
        adv_handle: AdvHandle,
        subevent: u8,
        initiator_filter_policy: bool,
        own_addr_kind: AddrKind,
        peer_addr_kind: AddrKind,
        peer_addr: BdAddr,
        initiating_phys: PhyParams<InitiatingPhy>,
    ) -> Self {
        Self(LeExtCreateConnV2Params {
            adv_handle,
            subevent,
            initiator_filter_policy,
            own_addr_kind,
            peer_addr_kind,
            peer_addr,
            initiating_phys,
        })
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
        LePeriodicAdvCreateSyncParams {
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
        LeAddDeviceToPeriodicAdvListParams {
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
        LeRemoveDeviceFromPeriodicAdvListParams {
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
        LeWriteRfPathCompensationParams {
            rf_tx_path_compensation_value: i16,
            rf_rx_path_compensation_value: i16,
        }
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.77
    LeSetPrivacyMode(LE, 0x004e) {
        LeSetPrivacyModeParams {
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
        LeSetConnectionlessCteTransmitParamsParams<'a> {
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
        LeSetConnectionlessCteTransmitEnableParams {
            adv_handle: AdvHandle,
            cte_enable: bool,
        }
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.84
    LeSetConnCteTransmitParams(LE, 0x0055) {
        LeSetConnCteTransmitParamsParams<'a> {
            cte_kinds: CteMask,
            switching_pattern: &'a [u8],
        }
        Return = ConnHandle;
        Handle = handle: ConnHandle;
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.86
    LeConnCteResponseEnable(LE, 0x0057) {
        LeConnCteResponseEnableParams {
            enable: bool,
        }
        Return = ConnHandle;
        Handle = handle: ConnHandle;
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
        LeSetPeriodicAdvReceiveEnableParams {
            sync_handle: SyncHandle,
            enable: LePeriodicAdvReceiveEnable,
        }
        Return = ();
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.89
    LePeriodicAdvSyncTransfer(LE, 0x005a) {
        LePeriodicAdvSyncTransferParams {
            service_data: u16,
            sync_handle: SyncHandle,
        }
        Return = ConnHandle;
        Handle = handle: ConnHandle;
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.90
    LePeriodicAdvSetInfoTransfer(LE, 0x005b) {
        LePeriodicAdvSetInfoTransferParams {
            service_data: u16,
            adv_handle: AdvHandle,
        }
        Return = ConnHandle;
        Handle = handle: ConnHandle;
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.91
    LeSetPeriodicAdvSyncTransferParams(LE, 0x005c) {
        LeSetPeriodicAdvSyncTransferParamsParams {
            mode: LePeriodicAdvSyncTransferMode,
            skip: u16,
            sync_timeout: Duration<10_000>,
            cte_kind: CteMask,
        }
        Return = ConnHandle;
        Handle = handle: ConnHandle;
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.92
    LeSetDefaultPeriodicAdvSyncTransferParams(LE, 0x005d) {
        LeSetDefaultPeriodicAdvSyncTransferParamsParams {
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
        Params = ConnHandle;
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.117
    LeEnhancedReadTransmitPowerLevel(LE, 0x0076) {
        LeEnhancedReadTransmitPowerLevelParams {
            phy: PhyKind,
        }
        /// Bluetooth Core Specification Vol 4, Part E, §7.8.117
        LeEnhancedReadTransmitPowerLevelReturn {
            phy: PhyKind,
            current_tx_power_level: i8,
            max_tx_power_level: i8,
        }
        Handle = handle: ConnHandle;
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.118
    LeReadRemoteTransmitPowerLevel(LE, 0x0077) {
        LeReadRemoteTransmitPowerLevelParams {
            handle: ConnHandle,
            phy: PhyKind,
        }
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.119
    LeSetPathLossReportingParams(LE, 0x0078) {
        LeSetPathLossReportingParamsParams {
            high_threshold: i8,
            high_hysteresis: i8,
            low_threshold: i8,
            low_hysteresis: i8,
            min_time_spent: u16,
        }
        Return = ConnHandle;
        Handle = handle: ConnHandle;
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.120
    LeSetPathLossReportingEnable(LE, 0x0079) {
        LeSetPathLossReportingEnableParams {
            enable: bool,
        }
        Return = ConnHandle;
        Handle = handle: ConnHandle;
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.121
    LeSetTransmitPowerReportingEnable(LE, 0x007a) {
        LeSetTransmitPowerReportingEnableParams {
            local_enable: bool,
            remote_enable: bool,
        }
        Return = ConnHandle;
        Handle = handle: ConnHandle;
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.122
    LeSetDataRelatedAddrChanges(LE, 0x007c) {
        LeSetDataRelatedAddrChangesParams {
            adv_handle: AdvHandle,
            change_reasons: LeDataRelatedAddrChangeReasons,
        }
        Return = ();
    }
}

cmd! {
    BASE
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.125
    LeSetPeriodicAdvSubeventData(LE, 0x0082) {
        Params<'a> = LeSetPeriodicAdvSubeventDataParams<'a, 'a>;
        Return = AdvHandle;
    }
}

impl<'a> LeSetPeriodicAdvSubeventData<'a> {
    pub fn new(adv_handle: AdvHandle, subevent: &'a [LePeriodicAdvSubeventData<'a>]) -> Self {
        Self(LeSetPeriodicAdvSubeventDataParams { adv_handle, subevent })
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct LeSetPeriodicAdvSubeventDataParams<'a, 'b> {
    pub adv_handle: AdvHandle,
    pub subevent: &'b [LePeriodicAdvSubeventData<'a>],
}

impl<'a, 'b> WriteHci for LeSetPeriodicAdvSubeventDataParams<'a, 'b> {
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
        LeSetPeriodicAdvResponseDataParams<'a> {
            adv_handle: SyncHandle,
            request_event: u16,
            request_subevent: u8,
            response_subevent: u8,
            response_slot: u8,
            response_data: &'a [u8],
        }
        Return = SyncHandle;
    }
}

cmd! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.8.127
    LeSetPeriodicSyncSubevent(LE, 0x007c) {
        LeSetPeriodicSyncSubeventParams<'a> {
            adv_handle: SyncHandle,
            periodic_adv_props: PeriodicAdvProps,
            subevents: &'a [u8],
        }
        Return = SyncHandle;
    }
}
