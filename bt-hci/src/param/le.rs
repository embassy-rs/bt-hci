use core::iter::FusedIterator;

use super::{param, param_slice, BdAddr, ConnHandle, Duration, RemainingBytes};
use crate::{ByteAlignedValue, FixedSizeValue, FromHciBytes, FromHciBytesError, WriteHci};

param!(struct AddrKind(u8));

#[allow(missing_docs)]
impl AddrKind {
    pub const PUBLIC: AddrKind = AddrKind(0);
    pub const RANDOM: AddrKind = AddrKind(1);
    pub const RESOLVABLE_PRIVATE_OR_PUBLIC: AddrKind = AddrKind(2);
    pub const RESOLVABLE_PRIVATE_OR_RANDOM: AddrKind = AddrKind(3);
    pub const ANONYMOUS_ADV: AddrKind = AddrKind(0xff);

    /// Create a new instance.
    pub const fn new(v: u8) -> Self {
        Self(v)
    }

    /// Get the inner representation.
    pub fn as_raw(&self) -> u8 {
        self.0
    }
}

unsafe impl ByteAlignedValue for AddrKind {}

impl<'de> crate::FromHciBytes<'de> for &'de AddrKind {
    #[inline(always)]
    fn from_hci_bytes(data: &'de [u8]) -> Result<(Self, &'de [u8]), crate::FromHciBytesError> {
        <AddrKind as crate::ByteAlignedValue>::ref_from_hci_bytes(data)
    }
}

param! {
    bitfield AdvChannelMap[1] {
        (0, is_channel_37_enabled, enable_channel_37);
        (1, is_channel_38_enabled, enable_channel_38);
        (2, is_channel_39_enabled, enable_channel_39);
    }
}

#[allow(missing_docs)]
impl AdvChannelMap {
    pub const ALL: AdvChannelMap = AdvChannelMap(0x07);
    pub const CHANNEL_37: AdvChannelMap = AdvChannelMap(0x01);
    pub const CHANNEL_38: AdvChannelMap = AdvChannelMap(0x02);
    pub const CHANNEL_39: AdvChannelMap = AdvChannelMap(0x04);
}

param!(struct ChannelMap([u8; 5]));

impl ChannelMap {
    /// Create a new instance.
    pub fn new() -> Self {
        Self([0xff, 0xff, 0xff, 0xff, 0x1f])
    }

    /// Check if channel is marked as bad.
    pub fn is_channel_bad(&self, channel: u8) -> bool {
        let byte = usize::from(channel / 8);
        let bit = channel % 8;
        (self.0[byte] & (1 << bit)) == 0
    }

    /// Set channel to be marked as bad.
    pub fn set_channel_bad(&mut self, channel: u8, bad: bool) {
        let byte = usize::from(channel / 8);
        let bit = channel % 8;
        self.0[byte] = (self.0[byte] & !(1 << bit)) | (u8::from(!bad) << bit);
    }
}

unsafe impl ByteAlignedValue for ChannelMap {}

impl<'de> crate::FromHciBytes<'de> for &'de ChannelMap {
    #[inline(always)]
    fn from_hci_bytes(data: &'de [u8]) -> Result<(Self, &'de [u8]), crate::FromHciBytesError> {
        <ChannelMap as crate::ByteAlignedValue>::ref_from_hci_bytes(data)
    }
}

param! {
    #[derive(Default)]
    enum AdvKind {
        #[default]
        AdvInd = 0,
        AdvDirectIndHigh = 1,
        AdvScanInd = 2,
        AdvNonconnInd = 3,
        AdvDirectIndLow = 4,
    }
}

param! {
    #[derive(Default)]
    enum AdvFilterPolicy {
        #[default]
        Unfiltered = 0,
        FilterScan = 1,
        FilterConn = 2,
        FilterConnAndScan = 3,
    }
}

param! {
    #[derive(Default)]
    enum LeScanKind {
        #[default]
        Passive = 0,
        Active = 1,
    }
}

param! {
    #[derive(Default)]
    enum ScanningFilterPolicy {
        #[default]
        BasicUnfiltered = 0,
        BasicFiltered = 1,
        ExtUnfiltered = 2,
        ExtFiltered = 3,
    }
}

param! {
    #[derive(Default)]
    enum PhyKind {
        #[default]
        Le1M = 1,
        Le2M = 2,
        LeCoded = 3,
        LeCodedS2 = 4,
    }
}

param! {
    bitfield SpacingTypes[2] {
        (0, has_t_ifs_acl_cp, set_t_ifs_acl_cp);
        (1, has_t_ifs_acl_pc, set_t_ifs_acl_pc);
        (2, has_t_mces, set_t_mces);
        (3, has_t_ifs_cis, set_t_ifs_cis);
        (4, has_t_mss_cis, set_t_mss_cis);
    }
}

