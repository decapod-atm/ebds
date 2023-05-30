use crate::std;
use std::fmt;

use crate::{
    impl_default, impl_message_ops, impl_omnibus_nop_reply, len::FLASH_DOWNLOAD_REPLY_8BIT,
    MessageOps, MessageType,
};

use super::FlashDownloadReply;

pub mod index {
    pub const PACKET0: usize = 3;
    pub const PACKET1: usize = 4;
}

/// Flash Download Reply - 8-bit protocol (Fast serial algorithm)
///
/// The Packet Number reported by the device is the last successfully received packet number in little
/// endian format.
///
/// The ACK/NAK of the reply is important.
///
/// * If the device ACKs the packet, the host should step to the next packet.
///
/// * If the device NAKs the packet, then the host needs to resynchronize with the device. This is
/// accomplished by changing the block number to the value contained in the reply plus one.
///
/// The Flash Download Reply (8-bit) is formatted as follows:
///
/// | Name  | STX  | LEN  | CTRL | Packet #0 | Packet #1 | ETX  | CHK |
/// |:------|:----:|:----:|:----:|:---------:|:---------:|:----:|:---:|
/// | Byte  | 0    | 1    | 2    | 3         | 4         | 5    | 6   |
/// | Value | 0x02 | 0x07 | 0x5n | 0xnn      | 0xnn      | 0x03 | zz  |
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FlashDownloadReply8bit {
    buf: [u8; FLASH_DOWNLOAD_REPLY_8BIT],
}

impl FlashDownloadReply8bit {
    /// Creates a new [FlashDownloadReply8bit] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; FLASH_DOWNLOAD_REPLY_8BIT],
        };

        msg.init();
        msg.set_message_type(MessageType::FirmwareDownload);

        msg
    }
}

impl_default!(FlashDownloadReply8bit);
impl_message_ops!(FlashDownloadReply8bit);
impl_omnibus_nop_reply!(FlashDownloadReply8bit);

impl FlashDownloadReply for FlashDownloadReply8bit {
    fn packet_number(&self) -> u16 {
        // In the 8-bit protocol, packet numbers are stored as 16-bit little endian values
        u16::from_le_bytes([self.buf[index::PACKET0], self.buf[index::PACKET1]])
    }

    fn set_packet_number(&mut self, n: u16) {
        // In the 8-bit protocol, packet numbers are stored as 16-bit little endian values
        self.buf[index::PACKET0..=index::PACKET1].copy_from_slice(n.to_le_bytes().as_ref());
    }
}

impl fmt::Display for FlashDownloadReply8bit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AckNak: {}, DeviceType: {}, MessageType: {}, PacketNumber: {}",
            self.acknak(),
            self.device_type(),
            self.message_type(),
            self.packet_number(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;

    #[test]
    #[rustfmt::skip]
    fn flash_download_reply_8bit_from_buf() -> Result<()> {
        let msg_bytes = [
            // STX | LEN | Message Type
            0x02, 0x07, 0x50,
            // Packet number (little-endian)
            0x34, 0x12,
            // ETX | Checksum
            0x03, 0x71,
        ];

        let mut msg = FlashDownloadReply8bit::new();
        msg.from_buf(msg_bytes.as_ref())?;

        assert_eq!(msg.message_type(), MessageType::FirmwareDownload);
        assert_eq!(msg.packet_number(), 0x1234);

        Ok(())
    }
}
