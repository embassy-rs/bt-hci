//! LE Controller commands [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-0f07d2b9-81e3-6508-ee08-8c808e468fed)

use crate::param::{
    AddrKind, AdvChannelMap, AdvEventProps, AdvFilterPolicy, AdvHandle, AdvKind, AdvPhyOptions, AdvSet, AllPhys,
    BdAddr, ChannelMap, ConnHandle, CteKind, CteMask, Duration, ExtDuration, FilterDuplicates, InitiatingPhy,
    LeDataRelatedAddrChangeReasons, LeEventMask, LeFeatureMask, LePeriodicAdvCreateSyncOptions,
    LePeriodicAdvReceiveEnable, LePeriodicAdvSubeventData, LePeriodicAdvSyncTransferMode, LeScanKind, Operation,
    PeriodicAdvProps, PhyKind, PhyMask, PhyOptions, PhyParams, PrivacyMode, ScanningFilterPolicy, ScanningPhy,
    SwitchingSamplingRates, SyncHandle,
};
use crate::{cmd, WriteHci};

cmd! {
    /// LE Set Event Mask command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-8d6890a5-79b9-ba8a-2079-4efa3128263c)
    LeSetEventMask(LE, 0x0001) {
        Params = LeEventMask;
        Return = ();
    }
}

cmd! {
    /// LE Read Buffer Size command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-1446fd8f-af42-e54c-890d-3cc275ed372f)
    LeReadBufferSize(LE, 0x0002) {
        Params = ();
        LeReadBufferSizeReturn {
            le_acl_data_packet_length: u16,
            total_num_le_acl_data_packets: u8,
        }
    }
}

cmd! {
    /// LE Read Local Supported Features command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-3ad76ac5-3812-cca4-a4f5-f73e96cebcba)
    LeReadLocalSupportedFeatures(LE, 0x0003) {
        Params = ();
        Return = LeFeatureMask;
    }
}

cmd! {
    /// LE Set Random Address command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-89d45457-bdb5-cade-32c5-a27240733659)
    LeSetRandomAddr(LE, 0x0005) {
        Params = BdAddr;
        Return = ();
    }
}

cmd! {
    /// LE Set Advertising Parameters command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-d33351b8-bd92-76f5-47b7-2051fc6e7379)
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
    /// LE Read Advertising Physical Channel Tx Power command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-9b984fde-3e9f-05f3-d698-054ca618bcf3)
    LeReadAdvPhysicalChannelTxPower(LE, 0x0007) {
        Params = ();
        Return = i8;
    }
}

cmd! {
    /// LE Set Advertising Data command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-7b83376f-e321-e7ce-3bf1-2af187798be4)
    LeSetAdvData(LE, 0x0008) {
        LeSetAdvDataParams {
            data_len: u8,
            data: [u8; 31],
        }
        Return = ();
    }
}

cmd! {
    /// LE Set Scan Response Data command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-7f9e3a5b-fa09-5292-b64e-1a5304b6a255)
    LeSetScanResponseData(LE, 0x0009) {
        LeSetScanResponseDataParams {
            data_len: u8,
            data: [u8; 31],
        }
        Return = ();
    }
}

cmd! {
    /// LE Set Advertising Enable command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-79bb990c-0338-9c1e-e615-7be8508c8e12)
    LeSetAdvEnable(LE, 0x000a) {
        Params = bool;
        Return = ();
    }
}

cmd! {
    /// LE Set Scan Parameters command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-28d28698-0fd7-d273-2e31-7a6731617c77)
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
    /// LE Set Scan Enable command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-10327f75-4024-80df-14bc-68fe1e42b9e0)
    LeSetScanEnable(LE, 0x000c) {
        LeSetScanEnableParams {
            enable: bool,
            filter_duplicates: bool,
        }
        Return = ();
    }
}