param! {
    enum FrameSpaceInitiator {
        LocalHostInitiated = 0x00,
        LocalControllerInitiated = 0x01,
        PeerInitiated = 0x02,
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
        (0, has_le_1m_phy, set_le_1m_phy);
        (1, has_le_2m_phy, set_le_2m_phy);
        (2, has_le_coded_phy, set_le_coded_phy);
    }
}

/// Preferences when one can choose the phy.
#[derive(Default)]
#[repr(u16, align(1))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[allow(missing_docs)]
pub enum PhyOptions {
    #[default]
    NoPreferredCoding = 0,
    S2CodingPreferred = 1,
    S8CodingPreferred = 2,
}

unsafe impl FixedSizeValue for PhyOptions {
    #[inline(always)]
    fn is_valid(data: &[u8]) -> bool {
        data[0] == 0 || data[0] == 1 || data[0] == 2
    }
}

unsafe impl ByteAlignedValue for PhyOptions {}

impl<'de> FromHciBytes<'de> for &'de PhyOptions {
    #[inline(always)]
    fn from_hci_bytes(data: &'de [u8]) -> Result<(Self, &'de [u8]), FromHciBytesError> {
        <PhyOptions as ByteAlignedValue>::ref_from_hci_bytes(data)
    }
}

/// PHY preference or requirement during extended advertisement (BLE5.4)
#[derive(Default)]
#[repr(u16, align(1))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[allow(missing_docs)]
pub enum AdvPhyOptions {
    #[default]
    NoPreferredCoding = 0,
    S2CodingPreferred = 1,
    S8CodingPreferred = 2,
    S2CodingRequired = 3,
    S8CodingRequired = 4,
}

unsafe impl FixedSizeValue for AdvPhyOptions {
    #[inline(always)]
    fn is_valid(data: &[u8]) -> bool {
        data[0] == 0 || data[0] == 1 || data[0] == 2 || data[0] == 3 || data[0] == 4
    }
}

unsafe impl ByteAlignedValue for AdvPhyOptions {}

impl<'de> FromHciBytes<'de> for &'de AdvPhyOptions {
    #[inline(always)]
    fn from_hci_bytes(data: &'de [u8]) -> Result<(Self, &'de [u8]), FromHciBytesError> {
        <AdvPhyOptions as ByteAlignedValue>::ref_from_hci_bytes(data)
    }
}

param! {
    struct ScanningPhy {
        active_scan: bool,
        scan_interval: Duration<625>,
        scan_window: Duration<625>,
    }
}

param! {
    struct InitiatingPhy {
        scan_interval: Duration<625>,
        scan_window: Duration<625>,
        conn_interval_min: Duration<1_250>,
        conn_interval_max: Duration<1_250>,
        max_latency: u16,
        supervision_timeout: Duration<10_000>,
        min_ce_len: Duration<625>,
        max_ce_len: Duration<625>,
    }
}

param! {
    struct ConnIntervalGroup {
        min: Duration<125>,
        max: Duration<125>,
        stride: Duration<125>,
    }
}

/// Parameters for different phy representations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct PhyParams<T> {
    /// 1M phy parameters.
    pub le_1m_phy: Option<T>,
    /// 2M phy parameters.
    pub le_2m_phy: Option<T>,
    /// Coded phy parameters.
    pub le_coded_phy: Option<T>,
}

impl<T> PhyParams<T> {
    /// Get the mask associated with the parameters.
    pub fn scanning_phys(&self) -> PhyMask {
        PhyMask::new()
            .set_le_1m_phy(self.le_1m_phy.is_some())
            .set_le_2m_phy(self.le_2m_phy.is_some())
            .set_le_coded_phy(self.le_coded_phy.is_some())
    }
}

impl<T: WriteHci> WriteHci for PhyParams<T> {
    #[inline(always)]
    fn size(&self) -> usize {
        1 + self.le_1m_phy.size() + self.le_2m_phy.size() + self.le_coded_phy.size()
    }

    #[inline(always)]
    fn write_hci<W: embedded_io::Write>(&self, mut writer: W) -> Result<(), W::Error> {
        self.scanning_phys().write_hci(&mut writer)?;
        self.le_1m_phy.write_hci(&mut writer)?;
        self.le_2m_phy.write_hci(&mut writer)?;
        self.le_coded_phy.write_hci(&mut writer)?;
        Ok(())
    }

    #[inline(always)]
    async fn write_hci_async<W: ::embedded_io_async::Write>(&self, mut writer: W) -> Result<(), W::Error> {
        self.scanning_phys().write_hci_async(&mut writer).await?;
        self.le_1m_phy.write_hci_async(&mut writer).await?;
        self.le_2m_phy.write_hci_async(&mut writer).await?;
        self.le_coded_phy.write_hci_async(&mut writer).await?;
        Ok(())
    }
}

