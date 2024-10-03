// Uses lots of alignment which rustfmt destroys
#![cfg_attr(rustfmt, rustfmt::skip)]

use std::os::raw::*;

use crate::public_definitions::*;
use crate::plugin_definitions::*;

/// Functions exported to plugin from main binary
#[repr(C)]
pub struct Ts3Functions {
	pub get_client_lib_version:                         extern fn(result: *mut *mut c_char) -> c_uint,
	pub get_client_lib_version_number:                  extern fn(result: *mut u64) -> c_uint,
	pub spawn_new_server_connection_handler:            extern fn(port: c_int, result: *mut u64) -> c_uint,
	pub destroy_server_connection_handler:              extern fn(server_connection_handler_id: u64) -> c_uint,

	/// Error handling
	pub get_error_message:                              extern fn(error_code: c_uint, error: *mut *mut c_char) -> c_uint,

	/// Memory management
	pub free_memory:                                    extern fn(pointer: *mut c_void) -> c_uint,

	/// Logging
	pub log_message:                                    extern fn(log_message: *const c_char, severity: LogLevel, channel: *const c_char, log_id: u64) -> c_uint,

	/// Sound
	pub get_playback_device_list:                       extern fn(mode_id: *const c_char, result: *mut *mut *mut *mut c_char) -> c_uint,
	pub get_playback_mode_list:                         extern fn(result: *mut *mut *mut c_char) -> c_uint,
	pub get_capture_device_list:                        extern fn(mode_id: *const c_char, result: *mut *mut *mut *mut c_char) -> c_uint,
	pub get_capture_mode_list:                          extern fn(result: *mut *mut *mut c_char) -> c_uint,
	pub get_default_playback_device:                    extern fn(mode_id: *const c_char, result: *mut *mut *mut c_char) -> c_uint,
	pub get_default_playback_mode:                      extern fn(result: *mut *mut c_char) -> c_uint,
	pub get_default_capture_device:                     extern fn(mode_id: *const c_char, result: *mut *mut *mut c_char) -> c_uint,
	pub get_default_capture_mode:                       extern fn(result: *mut *mut c_char) -> c_uint,
	pub open_playback_device:                           extern fn(server_connection_handler_id: u64, mode_id: *const c_char, playback_device: *const c_char) -> c_uint,
	pub open_capture_device:                            extern fn(server_connection_handler_id: u64, mode_id: *const c_char, capture_device: *const c_char) -> c_uint,
	pub get_current_playback_device_name:               extern fn(server_connection_handler_id: u64, result: *mut *mut c_char, is_default: *mut c_int) -> c_uint,
	pub get_current_play_back_mode:                     extern fn(server_connection_handler_id: u64, result: *mut *mut c_char) -> c_uint,
	pub get_current_capture_device_name:                extern fn(server_connection_handler_id: u64, result: *mut *mut c_char, is_default: *mut c_int) -> c_uint,
	pub get_current_capture_mode:                       extern fn(server_connection_handler_id: u64, result: *mut *mut c_char) -> c_uint,
	pub initiate_graceful_playback_shutdown:            extern fn(server_connection_handler_id: u64) -> c_uint,
	pub close_playback_device:                          extern fn(server_connection_handler_id: u64) -> c_uint,
	pub close_capture_device:                           extern fn(server_connection_handler_id: u64) -> c_uint,
	pub activate_capture_device:                        extern fn(server_connection_handler_id: u64) -> c_uint,
	pub play_wave_file_handle:                          extern fn(server_connection_handler_id: u64, path: *const c_char, play_loop: c_int, wave_handle: *mut u64) -> c_uint,
	pub pause_wave_file_handle:                         extern fn(server_connection_handler_id: u64, wave_handle: u64, pause: c_int) -> c_uint,
	pub close_wave_file_handle:                         extern fn(server_connection_handler_id: u64, wave_handle: u64) -> c_uint,
	pub play_wave_file:                                 extern fn(server_connection_handler_id: u64, path: *const c_char) -> c_uint,
	pub register_custom_device:                         extern fn(device_id: *const c_char, device_display_name: *const c_char, cap_frequency: c_int, cap_channels: c_int, play_frequency: c_int, play_channels: c_int) -> c_uint,
	pub unregister_custom_device:                       extern fn(device_id: *const c_char) -> c_uint,
	pub process_custom_capture_data:                    extern fn(device_name: *const c_char, buffer: *const c_short, samples: c_int) -> c_uint,
	pub acquire_custom_playback_data:                   extern fn(device_name: *const c_char, buffer: *mut c_short, samples: c_int) -> c_uint,

