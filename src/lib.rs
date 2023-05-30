#![cfg_attr(not(feature = "std"), no_std)]

//! # EBDS Serial Protocol
//! 
//! This crate implements the EBDS serial protocol messages, and related types for communication with bill acceptor unit devices.
//! 
//! The currently supported messages are implemented in the various modules in this crate, along with some common types used across multiple messages.
//! 
//! If adding a new message, please follow the existing pattern of placing `...Command` (host-initiated) messages in `<message-type>/command.rs` files, and `...Reply` (device-initiated) messages in `<message-type>/reply.rs` files.
//! 
//! There are some exceptions to the general rule, e.g. when the types in the documenation do not follow the `Command/Reply` naming convention.
//! 
//! In those cases, the suffix is omitted to aid in readability when comparing with the EBDS specification.
//! 
//! ## Macros
//! 
//! Some simple macros exist for implementing traits over the various message types. All message types should implement `MessageOps`, and all reply types should implement `OmnibusReplyOps`.
//! 
//! `MessageOps` can be implemented with the helper macro `impl_message_ops!`, e.g. for a new `SomeNewReply` message:
//! 
//! ```rust
//! use crate::impl_message_ops;
//! 
//! pub struct SomeNewReply {
//!     // For the example, we are just using a number for the length.
//!     // In real implementations, please add a constant to the `len` module.
//!     buf: [u8; 11],
//! }
//! 
//! impl_message_ops!(SomeNewReply);
//! ```
//! 
//! This will implement the `MessageOps` trait for `SomeNewReply`, and provide all of the associated functions. Traits are how Rust does polymorphism, similar to Go's `interface` and C++'s `template`, with important differences.
//! 
//! All of the macro implementations live in `src/macros.rs`.
//! 
//! ## Using with `std`
//! 
//! This library is `no-std` compatible by default. To use `std`-only features, add the `std` feature to the dependency:
//! 
//! ```toml
//! ebds = { version = "0.1", features = ["std"] }
//! ```

#[cfg(not(feature = "std"))]
#[macro_use(format)]
extern crate alloc;

#[macro_use(bitfield)]
extern crate bitfield;

#[cfg(not(feature = "std"))]
pub(crate) use core as std;
#[cfg(feature = "std")]
pub(crate) use std;

use std::{fmt, ops::Not};

/// Banknote types used across multiple messages
pub mod banknote;
/// Cash types used across multiple messages
pub mod cash;
/// Denomination types used
pub mod denomination;
/// Library error types
pub mod error;
/// Hardware status and related types
pub mod hardware;
/// JSON-RPC message and related types
pub mod jsonrpc;
/// Logging convenience helpers
pub mod logging;
mod macros;
/// JSON-RPC and device method types
pub mod method;
/// Device status types
pub mod status;

pub use banknote::*;
pub use cash::*;
pub use denomination::*;
pub use error::*;
pub use hardware::*;
pub use jsonrpc::*;
pub use logging::*;
pub use method::*;
pub use status::*;

