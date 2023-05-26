pub const OMNIBUS_COMMAND: usize = 8;
pub const OMNIBUS_REPLY: usize = 11;

pub const AUX_COMMAND: usize = 8;
pub const AUX_REPLY: usize = 11;

pub const CALIBRATE_COMMAND: usize = 8;
pub const CALIBRATE_REPLY: usize = 11;

pub const START_DOWNLOAD_COMMAND: usize = 8;
pub const START_DOWNLOAD_REPLY: usize = 11;

pub const BAUD_CHANGE_REQUEST: usize = 6;
pub const BAUD_CHANGE_REPLY: usize = 6;

pub const FLASH_DATA_PACKET: usize = 32;
pub const FLASH_DATA_PACKET_64: usize = 64;

pub const FLASH_DOWNLOAD_MESSAGE_7BIT: usize = 73;
pub const FLASH_DOWNLOAD_REPLY_7BIT: usize = 9;

pub const FLASH_DOWNLOAD_MESSAGE_8BIT_64: usize = 71;
pub const FLASH_DOWNLOAD_MESSAGE_8BIT_32: usize = 39;
pub const FLASH_DOWNLOAD_REPLY_8BIT: usize = 7;

pub const QUERY_DEVICE_CAPABILITIES_COMMAND: usize = 8;
pub const QUERY_DEVICE_CAPABILITIES_REPLY: usize = 11;

pub const QUERY_VARIANT_NAME_COMMAND: usize = 8;
pub const QUERY_VARIANT_NAME_REPLY: usize = 37;

pub const QUERY_EXTENDED_NOTE_SPECIFICATION: usize = 10;
pub const EXTENDED_NOTE_REPLY: usize = 30;

pub const SET_EXTENDED_NOTE_INHIBITS_BASE: usize = 9;
pub const EXTENDED_NOTE_INHIBITS_REPLY: usize = 11;
pub const EXTENDED_NOTE_INHIBITS_REPLY_ALT: usize = 12;

pub const QUERY_VALUE_TABLE_COMMAND: usize = 9;
pub const QUERY_VALUE_TABLE_REPLY: usize = 82;

pub const APPLICATION_ID_COMMAND: usize = 8;
pub const APPLICATION_ID_REPLY: usize = 14;

pub const ADVANCED_BOOKMARK_MODE_COMMAND: usize = 10;
pub const ADVANCED_BOOKMARK_MODE_REPLY: usize = 13;

pub const QUERY_SOFTWARE_CRC_COMMAND: usize = 8;
pub const QUERY_SOFTWARE_CRC_REPLY: usize = 11;

pub const QUERY_BOOT_PART_NUMBER_COMMAND: usize = 8;
pub const QUERY_BOOT_PART_NUMBER_REPLY: usize = 14;

pub const QUERY_APPLICATION_PART_NUMBER_COMMAND: usize = 8;
pub const QUERY_APPLICATION_PART_NUMBER_REPLY: usize = 14;

pub const QUERY_APPLICATION_ID_COMMAND: usize = 8;
pub const QUERY_APPLICATION_ID_REPLY: usize = 14;

pub const QUERY_VARIANT_PART_NUMBER_COMMAND: usize = 8;
pub const QUERY_VARIANT_PART_NUMBER_REPLY: usize = 14;

pub const QUERY_VARIANT_ID_COMMAND: usize = 8;
pub const QUERY_VARIANT_ID_REPLY: usize = 14;

pub const CLEAR_AUDIT_DATA_REQUEST: usize = 9;
pub const CLEAR_AUDIT_DATA_REQUEST_ACK: usize = 13;
pub const CLEAR_AUDIT_DATA_REQUEST_RESULTS: usize = 13;

pub const SOFT_RESET: usize = 8;

pub const SET_ESCROW_TIMEOUT_COMMAND: usize = 11;
pub const SET_ESCROW_TIMEOUT_REPLY: usize = 12;

pub const NOTE_RETRIEVED_COMMAND: usize = 10;
pub const NOTE_RETRIEVED_REPLY: usize = 13;
pub const NOTE_RETRIEVED_EVENT: usize = 13;

pub const METADATA: usize = 5;
pub const MIN_MESSAGE: usize = 5;
pub const MAX_MESSAGE: usize = 255;
