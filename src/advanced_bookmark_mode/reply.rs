use crate::std;
use std::fmt;

use crate::{
    bool_enum, impl_default, impl_extended_ops, impl_message_ops, impl_omnibus_extended_reply,
    len::ADVANCED_BOOKMARK_MODE_REPLY, ExtendedCommand, ExtendedCommandOps, MessageOps,
    MessageType, OmnibusReplyOps,
};

pub mod index {
    pub const ACKNAK: usize = 10;
}

bool_enum!(
    AdvancedBookmarkAckNak,
    r"
Whether the Advance Bookmark Mode is enabled on the device.

May also indicate the device was busy (NAKs when stacking or powering up).
"
);

/// Advanced Bookmark Mode - Reply (Subtype 0x0D)
///
/// The device will respond with an ACK or NAK message.
///
/// The Advanced Bookmark Mode Reply is formatted as follows:
///
/// | Name  | STX  | LEN  | CTRL | Subtype | Data 0 | Data 1 | Data 2 | Data 3 | Data 4 | Data 5 | ACK/NAK | ETX  | CHK |
/// |:------|:----:|:----:|:----:|:-------:|:------:|:------:|:------:|:------:|:------:|:------:|:-------:|:----:|:---:|
/// | Byte  | 0    | 1    | 2    | 3       | 4      | 5      | 6      | 7      | 8      | 9      | 10      | 11   | 12  |
/// | Value | 0x02 | 0x0D | 0x7n | 0x04    | nn     | nn     | nn     | nn     | nn     | nn     | 0x00/01 | 0x03 | zz  |
///
/// If the device ACKs the message with 0x01, then the mode has been entered. The device may NAK the
/// message if it is currently busy (processing a note or powering up).
///
/// When the device stacks a document in this mode, the Standard Omnibus Reply (section 7.1.2) is
/// reported with the stacked bit set and no value reported in data byte 2.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AdvancedBookmarkModeReply {
    buf: [u8; ADVANCED_BOOKMARK_MODE_REPLY],
}

impl AdvancedBookmarkModeReply {
    /// Creates a new [AdvancedBookmarkModeReply].
    pub fn new() -> Self {
        let mut message = Self {
            buf: [0u8; ADVANCED_BOOKMARK_MODE_REPLY],
        };

        message.init();
        message.set_message_type(MessageType::Extended);
        message.set_extended_command(ExtendedCommand::AdvancedBookmark);

        message
    }

    /// Gets the ACKNAK reply of [AdvancedBookmarkModeReply].
    pub fn mode_acknak(&self) -> AdvancedBookmarkAckNak {
        self.buf[index::ACKNAK].into()
    }

    /// Sets the status of [AdvancedBookmarkModeReply].
    pub fn set_mode_acknak(&mut self, acknak: AdvancedBookmarkAckNak) {
        self.buf[index::ACKNAK] = acknak.into();
    }
}

impl_default!(AdvancedBookmarkModeReply);
impl_message_ops!(AdvancedBookmarkModeReply);
impl_omnibus_extended_reply!(AdvancedBookmarkModeReply);
impl_extended_ops!(AdvancedBookmarkModeReply);

impl fmt::Display for AdvancedBookmarkModeReply {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AckNak: {}, DeviceType: {}, MessageType: {}, Subtype: {}, DeviceState: {}, DeviceStatus: {}, ExceptionStatus: {}, MiscDeviceState: {}, ModelNumber: {}, CodeRevision: {}, ModeAckNak: {}",
            self.acknak(),
            self.device_type(),
            self.message_type(),
            self.extended_command(),
            self.device_state(),
            self.device_status(),
            self.exception_status(),
            self.misc_device_state(),
            self.model_number(),
            self.code_revision(),
            self.mode_acknak(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;

    #[test]
    #[rustfmt::skip]
    fn test_advanced_bookmark_reply_from_bytes() -> Result<()> {
        let msg_bytes = [
            // STX | LEN | Message type | Subtype
            0x02, 0x0d, 0x70, 0x0d,
            // Data
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            // ACK/NAK
            0x01,
            // ETX | Checksum
            0x03, 0x71,
        ];

        let mut msg = AdvancedBookmarkModeReply::new();
        msg.from_buf(msg_bytes.as_ref())?;

        assert_eq!(msg.message_type(), MessageType::Extended);
        assert_eq!(msg.extended_command(), ExtendedCommand::AdvancedBookmark);
        assert_eq!(msg.mode_acknak(), AdvancedBookmarkAckNak::Set);

        let msg_bytes = [
            // STX | LEN | Message type | Subtype
            0x02, 0x0d, 0x70, 0x0d,
            // Data
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            // ACK/NAK
            0x00,
            // ETX | Checksum
            0x03, 0x70,
        ];

        msg.from_buf(msg_bytes.as_ref())?;

        assert_eq!(msg.message_type(), MessageType::Extended);
        assert_eq!(msg.extended_command(), ExtendedCommand::AdvancedBookmark);
        assert_eq!(msg.mode_acknak(), AdvancedBookmarkAckNak::Unset);

        Ok(())
    }
}