/// Advanced Bookmark Mode - Extended (Type 0x07, Subtype 0x0D)
pub mod advanced_bookmark_mode;
/// Generic types for Auxilliary Command/Reply messages - Auxilliary (Type 0x06)
pub mod aux_command;
/// Clear Audit Data - Extended (Type 0x07, Subtype 0x1D)
pub mod clear_audit_data;
/// Generic types for Extended Command messages - Extended (Type 0x07)
pub mod extended_command;
/// Extended Note Inhibits - Extended (Type 0x07, Subtype 0x03)
pub mod extended_note_inhibits;
/// Extended Note Specification - Extended (Type 0x07, Subtype 0x02)
pub mod extended_note_specification;
/// Generic types for Extended Reply messages - Extended (Type 0x07)
pub mod extended_reply;
/// Flash Download - (Type 0x05)
pub mod flash_download;
/// Total message lengths for various messages
///
/// IMPORTANT: this is the total byte length of the packet,
/// to get the length of data bytes, subtract 5 from these constants.
///
/// Example:
///
/// ```rust
/// # use ebds::{OmnibusCommand, MessageOps, len::{OMNIBUS_COMMAND, METADATA}};
/// let message = OmnibusCommand::new();
///
/// assert_eq!(message.len(), OMNIBUS_COMMAND);
/// assert_eq!(message.data_len(), OMNIBUS_COMMAND - METADATA);
/// ```
pub mod len;
/// Note Retrieved - Extended (Type 0x07, Subtype 0x0B)
pub mod note_retrieved;
/// Omnibus - Command (Type 0x01), Reply (Type 0x02)
pub mod omnibus;
/// Part number type definitions, used across multiple messages
pub mod part_number;
/// Query Application ID - Auxilliary (Type 0x06, Subtype 0x0E)
pub mod query_application_id;
/// Query Application Part Number - Auxilliary (Type 0x06, Subtype 0x07)
pub mod query_application_part_number;
/// Query Boot Part Number - Auxilliary (Type 0x06, Subtype 0x06)
pub mod query_boot_part_number;
/// Query Device Capabilities - Auxilliary (Type 0x06, Subtype 0x0D)
pub mod query_device_capabilities;
/// Query Software CRC - Auxilliary (Type 0x06, Subtype 0x00)
pub mod query_software_crc;
/// Query Value Table - Extended (Type 0x07, Subtype 0x06)
pub mod query_value_table;
/// Query Variant ID - Auxilliary (Type 0x06, Subtype 0x0F)
pub mod query_variant_id;
/// Query Variant Name - Auxilliary (Type 0x06, Subtype 0x08)
pub mod query_variant_name;
/// Query Variant Part Number - Auxilliary (Type 0x06, Subtype 0x09)
pub mod query_variant_part_number;
/// Set Escrow Timeout - Extended (Type 0x07, Subtype 0x04)
pub mod set_escrow_timeout;
/// Soft Reset - Auxilliary (Type 0x06, Subtype 0x7F)
pub mod soft_reset;
/// Message variant for building messages from raw bytes
pub mod variant;

pub use advanced_bookmark_mode::*;
pub use aux_command::*;
pub use clear_audit_data::*;
pub use extended_command::*;
pub use extended_note_inhibits::*;
pub use extended_note_specification::*;
pub use extended_reply::*;
pub use flash_download::*;
pub use note_retrieved::*;
pub use omnibus::*;
pub use part_number::*;
pub use query_application_id::*;
pub use query_application_part_number::*;
pub use query_boot_part_number::*;
pub use query_device_capabilities::*;
pub use query_software_crc::*;
pub use query_value_table::*;
pub use query_variant_id::*;
pub use query_variant_name::*;
pub use query_variant_part_number::*;
pub use set_escrow_timeout::*;
pub use soft_reset::*;
pub use variant::*;

pub use crate::error::{Error, JsonRpcError, JsonRpcResult, Result};

/// Start byte for EBDS packet
pub const STX: u8 = 0x02;
/// End byte for EBDS packet
pub const ETX: u8 = 0x03;
/// Magic byte for Special Interrupt Mode (not supported).
pub const ENQ: u8 = 0x05;
/// Constant for the environment variable defining the default Currency set
pub const ENV_CURRENCY: &str = "BAU_CURRENCY";

#[cfg(feature = "usd")]
pub const DEFAULT_CURRENCY: Currency = Currency::USD;
#[cfg(feature = "cny")]
pub const DEFAULT_CURRENCY: Currency = Currency::CNY;
#[cfg(feature = "gbp")]
pub const DEFAULT_CURRENCY: Currency = Currency::GBP;
#[cfg(feature = "jpy")]
pub const DEFAULT_CURRENCY: Currency = Currency::JPY;
#[cfg(feature = "aud")]
pub const DEFAULT_CURRENCY: Currency = Currency::AUD;
#[cfg(feature = "cad")]
pub const DEFAULT_CURRENCY: Currency = Currency::CAD;
#[cfg(feature = "mxn")]
pub const DEFAULT_CURRENCY: Currency = Currency::MXN;
#[cfg(feature = "amd")]
pub const DEFAULT_CURRENCY: Currency = Currency::AMD;

