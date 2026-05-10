use super::param;
use crate::FixedSizeValue;

/// Extended LMP features that can be viewed as different page types.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ExtendedLmpFeatures([u8; 8]);

impl ExtendedLmpFeatures {
    /// View as page 0 LMP features.
    pub const fn as_page_0(&self) -> &LmpFeatureMask {
        unsafe { &*(&self.0 as *const [u8; 8] as *const LmpFeatureMask) }
    }

    /// View as page 1 LMP features.
    pub const fn as_page_1(&self) -> &LmpFeatureMaskPage1 {
        unsafe { &*(&self.0 as *const [u8; 8] as *const LmpFeatureMaskPage1) }
    }

    /// View as page 2 LMP features.
    pub const fn as_page_2(&self) -> &LmpFeatureMaskPage2 {
        unsafe { &*(&self.0 as *const [u8; 8] as *const LmpFeatureMaskPage2) }
    }
}

unsafe impl FixedSizeValue for ExtendedLmpFeatures {
    #[inline(always)]
    fn is_valid(_data: &[u8]) -> bool {
        true
    }
}

param! {
    /// [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core_v6.3/out/en/br-edr-controller/link-manager-protocol-specification.html#UUID-c1d8d04b-edcc-8fea-a3f6-f41b520a03de)
    bitfield LmpFeatureMask[8] {
        (0, supports_3_slot_packets, set_3_slot_packets);
        (1, supports_5_slot_packets, set_5_slot_packets);
        (2, supports_encryption, set_encryption);
        (3, supports_slot_offset, set_slot_offset);
        (4, supports_timing_accuracy, set_timing_accuracy);
        (5, supports_role_switch, set_role_switch);
        (6, supports_hold_mode, set_hold_mode);
        (7, supports_sniff_mode, set_sniff_mode);
        (9, supports_power_control_requests, set_power_control_requests);
        (10, supports_cqddr, set_cqddr);
        (11, supports_sco_link, set_sco_link);
        (12, supports_hv2_packets, set_hv2_packets);
        (13, supports_hv3_packets, set_hv3_packets);
        (14, supports_mu_law_log_synchronous_data, set_mu_law_log_synchronous_data);
        (15, supports_a_law_log_synchronous_data, set_a_law_log_synchronous_data);
        (16, supports_cvsd_synchronous_data, set_cvsd_synchronous_data);
        (17, supports_paging_parameter_negotiation, set_paging_parameter_negotiation);
        (18, supports_power_control, set_power_control);
        (19, supports_transparent_synchronous_data, set_transparent_synchronous_data);
        (20, supports_flow_control_lag_lsb, set_flow_control_lag_lsb);
        (21, supports_flow_control_lag_middle_bit, set_flow_control_lag_middle_bit);
        (22, supports_flow_control_lag_msb, set_flow_control_lag_msb);
        (23, supports_broadcast_encryption, set_broadcast_encryption);
        (25, supports_enhanced_data_rate_acl_2mbps_mode, set_enhanced_data_rate_acl_2mbps_mode);
        (26, supports_enhanced_data_rate_acl_3mbps_mode, set_enhanced_data_rate_acl_3mbps_mode);
        (27, supports_enhanced_inquiry_scan, set_enhanced_inquiry_scan);
        (28, supports_interlaced_inquiry_scan, set_interlaced_inquiry_scan);
        (29, supports_interlaced_page_scan, set_interlaced_page_scan);
        (30, supports_rssi_with_inquiry_results, set_rssi_with_inquiry_results);
        (31, supports_ext_sco_link, set_ext_sco_link);
        (32, supports_ev4_packets, set_ev4_packets);
        (33, supports_ev5_packets, set_ev5_packets);
        (35, supports_afh_capable_peripheral, set_afh_capable_peripheral);
        (36, supports_afh_classification_peripheral, set_afh_classification_peripheral);
        (37, supports_br_edr_not, set_br_edr_not);
        (38, supports_le, set_le);
        (39, supports_3_slot_enhanced_data_rate_acl_packets, set_3_slot_enhanced_data_rate_acl_packets);
        (40, supports_5_slot_enhanced_data_rate_acl_packets, set_5_slot_enhanced_data_rate_acl_packets);
        (41, supports_sniff_subrating, set_sniff_subrating);
        (42, supports_pause_encryption, set_pause_encryption);
        (43, supports_afh_capable_central, set_afh_capable_central);
        (44, supports_afh_classification_central, set_afh_classification_central);
        (45, supports_enhanced_data_rate_esco_2mbps_mode, set_enhanced_data_rate_esco_2mbps_mode);
        (46, supports_enhanced_data_rate_esco_3mbps_mode, set_enhanced_data_rate_esco_3mbps_mode);
        (47, supports_3_slot_enhanced_data_rate_esco_packets, set_3_slot_enhanced_data_rate_esco_packets);
        (48, supports_ext_inquiry_response, set_ext_inquiry_response);
        (49, supports_simultaneous_le_and_br_edr_to_same_devi, set_simultaneous_le_and_br_edr_to_same_devi);
        (51, supports_secure_simple_pairing, set_secure_simple_pairing);
        (52, supports_encapsulated_pdu, set_encapsulated_pdu);
        (53, supports_erroneous_data_reporting, set_erroneous_data_reporting);
        (54, supports_non_flushable_packet_boundary_flag, set_non_flushable_packet_boundary_flag);
        (56, supports_hci_link_supervision_timeout_changed_event, set_hci_link_supervision_timeout_changed_event);
        (57, supports_variable_inquiry_tx_power_level, set_variable_inquiry_tx_power_level);
        (58, supports_enhanced_power_control, set_enhanced_power_control);
        (63, supports_ext_features, set_ext_features);
    }
}

