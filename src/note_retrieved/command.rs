use crate::{
    bool_enum, impl_default, impl_extended_ops, impl_message_ops, impl_omnibus_extended_command,
    len::NOTE_RETRIEVED_COMMAND, ExtendedCommand, ExtendedCommandOps, MessageOps, MessageType,
};

mod index {
    pub const STATUS: usize = 7;
}

bool_enum!(
    Status,
    "Whether to enable/disable Note Retrieved functionality"
);

/// Note Retrieved - Command (Subtype 0x0B)
///
/// [NoteRetrievedCommand] represents a message sent to enable/disable Note Retrieved functionality.
///
/// The note retrieved message is used to turn on optional functionality in the device. Some software has
/// the ability to report to the host when the customer has retrieved a returned or rejected note from the
/// device. Once a document is returned/rejected by the device, the document will sit partially in the device
/// in a manner that is convenient for the customer. This message allows the host to detect the moment the
/// customer removes the note from the mouth of the device.
///
/// The host must enable this feature every time the device powers up. This is done with the following
/// message. (The same message structure can also be used to disable the feature).
///
/// **Warning** The note retrieved event is disabled by default and is required to be turned on explicitly by
/// the host every time the device powers up.
///
/// The Note Retrieved Command is formatted as follows:
///
/// | Name  | STX  | LEN  | CTRL | Subtype | Data 0 | Data 1 | Data 2 | Status  | ETX  | CHK |
/// |:------|:----:|:----:|:----:|:-------:|:------:|:------:|:------:|:-------:|:----:|:---:|
/// | Byte  | 0    | 1    | 2    | 3       | 4      | 5      | 6      | 7       | 8    | 9   |
/// | Value | 0x02 | 0x0A | 0x7n | 0x0B    | nn     | nn     | nn     | 0x00/01 | 0x03 | zz  |
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NoteRetrievedCommand {
    buf: [u8; NOTE_RETRIEVED_COMMAND],
}

impl NoteRetrievedCommand {
    pub fn new() -> Self {
        let mut message = Self {
            buf: [0u8; NOTE_RETRIEVED_COMMAND],
        };

        message.init();
        message.set_message_type(MessageType::Extended);
        message.set_extended_command(ExtendedCommand::NoteRetrieved);

        message
    }

    /// Gets the [Status] data field.
    pub fn status(&self) -> Status {
        self.buf[index::STATUS].into()
    }

    /// Sets the [Status] data field.
    pub fn set_status(&mut self, status: Status) {
        self.buf[index::STATUS] = status.into();
    }
}

impl_default!(NoteRetrievedCommand);
impl_message_ops!(NoteRetrievedCommand);
impl_omnibus_extended_command!(NoteRetrievedCommand);
impl_extended_ops!(NoteRetrievedCommand);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;

    #[test]
    #[rustfmt::skip]
    fn test_note_retrieved_command_from_buf() -> Result<()> {
        let msg_bytes = [
            // STX | LEN | Message Type | Subtype
            0x02, 0x0a, 0x70, 0x0b,
            // Data
            0x00, 0x00, 0x00,
            // Status
            0x01,
            // ETX | Checksum
            0x03, 0x70,
        ];

        let mut msg = NoteRetrievedCommand::new();
        msg.from_buf(msg_bytes.as_ref())?;

        assert_eq!(msg.message_type(), MessageType::Extended);
        assert_eq!(msg.extended_command(), ExtendedCommand::NoteRetrieved);
        assert_eq!(msg.status(), Status::Set);

        let msg_bytes = [
            // STX | LEN | Message Type | Subtype
            0x02, 0x0a, 0x70, 0x0b,
            // Data
            0x00, 0x00, 0x00,
            // Status
            0x00,
            // ETX | Checksum
            0x03, 0x71,
        ];

        msg.from_buf(msg_bytes.as_ref())?;

        assert_eq!(msg.message_type(), MessageType::Extended);
        assert_eq!(msg.extended_command(), ExtendedCommand::NoteRetrieved);
        assert_eq!(msg.status(), Status::Unset);

        Ok(())
    }
}
