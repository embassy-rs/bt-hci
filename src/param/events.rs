use super::param;

param! {
    bitfield EventMask[8] {
        (0, is_inquiry_complete_enabled, enable_inquiry_complete);
        (1, is_inquiry_result_enabled, enable_inquiry_result);
        (2, is_conn_complete_enabled, enable_conn_complete);
        (3, is_conn_request_enabled, enable_conn_request);
        (4, is_disconnection_complete_enabled, enable_disconnection_complete);
        (5, is_authentication_complete_enabled, enable_authentication_complete);
        (6, is_remote_name_request_complete_enabled, enable_remote_name_request_complete);
        (7, is_encryption_change_v1_enabled, enable_encryption_change_v1);
        (8, is_change_conn_link_key_complete_enabled, enable_change_conn_link_key_complete);
        (9, is_link_key_type_changed_enabled, enable_link_key_type_changed);
        (10, supports_read_remote_features_complete_enabled, enable_read_remote_supported_features_complete);
        (11, is_read_remote_version_information_complete_enabled, enable_read_remote_version_information_complete);
        (12, is_qos_setup_complete_enabled, enable_qos_setup_complete);
        (15, is_hardware_error_enabled, enable_hardware_error);
        (16, is_flush_occurred_enabled, enable_flush_occurred);
        (17, is_role_change_enabled, enable_role_change);
        (19, is_mode_change_enabled, enable_mode_change);
        (20, is_return_link_keys_enabled, enable_return_link_keys);
        (21, is_pin_code_request_enabled, enable_pin_code_request);
        (22, is_link_key_request_enabled, enable_link_key_request);
        (23, is_link_key_notification_enabled, enable_link_key_notification);
        (24, is_loopback_cmd_enabled, enable_loopback_cmd);
        (25, is_data_buffer_overflow_enabled, enable_data_buffer_overflow);
        (26, is_max_slots_change_enabled, enable_max_slots_change);
        (27, is_read_clock_offset_complete_enabled, enable_read_clock_offset_complete);
        (28, is_conn_packet_type_changed_enabled, enable_conn_packet_type_changed);
        (29, is_qos_violation_enabled, enable_qos_violation);
        (31, is_page_scan_repetition_mode_change_enabled, enable_page_scan_repetition_mode_change);
        (32, is_flow_specification_complete_enabled, enable_flow_specification_complete);
        (33, is_inquiry_result_with_rssi_enabled, enable_inquiry_result_with_rssi);
        (34, is_read_remote_ext_features_complete_enabled, enable_read_remote_ext_features_complete);
        (43, is_synchronous_conn_complete_enabled, enable_synchronous_conn_complete);
        (44, is_synchronous_conn_changed_enabled, enable_synchronous_conn_changed);
        (45, is_sniff_subrating_enabled, enable_sniff_subrating);
        (46, is_ext_inquiry_result_enabled, enable_ext_inquiry_result);
        (47, is_encryption_key_refresh_complete_enabled, enable_encryption_key_refresh_complete);
        (48, is_io_capability_request_enabled, enable_io_capability_request);
        (49, is_io_capability_response_enabled, enable_io_capability_response);
        (50, is_user_confirmation_request_enabled, enable_user_confirmation_request);
        (51, is_user_passkey_request_enabled, enable_user_passkey_request);
        (52, is_remote_oob_data_request_enabled, enable_remote_oob_data_request);
        (53, is_simple_pairing_complete_enabled, enable_simple_pairing_complete);
        (55, is_link_supervision_timeout_changed_enabled, enable_link_supervision_timeout_changed);
        (56, is_enhanced_flush_complete_enabled, enable_enhanced_flush_complete);
        (58, is_user_passkey_notification_enabled, enable_user_passkey_notification);
        (59, is_keypress_notification_enabled, enable_keypress_notification);
        (60, supports_remote_host_features_notification_enabled, enable_remote_host_supported_features_notification);
        (61, is_le_meta_enabled, enable_le_meta);
    }
}

