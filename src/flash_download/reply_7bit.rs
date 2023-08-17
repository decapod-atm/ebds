use crate::std;
use std::fmt;

use crate::{
    impl_message_ops, impl_omnibus_nop_reply, len::FLASH_DOWNLOAD_REPLY_7BIT, seven_bit_u16,
    u16_seven_bit, MessageOps, MessageType,
};

use super::FlashDownloadReply;

pub mod index {
    pub const PACKET0: usize = 3;
    //pub const PACKET1: usize = 4;
    //pub const PACKET2: usize = 5;
    pub const PACKET3: usize = 6;
}

/// Flash Download Reply - 7-bit protocol (Original algorithm)
///
/// The Packet Number reported by the device is the last successfully received packet number.
/// The ACK/NAK of the reply is important.
///
/// * If the device ACKs the packet, the host should step to the next packet.
///
/// * If the device NAKs the packet, then the host needs to resynchronize with the device. This is
/// accomplished by changing the block number to the value contained in the reply plus one. An
/// example is shown below.
///
/// Example:
///
/// ```rust
/// let rpn = [0x1, 0x2, 0x3, 0x4];
/// let hi = ((rpn[0] & 0xf) << 4) | (rpn[1] & 0xf);
/// let lo = ((rpn[2] & 0xf) << 4) | (rpn[3] & 0xf);
/// let packet_num = u16::from_be_bytes([hi, lo]);
/// assert_eq!(packet_num, 0x12_34);
///
/// let reply_block_num = packet_num + 1;
/// ```
///
/// The Flash Download Reply (7-bit) is formatted as follows:
///
/// | Name  | STX  | LEN  | CTRL | Packet #0 | Packet #1 | Packet #2 | Packet #3 | ETX  | CHK |
/// |:------|:----:|:----:|:----:|:---------:|:---------:|:---------:|:---------:|:----:|:---:|
/// | Byte  | 0    | 1    | 2    | 3         | 4         | 5         | 6         | 7    | 8   |
/// | Value | 0x02 | 0x09 | 0x5n | 0x0n      | 0x0n      | 0x0n      | 0x0n      | 0x03 | zz  |
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FlashDownloadReply7bit {
    buf: [u8; FLASH_DOWNLOAD_REPLY_7BIT],
}

impl FlashDownloadReply7bit {
    /// Creates a new [FlashDownloadReply7bit] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; FLASH_DOWNLOAD_REPLY_7BIT],
        };

        msg.init();
        msg.set_message_type(MessageType::FirmwareDownload);

        msg
    }
}

impl_message_ops!(FlashDownloadReply7bit);
impl_omnibus_nop_reply!(FlashDownloadReply7bit);

impl FlashDownloadReply for FlashDownloadReply7bit {
    fn packet_number(&self) -> u16 {
        // In the 7-bit protocol, packet numbers are stored as 16-bit big-endian numbers encoded as 4-byte slices with the
        // significant bits in the lower nibble of each byte
        seven_bit_u16(self.buf[index::PACKET0..=index::PACKET3].as_ref())
    }

    fn set_packet_number(&mut self, n: u16) {
        // In the 7-bit protocol, packet numbers are stored as 16-bit big-endian numbers encoded as 4-byte slices with the
        // significant bits in the lower nibble of each byte
        self.buf[index::PACKET0..=index::PACKET3].copy_from_slice(u16_seven_bit(n).as_ref());
    }
}

impl fmt::Display for FlashDownloadReply7bit {
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
    fn flash_download_reply_7bit_from_buf() -> Result<()> {
        let msg_bytes = [
            // STX | LEN | Message Type
            0x02, 0x09, 0x50,
            // Packet number
            0x01, 0x02, 0x03, 0x04,
            // ETX | Checksum
            0x03, 0x5d,
        ];

        let mut msg = FlashDownloadReply7bit::new();
        msg.from_buf(msg_bytes.as_ref())?;

        assert_eq!(msg.message_type(), MessageType::FirmwareDownload);
        assert_eq!(msg.packet_number(), 0x1234);

        Ok(())
    }
}