#[cfg(feature = "std")]
pub fn bau_currency() -> Currency {
    std::env::var(ENV_CURRENCY)
        .unwrap_or(format!("{DEFAULT_CURRENCY}"))
        .into()
}

#[cfg(not(feature = "std"))]
pub fn bau_currency() -> Currency {
    DEFAULT_CURRENCY
}

/// Calculate the XOR checksum of a byte range
///
/// This range should be the first non-control byte (LEN-byte),
/// through the first byte before ETX
pub fn checksum(data: &[u8]) -> u8 {
    let mut sum = 0u8;
    data.iter().for_each(|&b| sum ^= b);
    sum
}

// Under the 7-bit protocol, constructs a 16-bit number from a 4-byte slice.
//
// Each byte stores the significant bits in the lower nibble (4-bits),
// most significant nibble first (big-endian).
pub(crate) fn seven_bit_u16(b: &[u8]) -> u16 {
    debug_assert_eq!(b.len(), 4);

    let hi = ((b[0] & 0xf) << 4) | (b[1] & 0xf);
    let lo = ((b[2] & 0xf) << 4) | (b[3] & 0xf);

    u16::from_be_bytes([hi, lo])
}

// Under the 7-bit protocol, transforms a 16-bit number
// into a 4-byte slice.
//
// Each byte stores the significant bits in the lower nibble (4-bits),
// most significant nibble first (big-endian).
pub(crate) fn u16_seven_bit(n: u16) -> [u8; 4] {
    let b = n.to_be_bytes();
    [b[0] >> 4, b[0] & 0xf, b[1] >> 4, b[1] & 0xf]
}

// Under the 7-bit protocol, constructs a 8-bit number from a 2-byte slice.
//
// Each byte stores the significant bits in the lower nibble (4-bits),
// most significant nibble first (big-endian).
pub(crate) fn seven_bit_u8(b: &[u8]) -> u8 {
    debug_assert_eq!(b.len(), 2);

    ((b[0] & 0xf) << 4) | (b[1] & 0xf)
}

// Under the 7-bit protocol, transforms a 8-bit number into a 2-byte slice.
//
// Each byte stores the significant bits in the lower nibble (4-bits),
// most significant nibble first (big-endian).
pub(crate) fn u8_seven_bit(n: u8) -> [u8; 2] {
    [n >> 4, n & 0xf]
}

bitfield! {
    /// Control field for EBDS messages
    pub struct Control(u8);
    u8;
    /// AckNak bit for message acknowledgement, see [AckNak](AckNak)
    pub acknak, set_acknak: 0;
    /// Device type, see [DeviceType](DeviceType)
    pub device_type, set_device_type: 3, 1;
    /// Message type, see [MessageType](MessageType)
    pub message_type, set_message_type: 6, 4;
}

impl From<u8> for Control {
    fn from(b: u8) -> Self {
        Self(b & 0b111_1111)
    }
}

impl From<Control> for u8 {
    fn from(c: Control) -> Self {
        c.0
    }
}

impl From<&Control> for u8 {
    fn from(c: &Control) -> Self {
        c.0
    }
}

/// Set the ACK field in the control byte
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AckNak {
    Ack = 0b0,
    Nak = 0b1,
}

impl Not for AckNak {
    type Output = AckNak;

    fn not(self) -> Self::Output {
        Self::from(!(self as u8))
    }
}

impl fmt::Display for AckNak {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", <&'static str>::from(self))
    }
}

impl From<AckNak> for &'static str {
    fn from(a: AckNak) -> &'static str {
        match a {
            AckNak::Ack => "ACK",
            AckNak::Nak => "NAK",
        }
    }
}

impl From<&AckNak> for &'static str {
    fn from(a: &AckNak) -> Self {
        (*a).into()
    }
}

