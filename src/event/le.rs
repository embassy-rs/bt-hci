//! Bluetooth LE events.
//!
//! This module contains the sub-events of the LE Meta event (see Bluetooth Core Specification Vol 4, Part E, §7.7.65).

use crate::param::{
    AddrKind, AdvHandle, BdAddr, BigHandle, BisConnHandle, ClockAccuracy, ConnHandle, CteKind, DataStatus, Duration,
    IsoDuration, LeAdvReports, LeConnRole, LeDirectedAdvertisingReportParam, LeExtAdvReports, LeFeatureMask,
    LeIQSample, LeTxPowerReportingReason, PacketStatus, PhyKind, PowerLevelKind, Status, SyncHandle, ZoneEntered,
};
use crate::{FromHciBytes, FromHciBytesError};

/// A trait for objects which contain the parameters for a specific HCI LE event
pub trait LeEventParams<'a>: FromHciBytes<'a> {
    /// The LE meta event subevent code these parameters are for
    const SUBEVENT_CODE: u8;
}

macro_rules! le_events {
    (
        $(
            $(#[$attrs:meta])*
            struct $name:ident$(<$life:lifetime>)?($code:expr) {
                $($field:ident: $ty:ty),*
                $(,)?
            }
        )+
    ) => {
        /// Bluetooth Core Specification Vol 4, Part E, §7.7.65
        #[non_exhaustive]
        #[derive(Debug, Clone, Hash)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum LeEvent<'a> {
            $(
                #[allow(missing_docs)]
                $name($name$(<$life>)?),
            )+
        }

        impl<'a> $crate::FromHciBytes<'a> for LeEvent<'a> {
            fn from_hci_bytes(data: &'a [u8]) -> Result<(Self, &'a [u8]), FromHciBytesError> {
                let (subcode, data) = data.split_first().ok_or(FromHciBytesError::InvalidSize)?;
                match subcode {
                    $($code => $name::from_hci_bytes(data).map(|(x, y)| (Self::$name(x), y)),)+
                    _ => Err(FromHciBytesError::InvalidValue),
                }
            }
        }

        $(
            $(#[$attrs])*
            #[derive(Debug, Clone, Hash)]
            #[cfg_attr(feature = "defmt", derive(defmt::Format))]
            pub struct $name$(<$life>)? {
                $(
                    #[allow(missing_docs)]
                    pub $field: $ty,
                )*
            }

            #[automatically_derived]
            impl<'a> $crate::FromHciBytes<'a> for $name$(<$life>)? {
                #[allow(unused_variables)]
                fn from_hci_bytes(data: &'a [u8]) -> Result<(Self, &'a [u8]), $crate::FromHciBytesError> {
                    let total = 0;
                    $(
                        let ($field, data) = <$ty as $crate::FromHciBytes>::from_hci_bytes(data)?;
                    )*
                    Ok((Self {
                        $($field,)*
                    }, data))
                }
            }

            #[automatically_derived]
            impl<'a> LeEventParams<'a> for $name$(<$life>)? {
                const SUBEVENT_CODE: u8 = $code;
            }
        )+
    };
}