param! {
    /// [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core_v6.3/out/en/br-edr-controller/link-manager-protocol-specification.html#UUID-c1d8d04b-edcc-8fea-a3f6-f41b520a03de)
    bitfield LmpFeatureMaskPage1[8] {
        (0, supports_secure_simple_pairing_host, set_secure_simple_pairing_host);
        (1, supports_le_host, set_le_host);
        (3, supports_secure_connections_host, set_secure_connections_host);
    }
}

param! {
    /// [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core_v6.3/out/en/br-edr-controller/link-manager-protocol-specification.html#UUID-c1d8d04b-edcc-8fea-a3f6-f41b520a03de)
    bitfield LmpFeatureMaskPage2[8] {
        (0, supports_connectionless_peripheral_broadcast_transmitter, set_connectionless_peripheral_broadcast_transmitter);
        (1, supports_connectionless_peripheral_broadcast_receiver, set_connectionless_peripheral_broadcast_receiver);
        (2, supports_synchronization_train, set_synchronization_train);
        (3, supports_synchronization_scan, set_synchronization_scan);
        (4, supports_hci_inquiry_response_notification_event, set_hci_inquiry_response_notification_event);
        (5, supports_generalized_interlaced_scan, set_generalized_interlaced_scan);
        (6, supports_coarse_clock_adjustment, set_coarse_clock_adjustment);
        (8, supports_secure_connections_controller, set_secure_connections_controller);
        (9, supports_ping, set_ping);
        (10, supports_slot_availability_mask, set_slot_availability_mask);
        (11, supports_train_nudging, set_train_nudging);
    }
}

