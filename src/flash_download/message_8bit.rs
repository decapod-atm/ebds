use crate::std;
use std::fmt;

use crate::{
    impl_default, impl_message_ops,
    len::{
        FLASH_DATA_PACKET, FLASH_DATA_PACKET_64, FLASH_DOWNLOAD_MESSAGE_8BIT_32,
        FLASH_DOWNLOAD_MESSAGE_8BIT_64,
    },
    MessageOps, MessageType,
};

use super::FlashDownloadMessage;

pub mod index {
    pub const PACKET0: usize = 3;
    pub const PACKET1: usize = 4;
    pub const DATA0: usize = 5;
    pub const DATA31: usize = 36;
    pub const DATA63: usize = 68;
}

/// Flash Download Message - 8-Bit protocol (Fast Serial Algorithm) (64-byte packet)
///
/// Starting at the beginning of the file, the host sends 64 byte blocks of data to the device. Downloading is
/// completed through the Download Data command shown below. The full 8-bits can be used for packet
/// numbers and data packets.
///
/// **Warning** The file is required to be a multiple of 32 bytes long therefore it is possible for the final
/// packet to only contain the remaining 32 data bytes. Do not pad the message with empty values.
///
/// The Flash Download Message (8-bit, 64-byte packet) is formatted as follows:
///
/// | Name  | STX  | LEN  | CTRL | Packet #0 | Packet #1 | Data 0 | ...  | Data 63 | ETX  | CHK |
/// |:------|:----:|:----:|:----:|:---------:|:---------:|:------:|:----:|:-------:|:----:|:---:|
/// | Byte  | 0    | 1    | 2    | 3         | 4         | 5      | ...  | 68      | 69   | 70  |
/// | Value | 0x02 | 0x47 | 0x5n | 0xnn      | 0xnn      | 0xnn   | 0xnn | 0xnn    | 0x03 | zz  |
///
/// **Note**: the 16-bit packet numbers are stored in little-endian format (least-significant byte first)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FlashDownloadMessage8bit_64 {
    buf: [u8; FLASH_DOWNLOAD_MESSAGE_8BIT_64],
}

impl FlashDownloadMessage8bit_64 {
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; FLASH_DOWNLOAD_MESSAGE_8BIT_64],
        };

        msg.init();
        msg.set_message_type(MessageType::FirmwareDownload);

        msg
    }
}

impl_default!(FlashDownloadMessage8bit_64);
impl_message_ops!(FlashDownloadMessage8bit_64);

impl FlashDownloadMessage<FLASH_DATA_PACKET_64> for FlashDownloadMessage8bit_64 {
    fn packet_number(&self) -> u16 {
        u16::from_le_bytes([self.buf[index::PACKET0], self.buf[index::PACKET1]])
    }

    fn set_packet_number(&mut self, n: u16) {
        self.buf[index::PACKET0..=index::PACKET1].copy_from_slice(n.to_le_bytes().as_ref());
    }

    fn increment_packet_number(&mut self) -> u16 {
        // FIXME: the desired behavior of an increment past the max is unclear.
        //
        // C behavior is to overflow, restarting at 0, but it isn't obvious that's what CPI
        // intends.
        //
        // FWIW their firmware files all appear to be below the limit.
        let packet_number = self.packet_number().overflowing_add(1).0;
        self.set_packet_number(packet_number);
        packet_number
    }

    fn data(&self) -> [u8; FLASH_DATA_PACKET_64] {
        // The unwrap is safe here, and can never panic because the slice is guaranteed to be the
        // correct length.
        self.buf[index::DATA0..=index::DATA63].try_into().unwrap()
    }

    fn data_ref(&self) -> &[u8] {
        self.buf[index::DATA0..=index::DATA63].as_ref()
    }

    fn set_data(&mut self, data: &[u8]) {
        assert_eq!(data.len(), FLASH_DATA_PACKET_64);

        self.buf[index::DATA0..=index::DATA63].copy_from_slice(data);
    }
}

impl fmt::Display for FlashDownloadMessage8bit_64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AckNak: {}, DeviceType: {}, MessageType: {}, PacketNumber: {}, Data: {:x?}",
            self.acknak(),
            self.device_type(),
            self.message_type(),
            self.packet_number(),
            self.data(),
        )
    }
}

/// Flash Download Message - 8-Bit protocol (Fast Serial Algorithm) (32-byte packet)
///
/// Starting at the beginning of the file, the host sends 64 byte blocks of data to the device. Downloading is
/// completed through the Download Data command shown below. The full 8-bits can be used for packet
/// numbers and data packets.
///
/// **Warning** The file is required to be a multiple of 32 bytes long therefore it is possible for the final
/// packet to only contain the remaining 32 data bytes. Do not pad the message with empty values.
///
/// The Flash Download Message (8-bit, 32-byte packet) is formatted as follows:
///
/// | Name  | STX  | LEN  | CTRL | Packet #0 | Packet #1 | Data 0 | ...  | Data 31 | ETX  | CHK |
/// |:------|:----:|:----:|:----:|:---------:|:---------:|:------:|:----:|:-------:|:----:|:---:|
/// | Byte  | 0    | 1    | 2    | 3         | 4         | 5      | ...  | 36      | 37   | 38  |
/// | Value | 0x02 | 0x27 | 0x5n | 0xnn      | 0xnn      | 0xnn   | 0xnn | 0xnn    | 0x03 | zz  |
///
/// **Note**: the 16-bit packet numbers are stored in little-endian format (least-significant byte first)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FlashDownloadMessage8bit_32 {
    buf: [u8; FLASH_DOWNLOAD_MESSAGE_8BIT_32],
}