	/// Preprocessor
	pub get_pre_processor_info_value_float:             extern fn(server_connection_handler_id: u64, ident: *const c_char, result: *mut c_float) -> c_uint,
	pub get_pre_processor_config_value:                 extern fn(server_connection_handler_id: u64, ident: *const c_char, result: *mut *mut c_char) -> c_uint,
	pub set_pre_processor_config_value:                 extern fn(server_connection_handler_id: u64, ident: *const c_char, value: *const c_char) -> c_uint,

	/// Encoder
	pub get_encode_config_value:                        extern fn(server_connection_handler_id: u64, ident: *const c_char, result: *mut *mut c_char) -> c_uint,

	/// Playback
	pub get_playback_config_value_as_float:             extern fn(server_connection_handler_id: u64, ident: *const c_char, result: *mut c_float) -> c_uint,
	pub set_playback_config_value:                      extern fn(server_connection_handler_id: u64, ident: *const c_char, value: *const c_char) -> c_uint,
	pub set_client_volume_modifier:                     extern fn(server_connection_handler_id: u64, client_id: u16, value: c_float) -> c_uint,

	/// Recording
	pub start_voice_recording:                          extern fn(server_connection_handler_id: u64) -> c_uint,
	pub stop_voice_recording:                           extern fn(server_connection_handler_id: u64) -> c_uint,

	/// 3D sound positioning
	pub systemset3d_listener_attributes:                extern fn(server_connection_handler_id: u64, position: *const Ts3Vector, forward: *const Ts3Vector, up: *const Ts3Vector) -> c_uint,
	pub set3d_wave_attributes:                          extern fn(server_connection_handler_id: u64, wave_handle: u64, position: *const Ts3Vector) -> c_uint,
	pub systemset3d_settings:                           extern fn(server_connection_handler_id: u64, distance_factor: c_float, rolloff_scale: c_float) -> c_uint,
	pub channelset3d_attributes:                        extern fn(server_connection_handler_id: u64, client_id: u16, position: *const Ts3Vector) -> c_uint,

