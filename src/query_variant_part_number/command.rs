use crate::{
    impl_aux_ops, impl_default, impl_message_ops, len::QUERY_VARIANT_PART_NUMBER_COMMAND,
    AuxCommand, AuxCommandOps, MessageOps, MessageType,
};

/// Query Variant Part Number - Command (Subtype 0x09)
///
/// This command is used to return the software part number of the file containing the variant component of the device firmware.
///
/// The Query Variant Part Number Command is formatted as follows:
///
/// | Name  | STX  | LEN  | CTRL | Data A | Data B | Command | ETX  | CHK |
/// |:------|:----:|:----:|:----:|:------:|:------:|:-------:|:----:|:---:|
/// | Byte  | 0    | 1    | 2    | 3      | 4      | 5       | 6    | 7   |
/// | Value | 0x02 | 0x08 | 0x6n | 0x00   | 0x00   | 0x09    | 0x03 | zz  |
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QueryVariantPartNumberCommand {
    buf: [u8; QUERY_VARIANT_PART_NUMBER_COMMAND],
}

impl QueryVariantPartNumberCommand {
    pub fn new() -> Self {
        let mut message = Self {
            buf: [0u8; QUERY_VARIANT_PART_NUMBER_COMMAND],
        };

        message.init();
        message.set_message_type(MessageType::AuxCommand);
        message.set_aux_command(AuxCommand::QueryVariantPartNumber);

        message
    }
}

impl_default!(QueryVariantPartNumberCommand);
impl_message_ops!(QueryVariantPartNumberCommand);
impl_aux_ops!(QueryVariantPartNumberCommand);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;

    #[test]
    #[rustfmt::skip]
    fn test_query_variant_part_number_command_from_buf() -> Result<()> {
        let msg_bytes = [
            // STX | LEN | Message Type
            0x02, 0x08, 0x60,
            // Data
            0x00, 0x00,
            // Command
            0x09,
            // ETX | Checksum
            0x03, 0x61,
        ];

        let mut msg = QueryVariantPartNumberCommand::new();
        msg.from_buf(msg_bytes.as_ref())?;

        assert_eq!(msg.message_type(), MessageType::AuxCommand);
        assert_eq!(msg.aux_command(), AuxCommand::QueryVariantPartNumber);

        Ok(())
    }
}