cmd! {
    /// LE Create Connection command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-dc5080f3-63c3-ac48-23d1-2d35c6393ac2)
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
    /// LE Create Connection Cancel command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-37a5c913-6abe-c7b6-9fc8-0d94c8cd266d)
    LeCreateConnCancel(LE, 0x000e) {
        Params = ();
        Return = ();
    }
}

cmd! {
    /// LE Read Filter Accept List Size command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-453b3a49-0aba-3f5e-ee16-5919bea4903d)
    LeReadFilterAcceptListSize(LE, 0x000f) {
        Params = ();
        Return = u8;
    }
}

cmd! {
    /// LE Clear Filter Accept List command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-85c43f7b-9100-63a2-1e97-4def7f859861)
    LeClearFilterAcceptList(LE, 0x0010) {
        Params = ();
        Return = ();
    }
}

cmd! {
    /// LE Add Device To Filter Accept List command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-80115d56-8927-4ad8-8da7-b9c5ed728c3d)
    LeAddDeviceToFilterAcceptList(LE, 0x0011) {
        LeAddDeviceToFilterAcceptListParams {
            addr_kind: AddrKind,
            addr: BdAddr,
        }
        Return = ();
    }
}

cmd! {
    /// LE Remove Device From Filter Accept List command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-7e2ae96c-2231-3fe0-603e-c4e2d27e7448)
    LeRemoveDeviceFromFilterAcceptList(LE, 0x0012) {
        LeRemoveDeviceFromFilterAcceptListParams {
            addr_kind: AddrKind,
            addr: BdAddr,
        }
        Return = ();
    }
}

cmd! {
    /// LE Connection Update command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-9d9b11a8-7762-8a3a-5204-0b7f27eea504)
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
    /// LE Set Host Channel Classification command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-d76b3c58-aa23-3e1f-d771-fbc347ab29b5)
    LeSetHostChannelClassification(LE, 0x0014) {
        Params = ChannelMap;
        Return = ();
    }
}

cmd! {
    /// LE Read Channel Map command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-adaea68c-13d3-4d3d-dc76-317d6f7f606b)
    LeReadChannelMap(LE, 0x0015) {
        Params = ConnHandle;
        LeReadChannelMapReturn {
            channel_map: ChannelMap,
        }
        Handle = handle: ConnHandle;
    }
}

cmd! {
    /// LE Read Remote Features command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-5cef6d01-c629-c784-03a5-490eb9b0408e)
    LeReadRemoteFeatures(LE, 0x0016) {
        Params = ConnHandle;
    }
}

cmd! {
    /// LE Encrypt command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-a902995a-d073-7d20-bede-01bf748323e7)
    LeEncrypt(LE, 0x0017) {
        LeEncryptParams {
            key: [u8; 16],
            plaintext: [u8; 16],
        }
        Return = [u8; 16];
    }
}

cmd! {
    /// LE Rand command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-71fbbee4-dc3f-4c0e-2170-b7c25e22f18d)
    LeRand(LE, 0x0018) {
        Params = ();
        Return = [u8; 8];
    }
}

cmd! {
    /// LE Enable Encryption command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-d4c636e4-5e0c-39ca-09ec-43b7d229efa0)
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
    /// LE Long Term Key Request Reply command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-e89a5372-a7cd-ae2b-5c8d-e281694793ae)
    LeLongTermKeyRequestReply(LE, 0x001a) {
        LeLongTermKeyRequestReplyParams {
            long_term_key: [u8; 16],
        }
        Return = ConnHandle;
        Handle = handle: ConnHandle;
    }
}

cmd! {
    /// LE Long Term Key Request Negative Reply command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-e64f1aa8-6c23-0d70-c11d-4ac00fb2abe7)
    LeLongTermKeyRequestNegativeReply(LE, 0x001b) {
        Params = ConnHandle;
        Return = ConnHandle;
        Handle = ConnHandle;
    }
}