	/// Interaction with the server
	pub start_connection:                               extern fn(server_connection_handler_id: u64, identity: *const c_char, ip: *const c_char, port: c_uint, nickname: *const c_char, default_channel_array: *const *const c_char, default_channel_password: *const c_char, server_password: *const c_char) -> c_uint,
	pub stop_connection:                                extern fn(server_connection_handler_id: u64, quit_message: *const c_char) -> c_uint,
	pub request_client_move:                            extern fn(server_connection_handler_id: u64, client_id: u16, new_channel_id: u64, password: *const c_char, return_code: *const c_char) -> c_uint,
	pub request_client_variables:                       extern fn(server_connection_handler_id: u64, client_id: u16, return_code: *const c_char) -> c_uint,
	pub request_client_kick_from_channel:               extern fn(server_connection_handler_id: u64, client_id: u16, kick_reason: *const c_char, return_code: *const c_char) -> c_uint,
	pub request_client_kick_from_server:                extern fn(server_connection_handler_id: u64, client_id: u16, kick_reason: *const c_char, return_code: *const c_char) -> c_uint,
	pub request_channel_delete:                         extern fn(server_connection_handler_id: u64, channel_id: u64, force: c_int, return_code: *const c_char) -> c_uint,
	pub request_channel_move:                           extern fn(server_connection_handler_id: u64, channel_id: u64, new_channel_parent_id: u64, new_channel_order: u64, return_code: *const c_char) -> c_uint,
	pub request_send_private_text_msg:                  extern fn(server_connection_handler_id: u64, message: *const c_char, target_client_id: u16, return_code: *const c_char) -> c_uint,
	pub request_send_channel_text_msg:                  extern fn(server_connection_handler_id: u64, message: *const c_char, target_channel_id: u64, return_code: *const c_char) -> c_uint,
	pub request_send_server_text_msg:                   extern fn(server_connection_handler_id: u64, message: *const c_char, return_code: *const c_char) -> c_uint,
	pub request_connection_info:                        extern fn(server_connection_handler_id: u64, client_id: u16, return_code: *const c_char) -> c_uint,
	pub request_client_set_whisper_list:                extern fn(server_connection_handler_id: u64, client_id: u16, target_channel_id_array: *const u64, target_client_id_array: *const u16, return_code: *const c_char) -> c_uint,
	pub request_channel_subscribe:                      extern fn(server_connection_handler_id: u64, channel_id_array: *const u64, return_code: *const c_char) -> c_uint,
	pub request_channel_subscribe_all:                  extern fn(server_connection_handler_id: u64, return_code: *const c_char) -> c_uint,
	pub request_channel_unsubscribe:                    extern fn(server_connection_handler_id: u64, channel_id_array: *const u64, return_code: *const c_char) -> c_uint,
	pub request_channel_unsubscribe_all:                extern fn(server_connection_handler_id: u64, return_code: *const c_char) -> c_uint,
	pub request_channel_description:                    extern fn(server_connection_handler_id: u64, channel_id: u64, return_code: *const c_char) -> c_uint,
	pub request_mute_clients:                           extern fn(server_connection_handler_id: u64, client_id_array: *const u16, return_code: *const c_char) -> c_uint,
	pub requset_unmute_clients:                         extern fn(server_connection_handler_id: u64, client_id_array: *const u16, return_code: *const c_char) -> c_uint,
	pub request_client_poke:                            extern fn(server_connection_handler_id: u64, client_id: u16, message: *const c_char, return_code: *const c_char) -> c_uint,
	pub request_client_ids:                             extern fn(server_connection_handler_id: u64, client_unique_identifier: *const c_char, return_code: *const c_char) -> c_uint,
	pub client_chat_closed:                             extern fn(server_connection_handler_id: u64, client_unique_identifier: *const c_char, client_id: u16, return_code: *const c_char) -> c_uint,
	pub client_chat_composing:                          extern fn(server_connection_handler_id: u64, client_id: u16, return_code: *const c_char) -> c_uint,
	pub request_server_temporary_password_add:          extern fn(server_connection_handler_id: u64, password: *const c_char, description: *const c_char, duration: u64, target_channel_id: u64, target_channel_pw: *const c_char, return_code: *const c_char) -> c_uint,
	pub request_server_temporary_password_del:          extern fn(server_connection_handler_id: u64, password: *const c_char, return_code: *const c_char) -> c_uint,
	pub request_server_temporary_password_list:         extern fn(server_connection_handler_id: u64, return_code: *const c_char) -> c_uint,

	/// Access clientlib information

	/// Query own client id
	pub get_client_id:                                  extern fn(server_connection_handler_id: u64, result: *mut u16) -> c_uint,

	/// Client info
	pub get_client_self_variable_as_int:                extern fn(server_connection_handler_id: u64, flag: usize, result: *mut c_int) -> c_uint,
	pub get_client_self_variable_as_string:             extern fn(server_connection_handler_id: u64, flag: usize, result: *mut *mut c_char) -> c_uint,
	pub set_client_self_variable_as_int:                extern fn(server_connection_handler_id: u64, flag: usize, value: c_int) -> c_uint,
	pub set_client_self_variable_as_string:             extern fn(server_connection_handler_id: u64, flag: usize, value: *const c_char) -> c_uint,
	pub flush_client_self_updates:                      extern fn(server_connection_handler_id: u64, return_code: *const c_char) -> c_uint,
	pub get_client_variable_as_int:                     extern fn(server_connection_handler_id: u64, client_id: u16, flag: usize, result: *mut c_int) -> c_uint,
	pub get_client_variable_as_uint64:                  extern fn(server_connection_handler_id: u64, client_id: u16, flag: usize, result: *mut u64) -> c_uint,
	pub get_client_variable_as_string:                  extern fn(server_connection_handler_id: u64, client_id: u16, flag: usize, result: *mut *mut c_char) -> c_uint,
	pub get_client_list:                                extern fn(server_connection_handler_id: u64, result: *mut *mut u16) -> c_uint,
	pub get_channel_of_client:                          extern fn(server_connection_handler_id: u64, client_id: u16, result: *mut u64) -> c_uint,

