use crate::std;
use std::fmt;

use crate::{
    bool_enum, impl_extended_ops, impl_message_ops, impl_omnibus_extended_reply,
    len::{NOTE_RETRIEVED_EVENT, NOTE_RETRIEVED_REPLY},
    ExtendedCommand, ExtendedCommandOps, MessageOps, MessageType, OmnibusReplyOps,
};

pub const EVENT: u8 = 0x7f;

bool_enum!(
    RetrieveAckNak,
    "Indicates success(0x01) / failure(0x00) of the NoteRetrievedCommand"
);

pub mod index {
    pub const ACKNAK: usize = 10;
    pub const EVENT: usize = 10;
}

/// Note Retrieved - Reply (Subtype 0x0B)
///
/// NoteRetrievedReply represents an immediate reply to the
/// [NoteRetrievedCommand](crate::NoteRetrievedCommand).
///
/// The device will respond to the enable/disable command with an ACK or NAK.
///
/// The device ACKs with 0x01 if it can honor the hosts request to either enable or disable. The device will
/// NAK the command if it is not supported for the current configuration. (ex. BNF is attached).
///
/// The Note Retrieved Reply is formatted as follows:
///
/// | Name  | STX  | LEN  | CTRL | Subtype | Data 0 | Data 1 | Data 2 | Data 3 | Data 4 | Data 5 | ACK/NAK | ETX  | CHK |
/// |:------|:----:|:----:|:----:|:-------:|:------:|:------:|:------:|:------:|:------:|:------:|:-------:|:----:|:---:|
/// | Byte  | 0    | 1    | 2    | 3       | 4      | 5      | 6      | 7      | 8      | 9      | 10      | 11   | 12  |
/// | Value | 0x02 | 0x0D | 0x7n | 0x0B    | nn     | nn     | nn     | nn     | nn     | nn     | 0x01/00 | 0x03 | zz  |
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NoteRetrievedReply {
    buf: [u8; NOTE_RETRIEVED_REPLY],
}

impl NoteRetrievedReply {
    pub fn new() -> Self {
        let mut message = Self {
            buf: [0u8; NOTE_RETRIEVED_REPLY],
        };

        message.init();
        message.set_message_type(MessageType::Extended);
        message.set_extended_command(ExtendedCommand::NoteRetrieved);

        message
    }

    pub fn retrieved_acknak(&self) -> RetrieveAckNak {
        self.buf[index::ACKNAK].into()
    }

    pub fn set_retrieved_acknak(&mut self, acknak: RetrieveAckNak) {
        self.buf[index::ACKNAK] = acknak.into()
    }
}

impl_message_ops!(NoteRetrievedReply);
impl_omnibus_extended_reply!(NoteRetrievedReply);
impl_extended_ops!(NoteRetrievedReply);

impl fmt::Display for NoteRetrievedReply {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AckNak: {}, DeviceType: {}, MessageType: {}, DeviceState: {}, DeviceStatus: {}, ExceptionStatus: {}, MiscDeviceState: {}, ModelNumber: {}, CodeRevision: {}, Retrieved AckNak: {}",
            self.acknak(),
            self.device_type(),
            self.message_type(),
            self.device_state(),
            self.device_status(),
            self.exception_status(),
            self.misc_device_state(),
            self.model_number(),
            self.code_revision(),
            self.retrieved_acknak(),
        )
    }
}

/// Note Retrieved - Event (Subtype 0x0B)
///
/// If the functionality has been enabled, the device will send out a message each time the note is removed
/// after a return/reject.
///
/// The Note Retrieved Event is formatted as follows:
///
/// | Name  | STX  | LEN  | CTRL | Subtype | Data 0 | Data 1 | Data 2 | Data 3 | Data 4 | Data 5 | Event | ETX  | CHK |
/// |:------|:----:|:----:|:----:|:-------:|:------:|:------:|:------:|:------:|:------:|:------:|:-----:|:----:|:---:|
/// | Byte  | 0    | 1    | 2    | 3       | 4      | 5      | 6      | 7      | 8      | 9      | 10    | 11   | 12  |
/// | Value | 0x02 | 0x0D | 0x7n | 0x0B    | nn     | nn     | nn     | nn     | nn     | nn     | 0x7F  | 0x03 | zz  |
///
/// The `0x7F` for the `Event byte signifies that the note has been removed by the user.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NoteRetrievedEvent {
    buf: [u8; NOTE_RETRIEVED_EVENT],
}