cmd! {
    /// LE Read Supported States command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-69afd9a8-4d1a-18fe-ccd0-adf5080328f5)
    LeReadSupportedStates(LE, 0x001c) {
        Params = ();
        Return = [u8; 8];
    }
}

cmd! {
    /// LE Test End command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-cafabb90-1522-7ad5-8def-5d6dc5a8dabd)
    LeTestEnd(LE, 0x001f) {
        Params = ();
        Return = u16;
    }
}

cmd! {
    /// LE Set Data Length command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-242f8446-8cd1-8293-a341-b09354bae550)
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
    /// LE Read Suggested Default Data Length command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-63da6c78-1392-4471-9a11-a31567c10655)
    LeReadSuggestedDefaultDataLength(LE, 0x0023) {
        Params = ();
        LeReadSuggestedDefaultDataLengthReturn {
            suggested_max_tx_octets: u16,
            suggested_max_tx_time: u16,
        }
    }
}

cmd! {
    /// LE Write Suggested Default Data Length command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-ab836cdb-5055-0155-37fe-e480ac02bd20)
    LeWriteSuggestedDefaultDataLength(LE, 0x0024) {
        LeWriteSuggestedDefaultDataLengthParams {
            suggested_max_tx_octets: u16,
            suggested_max_tx_time: u16,
        }
        Return = ();
    }
}

cmd! {
    /// LE Add Device To Resolving List command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-d9a635b2-e7bc-359b-2e6a-5e8b45b38df3)
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
    /// LE Remove Device From Resolving List command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-4e58626c-6af2-3797-18f1-7ccf643377d8)
    LeRemoveDeviceFromResolvingList(LE, 0x0028) {
        LeRemoveDeviceFromResolvingListParams {
            peer_id_addr_kind: AddrKind,
            peer_id_addr: BdAddr,
        }
        Return = ();
    }
}

cmd! {
    /// LE Clear Resolving List command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-7db0d65f-6851-c005-634b-6e7c31835a2e)
    LeClearResolvingList(LE, 0x0029) {
        Params = ();
        Return = ();
    }
}

cmd! {
    /// LE Read Resolving List Size command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-dc9dd4cf-11ac-ed84-5101-d7c80ba06f2e)
    LeReadResolvingListSize(LE, 0x002a) {
        Params = ();
        Return = u8;
    }
}

cmd! {
    /// LE Set Address Resolution Enable command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-ff994d82-39b7-a0a3-dab9-d145d42a35bf)
    LeSetAddrResolutionEnable(LE, 0x002d) {
        Params = bool;
        Return = ();
    }
}

cmd! {
    /// LE Set Resolvable Private Address Timeout command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-f539d523-751b-6ffa-fa0c-e9633f0d7cc9)
    LeSetResolvablePrivateAddrTimeout(LE, 0x002e) {
        Params = Duration<1_000_000>;
        Return = ();
    }
}

cmd! {
    /// LE Read Maximum Data Length command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-467bd8e9-d61a-7a76-593f-02cfe246d126)
    LeReadMaxDataLength(LE, 0x002f) {
        Params = ();
        LeReadMaxDataLengthReturn {
            supported_max_tx_octets: u16,
            supported_max_tx_time: u16,
            supported_max_rx_octets: u16,
            supported_max_rx_time: u16,
        }
    }
}

cmd! {
    /// LE Read PHY command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-d5598779-e01d-73fc-143f-749484029046)
    LeReadPhy(LE, 0x0030) {
        Params = ConnHandle;
        LeReadPhyReturn {
            tx_phy: PhyKind,
            rx_phy: PhyKind,
        }
        Handle = handle: ConnHandle;
    }
}

cmd! {
    /// LE Set Default PHY command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-f0d3d393-85fa-9083-d9c0-589b3800a153)
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
    /// LE Set PHY command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-8b2521a7-2192-15c2-1815-bcd8fa11da15)
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
    /// LE Set Advertising Set Random Address command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-7c76cb49-d60d-1b5a-09ba-e3637d96d41d)
    LeSetAdvSetRandomAddr(LE, 0x0035) {
        LeSetAdvSetRandomAddrParams {
            adv_handle: AdvHandle,
            random_addr: BdAddr,
        }
        Return = ();
    }
}