param!(struct AdvHandle(u8));

impl AdvHandle {
    /// Create a new instance.
    pub const fn new(v: u8) -> Self {
        Self(v)
    }

    /// Get the inner representation.
    pub fn as_raw(&self) -> u8 {
        self.0
    }
}

unsafe impl ByteAlignedValue for AdvHandle {}

impl<'de> crate::FromHciBytes<'de> for &'de AdvHandle {
    #[inline(always)]
    fn from_hci_bytes(data: &'de [u8]) -> Result<(Self, &'de [u8]), crate::FromHciBytesError> {
        <AdvHandle as crate::ByteAlignedValue>::ref_from_hci_bytes(data)
    }
}

param! {
    bitfield AdvEventProps[2] {
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
    #[derive(Default)]
    enum Operation {
        #[default]
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
        duration: Duration<10_000>,
        max_ext_adv_events: u8,
    }
}

param_slice!(&'a [AdvSet]);

param! {
    bitfield PeriodicAdvProps[2] {
        (6, is_tx_power_included, include_tx_power);
    }
}

param! {
    #[derive(Default)]
    enum FilterDuplicates {
        #[default]
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

param!(struct BigHandle(u16));

param! {
    #[derive(Default)]
    enum PrivacyMode {
        #[default]
        Network = 0,
        Device = 1,
    }
}

param! {
    #[derive(Default)]
    enum CteKind {
        #[default]
        AoA = 0,
        AoD1Us = 1,
        AoD2Us = 2,
        NoCte = 0xff,
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
    #[derive(Default)]
    enum LePeriodicAdvSyncTransferMode {
        #[default]
        NoSync = 0,
        SyncRx = 1,
        SyncRxReport = 2,
        SyncRxReportFilterDuplicates = 3,
    }
}

param! {
    bitfield LeDataRelatedAddrChangeReasons[1] {
        (0, change_on_adv_data_change, set_change_addr_on_adv_data_changes);
        (1, change_on_scan_response_data_change, set_change_addr_on_scan_response_data_changes);
    }
}

param! {
    #[derive(Default)]
    enum LeConnRole {
        #[default]
        Central = 0,
        Peripheral = 1,
    }
}

param! {
    #[derive(Default)]
    enum ClockAccuracy {
        #[default]
        Ppm500 = 0,
        Ppm250 = 1,
        Ppm150 = 2,
        Ppm100 = 3,
        Ppm75 = 4,
        Ppm50 = 5,
        Ppm30 = 6,
        Ppm20 = 7,
    }
}

param! {
    struct LeAdvertisingReportParam<'a> {
        event_type: u8,
        addr_kind: AddrKind,
        addr: BdAddr,
        data: &'a [u8],
        rssi: i8,
    }
}

param_slice! {
    [LeDirectedAdvertisingReportParam; 16] {
        event_type[0]: u8,
        addr_kind[1]: AddrKind,
        addr[2]: BdAddr,
        direct_addr_kind[8]: AddrKind,
        direct_addr[9]: BdAddr,
        rssi[15]: i8,
    }
}

param_slice! {
    [LeIQSample; 2] {
        i_sample[0]: i8,
        q_sample[1]: i8,
    }
}

param_slice! {
    [BisConnHandle; 2] {
        handle[0]: ConnHandle,
    }
}

param! {
    #[derive(Default)]
    enum DataStatus {
        #[default]
        Complete = 0,
        Incomplete = 1,
        Failed = 0xff,
    }
}

param! {
    #[derive(Default)]
    enum PacketStatus {
        #[default]
        CrcCorrect = 0,
        CrcIncorrectUsedLength = 1,
        CrcIncorrectUsedOther = 2,
        InsufficientResources = 0xff,
    }
}

param! {
    #[derive(Default)]
    enum TxStatus {
        #[default]
        Transmitted = 0,
        NotTransmitted = 1,
    }
}

param! {
    #[derive(Default)]
    enum ZoneEntered {
        #[default]
        Low = 0,
        Middle = 1,
        High = 2,
    }
}

param! {
    #[derive(Default)]
    enum LeTxPowerReportingReason {
        #[default]
        LocalTxPowerChanged = 0,
        RemoteTxPowerChanged = 1,
        LeReadRemoteTxPowerLevelCompleted = 2,
    }
}

param! {
    #[derive(Default)]
    enum LeAdvEventKind {
        #[default]
        AdvInd = 0,
        AdvDirectInd = 1,
        AdvScanInd = 2,
        AdvNonconnInd = 3,
        ScanRsp = 4,
    }
}

param! {
    bitfield LeExtAdvEventKind[2] {
        (0, connectable, set_connectable);
        (1, scannable, set_scannable);
        (2, directed, set_directed);
        (3, scan_response, set_scan_response);
        (4, legacy, set_legacy);
    }
}

/// Advertising data status.
#[allow(missing_docs)]
pub enum LeExtAdvDataStatus {
    Complete,
    IncompleteMoreExpected,
    IncompleteTruncated,
    Reserved,
}

impl LeExtAdvEventKind {
    /// Get data status.
    pub fn data_status(&self) -> LeExtAdvDataStatus {
        let data_status = (self.0[0] >> 5) & 0x03;
        match data_status {
            0 => LeExtAdvDataStatus::Complete,
            1 => LeExtAdvDataStatus::IncompleteMoreExpected,
            2 => LeExtAdvDataStatus::IncompleteTruncated,
            _ => LeExtAdvDataStatus::Reserved,
        }
    }

    /// Set data status.
    pub fn set_data_status(mut self, status: LeExtAdvDataStatus) -> Self {
        let value = match status {
            LeExtAdvDataStatus::Complete => 0,
            LeExtAdvDataStatus::IncompleteMoreExpected => 1,
            LeExtAdvDataStatus::IncompleteTruncated => 2,
            LeExtAdvDataStatus::Reserved => 3,
        };
        self.0[0] &= !(0x03 << 5);
        self.0[0] |= value << 5;
        self
    }
}

param! {
    struct LeAdvReport<'a> {
        event_kind: LeAdvEventKind,
        addr_kind: AddrKind,
        addr: BdAddr,
        data: &'a [u8],
        rssi: i8,
    }
}

param! {
    struct LeAdvReports<'a> {
        num_reports: u8,
        bytes: RemainingBytes<'a>,
    }
}

impl LeAdvReports<'_> {
    /// Check if there are more reports available.
    pub fn is_empty(&self) -> bool {
        self.num_reports == 0
    }

    /// Number of advertising reports.
    pub fn len(&self) -> usize {
        usize::from(self.num_reports)
    }

    /// Create an iterator over the advertising reports.
    pub fn iter(&self) -> LeAdvReportsIter<'_> {
        LeAdvReportsIter {
            len: self.len(),
            bytes: &self.bytes,
        }
    }
}