param! {
    bitfield EventMaskPage2[8] {
        (8, is_number_of_completed_data_blocks_enabled, enable_number_of_completed_data_blocks);
        (14, is_triggered_clock_capture_enabled, enable_triggered_clock_capture);
        (15, is_synchronization_train_complete_enabled, enable_synchronization_train_complete);
        (16, is_synchronization_train_received_enabled, enable_synchronization_train_received);
        (17, is_connectionless_peripheral_broadcast_receive_enabled, enable_connectionless_peripheral_broadcast_receive);
        (18, is_connectionless_peripheral_broadcast_timeout_enabled, enable_connectionless_peripheral_broadcast_timeout);
        (19, is_truncated_page_complete_enabled, enable_truncated_page_complete);
        (20, is_peripheral_page_response_timeout_enabled, enable_peripheral_page_response_timeout);
        (21, is_connectionless_peripheral_broadcast_channel_map_change_enabled, enable_connectionless_peripheral_broadcast_channel_map_change);
        (22, is_inquiry_response_notification_enabled, enable_inquiry_response_notification);
        (23, is_authenticated_payload_timeout_expired_enabled, enable_authenticated_payload_timeout_expired);
        (24, is_sam_status_change_enabled, enable_sam_status_change);
        (25, is_encryption_change_v2_enabled, enable_encryption_change_v2);
    }
}

param! {
    bitfield LeEventMask[8] {
        (0, is_le_conn_complete_enabled, enable_le_conn_complete);
        (1, is_le_adv_report_enabled, enable_le_adv_report);
        (2, is_le_conn_update_complete_enabled, enable_le_conn_update_complete);
        (3, is_le_read_remote_features_complete_enabled, enable_le_read_remote_features_complete);
        (4, is_le_long_term_key_request_enabled, enable_le_long_term_key_request);
        (5, is_le_remote_conn_parameter_request_enabled, enable_le_remote_conn_parameter_request);
        (6, is_le_data_length_change_enabled, enable_le_data_length_change);
        (7, is_le_read_local_p256_public_key_complete_enabled, enable_le_read_local_p256_public_key_complete);
        (8, is_le_generate_dhkey_complete_enabled, enable_le_generate_dhkey_complete);
        (9, is_le_enhanced_conn_complete_enabled, enable_le_enhanced_conn_complete);
        (10, is_le_directed_adv_report_enabled, enable_le_directed_adv_report);
        (11, is_le_phy_update_complete_enabled, enable_le_phy_update_complete);
        (12, is_le_ext_adv_report_enabled, enable_le_ext_adv_report);
        (13, is_le_periodic_adv_sync_established_enabled, enable_le_periodic_adv_sync_established);
        (14, is_le_periodic_adv_report_enabled, enable_le_periodic_adv_report);
        (15, is_le_periodic_adv_sync_lost_enabled, enable_le_periodic_adv_sync_lost);
        (16, is_le_scan_timeout_enabled, enable_le_scan_timeout);
        (17, is_le_adv_set_terminated_enabled, enable_le_adv_set_terminated);
        (18, is_le_scan_request_received_enabled, enable_le_scan_request_received);
        (19, is_le_channel_selection_algorithm_enabled, enable_le_channel_selection_algorithm);
        (20, is_le_connectionless_iq_report_enabled, enable_le_connectionless_iq_report);
        (21, is_le_conn_iq_report_enabled, enable_le_conn_iq_report);
        (22, is_le_cte_request_failed_enabled, enable_le_cte_request_failed);
        (23, is_le_periodic_adv_sync_transfer_received_enabled, enable_le_periodic_adv_sync_transfer_received);
        (24, is_le_cis_established_enabled, enable_le_cis_established);
        (25, is_le_cis_request_enabled, enable_le_cis_request);
        (26, is_le_create_big_complete_enabled, enable_le_create_big_complete);
        (27, is_le_terminate_big_complete_enabled, enable_le_terminate_big_complete);
        (28, is_le_big_sync_established_enabled, enable_le_big_sync_established);
        (29, is_le_big_sync_lost_enabled, enable_le_big_sync_lost);
        (30, is_le_request_peer_sca_complete_enabled, enable_le_request_peer_sca_complete);
        (31, is_le_path_loss_threshold_enabled, enable_le_path_loss_threshold);
        (32, is_le_transmit_power_reporting_enabled, enable_le_transmit_power_reporting);
        (33, is_le_biginfo_adv_report_enabled, enable_le_biginfo_adv_report);
        (34, is_le_subrate_change_enabled, enable_le_subrate_change);
    }
}