cmd! {
    /// LE Set Extended Advertising Parameters command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-2d5f3e1f-6666-baa9-dcc2-5d8af3709dac)
    LeSetExtAdvParams(LE, 0x0036) {
        LeSetExtAdvParamsParams {
                adv_handle: AdvHandle,
                adv_event_props: AdvEventProps,
                primary_adv_interval_min: ExtDuration<625>,
                primary_adv_interval_max: ExtDuration<625>,
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
    /// LE Set Extended Advertising Parameters command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-2d5f3e1f-6666-baa9-dcc2-5d8af3709dac)
    LeSetExtAdvParamsV2(LE, 0x007F) {
        LeSetExtAdvParamsV2Params {
                adv_handle: AdvHandle,
                adv_event_props: AdvEventProps,
                primary_adv_interval_min: ExtDuration<625>,
                primary_adv_interval_max: ExtDuration<625>,
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
                primary_adv_phy_options: AdvPhyOptions,
                secondary_adv_phy_options: AdvPhyOptions,
        }
        Return = i8;
    }
}

cmd! {
    /// LE Set Extended Advertising Data command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-b63f0c30-a14d-ffdf-8a57-73cfe903c539)
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
    /// LE Set Extended Scan Response Data command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-e3dc6edb-4cde-4655-8186-cbde281bcb53)
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
    /// LE Set Extended Advertising Enable command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-d05d4cfe-f0b5-b0e2-1a63-672c960dc088)
    LeSetExtAdvEnable(LE, 0x0039) {
        LeSetExtAdvEnableParams<'a> {
            enable: bool,
            sets: &'a [AdvSet],
        }
        Return = ();
    }
}

cmd! {
    /// LE Read Maximum Advertising Data Length command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-762a3b37-a74d-a5b8-31ea-2d1ad02f257d)
    LeReadMaxAdvDataLength(LE, 0x003a) {
        Params = ();
        Return = u16;
    }
}

cmd! {
    /// LE Read Number of Supported Advertising Sets command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-2b7bf2df-474b-031d-35c5-74623905cfe8)
    LeReadNumberOfSupportedAdvSets(LE, 0x003b) {
        Params = ();
        Return = u8;
    }
}

cmd! {
    /// LE Remove Advertising Set command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-eaf84df1-f597-9d23-067d-29931a2bdac9)
    LeRemoveAdvSet(LE, 0x003c) {
        Params = AdvHandle;
        Return = ();
    }
}

cmd! {
    /// LE Clear Advertising Sets command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-6b002f0a-b5b7-cb02-bc4d-deb80c9bab77)
    LeClearAdvSets(LE, 0x003d) {
        Params = ();
        Return = ();
    }
}

cmd! {
    /// LE Set Periodic Advertising Parameters (v1) command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-e0bc9abb-d57e-8a02-b515-428c7f30e7d2)
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
    /// LE Set Periodic Advertising Parameters (v2) command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-e0bc9abb-d57e-8a02-b515-428c7f30e7d2)
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
    /// LE Set Periodic Advertising Data command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-6154cdf5-c004-4eb1-7bdf-13ecda5bba78)
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
    /// LE Set Periodic Advertising Enable command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-f69a3383-9839-e8f2-5339-a1e9327d5bf8)
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
    /// LE Set Extended Scan Parameters command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-431e2ed0-fe1f-17bd-6e6a-91ff801c6063)
    LeSetExtScanParams(LE, 0x0041) {
        Params = LeSetExtScanParamsParams;
        Return = ();
      }
}

impl LeSetExtScanParams {
    /// Create a new instance of scan parameters.
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

/// Parameters for LE Set Extended Scan Parameters command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-431e2ed0-fe1f-17bd-6e6a-91ff801c6063)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct LeSetExtScanParamsParams {
    /// Kind of address for self.
    pub own_addr_kind: AddrKind,
    /// Scanning filter policy.
    pub scanning_filter_policy: ScanningFilterPolicy,
    /// Phys used when scanning.
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
    /// LE Set Extended Scan Enable command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-bfe8407c-4def-2ded-51dd-e47cf9e8916c)
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
    /// LE Extended Create Connection (v1) command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-1dad213e-f660-2937-c94d-7a3162e94105)
    LeExtCreateConn(LE, 0x0043) {
        Params = LeExtCreateConnParams;
    }
}

impl LeExtCreateConn {
    /// Create a new instance of this command.
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

/// Parameters for LE Extended Create Connection (v1) command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-1dad213e-f660-2937-c94d-7a3162e94105)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct LeExtCreateConnParams {
    /// Should use the initiator filter policy or not.
    pub initiator_filter_policy: bool,
    /// What kind of address for self.
    pub own_addr_kind: AddrKind,
    /// What kind of address for peer.
    pub peer_addr_kind: AddrKind,
    /// The peer address to connect to.
    pub peer_addr: BdAddr,
    /// Which phys used for connecting.
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
    /// LE Extended Create Connection (v2) command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-1dad213e-f660-2937-c94d-7a3162e94105)
    LeExtCreateConnV2(LE, 0x0085) {
        Params = LeExtCreateConnV2Params;
    }
}

impl LeExtCreateConnV2 {
    /// Create a new instance.
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

/// LE Extended Create Connection (v2) command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-1dad213e-f660-2937-c94d-7a3162e94105)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct LeExtCreateConnV2Params {
    /// The advertising handle to use.
    pub adv_handle: AdvHandle,
    /// Use connection sub-events.
    pub subevent: u8,
    /// Should use the initiator filter policy or not.
    pub initiator_filter_policy: bool,
    /// What kind of address for self.
    pub own_addr_kind: AddrKind,
    /// What kind of address for peer.
    pub peer_addr_kind: AddrKind,
    /// The peer address to connect to.
    pub peer_addr: BdAddr,
    /// Which phys used for connecting.
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
    /// LE Periodic Advertising Create Sync command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-29188ef0-bf80-7807-2c96-385e7d9782ed)
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
    /// LE Periodic Advertising Create Sync Cancel command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-203d8b19-9d3f-65b2-82c7-96c6abdc5928)
    LePeriodicAdvCreateSyncCancel(LE, 0x0045) {
        Params = ();
        Return = ();
    }
}

cmd! {
    /// LE Periodic Advertising Terminate Sync command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-c47f3b5c-e6db-f151-3a45-203bfe8b4fb9)
    LePeriodicAdvTerminateSync(LE, 0x0046) {
        Params = SyncHandle;
        Return = ();
    }
}

cmd! {
    /// LE Add Device To Periodic Advertiser List command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-f73da323-a9c4-78cc-669d-6ebf05029416)
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
    /// LE Remove Device From Periodic Advertiser List command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-2f419914-71e3-3886-cecb-c9b5da98b538)
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
    /// LE Clear Periodic Advertiser List command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-049826bb-cdb2-df43-7c18-c5232ad99ef8)
    LeClearPeriodicAdvList(LE, 0x0049) {
        Params = ();
        Return = ();
    }
}

cmd! {
    /// LE Read Periodic Advertiser List Size command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-3212f6f2-a838-872c-9ead-dd81ac4b7a66)
    LeReadPeriodicAdvListSize(LE, 0x004a) {
        Params = ();
        Return = u8;
    }
}

cmd! {
    /// LE Read Transmit Power command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-66ee9869-6f38-c57f-0ab5-b5228ff5302c)
    LeReadTransmitPower(LE, 0x004b) {
        Params = ();
        LeReadTransmitPowerReturn {
            min_tx_power: i8,
            max_tx_power: i8,
        }
    }
}

cmd! {
    /// LE Read RF Path Compensation command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-8bba170c-e42b-8b42-f44b-64a0bc73af1d)
    LeReadRfPathCompensation(LE, 0x004c) {
        Params = ();
        LeReadRfPathCompensationReturn {
            rf_tx_path_compensation_value: i16,
            rf_rx_path_compenstaion_value: i16,
        }
    }
}

cmd! {
    /// LE Write RF Path Compensation command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-85f69948-b4de-6b55-8282-77208c43f3aa)
    LeWriteRfPathCompensation(LE, 0x004d) {
        LeWriteRfPathCompensationParams {
            rf_tx_path_compensation_value: i16,
            rf_rx_path_compensation_value: i16,
        }
        Return = ();
    }
}

cmd! {
    /// LE Set Privacy Mode command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-6a48bb1b-af90-9620-22c8-a4b9297d257c)
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
    /// LE Set Connectionless CTE Transmit Parameters command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-53d42c3c-cefd-aff5-f3f4-a27d02c8d46e)
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
    /// LE Set Connectionless CTE Transmit Enable command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-f816ec0f-6970-7548-4370-b009c693259f)
    LeSetConnectionlessCteTransmitEnable(LE, 0x0052) {
        LeSetConnectionlessCteTransmitEnableParams {
            adv_handle: AdvHandle,
            cte_enable: bool,
        }
        Return = ();
    }
}

cmd! {
    /// LE Set Connection CTE Transmit Parameters command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-a8cd66e1-b702-2d8e-c027-4f0a89d4f8a1)
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
    /// LE Connection CTE Response Enable command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-cbb39f47-1d2f-e229-ff9e-d716111b38a8)
    LeConnCteResponseEnable(LE, 0x0057) {
        LeConnCteResponseEnableParams {
            enable: bool,
        }
        Return = ConnHandle;
        Handle = handle: ConnHandle;
    }
}

cmd! {
    /// LE Read Antenna Information command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-7a80c226-cea1-de43-ff2b-2503b2e7f91e)
    LeReadAntennaInformation(LE, 0x0058) {
        Params = ();
        LeReadAntennaInformationReturn {
            supported_switching_sampling_rates: SwitchingSamplingRates,
            num_antennae: u8,
            max_switching_pattern_len: u8,
            max_cte_len: u8,
        }
    }
}

cmd! {
    /// LE Set Periodic Advertising Receive Enable command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-b055b724-7607-bf63-3862-3e164dfc2251)
    LeSetPeriodicAdvReceiveEnable(LE, 0x0059) {
        LeSetPeriodicAdvReceiveEnableParams {
            sync_handle: SyncHandle,
            enable: LePeriodicAdvReceiveEnable,
        }
        Return = ();
    }
}

cmd! {
    /// LE Periodic Advertising Sync Transfer command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-28bd445b-503e-6c1e-7d80-7bf4c8bd5e8d)
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
    /// LE Periodic Advertising Set Info Transfer command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-b1707675-34fd-4375-7c1c-354098a52db6)
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
    /// LE Set Periodic Advertising Sync Transfer Parameters command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-5ae627f7-febe-a209-e4fb-275a35572ae7)
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
    /// LE Set Default Periodic Advertising Sync Transfer Parameters command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-5c2a7c01-0658-1b94-ae09-16a436c9ef4f)
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
    /// LE Request Peer SCA command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-4960a916-5311-968d-b432-8537b2dd12ed)
    LeRequestPeerSca(LE, 0x006d) {
        Params = ConnHandle;
    }
}

cmd! {
    /// LE Set Host Feature command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-873282cd-6e49-e9aa-cf6f-02fb4c0ea924)
    LeSetHostFeature(LE, 0x0074) {
        LeSetHostFeatureParams {
            bit_number: u8,
            bit_value: u8,
        }
        Return = ();
    }
}

cmd! {
    /// LE Enhanced Read Transmit Power Level command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-9460c908-2c3d-5915-e04b-25ce98dda7a8)
    LeEnhancedReadTransmitPowerLevel(LE, 0x0076) {
        LeEnhancedReadTransmitPowerLevelParams {
            phy: PhyKind,
        }
        LeEnhancedReadTransmitPowerLevelReturn {
            phy: PhyKind,
            current_tx_power_level: i8,
            max_tx_power_level: i8,
        }
        Handle = handle: ConnHandle;
    }
}

cmd! {
    /// LE Read Remote Transmit Power Level command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-e0399448-0f93-7fcf-73ed-43171e6627ea)
    LeReadRemoteTransmitPowerLevel(LE, 0x0077) {
        LeReadRemoteTransmitPowerLevelParams {
            handle: ConnHandle,
            phy: PhyKind,
        }
    }
}

cmd! {
    /// LE Set Path Loss Reporting Parameters command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-40bf69ec-6834-33a5-2978-4bec08e281f2)
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
    /// LE Set Path Loss Reporting Enable command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-fd9496a1-33de-7228-a0c1-44b9024daae1)
    LeSetPathLossReportingEnable(LE, 0x0079) {
        LeSetPathLossReportingEnableParams {
            enable: bool,
        }
        Return = ConnHandle;
        Handle = handle: ConnHandle;
    }
}

cmd! {
    /// LE Set Transmit Power Reporting Enable command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-cdc834a7-3f30-7329-01d8-44a457aae980)
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
    /// LE Set Data Related Address Changes command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-cd6cc02d-3496-3d16-22cb-37f39c2df763)
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
    /// LE Set Periodic Advertising Subevent Data command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-927cb8c3-4a12-6154-d2f2-384f4a10f0a4)
    LeSetPeriodicAdvSubeventData(LE, 0x0082) {
        Params<'a> = LeSetPeriodicAdvSubeventDataParams<'a, 'a>;
        Return = AdvHandle;
    }
}

impl<'a> LeSetPeriodicAdvSubeventData<'a> {
    /// Create a new instance.
    pub fn new(adv_handle: AdvHandle, subevent: &'a [LePeriodicAdvSubeventData<'a>]) -> Self {
        Self(LeSetPeriodicAdvSubeventDataParams { adv_handle, subevent })
    }
}