/// An iterator for advertising reports.
pub struct LeAdvReportsIter<'a> {
    len: usize,
    bytes: &'a [u8],
}

impl<'a> Iterator for LeAdvReportsIter<'a> {
    type Item = Result<LeAdvReport<'a>, FromHciBytesError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            match LeAdvReport::from_hci_bytes(self.bytes) {
                Ok((report, rest)) => {
                    self.bytes = rest;
                    self.len -= 1;
                    Some(Ok(report))
                }
                Err(err) => {
                    self.len = 0;
                    Some(Err(err))
                }
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl ExactSizeIterator for LeAdvReportsIter<'_> {
    fn len(&self) -> usize {
        self.len
    }
}

impl FusedIterator for LeAdvReportsIter<'_> {}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[allow(missing_docs)]
pub struct LeExtAdvReport<'a> {
    pub event_kind: LeExtAdvEventKind,
    pub addr_kind: AddrKind,
    pub addr: BdAddr,
    pub primary_adv_phy: PhyKind,
    pub secondary_adv_phy: Option<PhyKind>,
    pub adv_sid: u8,
    pub tx_power: i8,
    pub rssi: i8,
    pub adv_interval: Duration<1_250>,
    pub direct_addr_kind: AddrKind,
    pub direct_addr: BdAddr,
    pub data: &'a [u8],
}

