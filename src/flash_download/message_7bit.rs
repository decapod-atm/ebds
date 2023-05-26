use crate::std;
use std::fmt;

use crate::{
    impl_default, impl_message_ops,
    len::{FLASH_DATA_PACKET, FLASH_DOWNLOAD_MESSAGE_7BIT},
    seven_bit_u16, seven_bit_u8, u16_seven_bit, u8_seven_bit, MessageOps, MessageType,
};

use super::FlashDownloadMessage;

mod index {
    pub const PACKET0: usize = 3;
    //pub const PACKET1: usize = 4;
    //pub const PACKET2: usize = 5;
    pub const PACKET3: usize = 6;
    // The spec is inconsistent with its numbering scheme.
    // Correct here to zero-index for consistency
    pub const DATA0_HI: usize = 7;
    pub const DATA31_LO: usize = 70;
}

/// 7-Bit protocol (Original Algorithm)
///
/// Starting at the beginning of the file, the host sends 32 byte blocks of data to the device (Note: the file is
/// required to be a multiple of 32 bytes long and the MEI firmware files are configured as such).
/// Downloading is completed through the Download Data command shown below. Each data byte contains
/// only 4 bits of data located in the least significant nibble
///
/// The Flash Download Message (7-bit) is formatted as follows:
///
/// | Name  | STX  | LEN  | CTRL | Packet #0 | Packet #1 | Packet #2 | Packet #3 | Data 0 Hi | Data 0 Lo | ...  | Data 31 Hi | Data 31 Lo | ETX  | CHK |
/// |:------|:----:|:----:|:----:|:---------:|:---------:|:---------:|:---------:|:---------:|:---------:|:----:|:----------:|:----------:|:----:|:---:|
/// | Byte  | 0    | 1    | 2    | 3         | 4         | 5         | 6         | 7         | 8         | ...  | 69         | 70         | 71   | 72  |
/// | Value | 0x02 | 0x49 | 0x5n | 0x0n      | 0x0n      | 0x0n      | 0x0n      | 0x0n      | 0x0n      | 0x0n | 0x0n       | 0x0n       | 0x03 | zz  |
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FlashDownloadMessage7bit {
    buf: [u8; FLASH_DOWNLOAD_MESSAGE_7BIT],
}

impl FlashDownloadMessage7bit {
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; FLASH_DOWNLOAD_MESSAGE_7BIT],
        };

        msg.init();
        msg.set_message_type(MessageType::FirmwareDownload);

        msg
    }
}

impl_default!(FlashDownloadMessage7bit);
impl_message_ops!(FlashDownloadMessage7bit);

impl FlashDownloadMessage<FLASH_DATA_PACKET> for FlashDownloadMessage7bit {
    fn packet_number(&self) -> u16 {
        seven_bit_u16(self.buf[index::PACKET0..=index::PACKET3].as_ref())
    }

    fn set_packet_number(&mut self, n: u16) {
        self.buf[index::PACKET0..=index::PACKET3].copy_from_slice(u16_seven_bit(n).as_ref());
    }

    fn data(&self) -> [u8; FLASH_DATA_PACKET] {
        let mut ret = [0u8; FLASH_DATA_PACKET];
        for (i, b) in self.buf[index::DATA0_HI..=index::DATA31_LO]
            .chunks_exact(2)
            .enumerate()
        {
            ret[i] = seven_bit_u8(b);
        }
        ret
    }

    fn data_ref(&self) -> &[u8] {
        self.buf[index::DATA0_HI..=index::DATA31_LO].as_ref()
    }

    fn set_data(&mut self, data: &[u8]) {
        assert_eq!(data.len(), FLASH_DATA_PACKET);

        for (i, &b) in data.iter().enumerate() {
            let start = index::DATA0_HI + (i * 2);
            let end = start + 2;

            self.buf[start..end].copy_from_slice(u8_seven_bit(b).as_ref());
        }
    }
}

impl fmt::Display for FlashDownloadMessage7bit {
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
    fn flash_download_message_fns() {
        let mut msg = FlashDownloadMessage7bit::new();

        let data_packet = [
            0x54, 0x01, 0x03, 0x04, 0x00, 0xa0, 0x00, 0x20, 0x00, 0x7f, 0x5a, 0x01, 0xc0, 0x1f,
            0x00, 0x00, 0x59, 0xae, 0x00, 0x20, 0x03, 0x00, 0x07, 0x32, 0x38, 0x36, 0x31, 0x30,
            0x31, 0x31, 0x30, 0x33,
        ];

        let expected_encoding = [
            0x05, 0x04, 0x00, 0x01, 0x00, 0x03, 0x00, 0x04, 0x00, 0x00, 0x0a, 0x00, 0x00, 0x00,
            0x02, 0x00, 0x00, 0x00, 0x07, 0x0f, 0x05, 0x0a, 0x00, 0x01, 0x0c, 0x00, 0x01, 0x0f,
            0x00, 0x00, 0x00, 0x00, 0x05, 0x09, 0x0a, 0x0e, 0x00, 0x00, 0x02, 0x00, 0x00, 0x03,
            0x00, 0x00, 0x00, 0x07, 0x03, 0x02, 0x03, 0x08, 0x03, 0x06, 0x03, 0x01, 0x03, 0x00,
            0x03, 0x01, 0x03, 0x01, 0x03, 0x00, 0x03, 0x03,
        ];

        msg.set_data(data_packet.as_ref());

        assert_eq!(msg.data_ref(), expected_encoding.as_ref());
        assert_eq!(msg.data(), data_packet);
    }

    #[test]
    #[rustfmt::skip]
    fn flash_download_message_7bit_from_buf() -> Result<()> {
        let data_packet = [
            0x54, 0x01, 0x03, 0x04, 0x00, 0xa0, 0x00, 0x20, 0x00, 0x7f, 0x5a, 0x01, 0xc0, 0x1f,
            0x00, 0x00, 0x59, 0xae, 0x00, 0x20, 0x03, 0x00, 0x07, 0x32, 0x38, 0x36, 0x31, 0x30,
            0x31, 0x31, 0x30, 0x33,
        ];

        let msg_bytes = [
            // STX | LEN | Message Type
            0x02, 0x49, 0x50,
            // Packet number
            0x01, 0x02, 0x03, 0x04,
            // Data
            0x05, 0x04, 0x00, 0x01, 0x00, 0x03, 0x00, 0x04, 0x00, 0x00, 0x0a, 0x00, 0x00, 0x00,
            0x02, 0x00, 0x00, 0x00, 0x07, 0x0f, 0x05, 0x0a, 0x00, 0x01, 0x0c, 0x00, 0x01, 0x0f,
            0x00, 0x00, 0x00, 0x00, 0x05, 0x09, 0x0a, 0x0e, 0x00, 0x00, 0x02, 0x00, 0x00, 0x03,
            0x00, 0x00, 0x00, 0x07, 0x03, 0x02, 0x03, 0x08, 0x03, 0x06, 0x03, 0x01, 0x03, 0x00,
            0x03, 0x01, 0x03, 0x01, 0x03, 0x00, 0x03, 0x03,
            // ETX | Checksum
            0x03, 0x15,
        ];

        let mut msg = FlashDownloadMessage7bit::new();
        msg.from_buf(msg_bytes.as_ref())?;

        assert_eq!(msg.message_type(), MessageType::FirmwareDownload);
        assert_eq!(msg.packet_number(), 0x1234);
        assert_eq!(msg.data_ref(), msg_bytes[index::DATA0_HI..=index::DATA31_LO].as_ref());
        assert_eq!(msg.data(), data_packet);

        Ok(())
    }
}
