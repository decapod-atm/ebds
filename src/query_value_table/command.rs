use crate::{
    impl_extended_ops, impl_message_ops, impl_omnibus_extended_command,
    len::QUERY_VALUE_TABLE_COMMAND, ExtendedCommand, ExtendedCommandOps, MessageOps, MessageType,
};

/// Query Value Table - Command (Subtype 0x06)
///
/// This command sends a request to the device for the entire note table. The device will respond with a
/// message containing all known denominations. The purpose of this message is to allow the host to know
/// the exact denomination of a note that is usually only reported as Note Index Value “X”.
///
/// **WARNING** This message is only compatible with variants of a single currency and running in Non-Extended Mode (4.2.1).
///
/// The Query Value Table Command is formatted as follows:
///
/// | Name  | STX  | LEN  | CTRL | Subtype | Data 0 | Data 1 | Data 2 | ETX  | CHK |
/// |:------|:----:|:----:|:----:|:-------:|:------:|:------:|:------:|:----:|:---:|
/// | Byte  | 0    | 1    | 2    | 3       | 4      | 5      | 6      | 7    | 8   |
/// | Value | 0x02 | 0x09 | 0x7n | 0x06    | nn     | nn     | nn     | 0x03 | zz  |
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct QueryValueTableCommand {
    buf: [u8; QUERY_VALUE_TABLE_COMMAND],
}

impl QueryValueTableCommand {
    /// Creates a new [QueryValueTableCommand].
    pub fn new() -> Self {
        let mut message = Self {
            buf: [0u8; QUERY_VALUE_TABLE_COMMAND],
        };

        message.init();
        message.set_message_type(MessageType::Extended);
        message.set_extended_command(ExtendedCommand::QueryValueTable);

        message
    }
}

impl_message_ops!(QueryValueTableCommand);
impl_omnibus_extended_command!(QueryValueTableCommand);
impl_extended_ops!(QueryValueTableCommand);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;

    #[test]
    #[rustfmt::skip]
    fn test_query_value_table_command_from_bytes() -> Result<()> {
        let msg_bytes = [
            // STX | LEN | Message type | Subtype
            0x02, 0x09, 0x70, 0x06,
            // Data
            0x00, 0x00, 0x00,
            // ETX | Checksum
            0x03, 0x7f,
        ];

        let mut msg = QueryValueTableCommand::new();
        msg.from_buf(msg_bytes.as_ref())?;

        assert_eq!(msg.message_type(), MessageType::Extended);
        assert_eq!(msg.extended_command(), ExtendedCommand::QueryValueTable);

        Ok(())
    }
}