impl WriteHci for LeExtAdvReport<'_> {
    #[inline(always)]
    fn size(&self) -> usize {
        WriteHci::size(&self.event_kind)
            + WriteHci::size(&self.addr_kind)
            + WriteHci::size(&self.addr)
            + WriteHci::size(&self.primary_adv_phy)
            + 1 //secondary_adv_phy
            + WriteHci::size(&self.adv_sid)
            + WriteHci::size(&self.tx_power)
            + WriteHci::size(&self.rssi)
            + WriteHci::size(&self.adv_interval)
            + WriteHci::size(&self.direct_addr_kind)
            + WriteHci::size(&self.direct_addr)
            + WriteHci::size(&self.data)
    }
    #[inline(always)]
    fn write_hci<W: ::embedded_io::Write>(&self, mut writer: W) -> Result<(), W::Error> {
        self.event_kind.write_hci(&mut writer)?;
        self.addr_kind.write_hci(&mut writer)?;
        self.addr.write_hci(&mut writer)?;
        self.primary_adv_phy.write_hci(&mut writer)?;
        match self.secondary_adv_phy {
            None => 0u8.write_hci(&mut writer)?,
            Some(val) => val.write_hci(&mut writer)?,
        };
        self.adv_sid.write_hci(&mut writer)?;
        self.tx_power.write_hci(&mut writer)?;
        self.rssi.write_hci(&mut writer)?;
        self.adv_interval.write_hci(&mut writer)?;
        self.direct_addr_kind.write_hci(&mut writer)?;
        self.direct_addr.write_hci(&mut writer)?;
        self.data.write_hci(&mut writer)?;
        Ok(())
    }
    #[inline(always)]
    async fn write_hci_async<W: ::embedded_io_async::Write>(&self, mut writer: W) -> Result<(), W::Error> {
        self.event_kind.write_hci_async(&mut writer).await?;
        self.addr_kind.write_hci_async(&mut writer).await?;
        self.addr.write_hci_async(&mut writer).await?;
        self.primary_adv_phy.write_hci_async(&mut writer).await?;
        match self.secondary_adv_phy {
            None => 0u8.write_hci_async(&mut writer).await?,
            Some(val) => val.write_hci_async(&mut writer).await?,
        };
        self.adv_sid.write_hci_async(&mut writer).await?;
        self.tx_power.write_hci_async(&mut writer).await?;
        self.rssi.write_hci_async(&mut writer).await?;
        self.adv_interval.write_hci_async(&mut writer).await?;
        self.direct_addr_kind.write_hci_async(&mut writer).await?;
        self.direct_addr.write_hci_async(&mut writer).await?;
        self.data.write_hci_async(&mut writer).await?;
        Ok(())
    }
}

impl<'de> crate::FromHciBytes<'de> for LeExtAdvReport<'de> {
    #[allow(unused_variables)]
    fn from_hci_bytes(data: &'de [u8]) -> Result<(Self, &'de [u8]), crate::FromHciBytesError> {
        let (event_kind, data) = <LeExtAdvEventKind as crate::FromHciBytes>::from_hci_bytes(data)?;
        let (addr_kind, data) = <AddrKind as crate::FromHciBytes>::from_hci_bytes(data)?;
        let (addr, data) = <BdAddr as crate::FromHciBytes>::from_hci_bytes(data)?;
        let (primary_adv_phy, data) = <PhyKind as crate::FromHciBytes>::from_hci_bytes(data)?;
        let (secondary_adv_phy, data) = if data[0] == 0 {
            (None, &data[1..])
        } else {
            let (ret, rest) = <PhyKind as crate::FromHciBytes>::from_hci_bytes(data)?;
            (Some(ret), rest)
        };
        let (adv_sid, data) = <u8 as crate::FromHciBytes>::from_hci_bytes(data)?;
        let (tx_power, data) = <i8 as crate::FromHciBytes>::from_hci_bytes(data)?;
        let (rssi, data) = <i8 as crate::FromHciBytes>::from_hci_bytes(data)?;
        let (adv_interval, data) = <Duration<1_250> as crate::FromHciBytes>::from_hci_bytes(data)?;
        let (direct_addr_kind, data) = <AddrKind as crate::FromHciBytes>::from_hci_bytes(data)?;
        let (direct_addr, data) = <BdAddr as crate::FromHciBytes>::from_hci_bytes(data)?;
        let (data, rest) = <&'de [u8] as crate::FromHciBytes>::from_hci_bytes(data)?;
        Ok((
            Self {
                event_kind,
                addr_kind,
                addr,
                primary_adv_phy,
                secondary_adv_phy,
                adv_sid,
                tx_power,
                rssi,
                adv_interval,
                direct_addr_kind,
                direct_addr,
                data,
            },
            rest,
        ))
    }
}

param! {
    struct LeExtAdvReports<'a> {
        num_reports: u8,
        bytes: RemainingBytes<'a>,
    }
}

impl LeExtAdvReports<'_> {
    /// Check if there are more reports available.
    pub fn is_empty(&self) -> bool {
        self.num_reports == 0
    }

    /// Number of advertising reports.
    pub fn len(&self) -> usize {
        usize::from(self.num_reports)
    }

    /// Create an iterator over the advertising reports.
    pub fn iter(&self) -> LeExtAdvReportsIter<'_> {
        LeExtAdvReportsIter {
            len: self.len(),
            bytes: &self.bytes,
        }
    }
}

/// An iterator for extended advertising reports.
pub struct LeExtAdvReportsIter<'a> {
    len: usize,
    bytes: &'a [u8],
}

impl<'a> Iterator for LeExtAdvReportsIter<'a> {
    type Item = Result<LeExtAdvReport<'a>, FromHciBytesError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            match LeExtAdvReport::from_hci_bytes(self.bytes) {
                Ok((report, rest)) => {
                    self.bytes = rest;
                    self.len -= 1;
                    Some(Ok(report))
                }
                Err(err) => {
                    self.len = 0;
                    Some(Err(err))
                }
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl ExactSizeIterator for LeExtAdvReportsIter<'_> {
    fn len(&self) -> usize {
        self.len
    }
}