impl From<bool> for AckNak {
    fn from(b: bool) -> Self {
        match b {
            false => Self::Ack,
            true => Self::Nak,
        }
    }
}

impl From<u8> for AckNak {
    fn from(b: u8) -> Self {
        match b & bitmask::ACK_NAK {
            0b0 => Self::Ack,
            0b1 => Self::Nak,
            _ => unreachable!("invalid AckNak"),
        }
    }
}

impl From<AckNak> for bool {
    fn from(a: AckNak) -> bool {
        a == AckNak::Nak
    }
}

/// Device type control bits
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DeviceType {
    /// Bill acceptor device
    BillAcceptor = 0b000,
    /// Bill recycler device
    BillRecycler = 0b001,
    /// All other 3-bit values reserved for future use
    Reserved = 0b111,
}

impl fmt::Display for DeviceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dev_str: &'static str = self.into();
        write!(f, "{}", dev_str)
    }
}

impl From<DeviceType> for &'static str {
    fn from(d: DeviceType) -> Self {
        match d {
            DeviceType::BillAcceptor => "BillAcceptor",
            DeviceType::BillRecycler => "BillRecycler",
            DeviceType::Reserved => "Reserved",
        }
    }
}

impl From<&DeviceType> for &'static str {
    fn from(d: &DeviceType) -> Self {
        (*d).into()
    }
}

impl From<u8> for DeviceType {
    fn from(b: u8) -> Self {
        match b & bitmask::DEVICE_TYPE {
            0b000 => Self::BillAcceptor,
            0b001 => Self::BillRecycler,
            _ => Self::Reserved,
        }
    }
}

/// Various message types for different device interactions
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MessageType {
    /// Generic omnibus command message, see [OmnibusCommand](crate::OmnibusCommand)
    OmnibusCommand = 0b001,
    /// Generic omnibus reply message, see [OmnibusReply](crate::OmnibusReply)
    OmnibusReply = 0b010,
    /// Generic omnibus bookmark message, see [OmnibusBookmark](crate::OmnibusBookmark)
    OmnibusBookmark = 0b011,
    /// Calibrate message, see [Calibrate](crate::Calibrate)
    Calibrate = 0b100,
    /// Firmware download message (response and reply), see [FirmwareDownload](crate::flash_download)
    FirmwareDownload = 0b101,
    /// Auxilliary command, see [AuxCommand](crate::AuxCommand)
    AuxCommand = 0b110,
    /// Extended message, see [Extended](crate::ExtendedCommand)
    Extended = 0b111,
    /// Variant to represent reserved values
    Reserved = 0xff,
}

impl fmt::Display for MessageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", <&'static str>::from(self))
    }
}

impl From<MessageType> for &'static str {
    fn from(m: MessageType) -> &'static str {
        match m {
            MessageType::OmnibusCommand => "OmnibusCommand",
            MessageType::OmnibusReply => "OmnibusReply",
            MessageType::OmnibusBookmark => "OmnibusBookmark",
            MessageType::Calibrate => "Calibrate",
            MessageType::FirmwareDownload => "FirmwareDownload",
            MessageType::AuxCommand => "AuxCommand",
            MessageType::Extended => "Extended",
            MessageType::Reserved => "Reserved",
        }
    }
}

impl From<&MessageType> for &'static str {
    fn from(m: &MessageType) -> Self {
        (*m).into()
    }
}

impl From<u8> for MessageType {
    fn from(b: u8) -> Self {
        match b & bitmask::MESSAGE_TYPE {
            0b001 => Self::OmnibusCommand,
            0b010 => Self::OmnibusReply,
            0b011 => Self::OmnibusBookmark,
            0b100 => Self::Calibrate,
            0b101 => Self::FirmwareDownload,
            0b110 => Self::AuxCommand,
            0b111 => Self::Extended,
            _ => Self::Reserved,
        }
    }
}