	/// Channel info
	pub get_channel_variable_as_int:                    extern fn(server_connection_handler_id: u64, channel_id: u64, flag: usize, result: *mut c_int) -> c_uint,
	pub get_channel_variable_as_uint64:                 extern fn(server_connection_handler_id: u64, channel_id: u64, flag: usize, result: *mut u64) -> c_uint,
	pub get_channel_variable_as_string:                 extern fn(server_connection_handler_id: u64, channel_id: u64, flag: usize, result: *mut *mut c_char) -> c_uint,
	pub get_channel_id_from_channel_names:              extern fn(server_connection_handler_id: u64, channel_name_array: *mut *mut c_char, result: *mut u64) -> c_uint,
	pub set_channel_variable_as_int:                    extern fn(server_connection_handler_id: u64, channel_id: u64, flag: usize, value: c_int) -> c_uint,
	pub set_channel_variable_as_uint64:                 extern fn(server_connection_handler_id: u64, channel_id: u64, flag: usize, value: u64) -> c_uint,
	pub set_channel_variable_as_string:                 extern fn(server_connection_handler_id: u64, channel_id: u64, flag: usize, value: *const c_char) -> c_uint,
	pub flush_channel_updates:                          extern fn(server_connection_handler_id: u64, channel_id: u64, return_code: *const c_char) -> c_uint,
	pub flush_channel_creation:                         extern fn(server_connection_handler_id: u64, channel_id: u64, return_code: *const c_char) -> c_uint,
	pub get_channel_list:                               extern fn(server_connection_handler_id: u64, result: *mut *mut u64) -> c_uint,
	pub get_channel_client_list:                        extern fn(server_connection_handler_id: u64, channel_id: u64, result: *mut *mut u16) -> c_uint,
	pub get_parent_channel_of_channel:                  extern fn(server_connection_handler_id: u64, channel_id: u64, result: *mut u64) -> c_uint,

	/// Server info
	pub get_server_connection_handler_list:             extern fn(result: *mut *mut u64) -> c_uint,
	pub get_server_variable_as_int:                     extern fn(server_connection_handler_id: u64, flag: usize, result: *mut c_int) -> c_uint,
	pub get_server_variable_as_uint64:                  extern fn(server_connection_handler_id: u64, flag: usize, result: *mut u64) -> c_uint,
	pub get_server_variable_as_string:                  extern fn(server_connection_handler_id: u64, flag: usize, result: *mut *mut c_char) -> c_uint,
	pub request_server_variables:                       extern fn(server_connection_handler_id: u64) -> c_uint,

	/// Connection info
	pub get_connection_status:                          extern fn(server_connection_handler_id: u64, result: *mut c_int) -> c_uint,
	pub get_connection_variable_as_uint64:              extern fn(server_connection_handler_id: u64, client_id: u16, flag: usize, result: *mut u64) -> c_uint,
	pub get_connection_variable_as_double:              extern fn(server_connection_handler_id: u64, client_id: u16, flag: usize, result: *mut c_double) -> c_uint,
	pub get_connection_variable_as_string:              extern fn(server_connection_handler_id: u64, client_id: u16, flag: usize, result: *mut *mut c_char) -> c_uint,
	pub clean_up_connection_info:                       extern fn(server_connection_handler_id: u64, client_id: u16) -> c_uint,

	/// Client related
	pub request_client_dbid_from_uid:                   extern fn(server_connection_handler_id: u64, client_unique_identifier: *const c_char, return_code: *const c_char) -> c_uint,
	pub request_client_name_from_uid:                   extern fn(server_connection_handler_id: u64, client_unique_identifier: *const c_char, return_code: *const c_char) -> c_uint,
	pub request_client_name_from_dbid:                  extern fn(server_connection_handler_id: u64, client_database_id: u64, return_code: *const c_char) -> c_uint,
	pub request_client_edit_description:                extern fn(server_connection_handler_id: u64, client_id: u16, client_description: *const c_char, return_code: *const c_char) -> c_uint,
	pub request_client_set_is_talker:                   extern fn(server_connection_handler_id: u64, client_id: u16, is_talker: c_int, return_code: *const c_char) -> c_uint,
	pub request_is_talker:                              extern fn(server_connection_handler_id: u64, is_talker_request: c_int, is_talker_request_message: *const c_char, return_code: *const c_char) -> c_uint,