impl FusedIterator for LeExtAdvReportsIter<'_> {}

param! {
    struct LePeriodicAdvSubeventData<'a> {
        subevent: u8,
        response_slot_start: u8,
        response_slot_count: u8,
        subevent_data: &'a [u8],
    }
}

impl<'a, 'b: 'a> WriteHci for &'a [LePeriodicAdvSubeventData<'b>] {
    #[inline(always)]
    fn size(&self) -> usize {
        1 + self.iter().map(WriteHci::size).sum::<usize>()
    }
    #[inline(always)]
    fn write_hci<W: ::embedded_io::Write>(&self, mut writer: W) -> Result<(), W::Error> {
        writer.write_all(&[self.len() as u8])?;
        for x in self.iter() {
            <LePeriodicAdvSubeventData as WriteHci>::write_hci(x, &mut writer)?;
        }
        Ok(())
    }
    #[inline(always)]
    async fn write_hci_async<W: ::embedded_io_async::Write>(&self, mut writer: W) -> Result<(), W::Error> {
        writer.write_all(&[self.len() as u8]).await?;
        for x in self.iter() {
            <LePeriodicAdvSubeventData as WriteHci>::write_hci_async(x, &mut writer).await?;
        }
        Ok(())
    }
}

#[allow(missing_docs)]
fn read_n<T: ByteAlignedValue>(data: &[u8], n: usize) -> Result<(&[T], &[u8]), FromHciBytesError> {
    let size = n * core::mem::size_of::<T>();
    if data.len() < size {
        return Err(FromHciBytesError::InvalidSize);
    }
    let (bytes, rest) = data.split_at(size);
    let slice = unsafe { core::slice::from_raw_parts(bytes.as_ptr() as *const T, n) };
    Ok((slice, rest))
}

param! {
    struct LePeriodicAdvertisingResponseReport<'a> {
        tx_power: i8,
        rssi: i8,
        cte_type: CteKind,
        response_slot: u8,
        data_status: DataStatus,
        data_length: u8,
        data: &'a [u8],
    }
}

/// Container for periodic advertising response report data.
#[derive(Debug, Clone, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct LePeriodicAdvertisingResponseReports<'a> {
    num_responses: u8,
    tx_power: &'a [i8],
    rssi: &'a [i8],
    cte_type: &'a [CteKind],
    response_slot: &'a [u8],
    data_status: &'a [DataStatus],
    data_length: &'a [u8],
    data: &'a [u8],
}

impl<'a> LePeriodicAdvertisingResponseReports<'a> {
    /// Returns `true` if there are no responses.
    pub fn is_empty(&self) -> bool {
        self.num_responses == 0
    }

    /// Returns the number of responses.
    pub fn len(&self) -> usize {
        usize::from(self.num_responses)
    }

    /// Returns the response entry at the given index, or `None` if out of bounds.
    pub fn get(&self, index: usize) -> Option<LePeriodicAdvertisingResponseReport<'a>> {
        if index >= self.len() {
            return None;
        }
        let data_offset: usize = self.data_length[..index].iter().map(|&l| l as usize).sum();
        let data_len = self.data_length[index] as usize;
        Some(LePeriodicAdvertisingResponseReport {
            tx_power: self.tx_power[index],
            rssi: self.rssi[index],
            cte_type: self.cte_type[index],
            response_slot: self.response_slot[index],
            data_status: self.data_status[index],
            data_length: self.data_length[index],
            data: &self.data[data_offset..data_offset + data_len],
        })
    }

    /// Returns an iterator over all response entries.
    pub fn iter(&self) -> LePeriodicAdvertisingResponseReportsIter<'_> {
        LePeriodicAdvertisingResponseReportsIter {
            reports: self,
            index: 0,
        }
    }
}

impl<'de> FromHciBytes<'de> for LePeriodicAdvertisingResponseReports<'de> {
    fn from_hci_bytes(data: &'de [u8]) -> Result<(Self, &'de [u8]), FromHciBytesError> {
        let (num_responses, data) = u8::from_hci_bytes(data)?;
        let n = num_responses as usize;

        let (tx_power, data) = read_n::<i8>(data, n)?;
        let (rssi, data) = read_n::<i8>(data, n)?;
        let (cte_type, data) = read_n::<CteKind>(data, n)?;
        let (response_slot, data) = read_n::<u8>(data, n)?;
        let (data_status, data) = read_n::<DataStatus>(data, n)?;
        let (data_length, data) = read_n::<u8>(data, n)?;

        Ok((
            Self {
                num_responses,
                tx_power,
                rssi,
                cte_type,
                response_slot,
                data_status,
                data_length,
                data,
            },
            &[],
        ))
    }
}