impl NoteRetrievedEvent {
    pub fn new() -> Self {
        let mut message = Self {
            buf: [0u8; NOTE_RETRIEVED_EVENT],
        };

        message.init();
        message.set_message_type(MessageType::Extended);
        message.set_extended_command(ExtendedCommand::NoteRetrieved);
        message.buf[index::EVENT] = EVENT;

        message
    }

    pub fn retrieved_event(&self) -> u8 {
        self.buf[index::EVENT]
    }
}

impl_message_ops!(NoteRetrievedEvent);
impl_omnibus_extended_reply!(NoteRetrievedEvent);
impl_extended_ops!(NoteRetrievedEvent);

impl fmt::Display for NoteRetrievedEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AckNak: {}, DeviceType: {}, MessageType: {}, DeviceState: {}, DeviceStatus: {}, ExceptionStatus: {}, MiscDeviceState: {}, ModelNumber: {}, CodeRevision: {}, Retrieved Event: {}",
            self.acknak(),
            self.device_type(),
            self.message_type(),
            self.device_state(),
            self.device_status(),
            self.exception_status(),
            self.misc_device_state(),
            self.model_number(),
            self.code_revision(),
            self.retrieved_event(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;

    #[test]
    #[rustfmt::skip]
    fn test_note_retrieved_reply_from_buf() -> Result<()> {
        let msg_bytes = [
            // STX | LEN | Message Type | Subtype
            0x02, 0x0d, 0x70, 0x0b,
            // Data
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            // ACK/NAK
            0x01,
            // ETX | Checksum
            0x03, 0x77,
        ];

        let mut msg = NoteRetrievedReply::new();
        msg.from_buf(msg_bytes.as_ref())?;

        assert_eq!(msg.message_type(), MessageType::Extended);
        assert_eq!(msg.extended_command(), ExtendedCommand::NoteRetrieved);
        assert_eq!(msg.retrieved_acknak(), RetrieveAckNak::Set);

        let msg_bytes = [
            // STX | LEN | Message Type | Subtype
            0x02, 0x0d, 0x70, 0x0b,
            // Data
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            // ACK/NAK
            0x00,
            // ETX | Checksum
            0x03, 0x76,
        ];

        msg.from_buf(msg_bytes.as_ref())?;

        assert_eq!(msg.message_type(), MessageType::Extended);
        assert_eq!(msg.extended_command(), ExtendedCommand::NoteRetrieved);
        assert_eq!(msg.retrieved_acknak(), RetrieveAckNak::Unset);

        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn test_note_retrieved_event_from_buf() -> Result<()> {
        let msg_bytes = [
            // STX | LEN | Message Type | Subtype
            0x02, 0x0d, 0x70, 0x0b,
            // Data
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            // Event
            0x7f,
            // ETX | Checksum
            0x03, 0x09,
        ];

        let mut msg = NoteRetrievedEvent::new();
        msg.from_buf(msg_bytes.as_ref())?;

        assert_eq!(msg.message_type(), MessageType::Extended);
        assert_eq!(msg.extended_command(), ExtendedCommand::NoteRetrieved);
        assert_eq!(msg.retrieved_event(), 0x7f);

        let msg_bytes = [
            // STX | LEN | Message Type | Subtype
            0x02, 0x0d, 0x70, 0x0b,
            // Data
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            // Event (any non-0x7f value is invalid)
            0x7e,
            // ETX | Checksum
            0x03, 0x08,
        ];

        msg.from_buf(msg_bytes.as_ref())?;

        assert_eq!(msg.retrieved_event(), 0x7e);

        Ok(())
    }
}