param! {
    /// [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core_v6.3/out/en/low-energy-controller/link-layer-specification.html#UUID-56ada5ed-4ae3-acee-198f-27ead57d86f1)
    bitfield LeFeatureMask[8] {
        (0, supports_le_encryption, set_le_encryption);
        (1, supports_conn_parameters_request_procedure, set_conn_parameters_request_procedure);
        (2, supports_ext_reject_indication, set_ext_reject_indication);
        (3, supports_peripheral_initiated_features_exchange, set_peripheral_initiated_features_exchange);
        (4, supports_le_ping, set_le_ping);
        (5, supports_le_data_packet_length_extension, set_le_data_packet_length_extension);
        (6, supports_ll_privacy, set_ll_privacy);
        (7, supports_ext_scanner_filter_policies, set_ext_scanner_filter_policies);
        (8, supports_le_2m_phy, set_le_2m_phy);
        (9, supports_stable_modulation_index_tx, set_stable_modulation_index_tx);
        (10, supports_stable_modulation_index_rx, set_stable_modulation_index_rx);
        (11, supports_le_coded_phy, set_le_coded_phy);
        (12, supports_le_ext_adv, set_le_ext_adv);
        (13, supports_le_periodic_adv, set_le_periodic_adv);
        (14, supports_channel_selection_algorithm_2, set_channel_selection_algorithm_2);
        (15, supports_le_power_class_1, set_le_power_class_1);
        (16, supports_min_used_channels_procedure, set_min_used_channels_procedure);
        (17, supports_conn_cte_request, set_conn_cte_request);
        (18, supports_conn_cte_response, set_conn_cte_response);
        (19, supports_connectionless_cte_tx, set_connectionless_cte_tx);
        (20, supports_connectionless_cte_rx, set_connectionless_cte_rx);
        (21, supports_antenna_switching_during_cte_tx, set_antenna_switching_during_cte_tx);
        (22, supports_antenna_switching_during_cte_rx, set_antenna_switching_during_cte_rx);
        (23, supports_receiving_constant_tone_extensions, set_receiving_constant_tone_extensions);
        (24, supports_periodic_adv_sync_transfer_sender, set_periodic_adv_sync_transfer_sender);
        (25, supports_periodic_adv_sync_transfer_recipient, set_periodic_adv_sync_transfer_recipient);
        (26, supports_sleep_clock_accuracy_updates, set_sleep_clock_accuracy_updates);
        (27, supports_remote_public_key_validation, set_remote_public_key_validation);
        (28, supports_connected_isochronous_stream_central, set_connected_isochronous_stream_central);
        (29, supports_connected_isochronous_stream_peripheral, set_connected_isochronous_stream_peripheral);
        (30, supports_isochronous_broadcaster, set_isochronous_broadcaster);
        (31, supports_synchronized_receiver, set_synchronized_receiver);
        (32, supports_connected_isochronous_stream, set_connected_isochronous_stream);
        (33, supports_le_power_control_request, set_le_power_control_request);
        (35, supports_le_path_loss_monitoring, set_le_path_loss_monitoring);
        (36, supports_periodic_adv_adi, set_periodic_adv_adi);
        (37, supports_conn_subrating, set_conn_subrating);
        (38, supports_conn_subrating_host, set_conn_subrating_host);
        (39, supports_channel_classification, set_channel_classification);
        (40, supports_adv_coding_selection, set_adv_coding_selection);
        (41, supports_adv_coding_selection_host, set_adv_coding_selection_host);
        (43, supports_periodic_adv_with_resp_advertiser, set_periodic_adv_with_resp_advertiser);
        (44, supports_periodic_adv_with_resp_scanner, set_periodic_adv_with_resp_scanner);
        (45, supports_unsegmented_framed_mode, set_unsegmented_framed_mode);
        (46, supports_channel_sounding, set_channel_sounding);
        (47, supports_channel_sounding_host, set_channel_sounding_host);
        (48, supports_channel_sounding_tone_quality_indication, set_channel_sounding_tone_quality_indication);
        (63, supports_ll_extended_feature_set, set_ll_extended_feature_set);
    }
}

param! {
    /// [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core_v6.3/out/en/low-energy-controller/link-layer-specification.html#UUID-56ada5ed-4ae3-acee-198f-27ead57d86f1)
    bitfield LeFeatureMaskPage1[8] {
        (0, supports_monitoring_advertisers, set_monitoring_advertisers);
        (1, supports_frame_space_update, set_frame_space_update);
        (2, supports_utp_ota_mode, set_utp_ota_mode);
        (3, supports_utp_hci_mode, set_utp_hci_mode);
        (4, ll_ota_utp_ind_max_length_bit_68, set_ll_ota_utp_ind_max_length_bit_68);
        (5, ll_ota_utp_ind_max_length_bit_69, set_ll_ota_utp_ind_max_length_bit_69);
        (8, supports_shorter_connection_intervals, set_shorter_connection_intervals);
        (9, supports_shorter_connection_intervals_host, set_shorter_connection_intervals_host);
        (10, supports_le_flushable_acl_data, set_le_flushable_acl_data);
        (11, supports_channel_sounding_enhancement_1, set_channel_sounding_enhancement_1);
    }
}