/// Parameters for LE Set Periodic Advertising Subevent Data command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-927cb8c3-4a12-6154-d2f2-384f4a10f0a4)
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct LeSetPeriodicAdvSubeventDataParams<'a, 'b> {
    /// Which advertising handle to use.
    pub adv_handle: AdvHandle,
    /// List of sub events used.
    pub subevent: &'b [LePeriodicAdvSubeventData<'a>],
}

impl WriteHci for LeSetPeriodicAdvSubeventDataParams<'_, '_> {
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
    /// LE Set Periodic Advertising Response Data command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-8ada75aa-8e1f-c742-6441-8dd1164fc646)
    LeSetPeriodicAdvResponseData(LE, 0x0083) {
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
    /// LE Set Periodic Sync Subevent command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-e3b9bdd9-b435-9adf-2516-ad94e14ece70)
    LeSetPeriodicSyncSubevent(LE, 0x0084) {
        LeSetPeriodicSyncSubeventParams<'a> {
            adv_handle: SyncHandle,
            periodic_adv_props: PeriodicAdvProps,
            subevents: &'a [u8],
        }
        Return = SyncHandle;
    }
}

cmd! {
    /// LE Set Host Feature V2 command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-c8e1603a-50b7-6ba6-2867-d9c78fd5c89d)
    LeSetHostFeatureV2(LE, 0x0097) {
        LeSetHostFeatureV2Params {
            bit_number: u16,
            bit_value: u8,
        }
        Return = ();
    }
}
