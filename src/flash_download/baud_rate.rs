use crate::std;
use std::fmt;

use crate::{
    impl_message_ops, impl_omnibus_nop_reply,
    len::{BAUD_CHANGE_REPLY, BAUD_CHANGE_REQUEST},
    MessageOps, MessageType,
};

#[allow(dead_code)]
pub mod index {
    pub const DATA0: usize = 3;
    pub const BAUD_RATE: usize = 3;
}

/// Represents the acceptable values for host-device serial baud rates.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum BaudRate {
    #[default]
    _9600 = 0x01,
    _19200 = 0x02,
    _38400 = 0x03,
    _115200 = 0x04,
    Reserved = 0xff,
}

impl From<u8> for BaudRate {
    fn from(b: u8) -> Self {
        match b {
            0x01 => Self::_9600,
            0x02 => Self::_19200,
            0x03 => Self::_38400,
            0x04 => Self::_115200,
            _ => Self::Reserved,
        }
    }
}

impl From<u32> for BaudRate {
    fn from(b: u32) -> Self {
        match b {
            9_600 => Self::_9600,
            19_200 => Self::_19200,
            38_400 => Self::_38400,
            115_200 => Self::_115200,
            _ => Self::_9600,
        }
    }
}

impl From<BaudRate> for u8 {
    fn from(b: BaudRate) -> Self {
        b as u8
    }
}

impl From<&BaudRate> for u8 {
    fn from(b: &BaudRate) -> Self {
        (*b).into()
    }
}

impl From<BaudRate> for u32 {
    fn from(b: BaudRate) -> Self {
        match b {
            BaudRate::_9600 => 9_600,
            BaudRate::_19200 => 19_200,
            BaudRate::_38400 => 38_400,
            BaudRate::_115200 => 115_200,
            _ => 9_600,
        }
    }
}

impl From<&BaudRate> for u32 {
    fn from(b: &BaudRate) -> Self {
        (*b).into()
    }
}

impl From<BaudRate> for &'static str {
    fn from(b: BaudRate) -> Self {
        match b {
            BaudRate::_9600 => "9600",
            BaudRate::_19200 => "19200",
            BaudRate::_38400 => "38400",
            BaudRate::_115200 => "115200",
            BaudRate::Reserved => "Reserved",
        }
    }
}

impl From<&BaudRate> for &'static str {
    fn from(b: &BaudRate) -> Self {
        (*b).into()
    }
}

impl fmt::Display for BaudRate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let b: &str = self.into();
        write!(f, "{b}")
    }
}

/// Setting Baud Rate (Optional)
///
/// An optional feature available on selected versions of firmware is the ability to perform a "Fast Serial
/// Download". To support this feature, the serial port settings need to be adjusted prior to downloading
/// packets. The serial port settings shall be reverted back to the original values after the last packet has
/// been acknowledged by the device.
///
/// | STX  | LEN  | CTRL | DATA0 | ETX  | CHK  |
/// |------|------|------|-------|------|------|
/// | 0x02 | 0x06 | 0x5n | baud  | 0x03 | zz   |
///
/// | Data0 | Description                                               |
/// |-------|-----------------------------------------------------------|
/// | 0x01  | Baud Rate 9600; Data Bits 8, Parity None; Stop Bit 0ne    |
/// | 0x02  | Baud Rate 19,200; Data Bits 8, Parity None; Stop Bit 0ne  |
/// | 0x03  | Baud Rate 38,400; Data Bits 8, Parity None; Stop Bit 0ne  |
/// | 0x04  | Baud Rate 115,200; Data Bits 8, Parity None; Stop Bit 0ne |
/// | Other | Reserved                                                  |
///
/// **Warning** If the device firmware does not support the fast serial download feature, the device will not
/// respond to the baud rate change request. This means the host will be required to perform the download
/// using the original algorithm.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BaudRateChangeRequest {
    buf: [u8; BAUD_CHANGE_REQUEST],
}

impl BaudRateChangeRequest {
    /// Creates a new [BaudRateChangeRequest] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; BAUD_CHANGE_REQUEST],
        };

        msg.init();
        msg.set_message_type(MessageType::FirmwareDownload);

        msg
    }

    /// Gets the [BaudRate] for the [BaudRateChangeRequest].
    pub fn baud_rate(&self) -> BaudRate {
        self.buf[index::DATA0].into()
    }

    /// Sets the [BaudRate] for the [BaudRateChangeRequest].
    pub fn set_baud_rate(&mut self, baud_rate: BaudRate) {
        match baud_rate {
            BaudRate::Reserved => {
                self.buf[index::DATA0] = BaudRate::_9600.into();
            }
            _ => {
                self.buf[index::DATA0] = baud_rate.into();
            }
        }
    }
}

impl_message_ops!(BaudRateChangeRequest);

impl fmt::Display for BaudRateChangeRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AckNak: {}, DeviceType: {}, MessageType: {}, BaudRate: {}",
            self.acknak(),
            self.device_type(),
            self.message_type(),
            self.baud_rate(),
        )
    }
}