impl FlashDownloadMessage8bit_32 {
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; FLASH_DOWNLOAD_MESSAGE_8BIT_32],
        };

        msg.init();
        msg.set_message_type(MessageType::FirmwareDownload);

        msg
    }
}

impl_default!(FlashDownloadMessage8bit_32);
impl_message_ops!(FlashDownloadMessage8bit_32);

impl FlashDownloadMessage<FLASH_DATA_PACKET> for FlashDownloadMessage8bit_32 {
    fn packet_number(&self) -> u16 {
        u16::from_le_bytes([self.buf[index::PACKET0], self.buf[index::PACKET1]])
    }

    fn set_packet_number(&mut self, n: u16) {
        self.buf[index::PACKET0..=index::PACKET1].copy_from_slice(n.to_le_bytes().as_ref());
    }

    fn increment_packet_number(&mut self) -> u16 {
        // FIXME: the desired behavior of an increment past the max is unclear.
        //
        // C behavior is to overflow, restarting at 0, but it isn't obvious that's what CPI
        // intends.
        //
        // FWIW their firmware files all appear to be below the limit.
        let packet_number = self.packet_number().overflowing_add(1).0;
        self.set_packet_number(packet_number);
        packet_number
    }

    fn data(&self) -> [u8; FLASH_DATA_PACKET] {
        // The unwrap is safe here, and can never panic because the slice is guaranteed to be the
        // correct length.
        self.buf[index::DATA0..=index::DATA31].try_into().unwrap()
    }

    fn data_ref(&self) -> &[u8] {
        self.buf[index::DATA0..=index::DATA31].as_ref()
    }

    fn set_data(&mut self, data: &[u8]) {
        assert_eq!(data.len(), FLASH_DATA_PACKET);

        self.buf[index::DATA0..=index::DATA31].copy_from_slice(data);
    }
}

impl fmt::Display for FlashDownloadMessage8bit_32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AckNak: {}, DeviceType: {}, MessageType: {}, PacketNumber: {}, Data: {:x?}",
            self.acknak(),
            self.device_type(),
            self.message_type(),
            self.packet_number(),
            self.data(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;

    #[test]
    #[rustfmt::skip]
    fn flash_download_message_8bit_64_from_buf() -> Result<()> {
        let msg_bytes = [
            // STX | LEN | Message Type
            0x02, 0x47, 0x50,
            // Packet number (little-endian)
            0x34, 0x12,
            // Data
            0x54, 0x01, 0x03, 0x04, 0x00, 0xa0, 0x00, 0x20, 0x00, 0x7f, 0x5a, 0x01, 0xc0, 0x1f, 0x00, 0x00,
            0x59, 0xae, 0x00, 0x20, 0x03, 0x00, 0x07, 0x32, 0x38, 0x36, 0x31, 0x30, 0x31, 0x31, 0x30, 0x33,
            0x54, 0x01, 0x03, 0x04, 0x00, 0xa0, 0x00, 0x20, 0x00, 0x7f, 0x5a, 0x01, 0xc0, 0x1f, 0x00, 0x00,
            0x59, 0xae, 0x00, 0x20, 0x03, 0x00, 0x07, 0x32, 0x38, 0x36, 0x31, 0x30, 0x31, 0x31, 0x03, 0x33,
            // ETX | Checksum
            0x03, 0x02,
        ];

        let mut msg = FlashDownloadMessage8bit_64::new();
        msg.from_buf(msg_bytes.as_ref())?;

        assert_eq!(msg.message_type(), MessageType::FirmwareDownload);
        assert_eq!(msg.packet_number(), 0x1234);
        assert_eq!(msg.data_ref(), msg_bytes[index::DATA0..=index::DATA63].as_ref());
        assert_eq!(msg.data().as_ref(), msg_bytes[index::DATA0..=index::DATA63].as_ref());

        assert_eq!(msg.increment_packet_number(), 0x1235);
        assert_eq!(msg.packet_number(), 0x1235);

        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn flash_download_message_8bit_32_from_buf() -> Result<()> {
        let msg_bytes = [
            // STX | LEN | Message Type
            0x02, 0x27, 0x50,
            // Packet number (little-endian)
            0x34, 0x12,
            // Data
            0x54, 0x01, 0x03, 0x04, 0x00, 0xa0, 0x00, 0x20, 0x00, 0x7f, 0x5a, 0x01, 0xc0, 0x1f, 0x00, 0x00,
            0x59, 0xae, 0x00, 0x20, 0x03, 0x00, 0x07, 0x32, 0x38, 0x36, 0x31, 0x30, 0x31, 0x31, 0x30, 0x33,
            // ETX | Checksum
            0x03, 0x95,
        ];

        let mut msg = FlashDownloadMessage8bit_32::new();
        msg.from_buf(msg_bytes.as_ref())?;

        assert_eq!(msg.message_type(), MessageType::FirmwareDownload);
        assert_eq!(msg.packet_number(), 0x1234);
        assert_eq!(msg.data_ref(), msg_bytes[index::DATA0..=index::DATA31].as_ref());
        assert_eq!(msg.data().as_ref(), msg_bytes[index::DATA0..=index::DATA31].as_ref());

        assert_eq!(msg.increment_packet_number(), 0x1235);
        assert_eq!(msg.packet_number(), 0x1235);

        Ok(())
    }
}
