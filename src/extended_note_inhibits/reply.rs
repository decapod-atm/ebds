use crate::std;
use std::fmt;

use crate::{
    impl_default, impl_extended_reply_ops, impl_message_ops, impl_omnibus_extended_reply,
    len::EXTENDED_NOTE_INHIBITS_REPLY_ALT, ExtendedCommand, ExtendedReplyOps, MessageOps,
    MessageType, OmnibusReply,
};

pub mod index {
    pub const DATA: usize = 4;
}

/// Set Extended Note Inhibits - Reply (Subtype 0x03)
/// Represents a reply for a [SetExtendedNoteInhibits](crate::SetExtendedNoteInhibits) command.
///
/// The device will reply with a standard Omnibus reply detailed in section 7.1.2 with no extended data.
pub type ExtendedNoteInhibitsReply = OmnibusReply;

/// Represents an alternate reply for a [SetExtendedNoteInhibits](crate::SetExtendedNoteInhibits) command.
///
/// In some firmware, an alternate reply is given. This reply also contains no extended data.
///
/// The Extended Note Inhibits Alternate Reply is formatted as follows:
///
/// | Name  | STX  | LEN  | CTRL | Subtype | Data 0 | Data 1 | Data 2 | Data 3 | Data 4 | Data 5 | ETX  | CHK |
/// |:------|:----:|:----:|:----:|:-------:|:------:|:------:|:------:|:------:|:------:|:------:|:----:|:---:|
/// | Byte  | 0    | 1    | 2    | 3       | 4      | 5      | 6      | 7      | 8      | 9      | 10   | 11  |
/// | Value | 0x02 | 0x0C | 0x7n | 0x03    | nn     | nn     | nn     | nn     | nn     | nn     | 0x03 | zz  |
///
/// **WARNING** In order to avoid possible confusion processing the extended note data, this command should
/// only be sent when the device is in the idle state.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ExtendedNoteInhibitsReplyAlt {
    buf: [u8; EXTENDED_NOTE_INHIBITS_REPLY_ALT],
}

impl ExtendedNoteInhibitsReplyAlt {
    /// Creates a new [ExtendedNoteInhibitsReplyAlt] message.
    pub fn new() -> Self {
        let mut message = Self {
            buf: [0u8; EXTENDED_NOTE_INHIBITS_REPLY_ALT],
        };

        message.init();
        message.set_message_type(MessageType::Extended);
        message.set_extended_command(ExtendedCommand::SetExtendedNoteInhibits);

        message
    }
}

impl_default!(ExtendedNoteInhibitsReplyAlt);
impl_message_ops!(ExtendedNoteInhibitsReplyAlt);
impl_omnibus_extended_reply!(ExtendedNoteInhibitsReplyAlt);
impl_extended_reply_ops!(ExtendedNoteInhibitsReplyAlt);

impl From<ExtendedNoteInhibitsReply> for ExtendedNoteInhibitsReplyAlt {
    fn from(msg: ExtendedNoteInhibitsReply) -> Self {
        use crate::index as omnibus_index;

        let msg_buf = msg.buf();
        let msg_etx_index = msg.etx_index();

        let mut res = Self::new();
        let res_etx_index = res.etx_index();
        let res_buf = res.buf_mut();

        res_buf[index::DATA..res_etx_index]
            .copy_from_slice(msg_buf[omnibus_index::DATA..msg_etx_index].as_ref());

        res.calculate_checksum();

        res
    }
}

impl fmt::Display for ExtendedNoteInhibitsReplyAlt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AckNak: {}, DeviceType: {}, MessageType: {}, ExtendedCommand: {}, DeviceState: {}, DeviceStatus: {}, ExceptionStatus: {}, MiscDeviceState: {}, ModelNumber: {}, CodeRevision: {}",
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
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;

    #[test]
    #[rustfmt::skip]
    fn test_extended_note_inhibits_reply_alt_from_bytes() -> Result<()> {
        let msg_bytes = [
            // STX | LEN | Message type | Subtype
            0x02, 0x0c, 0x70, 0x03,
            // Data
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            // ETX | Checksum
            0x03, 0x7f,
        ];

        let mut msg = ExtendedNoteInhibitsReplyAlt::new();
        msg.from_buf(msg_bytes.as_ref())?;

        assert_eq!(msg.message_type(), MessageType::Extended);
        assert_eq!(
            msg.extended_command(),
            ExtendedCommand::SetExtendedNoteInhibits
        );

        Ok(())
    }
}
