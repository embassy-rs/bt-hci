//! LE Meta events [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-9bfbd351-a103-f197-b85f-ffd9dcc92872)

use crate::param::{
    AddrKind, AdvHandle, BdAddr, BigHandle, BisConnHandle, ClockAccuracy, ConnHandle, CteKind, DataStatus, Duration,
    ExtDuration, LeAdvReports, LeConnRole, LeDirectedAdvertisingReportParam, LeExtAdvReports, LeFeatureMask,
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
        /// LE Meta event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-9bfbd351-a103-f197-b85f-ffd9dcc92872)
        #[non_exhaustive]
        #[derive(Debug, Clone, Hash)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum LeEvent<'a> {
            $(
                #[doc = stringify!($name)]
                $name($name$(<$life>)?),
            )+
        }

        /// LE Meta event type [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-9bfbd351-a103-f197-b85f-ffd9dcc92872)
        #[non_exhaustive]
        #[derive(Debug, Clone, Copy, Hash, PartialEq)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub struct LeEventKind(pub u8);

        #[allow(non_upper_case_globals)]
        impl LeEventKind {
            $(
                #[doc = stringify!($name)]
                pub const $name: LeEventKind = LeEventKind($code);
            )+
        }

        impl<'a> $crate::FromHciBytes<'a> for LeEventKind {
            fn from_hci_bytes(data: &'a [u8]) -> Result<(Self, &'a [u8]), FromHciBytesError> {
                let (subcode, data) = data.split_first().ok_or(FromHciBytesError::InvalidSize)?;
                match subcode {
                    $($code => Ok((Self::$name, data)),)+
                    _ => Err(FromHciBytesError::InvalidValue),
                }
            }
        }

        /// An Le Event HCI packet
        #[derive(Debug, Clone, Hash)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub struct LeEventPacket<'a> {
            /// Which kind of le event.
            pub kind: LeEventKind,
            /// Le event data.
            pub data: &'a [u8],
        }

        impl<'a> LeEventPacket<'a> {
            fn from_kind_hci_bytes(kind: LeEventKind, data: &'a [u8]) -> Result<Self, FromHciBytesError> {
                Ok(Self {
                    kind,
                    data,
                })
            }
        }

        impl<'a> TryFrom<LeEventPacket<'a>> for LeEvent<'a> {
            type Error = FromHciBytesError;
            fn try_from(packet: LeEventPacket<'a>) -> Result<Self, Self::Error> {
                match packet.kind {
                    $(LeEventKind::$name => Ok(Self::$name($name::from_hci_bytes_complete(packet.data)?)),)+
                    _ => Err(FromHciBytesError::InvalidValue),
                }
            }
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
                    #[doc = stringify!($field)]
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

impl<'de> FromHciBytes<'de> for LeEventPacket<'de> {
    fn from_hci_bytes(data: &'de [u8]) -> Result<(Self, &'de [u8]), FromHciBytesError> {
        let (kind, data) = LeEventKind::from_hci_bytes(data)?;
        let pkt = Self::from_kind_hci_bytes(kind, data)?;
        Ok((pkt, &[]))
    }
}