pub(crate) mod index {
    pub const STX: usize = 0;
    pub const LEN: usize = 1;
    pub const CONTROL: usize = 2;
    pub const DATA: usize = 3;
    pub const EXT_SUBTYPE: usize = 3;
}

pub(crate) mod bitmask {
    pub const ACK_NAK: u8 = 0b1;
    pub const DEVICE_TYPE: u8 = 0b111;
    pub const MESSAGE_TYPE: u8 = 0b111;
}

/// Generic functions for all EBDS message types
pub trait MessageOps {
    /// Initialize common message fields
    fn init(&mut self) {
        let len = self.len();
        let etx_index = self.etx_index();
        let buf = self.buf_mut();

        buf[index::STX] = STX;
        buf[index::LEN] = len as u8;
        buf[etx_index] = ETX;
    }

    /// Get a reference to the message buffer.
    fn buf(&self) -> &[u8];

    /// Get a mutable reference to the message buffer.
    fn buf_mut(&mut self) -> &mut [u8];

    /// Get the length of the entire message.
    fn len(&self) -> usize {
        self.buf().len()
    }

    /// Gets whether the message buffer is empty (all zeros)
    fn is_empty(&self) -> bool {
        let mut ret = 0;
        self.buf().iter().for_each(|&b| ret ^= b);
        ret == 0
    }

    /// Get the length of data bytes.
    fn data_len(&self) -> usize {
        self.len() - len::METADATA
    }

    /// Get the ETX index.
    fn etx_index(&self) -> usize {
        self.buf().len() - 2
    }

    /// Get the checksum index.
    fn chk_index(&self) -> usize {
        self.buf().len() - 1
    }

    /// Get the ACKNAK control field.
    fn acknak(&self) -> AckNak {
        Control(self.buf()[index::CONTROL]).acknak().into()
    }

    /// Set the ACKNAK control field.
    fn set_acknak(&mut self, acknak: AckNak) {
        let mut control = Control(self.buf()[index::CONTROL]);
        control.set_acknak(acknak.into());
        self.buf_mut()[index::CONTROL] = control.into();
    }

    /// Switches the current ACKNAK control field value.
    fn switch_acknak(&mut self) {
        self.set_acknak(!self.acknak())
    }

    /// Get the device type control field.
    fn device_type(&self) -> DeviceType {
        Control(self.buf()[index::CONTROL]).device_type().into()
    }

    /// Set the device type control field
    fn set_device_type(&mut self, device_type: DeviceType) {
        let mut control = Control(self.buf()[index::CONTROL]);
        control.set_device_type(device_type as u8);
        self.buf_mut()[index::CONTROL] = control.into();
    }

    /// Get the message type control field
    fn message_type(&self) -> MessageType {
        Control(self.buf()[index::CONTROL]).message_type().into()
    }

    /// Set the message type control field
    fn set_message_type(&mut self, message_type: MessageType) {
        let mut control = Control(self.buf()[index::CONTROL]);
        control.set_message_type(message_type as u8);
        self.buf_mut()[index::CONTROL] = control.into();
    }

    /// Get the current checksum value
    ///
    /// Note: to ensure validity, call [calculate_checksum](Self::calculate_checksum) first
    fn checksum(&self) -> u8 {
        self.buf()[self.chk_index()]
    }

    fn checksum_bytes(&self) -> &[u8] {
        self.buf()[index::LEN..self.etx_index()].as_ref()
    }

    /// Calculate the message checksum
    fn calculate_checksum(&mut self) -> u8 {
        let csum = checksum(self.checksum_bytes());
        let csum_index = self.chk_index();
        self.buf_mut()[csum_index] = csum;
        csum
    }

    /// Validate the message checksum
    ///
    /// Calculates the checksum of the buffer, and checks for a match against the current checksum.
    fn validate_checksum(&self) -> Result<()> {
        let expected = checksum(self.checksum_bytes());
        let current = self.buf()[self.chk_index()];

        if expected == current {
            Ok(())
        } else {
            Err(Error::failure(format!(
                "invalid checksum, expected: {expected}, have: {current}"
            )))
        }
    }