le_events! {
    /// Bluetooth Core Specification Vol 4, Part E, §7.7.65.1
    struct LeConnectionComplete(1) {
        status: Status,
        handle: ConnHandle,
        role: LeConnRole,
        peer_addr_kind: AddrKind,
        peer_addr: BdAddr,
        conn_interval: Duration<1_250>,
        peripheral_latency: u16,
        supervision_timeout: Duration<10_000>,
        central_clock_accuracy: ClockAccuracy,
    }

    /// Bluetooth Core Specification Vol 4, Part E, §7.7.65.2
    struct LeAdvertisingReport<'a>(2) {
        reports: LeAdvReports<'a>,
    }

    /// Bluetooth Core Specification Vol 4, Part E, §7.7.65.3
    struct LeConnectionUpdateComplete(3) {
        status: Status,
        handle: ConnHandle,
        conn_interval: Duration<1_250>,
        peripheral_latency: u16,
        supervision_timeout: Duration<10_000>,
    }

    /// Bluetooth Core Specification Vol 4, Part E, §7.7.65.4
    struct LeReadRemoteFeaturesComplete(4) {
        status: Status,
        handle: ConnHandle,
        le_features: LeFeatureMask,
    }

    /// Bluetooth Core Specification Vol 4, Part E, §7.7.65.5
    struct LeLongTermKeyRequest(5) {
        handle: ConnHandle,
        random_number: [u8; 8],
        encrypted_diversifier: u16,
    }

    /// Bluetooth Core Specification Vol 4, Part E, §7.7.65.6
    struct LeRemoteConnectionParameterRequest(6) {
        handle: ConnHandle,
        interval_min: Duration<1_250>,
        interval_max: Duration<1_250>,
        max_latency: u16,
        timeout: Duration<10_000>,
    }

    /// Bluetooth Core Specification Vol 4, Part E, §7.7.65.7
    struct LeDataLengthChange(7) {
        handle: ConnHandle,
        max_tx_octets: u16,
        max_tx_time: u16,
        max_rx_octets: u16,
        max_rx_time: u16,
    }

    /// Bluetooth Core Specification Vol 4, Part E, §7.7.65.8
    struct LeReadLocalP256PublicKeyComplete(8) {
        status: Status,
        key_x_coordinate: [u8; 32],
        key_y_coordinate: [u8; 32],
    }

    /// Bluetooth Core Specification Vol 4, Part E, §7.7.65.9
    struct LeGenerateDhkeyComplete(9) {
        status: Status,
        dh_key: [u8; 32],
    }

    /// Bluetooth Core Specification Vol 4, Part E, §7.7.65.10
    struct LeEnhancedConnectionComplete(10) {
        status: Status,
        handle: ConnHandle,
        role: LeConnRole,
        peer_addr_kind: AddrKind,
        peer_addr: BdAddr,
        local_resolvable_private_addr: BdAddr,
        peer_resolvable_private_addr: BdAddr,
        conn_interval: Duration<1_250>,
        peripheral_latency: u16,
        supervision_timeout: Duration<10_000>,
        central_clock_accuracy: ClockAccuracy,
    }

    /// Bluetooth Core Specification Vol 4, Part E, §7.7.65.11
    struct LeDirectedAdvertisingReport<'a>(11) {
        reports: &'a [LeDirectedAdvertisingReportParam],
    }

    /// Bluetooth Core Specification Vol 4, Part E, §7.7.65.12
    struct LePhyUpdateComplete(12) {
        status: Status,
        handle: ConnHandle,
        tx_phy: PhyKind,
        rx_phy: PhyKind,
    }

    /// Bluetooth Core Specification Vol 4, Part E, §7.7.65.13
    struct LeExtendedAdvertisingReport<'a>(13) {
        reports: LeExtAdvReports<'a>
    }

    /// Bluetooth Core Specification Vol 4, Part E, §7.7.65.14
    struct LePeriodicAdvertisingSyncEstablished(14) {
        status: Status,
        sync_handle: SyncHandle,
        adv_sid: u8,
        adv_addr_kind: AddrKind,
        adv_addr: BdAddr,
        adv_phy: PhyKind,
        periodic_adv_interval: Duration<1_250>,
        adv_clock_accuracy: ClockAccuracy,
    }

    /// Bluetooth Core Specification Vol 4, Part E, §7.7.65.15
    struct LePeriodicAdvertisingReport<'a>(15) {
        sync_handle: SyncHandle,
        tx_power: i8,
        rssi: i8,
        cte_kind: CteKind,
        data_status: DataStatus,
        data: &'a [u8],
    }

    /// Bluetooth Core Specification Vol 4, Part E, §7.7.65.16
    struct LePeriodicAdvertisingSyncLost(16) {
        sync_handle: SyncHandle,
    }

    /// Bluetooth Core Specification Vol 4, Part E, §7.7.65.17
    struct LeScanTimeout(17) {}

    /// Bluetooth Core Specification Vol 4, Part E, §7.7.65.18
    struct LeAdvertisingSetTerminated(18) {
        status: Status,
        adv_handle: AdvHandle,
        handle: ConnHandle,
        num_completed_ext_adv_evts: u8,
    }

    /// Bluetooth Core Specification Vol 4, Part E, §7.7.65.19
    struct LeScanRequestReceived(19) {
        adv_handle: AdvHandle,
        scanner_addr_kind: AddrKind,
        scanner_addr: BdAddr,
    }

    /// Bluetooth Core Specification Vol 4, Part E, §7.7.65.20
    struct LeChannelSelectionAlgorithm(20) {
        handle: ConnHandle,
        channel_selection_algorithm: u8,
    }

    /// Bluetooth Core Specification Vol 4, Part E, §7.7.65.21
    struct LeConnectionlessIqReport<'a>(21) {
        sync_handle: SyncHandle,
        channel_index: u8,
        rssi: i16,
        rssi_antenna_id: u8,
        cte_kind: CteKind,
        slot_durations: u8,
        packet_status: PacketStatus,
        periodic_event_counter: u16,
        iq_samples: &'a [LeIQSample],
    }

    /// Bluetooth Core Specification Vol 4, Part E, §7.7.65.22
    struct LeConnectionIqReport<'a>(22) {
        handle: ConnHandle,
        rx_phy: PhyKind,
        data_channel_index: u8,
        rssi: i16,
        rssi_antenna_id: u8,
        cte_kind: CteKind,
        slot_durations: u8,
        packet_status: PacketStatus,
        connection_event_counter: u16,
        iq_samples: &'a [LeIQSample],
    }

    /// Bluetooth Core Specification Vol 4, Part E, §7.7.65.23
    struct LeCteRequestFailed(23) {
        status: Status,
        handle: ConnHandle,
    }

    /// Bluetooth Core Specification Vol 4, Part E, §7.7.65.24
    struct LePeriodicAdvertisingSyncTransferReceived(24) {
        status: Status,
        handle: ConnHandle,
        service_data: u16,
        sync_handle: SyncHandle,
        adv_sid: u8,
        adv_addr_kind: AddrKind,
        adv_addr: BdAddr,
        adv_phy: PhyKind,
        periodic_adv_interval: Duration<1_250>,
        adv_clock_accuracy: ClockAccuracy,
    }

    /// Bluetooth Core Specification Vol 4, Part E, §7.7.65.25
    struct LeCisEstablished(25) {
        status: Status,
        handle: ConnHandle,
        cig_sync_delay: IsoDuration,
        cis_sync_delay: IsoDuration,
        transport_latency_c_to_p: IsoDuration,
        transport_latency_p_to_c: IsoDuration,
        phy_c_to_p: PhyKind,
        phy_p_to_c: PhyKind,
        nse: u8,
        bn_c_to_p: u8,
        bn_p_to_c: u8,
        ft_c_to_p: u8,
        ft_p_to_c: u8,
        max_pdu_c_to_p: u16,
        max_pdu_p_to_c: u16,
        iso_interval: Duration<1_250>,
    }

    /// Bluetooth Core Specification Vol 4, Part E, §7.7.65.26
    struct LeCisRequest(26) {
        acl_handle: ConnHandle,
        cis_handle: ConnHandle,
        cig_id: u8,
        cis_id: u8,
    }

    /// Bluetooth Core Specification Vol 4, Part E, §7.7.65.27
    struct LeCreateBigComplete<'a>(27) {
        status: Status,
        big_handle: BigHandle,
        big_sync_delay: IsoDuration,
        transport_latency_big: IsoDuration,
        phy: PhyKind,
        nse: u8,
        bn: u8,
        pto: u8,
        irc: u8,
        max_pdu: u16,
        iso_interval: Duration<1_250>,
        bis_handles: &'a [BisConnHandle],
    }

    /// Bluetooth Core Specification Vol 4, Part E, §7.7.65.28
    struct LeTerminateBigComplete(28) {
        big_handle: BigHandle,
        reason: Status,
    }

    /// Bluetooth Core Specification Vol 4, Part E, §7.7.65.29
    struct LeBigSyncEstablished<'a>(29) {
        status: Status,
        big_handle: BigHandle,
        transport_latency_big: IsoDuration,
        nse: u8,
        bn: u8,
        pto: u8,
        irc: u8,
        max_pdu: u16,
        iso_interval: Duration<1_250>,
        bis_handles: &'a [BisConnHandle],
    }

    /// Bluetooth Core Specification Vol 4, Part E, §7.7.65.30
    struct LeBigSyncLost(30) {
        big_handle: BigHandle,
        reason: Status,
    }

    /// Bluetooth Core Specification Vol 4, Part E, §7.7.65.31
    struct LeRequestPeerScaComplete(31) {
        status: Status,
        handle: ConnHandle,
        peer_clock_accuracy: ClockAccuracy,
    }

    /// Bluetooth Core Specification Vol 4, Part E, §7.7.65.32
    struct LePathLossThreshold(32) {
        handle: ConnHandle,
        current_path_loss: u8,
        zone_entered: ZoneEntered,
    }

    /// Bluetooth Core Specification Vol 4, Part E, §7.7.65.33
    struct LeTransmitPowerReporting(33) {
        status: Status,
        handle: ConnHandle,
        reason: LeTxPowerReportingReason,
        phy: PhyKind,
        tx_power_level: i8,
        tx_power_level_flag: PowerLevelKind,
        delta: i8,
    }

    /// Bluetooth Core Specification Vol 4, Part E, §7.7.65.34
    struct LeBiginfoAdvertisingReport(34) {
        sync_handle: SyncHandle,
        num_bis: u8,
        nse: u8,
        iso_interval: u16,
        bn: u8,
        pto: u8,
        irc: u8,
        max_pdu: u16,
        sdu_interval: IsoDuration,
        max_sdu: u16,
        phy: PhyKind,
        is_framed: bool,
        is_encrypted: bool,
    }

    /// Bluetooth Core Specification Vol 4, Part E, §7.7.65.35
    struct LeSubrateChange(35) {
        status: Status,
        handle: ConnHandle,
        subrate_factor: u16,
        peripheral_latency: u16,
        continuation_number: u16,
        supervision_timeout: Duration<10_000>,
    }
}