	/// Plugin related
	pub request_send_client_query_command:              extern fn(server_connection_handler_id: u64, command: *const c_char, return_code: *const c_char) -> c_uint,

	/// Filetransfer
	pub get_transfer_file_name:                         extern fn(transfer_id: u16, result: *mut *mut c_char) -> c_uint,
	pub get_transfer_file_path:                         extern fn(transfer_id: u16, result: *mut *mut c_char) -> c_uint,
	pub get_transfer_file_size:                         extern fn(transfer_id: u16, result: *mut u64) -> c_uint,
	pub get_transfer_file_size_done:                    extern fn(transfer_id: u16, result: *mut u64) -> c_uint,
	/// 1 == upload, 0 == download
	pub is_transfer_sender:                             extern fn(transfer_id: u16, result: *mut c_int) -> c_uint,
	pub get_transfer_status:                            extern fn(transfer_id: u16, result: *mut c_int) -> c_uint,
	pub get_current_transfer_speed:                     extern fn(transfer_id: u16, result: *mut c_float) -> c_uint,
	pub get_average_transfer_speed:                     extern fn(transfer_id: u16, result: *mut c_float) -> c_uint,
	pub get_transfer_run_time:                          extern fn(transfer_id: u16, result: *mut u64) -> c_uint,
	pub send_file:                                      extern fn(server_connection_handler_id: u64, channel_id: u64, channel_pw: *const c_char, file: *const c_char, overwrite: c_int, resume: c_int, source_directory: *const c_char, result: *mut u16, return_code: *const c_char) -> c_uint,
	pub request_file:                                   extern fn(server_connection_handler_id: u64, channel_id: u64, channel_pw: *const c_char, file: *const c_char, overwrite: c_int, resume: c_int, destination_directory: *const c_char, result: *mut u16, return_code: *const c_char) -> c_uint,
	pub halt_transfer:                                  extern fn(server_connection_handler_id: u64, transfer_id: u16, delete_unfinished_file: c_int, return_code: *const c_char) -> c_uint,
	pub request_file_list:                              extern fn(server_connection_handler_id: u64, channel_id: u64, channel_pw: *const c_char, path: *const c_char, return_code: *const c_char) -> c_uint,
	pub request_file_info:                              extern fn(server_connection_handler_id: u64, channel_id: u64, channel_pw: *const c_char, file: *const c_char, return_code: *const c_char) -> c_uint,
	pub request_delete_file:                            extern fn(server_connection_handler_id: u64, channel_id: u64, channel_pw: *const c_char, file: *mut *const c_char, return_code: *const c_char) -> c_uint,
	pub request_create_directory:                       extern fn(server_connection_handler_id: u64, channel_id: u64, channel_pw: *const c_char, directory_path: *const char, return_code: *const char) -> c_uint,
	pub requset_rename_file:                            extern fn(server_connection_handler_id: u64, from_channel_id: u64, channel_pw: *const c_char, to_channel_id: u64, to_channel_pw: *const c_char, old_file: *const c_char, new_file: *const c_char, return_code: *const c_char) -> c_uint,

	/// Offline message management
	pub request_message_add:                            extern fn(server_connection_handler_id: u64, to_client_uid: *const c_char, subject: *const c_char, message: *const c_char, return_code: *const c_char) -> c_uint,
	pub request_message_del:                            extern fn(server_connection_handler_id: u64, message_id: u64, return_code: *const c_char) -> c_uint,
	pub request_message_get:                            extern fn(server_connection_handler_id: u64, message_id: u64, return_code: *const c_char) -> c_uint,
	pub request_message_list:                           extern fn(server_connection_handler_id: u64, return_code: *const c_char) -> c_uint,
	pub request_message_update_flag:                    extern fn(server_connection_handler_id: u64, message_id: u64, flag: c_int, return_code: *const c_char) -> c_uint,