    /// Get the message as a byte buffer
    ///
    /// Note: calculates the checksum, and sets the checksum byte.
    ///
    /// To get the buffer without calculating the checksum, use [as_bytes_unchecked](Self::as_bytes_unchecked)
    fn as_bytes(&mut self) -> &[u8] {
        self.calculate_checksum();
        self.buf()
    }

    /// Get a mutable reference to the byte buffer
    fn as_bytes_mut(&mut self) -> &mut [u8] {
        self.buf_mut()
    }

    /// Get the message as a byte buffer
    ///
    /// Note: does not perform checksum calculation, caller must call
    /// [calculate_checksum](Self::calculate_checksum) prior to calling this function.
    fn as_bytes_unchecked(&self) -> &[u8] {
        self.buf()
    }

    /// Deserializes a message type from a byte buffer.
    ///
    /// Returns `Ok(())` on success, and `Err(_)` for an invalid buffer.
    // FIXME: separate into another trait, and find a way to generically create message types
    #[allow(clippy::wrong_self_convention)]
    fn from_buf(&mut self, buf: &[u8]) -> Result<()> {
        if buf.len() < self.len() {
            return Err(Error::failure("invalid reply length"));
        }

        let stx = buf[index::STX];
        if stx != STX {
            return Err(Error::failure(format!(
                "invalid STX byte, expected: {STX}, have: {stx}"
            )));
        }

        let msg_len = buf[index::LEN] as usize;

        if msg_len != self.len() {
            return Err(Error::failure("invalid reply length"));
        }

        let etx = buf[msg_len - 2];
        if etx != ETX {
            return Err(Error::failure(format!(
                "invalid ETX byte, expected: {ETX}, have: {etx}"
            )));
        }

        validate_checksum(buf[..msg_len].as_ref())?;

        let control = Control::from(buf[index::CONTROL]);
        let msg_type = MessageType::from(control.message_type());
        let exp_msg_type = self.message_type();

        if msg_type != exp_msg_type {
            return Err(Error::failure(format!(
                "invalid message type, expected: {exp_msg_type}, have: {msg_type}"
            )));
        }

        self.buf_mut().copy_from_slice(buf[..msg_len].as_ref());

        Ok(())
    }
}

/// Validates a checksum matches the expected value.
///
/// Returns `Ok(())` on a match, `Err(_)` otherwise.
pub fn validate_checksum(buf: &[u8]) -> Result<()> {
    let len = buf.len();

    if !(len::MIN_MESSAGE..=len::MAX_MESSAGE).contains(&len) {
        return Err(Error::failure("invalid message length"));
    }

    let etx_index = len - 2;
    let chk_index = len - 1;

    let expected = checksum(buf[index::LEN..etx_index].as_ref());
    let current = buf[chk_index];

    if expected == current {
        Ok(())
    } else {
        Err(Error::failure(format!(
            "invalid checksum, expected: {expected}, have: {current}"
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u16_seven_bit() {
        let expected = 0x1234;
        let expected_bytes = [0x1, 0x2, 0x3, 0x4];

        assert_eq!(seven_bit_u16(expected_bytes.as_ref()), expected);
        assert_eq!(u16_seven_bit(expected), expected_bytes);

        assert_eq!(
            u16_seven_bit(seven_bit_u16(expected_bytes.as_ref())),
            expected_bytes
        );
        assert_eq!(seven_bit_u16(u16_seven_bit(expected).as_ref()), expected);

        for num in 0..u16::MAX {
            let n = num as u16;
            assert_eq!(seven_bit_u16(u16_seven_bit(n).as_ref()), n);
        }
    }

    #[test]
    fn test_u8_seven_bit() {
        let expected = 0x54;
        let expected_bytes = [0x5, 0x4];

        assert_eq!(u8_seven_bit(expected), expected_bytes);
        assert_eq!(seven_bit_u8(expected_bytes.as_ref()), expected);
    }
}
