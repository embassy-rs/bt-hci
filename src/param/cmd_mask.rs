use super::param;

param! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct CmdMask([u8; 64]);
}

impl Default for CmdMask {
    fn default() -> Self {
        Self([0; 64])
    }
}

macro_rules! commands {
    (
        $(
            $octet:expr => {
                $(
                    ($bit:expr, $getter:ident);
                )+
            }
        )+
    ) => {
        impl CmdMask {
            $(
                $(
                    pub fn $getter(&self) -> bool {
                        (self.0[$octet] & (1 << $bit)) != 0
                    }
                )+
            )+
        }
    }
}

commands! {
    0 => {
        (0, inquiry);
        (1, inquiry_cancel);
        (2, periodic_inquiry_mode);
        (3, exit_periodic_inquiry_mode);
        (4, create_conn);
        (5, disconnect);
        (7, create_conn_cancel);
    }
    1 => {
        (0, accept_conn_request);
        (1, reject_conn_request);
        (2, link_key_request_reply);
        (3, link_key_request_negative_reply);
        (4, pin_code_request_reply);
        (5, pin_code_request_negative_reply);
        (6, change_conn_packet_kind);
        (7, authentication_requested);
    }
    2 => {
        (0, set_conn_encryption);
        (1, change_conn_link_key);
        (2, link_key_selection);
        (3, remote_name_request);
        (4, remote_name_request_cancel);
        (5, read_remote_supported_features);
        (6, read_remote_ext_features);
        (7, read_remote_version_information);
    }
    3 => {
        (0, read_clock_offset);
        (1, read_lmp_handle);
    }
    4 => {
        (1, hold_mode);
        (2, sniff_mode);
        (3, exit_sniff_mode);
        (6, qos_setup);
        (7, role_discovery);
    }
    5 => {
        (0, switch_role);
        (1, read_link_policy_settings);
        (2, write_link_policy_settings);
        (3, read_default_link_policy_settings);
        (4, write_default_link_policy_settings);
        (5, flow_specification);
        (6, set_event_mask);
        (7, reset);
    }
    6 => {
        (0, set_event_filter);
        (1, flush);
        (2, read_pin_kind);
        (3, write_pin_kind);
        (5, read_stored_link_key);
        (6, write_stored_link_key);
        (7, delete_stored_link_key);
    }
    7 => {
        (0, write_local_name);
        (1, read_local_name);
        (2, read_conn_accept_timeout);
        (3, write_conn_accept_timeout);
        (4, read_page_timeout);
        (5, write_page_timeout);
        (6, read_scan_enable);
        (7, write_scan_enable);
    }
    8 => {
        (0, read_page_scan_activity);
        (1, write_page_scan_activity);
        (2, read_inquiry_scan_activity);
        (3, write_inquiry_scan_activity);
        (4, read_authentication_enable);
        (5, write_authentication_enable);
    }
    9 => {
        (0, read_class_of_device);
        (1, write_class_of_device);
        (2, read_voice_setting);
        (3, write_voice_setting);
        (4, read_automatic_flush_timeout);
        (5, write_automatic_flush_timeout);
        (6, read_num_broadcast_retransmissions);
        (7, write_num_broadcast_retransmissions);
    }
    10 => {
        (0, read_hold_mode_activity);
        (1, write_hold_mode_activity);
        (2, read_transmit_power_level);
        (3, read_synchronous_flow_control_enable);
        (4, write_synchronous_flow_control_enable);
        (5, set_controller_to_host_flow_control);
        (6, host_buffer_size);
        (7, host_number_of_completed_packets);
    }
    11 => {
        (0, read_link_supervision_timeout);
        (1, write_link_supervision_timeout);
        (2, read_number_of_supported_iac);
        (3, read_current_iac_lap);
        (4, write_current_iac_lap);
    }
    12 => {
        (1, set_afh_host_channel_classification);
        (4, read_inquiry_scan_kind);
        (5, write_inquiry_scan_kind);
        (6, read_inquiry_mode);
        (7, write_inquiry_mode);
    }
    13 => {
        (0, read_page_scan_kind);
        (1, write_page_scan_kind);
        (2, read_afh_channel_assessment_mode);
        (3, write_afh_channel_assessment_mode);
    }
    14 => {
        (3, read_local_version_information);
        (5, read_local_supported_features);
        (6, read_local_ext_features);
        (7, read_buffer_size);
    }
    15 => {
        (1, read_bd_addr);
        (2, read_failed_contact_counter);
        (3, reset_failed_contact_counter);
        (4, read_link_quality);
        (5, read_rssi);
        (6, read_afh_channel_map);
        (7, read_clock);
    }
    16 => {
        (0, read_loopback_mode);
        (1, write_loopback_mode);
        (2, enable_device_under_test_mode);
        (3, setup_synchronous_conn_request);
        (4, accept_synchronous_conn_request);
        (5, reject_synchronous_conn_request);
    }
    17 => {
        (0, read_ext_inquiry_response);
        (1, write_ext_inquiry_response);
        (2, refresh_encryption_key);
        (4, sniff_subrating);
        (5, read_simple_pairing_mode);
        (6, write_simple_pairing_mode);
        (7, read_local_oob_data);
    }
    18 => {
        (0, read_inquiry_response_transmit_power_level);
        (1, write_inquiry_transmit_power_level);
        (2, read_default_erroneous_data_reporting);
        (3, write_default_erroneous_data_reporting);
        (7, io_capability_request_reply);
    }
    19 => {
        (0, user_confirmation_request_reply);
        (1, user_confirmation_request_negative_reply);
        (2, user_passkey_request_reply);
        (3, user_passkey_request_negative_reply);
        (4, remote_oob_data_request_reply);
        (5, write_simple_pairing_debug_mode);
        (6, enhanced_flush);
        (7, remote_oob_data_request_negative_reply);
    }
    20 => {
        (2, send_keypress_notification);
        (3, io_capability_request_negative_reply);
        (4, read_encryption_key_size);
    }
    22 => {
        (2, set_event_mask_page_2);
    }
    23 => {
        (0, read_flow_control_mode);
        (1, write_flow_control_mode);
        (2, read_data_block_size);
    }
    24 => {
        (0, read_enhanced_transmit_power_level);
        (5, read_le_host_support);
        (6, write_le_host_support);
    }
    25 => {
        (0, le_set_event_mask);
        (1, le_read_buffer_size_v1);
        (2, le_read_local_supported_features);
        (4, le_set_random_addr);
        (5, le_set_adv_parameters);
        (6, le_read_adv_physical_channel_tx_power);
        (7, le_set_adv_data);
    }
    26 => {
        (0, le_set_scan_response_data);
        (1, le_set_adv_enable);
        (2, le_set_scan_parameters);
        (3, le_set_scan_enable);
        (4, le_create_conn);
        (5, le_create_conn_cancel);
        (6, le_read_filter_accept_list_size);
        (7, le_clear_filter_accept_list);
    }
    27 => {
        (0, le_add_device_to_filter_accept_list);
        (1, le_remove_device_from_filter_accept_list);
        (2, le_conn_update);
        (3, le_set_host_channel_classification);
        (4, le_read_channel_map);
        (5, le_read_remote_features);
        (6, le_encrypt);
        (7, le_rand);
    }
    28 => {
        (0, le_enable_encryption);
        (1, le_long_term_key_request_reply);
        (2, le_long_term_key_request_negative_reply);
        (3, le_read_supported_states);
        (4, le_receiver_test_v1);
        (5, le_transmitter_test_v1);
        (6, le_test_end);
    }
    29 => {
        (3, enhanced_setup_synchronous_conn);
        (4, enhanced_accept_synchronous_conn);
        (5, read_local_supported_codecs);
        (6, set_mws_channel_parameters);
        (7, set_external_frame_configuration);
    }
    30 => {
        (0, set_mws_signaling);
        (1, set_mws_transport_layer);
        (2, set_mws_scan_frequency_table);
        (3, get_mws_transport_layer_configuration);
        (4, set_mws_pattern_configuration);
        (5, set_triggered_clock_capture);
        (6, truncated_page);
        (7, truncated_page_cancel);
    }
    31 => {
        (0, set_connectionless_peripheral_broadcast);
        (1, set_connectionless_peripheral_broadcast_receive);
        (2, start_synchronization_train);
        (3, receive_synchronization_train);
        (4, set_reserved_lt_addr);
        (5, delete_reserved_lt_addr);
        (6, set_connectionless_peripheral_broadcast_data);
        (7, read_synchronization_train_parameters);
    }
    32 => {
        (0, write_synchronization_train_parameters);
        (1, remote_oob_ext_data_request_reply);
        (2, read_secure_conns_host_support);
        (3, write_secure_conns_host_support);
        (4, read_authenticated_payload_timeout);
        (5, write_authenticated_payload_timeout);
        (6, read_local_oob_ext_data);
        (7, write_secure_conns_test_mode);
    }
    33 => {
        (0, read_ext_page_timeout);
        (1, write_ext_page_timeout);
        (2, read_ext_inquiry_length);
        (3, write_ext_inquiry_length);
        (4, le_remote_conn_parameter_request_reply);
        (5, le_remote_conn_parameter_request_negative_reply);
        (6, le_set_data_length);
        (7, le_read_suggested_default_data_length);
    }
    34 => {
        (0, le_write_suggested_default_data_length);
        (1, le_read_local_p256_public_key);
        (2, le_generate_dhkey_v1);
        (3, le_add_device_to_resolving_list);
        (4, le_remove_device_from_resolving_list);
        (5, le_clear_resolving_list);
        (6, le_read_resolving_list_size);
        (7, le_read_peer_resolvable_addr);
    }
    35 => {
        (0, le_read_local_resolvable_addr);
        (1, le_set_addr_resolution_enable);
        (2, le_set_resolvable_private_addr_timeout);
        (3, le_read_maximum_data_length);
        (4, le_read_phy);
        (5, le_set_default_phy);
        (6, le_set_phy);
        (7, le_receiver_test_v2);
    }
    36 => {
        (0, le_transmitter_test_v2);
        (1, le_set_adv_set_random_addr);
        (2, le_set_ext_adv_parameters);
        (3, le_set_ext_adv_data);
        (4, le_set_ext_scan_response_data);
        (5, le_set_ext_adv_enable);
        (6, le_read_maximum_adv_data_length);
        (7, le_read_number_of_supported_adv_sets);
    }
    37 => {
        (0, le_remove_adv_set);
        (1, le_clear_adv_sets);
        (2, le_set_periodic_adv_parameters);
        (3, le_set_periodic_adv_data);
        (4, le_set_periodic_adv_enable);
        (5, le_set_ext_scan_parameters);
        (6, le_set_ext_scan_enable);
        (7, le_ext_create_conn);
    }
    38 => {
        (0, le_periodic_adv_create_sync);
        (1, le_periodic_adv_create_sync_cancel);
        (2, le_periodic_adv_terminate_sync);
        (3, le_add_device_to_periodic_adv_list);
        (4, le_remove_device_from_periodic_adv_list);
        (5, le_clear_periodic_adv_list);
        (6, le_read_periodic_adv_list_size);
        (7, le_read_transmit_power);
    }
    39 => {
        (0, le_read_rf_path_compensation);
        (1, le_write_rf_path_compensation);
        (2, le_set_privacy_mode);
        (3, le_receiver_test_v3);
        (4, le_transmitter_test_v3);
        (5, le_set_connectionless_cte_transmit_parameters);
        (6, le_set_connectionless_cte_transmit_enable);
        (7, le_set_connectionless_iq_sampling_enable);
    }
    40 => {
        (0, le_set_conn_cte_receive_parameters);
        (1, le_set_conn_cte_transmit_parameters);
        (2, le_conn_cte_request_enable);
        (3, le_conn_cte_response_enable);
        (4, le_read_antenna_information);
        (5, le_set_periodic_adv_receive_enable);
        (6, le_periodic_adv_sync_transfer);
        (7, le_periodic_adv_set_info_transfer);
    }
    41 => {
        (0, le_set_periodic_adv_sync_transfer_parameters);
        (1, le_set_default_periodic_adv_sync_transfer_parameters);
        (2, le_generate_dhkey_v2);
        (3, read_local_simple_pairing_options);
        (4, le_modify_sleep_clock_accuracy);
        (5, le_read_buffer_size_v2);
        (6, le_read_iso_tx_sync);
        (7, le_set_cig_parameters);
    }
    42 => {
        (0, le_set_cig_parameters_test);
        (1, le_create_cis);
        (2, le_remove_cig);
        (3, le_accept_cis_request);
        (4, le_reject_cis_request);
        (5, le_create_big);
        (6, le_create_big_test);
        (7, le_terminate_big);
    }
    43 => {
        (0, le_big_create_sync);
        (1, le_big_terminate_sync);
        (2, le_request_peer_sca);
        (3, le_setup_iso_data_path);
        (4, le_remove_iso_data_path);
        (5, le_iso_transmit_test);
        (6, le_iso_receive_test);
        (7, le_iso_read_test_counters);
    }
    44 => {
        (0, le_iso_test_end);
        (1, le_set_host_feature);
        (2, le_read_iso_link_quality);
        (3, le_enhanced_read_transmit_power_level);
        (4, le_read_remote_transmit_power_level);
        (5, le_set_path_loss_reporting_parameters);
        (6, le_set_path_loss_reporting_enable);
        (7, le_set_transmit_power_reporting_enable);
    }
    45 => {
        (0, le_transmitter_test_v4);
        (1, set_ecosystem_base_interval);
        (2, read_local_supported_codecs_v2);
        (3, read_local_supported_codec_capabilities);
        (4, read_local_supported_controller_delay);
        (5, configure_data_path);
        (6, le_set_data_related_addr_changes);
        (7, set_min_encryption_key_size);
    }
    46 => {
        (0, le_set_default_subrate);
        (1, le_subrate_request);
    }
}