	/// Interacting with the server - confirming passwords
	pub verify_server_password:                         extern fn(server_connection_handler_id: u64, server_password: *const c_char, return_code: *const c_char) -> c_uint,
	pub verify_channel_password:                        extern fn(server_connection_handler_id: u64, channel_id: u64, channel_password: *const c_char, return_code: *const c_char) -> c_uint,

	/// Interacting with the server - banning
	pub banclient:                                      extern fn(server_connection_handler_id: u64, client_id: u16, time_in_seconds: u64, ban_reason: *const c_char, return_code: *const c_char) -> c_uint,
	pub banadd:                                         extern fn(server_connection_handler_id: u64, ip_reg_exp: *const c_char, name_regexp: *const c_char, unique_identity: *const c_char, myts_id: *const c_char, time_in_seconds: u64, ban_reason: *const c_char, return_code: *const c_char) -> c_uint,
	pub banclient_dbid:                                 extern fn(server_connection_handler_id: u64, client_dbid: u64, time_in_seconds: u64, ban_reason: *const c_char, return_code: *const c_char) -> c_uint,
	pub bandel:                                         extern fn(server_connection_handler_id: u64, ban_id: u64, return_code: *const c_char) -> c_uint,
	pub bandelall:                                      extern fn(server_connection_handler_id: u64, return_code: *const c_char) -> c_uint,
	pub request_ban_list:                               extern fn(server_connection_handler_id: u64, start: u64, duration: c_uint, return_code: *const c_char) -> c_uint,

	/// Interacting with the server - complain
	pub request_complain_add:                           extern fn(server_connection_handler_id: u64, target_client_database_id: u64, complain_reason: *const c_char, return_code: *const c_char) -> c_uint,
	pub request_complain_del:                           extern fn(server_connection_handler_id: u64, target_client_database_id: u64, from_client_database_id: u64, return_code: *const c_char) -> c_uint,
	pub request_complain_del_all:                       extern fn(server_connection_handler_id: u64, target_client_database_id: u64, return_code: *const c_char) -> c_uint,
	pub request_complain_list:                          extern fn(server_connection_handler_id: u64, target_client_database_id: u64, return_code: *const c_char) -> c_uint,

