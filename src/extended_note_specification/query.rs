use crate::{
    impl_default, impl_extended_ops, impl_message_ops, impl_omnibus_extended_command,
    len::QUERY_EXTENDED_NOTE_SPECIFICATION, ExtendedCommand, ExtendedCommandOps,
    ExtendedNoteReporting, MessageOps, MessageType, OmnibusCommandOps,
};

mod index {
    pub const NOTE_INDEX: usize = 7;
}

/// Extended Note Specification - Query (Subtype 0x02)
///
/// This message serves two purposes; one purpose for this message is to allow the host to query the
/// extended note details for a specified index. The other use is by the device to inform the host when a
/// bank note has reached the escrow position or been stacked.
///
/// The Query Extended Note Specification message is formatted as follows:
///
/// | Name  | STX  | LEN  | CTRL | Subtype | Data 0 | Data 1 | Data 2 | Index | ETX  | CHK |
/// |:------|:----:|:----:|:----:|:-------:|:------:|:------:|:------:|:-----:|:----:|:---:|
/// | Byte  | 0    | 1    | 2    | 3       | 4      | 5      | 6      | 7     | 8    | 9   |
/// | Value | 0x02 | 0x0A | 0x7n | 0x02    | nn     | nn     | nn     | nn    | 0x03 | zz  |
///
/// The first extended data byte is the Index. This index value starts at `1` and represents the index of the
/// extended note table data in the device.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QueryExtendedNoteSpecification {
    buf: [u8; QUERY_EXTENDED_NOTE_SPECIFICATION],
}

impl QueryExtendedNoteSpecification {
    /// Create a new [QueryExtendedNoteSpecification] message
    pub fn new() -> Self {
        let mut message = Self {
            buf: [0u8; QUERY_EXTENDED_NOTE_SPECIFICATION],
        };

        message.init();
        message.set_message_type(MessageType::Extended);
        message.set_extended_command(ExtendedCommand::ExtendedNoteSpecification);
        message.set_extended_note(ExtendedNoteReporting::Set);

        message
    }

    /// Get the note index being queried
    pub fn note_index(&self) -> usize {
        self.buf[index::NOTE_INDEX] as usize
    }

    /// Set the note index being queried
    pub fn set_note_index(&mut self, index: usize) {
        self.buf[index::NOTE_INDEX] = index as u8;
    }
}

impl_default!(QueryExtendedNoteSpecification);
impl_message_ops!(QueryExtendedNoteSpecification);
impl_extended_ops!(QueryExtendedNoteSpecification);
impl_omnibus_extended_command!(QueryExtendedNoteSpecification);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;

    #[test]
    #[rustfmt::skip]
    fn test_query_extended_note_specification_from_bytes() -> Result<()> {
        let msg_bytes = [
            // STX | LEN | Message type | Subtype
            0x02, 0x0a, 0x70, 0x02,
            // Data
            0x00, 0x00, 0x00,
            // Index
            0x01,
            // ETX | Checksum
            0x03, 0x79,
        ];

        let mut msg = QueryExtendedNoteSpecification::new();
        msg.from_buf(msg_bytes.as_ref())?;

        assert_eq!(msg.message_type(), MessageType::Extended);
        assert_eq!(msg.extended_command(), ExtendedCommand::ExtendedNoteSpecification);
        assert_eq!(msg.note_index(), 1);

        Ok(())
    }
}