/// Setting Baud Rate Reply (Optional)
///
/// If supported, the device will respond back to the command with an ACK (toggle bits will be the same)
/// and the Data 0 response will equal the requested value. (ex. if the host requests 19,200, the device will
/// respond with 19,200). If the device supports fast download but cannot support the requested baud rate,
/// the device will NAK the request but transmit the maximum supported value (ex. Host requests 115,200
/// on SC Advance unit, device will NAK and return 38,400).
///
/// | STX  | LEN  | CTRL | DATA0 | ETX  | CHK  |
/// |------|------|------|-------|------|------|
/// | 0x02 | 0x06 | 0x5n | baud  | 0x03 | zz   |
///
/// | Data0 | Description                                               |
/// |-------|-----------------------------------------------------------|
/// | 0x01  | Baud Rate 9600; Data Bits 8, Parity None; Stop Bit 0ne    |
/// | 0x02  | Baud Rate 19,200; Data Bits 8, Parity None; Stop Bit 0ne  |
/// | 0x03  | Baud Rate 38,400; Data Bits 8, Parity None; Stop Bit 0ne  |
/// | 0x04  | Baud Rate 115,200; Data Bits 8, Parity None; Stop Bit 0ne |
/// | Other | Reserved                                                  |
///
/// **Warning** If the device firmware does not support the fast serial download feature, the device will not
/// respond to the baud rate change request. This means the host will be required to perform the download
/// using the original algorithm.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BaudRateChangeReply {
    buf: [u8; BAUD_CHANGE_REPLY],
}

impl BaudRateChangeReply {
    /// Creates a new [BaudRateChangeReply] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; BAUD_CHANGE_REQUEST],
        };

        msg.init();
        msg.set_message_type(MessageType::FirmwareDownload);

        msg
    }

    /// Gets the [BaudRate] for the [BaudRateChangeReply].
    pub fn baud_rate(&self) -> BaudRate {
        self.buf[index::DATA0].into()
    }

    /// Sets the [BaudRate] for the [BaudRateChangeReply].
    pub fn set_baud_rate(&mut self, baud_rate: BaudRate) {
        match baud_rate {
            BaudRate::Reserved => {
                self.buf[index::DATA0] = BaudRate::_9600.into();
            }
            _ => {
                self.buf[index::DATA0] = baud_rate.into();
            }
        }
    }
}

impl_message_ops!(BaudRateChangeReply);
impl_omnibus_nop_reply!(BaudRateChangeReply);

impl fmt::Display for BaudRateChangeReply {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AckNak: {}, DeviceType: {}, MessageType: {}, BaudRate: {}",
            self.acknak(),
            self.device_type(),
            self.message_type(),
            self.baud_rate(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;

    #[test]
    #[rustfmt::skip]
    fn test_baud_rate_change_request_from_bytes() -> Result<()> {
        let mut msg_bytes = [
            // STX | LEN | Message type
            0x02, 0x06, 0x50,
            // Baud rate (9,600)
            0x01,
            // ETX | Checksum
            0x03, 0x57,
        ];

        let mut msg = BaudRateChangeRequest::new();
        msg.from_buf(msg_bytes.as_ref())?;

        assert_eq!(msg.message_type(), MessageType::FirmwareDownload);
        assert_eq!(msg.baud_rate(), BaudRate::_9600);

        msg_bytes = [
            // STX | LEN | Message type
            0x02, 0x06, 0x50,
            // Baud rate (19,200)
            0x02,
            // ETX | Checksum
            0x03, 0x54,
        ];

        msg.from_buf(msg_bytes.as_ref())?;

        assert_eq!(msg.message_type(), MessageType::FirmwareDownload);
        assert_eq!(msg.baud_rate(), BaudRate::_19200);

        msg_bytes = [
            // STX | LEN | Message type
            0x02, 0x06, 0x50,
            // Baud rate (38,400)
            0x03,
            // ETX | Checksum
            0x03, 0x55,
        ];

        msg.from_buf(msg_bytes.as_ref())?;

        assert_eq!(msg.message_type(), MessageType::FirmwareDownload);
        assert_eq!(msg.baud_rate(), BaudRate::_38400);

        msg_bytes = [
            // STX | LEN | Message type
            0x02, 0x06, 0x50,
            // Baud rate (115,200)
            0x04,
            // ETX | Checksum
            0x03, 0x52,
        ];

        msg.from_buf(msg_bytes.as_ref())?;

        assert_eq!(msg.message_type(), MessageType::FirmwareDownload);
        assert_eq!(msg.baud_rate(), BaudRate::_115200);

        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn test_baud_rate_change_reply_from_bytes() -> Result<()> {
        let mut msg_bytes = [
            // STX | LEN | Message type
            0x02, 0x06, 0x50,
            // Baud rate (9,600)
            0x01,
            // ETX | Checksum
            0x03, 0x57,
        ];

        let mut msg = BaudRateChangeReply::new();
        msg.from_buf(msg_bytes.as_ref())?;

        assert_eq!(msg.message_type(), MessageType::FirmwareDownload);
        assert_eq!(msg.baud_rate(), BaudRate::_9600);

        msg_bytes = [
            // STX | LEN | Message type
            0x02, 0x06, 0x50,
            // Baud rate (19,200)
            0x02,
            // ETX | Checksum
            0x03, 0x54,
        ];

        msg.from_buf(msg_bytes.as_ref())?;

        assert_eq!(msg.message_type(), MessageType::FirmwareDownload);
        assert_eq!(msg.baud_rate(), BaudRate::_19200);

        msg_bytes = [
            // STX | LEN | Message type
            0x02, 0x06, 0x50,
            // Baud rate (38,400)
            0x03,
            // ETX | Checksum
            0x03, 0x55,
        ];

        msg.from_buf(msg_bytes.as_ref())?;

        assert_eq!(msg.message_type(), MessageType::FirmwareDownload);
        assert_eq!(msg.baud_rate(), BaudRate::_38400);

        msg_bytes = [
            // STX | LEN | Message type
            0x02, 0x06, 0x50,
            // Baud rate (115,200)
            0x04,
            // ETX | Checksum
            0x03, 0x52,
        ];

        msg.from_buf(msg_bytes.as_ref())?;

        assert_eq!(msg.message_type(), MessageType::FirmwareDownload);
        assert_eq!(msg.baud_rate(), BaudRate::_115200);

        Ok(())
    }
}