	/// Permissions
	pub request_server_group_list:                      extern fn(server_connection_handler_id: u64, return_code: *const c_char) -> c_uint,
	pub request_server_group_add:                       extern fn(server_connection_handler_id: u64, group_name: *const c_char, group_type: c_int, return_code: *const c_char) -> c_uint,
	pub request_server_group_del:                       extern fn(server_connection_handler_id: u64, server_group_id: u64, force: c_int, return_code: *const c_char) -> c_uint,
	pub request_server_group_add_client:                extern fn(server_connection_handler_id: u64, server_group_id: u64, client_database_id: u64, return_code: *const c_char) -> c_uint,
	pub request_server_group_del_client:                extern fn(server_connection_handler_id: u64, server_group_id: u64, client_database_id: u64, return_code: *const c_char) -> c_uint,
	pub request_server_groups_by_client_id:             extern fn(server_connection_handler_id: u64, client_database_id: u64, return_code: *const c_char) -> c_uint,
	pub request_server_group_add_perm:                  extern fn(server_connection_handler_id: u64, server_group_id: u64, continue_on_error: c_int, permission_id_array: *const c_uint, permission_value_array: *const c_int, permission_negated_array: *const c_int, permission_skip_array: *const c_int, array_size: c_int, return_code: *const c_char) -> c_uint,
	pub request_server_group_del_perm:                  extern fn(server_connection_handler_id: u64, server_group_id: u64, continue_on_error: c_int, permission_id_array: *const c_uint, array_size: c_int, return_code: *const c_char) -> c_uint,
	pub request_server_group_perm_list:                 extern fn(server_connection_handler_id: u64, server_group_id: u64, return_code: *const c_char) -> c_uint,
	pub request_server_group_client_list:               extern fn(server_connection_handler_id: u64, server_group_id: u64, with_names: c_int, return_code: *const c_char) -> c_uint,
	pub request_channel_group_list:                     extern fn(server_connection_handler_id: u64, return_code: *const c_char) -> c_uint,
	pub request_channel_group_add:                      extern fn(server_connection_handler_id: u64, group_name: *const c_char, group_type: c_int, return_code: *const c_char) -> c_uint,
	pub request_channel_group_del:                      extern fn(server_connection_handler_id: u64, channel_group_id: u64, force: c_int, return_code: *const c_char) -> c_uint,
	pub request_channel_group_add_perm:                 extern fn(server_connection_handler_id: u64, channel_group_id: u64, continue_on_error: c_int, permission_id_array: *const c_uint, permission_value_array: *const c_int, array_size: c_int, return_code: *const c_char) -> c_uint,
	pub request_channel_group_del_perm:                 extern fn(server_connection_handler_id: u64, channel_group_id: u64, continue_on_error: c_int, permission_id_array: *const c_uint, array_size: c_int, return_code: *const c_char) -> c_uint,
	pub request_channel_group_perm_list:                extern fn(server_connection_handler_id: u64, channel_group_id: u64, return_code: *const c_char) -> c_uint,
	pub request_set_client_channel_group:               extern fn(server_connection_handler_id: u64, channel_group_id_array: *const u64, channel_id_array: *const u64, client_database_id_array: *const u64, array_size: c_int, return_code: *const c_char) -> c_uint,
	pub request_channel_add_perm:                       extern fn(server_connection_handler_id: u64, channel_id: u64, permission_id_array: *const c_uint, permission_value_array: *const c_int, array_size: c_int, return_code: *const c_char) -> c_uint,
	pub request_channel_del_perm:                       extern fn(server_connection_handler_id: u64, channel_id: u64, permission_id_array: *const c_uint, array_size: c_int, return_code: *const c_char) -> c_uint,
	pub request_channel_perm_list:                      extern fn(server_connection_handler_id: u64, channel_id: u64, return_code: *const c_char) -> c_uint,
	pub request_client_add_perm:                        extern fn(server_connection_handler_id: u64, client_database_id: u64, permission_id_array: *const c_uint, permission_value_array: *const c_int, permission_skip_array: *const c_int, array_size: c_int, return_code: *const c_char) -> c_uint,
	pub request_client_del_perm:                        extern fn(server_connection_handler_id: u64, client_database_id: u64, permission_id_array: *const c_uint, array_size: c_int, return_code: *const c_char) -> c_uint,
	pub request_client_perm_list:                       extern fn(server_connection_handler_id: u64, client_database_id: u64, return_code: *const c_char) -> c_uint,
	pub request_channel_client_add_perm:                extern fn(server_connection_handler_id: u64, channel_id: u64, client_database_id: u64, permission_id_array: *const c_uint, permission_value_array: *const c_int, array_size: c_int, return_code: *const c_char) -> c_uint,
	pub request_channel_client_del_perm:                extern fn(server_connection_handler_id: u64, channel_id: u64, client_database_id: u64, permission_id_array: *const c_uint, array_size: c_int, return_code: *const c_char) -> c_uint,
	pub request_channel_client_perm_list:               extern fn(server_connection_handler_id: u64, channel_id: u64, client_database_id: u64, return_code: *const c_char) -> c_uint,
	pub priviledge_key_use:                             extern fn(server_connection_handler_id: u64, token_key: *const c_char, return_code: *const c_char) -> c_uint,
	pub request_permission_list:                        extern fn(server_connection_handler_id: u64, return_code: *const c_char) -> c_uint,
	pub request_permission_overview:                    extern fn(server_connection_handler_id: u64, client_dbid: u64, channel_id: u64, return_code: *const c_char) -> c_uint,

	/// Helper functions
	pub client_property_string_to_flag:                 extern fn(client_property_string: *const c_char, result_flag: *mut usize) -> c_uint,
	pub channel_property_string_to_flag:                extern fn(channel_property_string: *const c_char, result_flag: *mut usize) -> c_uint,
	pub server_property_string_to_flag:                 extern fn(server_property_string: *const c_char, result_flag: *mut usize) -> c_uint,