le_events! {
    /// LE Connection Complete event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-eee7884c-c6d1-159a-9fc2-aebf6bcf30e7)
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

    /// LE Advertising Report event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-bf6970a8-7187-7d2c-0408-b83aa09837e3)
    struct LeAdvertisingReport<'a>(2) {
        reports: LeAdvReports<'a>,
    }

    /// LE Connection Update Complete event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-5a3debb8-e7b3-44a4-9c42-317bc40ee1d2)
    struct LeConnectionUpdateComplete(3) {
        status: Status,
        handle: ConnHandle,
        conn_interval: Duration<1_250>,
        peripheral_latency: u16,
        supervision_timeout: Duration<10_000>,
    }

    /// LE Read Remote Features Complete event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-a33d9f3f-718b-016c-a89c-8a77c5589ba9)
    struct LeReadRemoteFeaturesComplete(4) {
        status: Status,
        handle: ConnHandle,
        le_features: LeFeatureMask,
    }

    /// LE Long Term Key Request event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-2bb7b9d8-d02b-0320-3dc8-9699e4b30332)
    struct LeLongTermKeyRequest(5) {
        handle: ConnHandle,
        random_number: [u8; 8],
        encrypted_diversifier: u16,
    }

    /// LE Remote Connection Parameter Request event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-98e6a66c-1561-7bd0-d4c6-79bff2ba7652)
    struct LeRemoteConnectionParameterRequest(6) {
        handle: ConnHandle,
        interval_min: Duration<1_250>,
        interval_max: Duration<1_250>,
        max_latency: u16,
        timeout: Duration<10_000>,
    }

    /// LE Data Length Change event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-631c5539-4155-072a-af53-a226e0bfb96a)
    struct LeDataLengthChange(7) {
        handle: ConnHandle,
        max_tx_octets: u16,
        max_tx_time: u16,
        max_rx_octets: u16,
        max_rx_time: u16,
    }

    /// LE Read Local P-256 Public Key Complete event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-b2210bd7-74d1-949a-091c-008440a8625f)
    struct LeReadLocalP256PublicKeyComplete(8) {
        status: Status,
        key_x_coordinate: [u8; 32],
        key_y_coordinate: [u8; 32],
    }

    /// LE Generate DHKey Complete event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-a5b34696-b6fa-ec1a-31ae-8a09db157322)
    struct LeGenerateDhkeyComplete(9) {
        status: Status,
        dh_key: [u8; 32],
    }

    /// LE Enhanced Connection Complete event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-ed5dc708-ff96-949f-586a-4d418466b226)
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

    /// LE Directed Advertising Report event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-3e0a8a2e-a30f-df11-7490-132e35ee5daa)
    struct LeDirectedAdvertisingReport<'a>(11) {
        reports: &'a [LeDirectedAdvertisingReportParam],
    }

    /// LE PHY Update Complete event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-1f6a363e-bd01-bbf7-be6f-fb86ffa645ec)
    struct LePhyUpdateComplete(12) {
        status: Status,
        handle: ConnHandle,
        tx_phy: PhyKind,
        rx_phy: PhyKind,
    }

    /// LE Extended Advertising Report event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-37c674d6-c93f-c46c-420a-b2569dff4fa0)
    struct LeExtendedAdvertisingReport<'a>(13) {
        reports: LeExtAdvReports<'a>
    }

    /// LE Periodic Advertising Sync Established event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-1f76b4e2-f279-1976-1e33-5ba86d0955c2)
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

    /// LE Periodic Advertising Report event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-e216c26f-0383-c651-4b8d-1409b91c7e34)
    struct LePeriodicAdvertisingReport<'a>(15) {
        sync_handle: SyncHandle,
        tx_power: i8,
        rssi: i8,
        cte_kind: CteKind,
        data_status: DataStatus,
        data: &'a [u8],
    }

    /// LE Periodic Advertising Sync Lost event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-a1a90403-d1b7-d117-0f30-620c8d3912ef)
    struct LePeriodicAdvertisingSyncLost(16) {
        sync_handle: SyncHandle,
    }

    /// LE Scan Timeout event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-fd746670-5772-b038-ad51-135d8371fb00)
    struct LeScanTimeout(17) {}

    /// LE Advertising Set Terminated event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-b7eedd85-4369-f88f-7872-7278f7778cd2)
    struct LeAdvertisingSetTerminated(18) {
        status: Status,
        adv_handle: AdvHandle,
        handle: ConnHandle,
        num_completed_ext_adv_evts: u8,
    }

    /// LE Scan Request Received event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-10c1448e-03ab-9ce7-8d90-7ad870c8da20)
    struct LeScanRequestReceived(19) {
        adv_handle: AdvHandle,
        scanner_addr_kind: AddrKind,
        scanner_addr: BdAddr,
    }

    /// LE Channel Selection Algorithm event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-8ccdba3a-0523-031c-bb51-7d4f2c2e1191)
    struct LeChannelSelectionAlgorithm(20) {
        handle: ConnHandle,
        channel_selection_algorithm: u8,
    }

    /// LE Connectionless IQ Report event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-fafa104b-6cb3-ef73-38bd-37539d44e865)
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

    /// LE Connection IQ Report event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-b38e7527-8dc5-ea71-c7ce-892e2362e338)
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

    /// LE CTE Request Failed event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-f546f56f-fff2-6ba2-1b86-77dbcf8fc012)
    struct LeCteRequestFailed(23) {
        status: Status,
        handle: ConnHandle,
    }

    /// LE Periodic Advertising Sync Transfer Received event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-6feaf258-f0a9-7db2-d837-5bd8c07ef396)
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

    /// LE CIS Established event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-3c346948-9111-a11b-fc1d-6249936d559a)
    struct LeCisEstablished(25) {
        status: Status,
        handle: ConnHandle,
        cig_sync_delay: ExtDuration,
        cis_sync_delay: ExtDuration,
        transport_latency_c_to_p: ExtDuration,
        transport_latency_p_to_c: ExtDuration,
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

    /// LE CIS Request event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-0e5babb6-41aa-4d16-41cb-062d2d7e51dd)
    struct LeCisRequest(26) {
        acl_handle: ConnHandle,
        cis_handle: ConnHandle,
        cig_id: u8,
        cis_id: u8,
    }

    /// LE Create BIG Complete event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-8ca85f3c-5223-d2b4-2b8d-58bb15d58c67)
    struct LeCreateBigComplete<'a>(27) {
        status: Status,
        big_handle: BigHandle,
        big_sync_delay: ExtDuration,
        transport_latency_big: ExtDuration,
        phy: PhyKind,
        nse: u8,
        bn: u8,
        pto: u8,
        irc: u8,
        max_pdu: u16,
        iso_interval: Duration<1_250>,
        bis_handles: &'a [BisConnHandle],
    }

    /// LE Terminate BIG Complete event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-4f2ef51e-85af-5f2a-fda4-822eb32bc6dd)
    struct LeTerminateBigComplete(28) {
        big_handle: BigHandle,
        reason: Status,
    }

    /// LE BIG Sync Established event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-5d591ea2-a469-5df9-bfd1-e7bf9f0d0e59)
    struct LeBigSyncEstablished<'a>(29) {
        status: Status,
        big_handle: BigHandle,
        transport_latency_big: ExtDuration,
        nse: u8,
        bn: u8,
        pto: u8,
        irc: u8,
        max_pdu: u16,
        iso_interval: Duration<1_250>,
        bis_handles: &'a [BisConnHandle],
    }

    /// LE BIG Sync Lost event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-1d70bc8d-a600-7f3e-a35e-4d198005b683)
    struct LeBigSyncLost(30) {
        big_handle: BigHandle,
        reason: Status,
    }

    /// LE Request Peer SCA Complete event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-c03d0f69-243c-20d1-c62d-eacbec5c6769)
    struct LeRequestPeerScaComplete(31) {
        status: Status,
        handle: ConnHandle,
        peer_clock_accuracy: ClockAccuracy,
    }

    /// LE Path Loss Threshold event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-ddc59a86-983e-07ab-8e1e-7a18a033f2da)
    struct LePathLossThreshold(32) {
        handle: ConnHandle,
        current_path_loss: u8,
        zone_entered: ZoneEntered,
    }

    /// LE Transmit Power Reporting event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-56658b09-da26-d33e-e960-35a474cda8b4)
    struct LeTransmitPowerReporting(33) {
        status: Status,
        handle: ConnHandle,
        reason: LeTxPowerReportingReason,
        phy: PhyKind,
        tx_power_level: i8,
        tx_power_level_flag: PowerLevelKind,
        delta: i8,
    }

    /// LE BIGInfo Advertising Report event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-9b86e844-492e-a003-21cb-ff92c6aedf06)
    struct LeBiginfoAdvertisingReport(34) {
        sync_handle: SyncHandle,
        num_bis: u8,
        nse: u8,
        iso_interval: u16,
        bn: u8,
        pto: u8,
        irc: u8,
        max_pdu: u16,
        sdu_interval: ExtDuration,
        max_sdu: u16,
        phy: PhyKind,
        is_framed: bool,
        is_encrypted: bool,
    }

    /// LE Subrate Change event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-dd0459e3-1dd8-6cf1-c591-307412647335)
    struct LeSubrateChange(35) {
        status: Status,
        handle: ConnHandle,
        subrate_factor: u16,
        peripheral_latency: u16,
        continuation_number: u16,
        supervision_timeout: Duration<10_000>,
    }
}
