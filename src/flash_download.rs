use crate::MessageOps;

mod baud_rate;
mod message_7bit;
mod message_8bit;
mod reply_7bit;
mod reply_8bit;
mod start_download;

pub use baud_rate::*;
pub use message_7bit::*;
pub use message_8bit::*;
pub use reply_7bit::*;
pub use reply_8bit::*;
pub use start_download::*;

pub trait FlashDownloadMessage<const DATA_LEN: usize>: MessageOps {
    /// Gets whether this is the initial polling message for a firmware download session.
    fn is_initial_poll(&self) -> bool {
        false
    }

    /// Gets the packet number of the [FlashDownloadMessage].
    ///
    /// Represents the last successfully received packet number.
    fn packet_number(&self) -> u16;

    /// Sets the packet number of the [FlashDownloadMessage].
    fn set_packet_number(&mut self, n: u16);

    /// Increments the packet number by one.
    ///
    /// If the packet number reaches u16::MAX (65_535), any additional increments will overflow,
    /// starting the count back at zero.
    ///
    /// It isn't clear that is what the vendor intends, but that is the behavior in C.
    ///
    /// Without overflow, this limits the firmware size to:
    ///
    /// * 8-bit protocol: ~4MB (4_194_240 = 65_535 * 64)
    /// * 7-bit protocol: ~2MB (2_097_120 = 65_535 * 32)
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

    /// Gets the data bytes as a 32-byte array.
    ///
    /// Performs the conversion from the seven-bit protocol encoding (nibble-per-byte);
    fn data(&self) -> [u8; DATA_LEN];

    /// Gets a reference to the data bytes, as the raw protocol encoding.
    ///
    /// For 7-bit messages, this means each byte contains the significant bits in the lower nibble
    /// of the byte.
    ///
    /// For 8-bit messages, there is no special encoding.
    fn data_ref(&self) -> &[u8];

    /// Sets the data bytes from a user-supplied array.
    ///
    /// **NOTE** user must supply:
    ///
    /// * 7-bit protocol: 32-byte array
    /// * 8-bit protocol, 32-byte message: 32-byte array
    /// * 8-bit protocol, 64-byte message: 64-byte array
    ///
    /// Performs the conversion:
    ///
    /// * 7-bit protocol: nibble-per-byte encoding
    /// * 8-bit protocol: no conversion
    fn set_data(&mut self, data: &[u8]);
}

pub trait FlashDownloadReply: MessageOps {
    /// Gets the packet number of the [FlashDownloadReply].
    ///
    /// Represents the last successfully received packet number.
    fn packet_number(&self) -> u16;

    /// Sets the packet number of the [FlashDownloadReply].
    fn set_packet_number(&mut self, n: u16);

    /// Gets whether the device experienced power loss during firmware download.
    ///
    /// A true value indicates the host should begin firmware download from the first packet.
    fn power_loss(&self) -> bool {
        // A value of -1 (0xffff) indicates the device experienced power loss
        self.packet_number() == 0xffff
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SerialProtocol {
    /// 7-bit serial protocol, 7 data bits, Parity: even, 1 stop bits
    _7bit,
    /// 8-bit serial protocol, 8 data bits, Parity: none, 1 stop bits
    _8bit,
}