	/// Client functions
	pub get_app_path:                                   extern fn(path: *mut c_char, max_len: usize),
	pub get_resources_path:                             extern fn(path: *mut c_char, max_len: usize),
	pub get_config_path:                                extern fn(path: *mut c_char, max_len: usize),
	pub get_plugin_path:                                extern fn(path: *mut c_char, max_len: usize, plugin_id: *const c_char),
	pub get_current_server_connection_handler_id:       extern fn() -> u64,
	pub print_message:                                  extern fn(server_connection_handler_id: u64, message: *const c_char, message_target: MessageTarget),
	pub print_message_to_current_tab:                   extern fn(message: *const c_char),
	pub urls_to_bb:                                     extern fn(text: *const c_char, result: *mut c_char, max_len: usize),
	pub send_plugin_command:                            extern fn(server_connection_handler_id: u64, plugin_id: *const c_char, command: *const c_char, target_mode: c_int, target_ids: *const u16, return_code: *const c_char),
	pub get_directories:                                extern fn(path: *const c_char, result: *mut c_char, max_len: usize),
	pub get_server_connect_info:                        extern fn(sc_handler_id: u64, host: *mut c_char, port: *mut c_ushort, password: *mut c_char, max_len: usize) -> c_uint,
	pub get_channel_connection_info:                    extern fn(sc_handler_id: u64, channel_id: u64, path: *mut c_char, password: *mut c_char, max_len: usize) -> c_uint,
	pub create_return_code:                             extern fn(plugin_id: *const c_char, return_code: *mut c_char, max_len: usize),
	pub request_info_update:                            extern fn(sc_handler_id: u64, item_type: ItemType, item_id: u64) -> c_uint,
	pub get_server_version:                             extern fn(sc_handler_id: u64) -> u64,
	pub is_whispering:                                  extern fn(sc_handler_id: u64, client_id: u16, result: *mut c_int) -> c_uint,
	pub is_receiving_whisper:                           extern fn(sc_handler_id: u64, client_id: u16, result: *mut c_int) -> c_uint,
	pub get_avatar:                                     extern fn(sc_handler_id: u64, client_id: u16, result: *mut c_char, max_len: usize) -> c_uint,
	pub set_plugin_menu_enabled:                        extern fn(plugin_id: *const c_char, menu_id: c_int, enabled: c_int),
	pub show_hotkey_setup:                              extern fn(),
	pub request_hotkey_input_dialog:                    extern fn(plugin_id: *const c_char, *const c_char, c_int, *mut c_void),
	pub get_hotkey_from_keyword:                        extern fn(plugin_id: *const c_char, keywords: *mut *const c_char, hotkeys: *mut *mut c_char, array_len: usize, hotkey_buf_size: usize) -> c_uint,
	pub get_client_display_name:                        extern fn(sc_handler_id: u64, client_id: u16, result: *mut c_char, max_len: usize) -> c_uint,
	pub get_bookmark_list:                              extern fn(list: *mut *mut BookmarkList) -> c_uint,
	pub get_profile_list:                               extern fn(profile: GuiProfile, default_profile_idx: *mut c_int, result: *mut *mut *mut c_char) -> c_uint,
	pub gui_connect:                                    extern fn(connect_tab: ConnectTab, server_label: *const c_char, server_address: *const c_char, server_password: *const c_char, nickname: *const c_char, channel: *const c_char, channel_password: *const c_char, capture_profile: *const c_char, playback_profile: *const c_char, hotkey_profile: *const c_char, sound_profile: *const c_char, user_identity: *const c_char, one_time_key: *const c_char, phonetic_name: *const c_char, sc_handler_id: *mut u64) -> c_uint,
	pub gui_connect_bookmark:                           extern fn(connect_tab: ConnectTab, bookmarkuuid: *const c_char, sc_handler_id: *mut u64) -> c_uint,
	pub create_bookmark:                                extern fn(bookmarkuuid: *const c_char, server_label: *const c_char, server_address: *const c_char, server_password: *const c_char, nickname: *const c_char, channel: *const c_char, channel_password: *const c_char, capture_profile: *const c_char, playback_profile: *const c_char, hotkey_profile: *const c_char, sound_profile: *const c_char, unique_user_id: *const c_char, one_time_key: *const c_char, phonetic_name: *const c_char) -> c_uint,
	pub get_permission_id_by_name:                      extern fn(server_connection_handler_id: u64, permission_name: *const c_char, result: *mut c_uint) -> c_uint,
	pub get_client_needed_permission:                   extern fn(server_connection_handler_id: u64, permission_name: *const c_char, result: *mut c_int) -> c_uint,
}