/// An iterator over the LePeriodicAdvertisingResponse reports.
pub struct LePeriodicAdvertisingResponseReportsIter<'a> {
    reports: &'a LePeriodicAdvertisingResponseReports<'a>,
    index: usize,
}

impl<'a> Iterator for LePeriodicAdvertisingResponseReportsIter<'a> {
    type Item = LePeriodicAdvertisingResponseReport<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let entry = self.reports.get(self.index)?;
        self.index += 1;
        Some(entry)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.reports.len() - self.index;
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for LePeriodicAdvertisingResponseReportsIter<'_> {
    fn len(&self) -> usize {
        self.reports.len() - self.index
    }
}

impl FusedIterator for LePeriodicAdvertisingResponseReportsIter<'_> {}

param! {
    struct LeCsSubeventStepEntry<'a> {
        step_mode: u8,
        step_channel: u8,
        step_data_length: u8,
        step_data: &'a [u8],
    }
}

/// Container for CS subevent step data.
///
/// Parses the column-major wire format:
/// `num_steps_reported | step_mode[] | step_channel[] | step_data_length[] | step_data[]`
///
/// Entries are accessed via [`get`](LeCsSubeventStepData::get)
/// or [`iter`](LeCsSubeventStepData::iter).
#[derive(Debug, Clone, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct LeCsSubeventStepData<'a> {
    num_steps_reported: u8,
    step_mode: &'a [u8],
    step_channel: &'a [u8],
    step_data_length: &'a [u8],
    step_data: &'a [u8],
}

impl<'a> LeCsSubeventStepData<'a> {
    /// Returns `true` if there are no steps.
    pub fn is_empty(&self) -> bool {
        self.num_steps_reported == 0
    }

    /// Returns the number of steps.
    pub fn len(&self) -> usize {
        usize::from(self.num_steps_reported)
    }

    /// Returns the step entry at the given index, or `None` if out of bounds.
    pub fn get(&self, index: usize) -> Option<LeCsSubeventStepEntry<'a>> {
        if index >= self.len() {
            return None;
        }
        let data_offset: usize = self.step_data_length[..index].iter().map(|&l| l as usize).sum();
        let data_len = self.step_data_length[index] as usize;
        Some(LeCsSubeventStepEntry {
            step_mode: self.step_mode[index],
            step_channel: self.step_channel[index],
            step_data_length: self.step_data_length[index],
            step_data: &self.step_data[data_offset..data_offset + data_len],
        })
    }

    /// Returns an iterator over all step entries.
    pub fn iter(&self) -> LeCsSubeventStepDataIter<'_> {
        LeCsSubeventStepDataIter { data: self, index: 0 }
    }
}

impl<'de> FromHciBytes<'de> for LeCsSubeventStepData<'de> {
    fn from_hci_bytes(data: &'de [u8]) -> Result<(Self, &'de [u8]), FromHciBytesError> {
        let (num_steps_reported, data) = u8::from_hci_bytes(data)?;
        let n = num_steps_reported as usize;

        let (step_mode, data) = read_n::<u8>(data, n)?;
        let (step_channel, data) = read_n::<u8>(data, n)?;
        let (step_data_length, data) = read_n::<u8>(data, n)?;

        Ok((
            Self {
                num_steps_reported,
                step_mode,
                step_channel,
                step_data_length,
                step_data: data,
            },
            &[],
        ))
    }
}

/// An iterator over LeCsSubeventStepEntry values.
pub struct LeCsSubeventStepDataIter<'a> {
    data: &'a LeCsSubeventStepData<'a>,
    index: usize,
}

impl<'a> Iterator for LeCsSubeventStepDataIter<'a> {
    type Item = LeCsSubeventStepEntry<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let entry = self.data.get(self.index)?;
        self.index += 1;
        Some(entry)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.data.len() - self.index;
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for LeCsSubeventStepDataIter<'_> {
    fn len(&self) -> usize {
        self.data.len() - self.index
    }
}

impl FusedIterator for LeCsSubeventStepDataIter<'_> {}

param! {
    #[derive(Default)]
    enum DoneStatus {
        #[default]
        Complete = 0,
        Partial = 1,
        Aborted = 0xf,
    }
}

/// Procedure abort reason.
#[repr(u8)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ProcedureAbortReason {
    #[default]
    /// Report with no abort.
    NoAbort = 0x0,
    /// Abort because of local Host or remote request.
    HostRequest = 0x1,
    /// Abort because filtered channel map has less than 15 channels.
    FilteredChannelMap = 0x2,
    /// Abort because the channel map update instant has passed.
    ChannelMapInstantPassed = 0x3,
    /// Abort because of unspecified reasons.
    Unspecified = 0xf,
}

