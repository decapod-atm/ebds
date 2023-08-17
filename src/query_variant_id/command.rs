use crate::{
    impl_aux_ops, impl_message_ops, len::QUERY_VARIANT_ID_COMMAND, AuxCommand, AuxCommandOps,
    MessageOps, MessageType,
};

/// Query Variant ID Number - Command (Subtype 0x0F)
///
/// This command is used to return the software part number of the actual variant component of the device firmware.
///
/// This is only applicable when the device has been loaded with a combine file (a file that
/// contains both the application and the variant) because the (Subtype 0x09) Query Acceptor Variant Part
/// Number command will return the part number of the combine file, not the underlying component’s part
/// number.
///
/// The device capabilities map (section 7.4.14) has an entry as to whether or not the device supports this command.
///
/// The Query Variant ID Command is formatted as follows:
///
/// | Name  | STX  | LEN  | CTRL | Data A | Data B | Command | ETX  | CHK |
/// |:------|:----:|:----:|:----:|:------:|:------:|:-------:|:----:|:---:|
/// | Byte  | 0    | 1    | 2    | 3      | 4      | 5       | 6    | 7   |
/// | Value | 0x02 | 0x08 | 0x6n | 0x00   | 0x00   | 0x0F    | 0x03 | zz  |
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct QueryVariantIdCommand {
    buf: [u8; QUERY_VARIANT_ID_COMMAND],
}

impl QueryVariantIdCommand {
    pub fn new() -> Self {
        let mut message = Self {
            buf: [0u8; QUERY_VARIANT_ID_COMMAND],
        };

        message.init();
        message.set_message_type(MessageType::AuxCommand);
        message.set_aux_command(AuxCommand::QueryVariantId);

        message
    }
}

impl_message_ops!(QueryVariantIdCommand);
impl_aux_ops!(QueryVariantIdCommand);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;

    #[test]
    #[rustfmt::skip]
    fn test_query_variant_id_command_from_buf() -> Result<()> {
        let msg_bytes = [
            // STX | LEN | Message Type
            0x02, 0x08, 0x60,
            // Data
            0x00, 0x00,
            // Command
            0x0f,
            // ETX | Checksum
            0x03, 0x67,
        ];

        let mut msg = QueryVariantIdCommand::new();
        msg.from_buf(msg_bytes.as_ref())?;

        assert_eq!(msg.message_type(), MessageType::AuxCommand);
        assert_eq!(msg.aux_command(), AuxCommand::QueryVariantId);

        Ok(())
    }
}
