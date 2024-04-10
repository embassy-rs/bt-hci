use core::iter::FusedIterator;

use super::{param, param_slice, BdAddr, ConnHandle, Duration, RemainingBytes};
use crate::{ByteAlignedValue, FixedSizeValue, FromHciBytes, FromHciBytesError, WriteHci};

param!(struct AddrKind(u8));

impl AddrKind {
    pub const PUBLIC: AddrKind = AddrKind(0);
    pub const RANDOM: AddrKind = AddrKind(1);
    pub const RESOLVABLE_PRIVATE_OR_PUBLIC: AddrKind = AddrKind(2);
    pub const RESOLVABLE_PRIVATE_OR_RANDOM: AddrKind = AddrKind(3);
    pub const ANONYMOUS_ADV: AddrKind = AddrKind(0xff);
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

impl AdvChannelMap {
    pub const ALL: AdvChannelMap = AdvChannelMap(0x07);
    pub const CHANNEL_37: AdvChannelMap = AdvChannelMap(0x01);
    pub const CHANNEL_38: AdvChannelMap = AdvChannelMap(0x02);
    pub const CHANNEL_39: AdvChannelMap = AdvChannelMap(0x04);
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

#[derive(Default)]
#[repr(u16, align(1))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PhyOptions {
    #[default]
    NoPreferredCoding = 0,
    S2CodingPreferred = 1,
    S8CodingPreferred = 2,
}

unsafe impl FixedSizeValue for PhyOptions {
    #[inline(always)]
    fn is_valid(data: &[u8]) -> bool {
        data[0] == 0 || data[0] == 1 || data[0] == 2 || false
    }
}

unsafe impl ByteAlignedValue for PhyOptions {}

impl<'de> FromHciBytes<'de> for &'de PhyOptions {
    #[inline(always)]
    fn from_hci_bytes(data: &'de [u8]) -> Result<(Self, &'de [u8]), FromHciBytesError> {
        <PhyOptions as ByteAlignedValue>::ref_from_hci_bytes(data)
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct PhyParams<T> {
    pub le_1m_phy: Option<T>,
    pub le_2m_phy: Option<T>,
    pub le_coded_phy: Option<T>,
}

impl<T> PhyParams<T> {
    pub fn scanning_phys(&self) -> PhyMask {
        PhyMask::new()
            .set_le_1m_preferred(self.le_1m_phy.is_some())
            .set_le_2m_preferred(self.le_2m_phy.is_some())
            .set_le_coded_preferred(self.le_coded_phy.is_some())
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
    pub const fn new(v: u8) -> Self {
        Self(v)
    }

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
        (1, change_on_scan_repsonse_data_change, set_change_addr_on_scan_response_data_changes);
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

pub enum LeExtAdvDataStatus {
    Complete,
    IncompleteMoreExpected,
    IncompleteTruncated,
    Reserved,
}

impl LeExtAdvEventKind {
    pub fn data_status(&self) -> LeExtAdvDataStatus {
        let data_status = self.0[0] >> 5 & 0x03;
        match data_status {
            0 => LeExtAdvDataStatus::Complete,
            1 => LeExtAdvDataStatus::IncompleteMoreExpected,
            2 => LeExtAdvDataStatus::IncompleteTruncated,
            _ => LeExtAdvDataStatus::Reserved,
        }
    }

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

impl<'a> LeAdvReports<'a> {
    pub fn is_empty(&self) -> bool {
        self.num_reports == 0
    }

    pub fn len(&self) -> usize {
        usize::from(self.num_reports)
    }

    pub fn iter(&self) -> LeAdvReportsIter<'_> {
        LeAdvReportsIter {
            len: self.len(),
            bytes: &self.bytes,
        }
    }
}

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

impl<'a> ExactSizeIterator for LeAdvReportsIter<'a> {
    fn len(&self) -> usize {
        self.len
    }
}

impl<'a> FusedIterator for LeAdvReportsIter<'a> {}

param! {
    struct LeExtAdvReport<'a> {
        event_kind: LeExtAdvEventKind,
        addr_kind: AddrKind,
        addr: BdAddr,
        data: &'a [u8],
        rssi: i8,
    }
}

param! {
    struct LeExtAdvReports<'a> {
        num_reports: u8,
        bytes: RemainingBytes<'a>,
    }
}

impl<'a> LeExtAdvReports<'a> {
    pub fn is_empty(&self) -> bool {
        self.num_reports == 0
    }

    pub fn len(&self) -> usize {
        usize::from(self.num_reports)
    }

    pub fn iter(&self) -> LeExtAdvReportsIter<'_> {
        LeExtAdvReportsIter {
            len: self.len(),
            bytes: &self.bytes,
        }
    }
}

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

impl<'a> ExactSizeIterator for LeExtAdvReportsIter<'a> {
    fn len(&self) -> usize {
        self.len
    }
}

impl<'a> FusedIterator for LeExtAdvReportsIter<'a> {}

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
}