impl From<u8> for ProcedureAbortReason {
    fn from(v: u8) -> Self {
        match v {
            0x0 => Self::NoAbort,
            0x1 => Self::HostRequest,
            0x2 => Self::FilteredChannelMap,
            0x3 => Self::ChannelMapInstantPassed,
            _ => Self::Unspecified,
        }
    }
}

/// Subevent abort reason.
#[repr(u8)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SubeventAbortReason {
    #[default]
    /// Report with no abort.
    NoAbort = 0x0,
    /// Abort because of local Host or remote request.
    HostRequest = 0x1,
    /// Abort because no CS_SYNC (mode-0) received.
    NoCsSync = 0x2,
    /// Abort because of scheduling conflicts or limited resources.
    SchedulingConflict = 0x3,
    /// Abort because of unspecified reasons.
    Unspecified = 0xf,
}

impl From<u8> for SubeventAbortReason {
    fn from(v: u8) -> Self {
        match v {
            0x0 => Self::NoAbort,
            0x1 => Self::HostRequest,
            0x2 => Self::NoCsSync,
            0x3 => Self::SchedulingConflict,
            _ => Self::Unspecified,
        }
    }
}

/// Abort reason for CS subevent result, packed as two 4-bit nibbles.
///
/// Bits 0-3: procedure abort reason ([`ProcedureAbortReason`])
/// Bits 4-7: subevent abort reason ([`SubeventAbortReason`])
#[repr(transparent)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct PackedAbortReasons(u8);

impl PackedAbortReasons {
    /// Returns the procedure-level abort reason.
    pub fn procedure_reason(&self) -> ProcedureAbortReason {
        ProcedureAbortReason::from(self.0 & 0x0f)
    }

    /// Returns the subevent-level abort reason.
    pub fn subevent_reason(&self) -> SubeventAbortReason {
        SubeventAbortReason::from((self.0 >> 4) & 0x0f)
    }
}

unsafe impl FixedSizeValue for PackedAbortReasons {
    fn is_valid(_data: &[u8]) -> bool {
        true
    }
}

/// Frequency compensation value in units of 0.01 ppm.
#[repr(transparent)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct FrequencyCompensation(u16);

impl FrequencyCompensation {
    /// Returns the raw 16-bit value.
    pub fn as_raw(&self) -> u16 {
        self.0
    }

    /// Returns `true` if the value is available.
    pub fn is_available(&self) -> bool {
        self.0 != 0xC000
    }

    /// Returns the frequency compensation in 0.01 ppm units, or `None` if not available.
    pub fn as_ppm_x100(&self) -> Option<i16> {
        if !self.is_available() {
            return None;
        }
        let val = self.0 & 0x7FFF;
        Some(((val << 1) as i16) >> 1)
    }
}

unsafe impl FixedSizeValue for FrequencyCompensation {
    fn is_valid(_data: &[u8]) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ext_adv_event_kind() {
        let k = LeExtAdvEventKind::new().set_connectable(true);
        assert_eq!(k.0[0], 0b0000001);
        let k = k.set_data_status(LeExtAdvDataStatus::Complete);
        assert_eq!(k.0[0], 0b0000001);
        let k = k.set_data_status(LeExtAdvDataStatus::IncompleteMoreExpected);
        assert_eq!(k.0[0], 0b0100001);
        let k = k.set_data_status(LeExtAdvDataStatus::IncompleteTruncated);
        assert_eq!(k.0[0], 0b1000001);
    }

    #[test]
    fn test_channel_map_new() {
        let m = ChannelMap::new();
        for chan in 0..37 {
            assert!(!m.is_channel_bad(chan));
        }

        for chan in 37..40 {
            assert!(m.is_channel_bad(chan));
        }
    }

    #[test]
    fn test_frequency_compensation_positive() {
        let fc = FrequencyCompensation(0x2710);
        assert!(fc.is_available());
        assert_eq!(fc.as_ppm_x100(), Some(10000));
        assert_eq!(fc.as_raw(), 0x2710);
    }

    #[test]
    fn test_frequency_compensation_negative() {
        let fc = FrequencyCompensation(0x58F0);
        assert!(fc.is_available());
        assert_eq!(fc.as_ppm_x100(), Some(-10000));
        assert_eq!(fc.as_raw(), 0x58F0);
    }

    #[test]
    fn test_frequency_compensation_not_available() {
        let fc = FrequencyCompensation(0xC000);
        assert!(!fc.is_available());
        assert_eq!(fc.as_ppm_x100(), None);
        assert_eq!(fc.as_raw(), 0xC000);
    }
}
