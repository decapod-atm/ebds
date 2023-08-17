use crate::std;
use std::fmt;

use crate::{
    impl_aux_ops, impl_message_ops, impl_omnibus_command_ops, len::QUERY_VARIANT_NAME_COMMAND,
    AuxCommand, AuxCommandOps, MessageOps, MessageType,
};

/// Query Variant Name - Command (Subtype 0x08)
///
/// This command is used to return the name of the variant component of the firmware. The variant
/// software determines which bank notes are accepted by the device and the name of the variant,
/// identifies the country of origin of those bank notes.
///
/// The Query Variant Name Command is formatted as follows:
///
/// | Name  | STX  | LEN  | CTRL | Data A | Data B | Command | ETX  | CHK |
/// |:------|:----:|:----:|:----:|:------:|:------:|:-------:|:----:|:---:|
/// | Byte  | 0    | 1    | 2    | 3      | 4      | 5       | 6    | 7   |
/// | Value | 0x02 | 0x08 | 0x6n | 0x00   | 0x00   | 0x08    | 0x03 | zz  |
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct QueryVariantNameCommand {
    buf: [u8; QUERY_VARIANT_NAME_COMMAND],
}

impl QueryVariantNameCommand {
    /// Creates a new [QueryVariantNameCommand]
    pub fn new() -> Self {
        let mut message = Self {
            buf: [0u8; QUERY_VARIANT_NAME_COMMAND],
        };

        message.init();
        message.set_message_type(MessageType::AuxCommand);
        message.set_aux_command(AuxCommand::QueryVariantName);

        message
    }
}

impl_message_ops!(QueryVariantNameCommand);
impl_omnibus_command_ops!(QueryVariantNameCommand);
impl_aux_ops!(QueryVariantNameCommand);

impl fmt::Display for QueryVariantNameCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AckNak: {}, DeviceType: {}, MessageType: {}, Command: {}, Checksum: {}",
            self.acknak(),
            self.device_type(),
            self.message_type(),
            self.aux_command(),
            self.checksum(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;

    #[test]
    #[rustfmt::skip]
    fn test_query_boot_part_number_command_from_buf() -> Result<()> {
        let msg_bytes = [
            // STX | LEN | Message Type
            0x02, 0x08, 0x60,
            // Data
            0x00, 0x00,
            // Command
            0x08,
            // ETX | Checksum
            0x03, 0x60,
        ];

        let mut msg = QueryVariantNameCommand::new();
        msg.from_buf(msg_bytes.as_ref())?;

        assert_eq!(msg.message_type(), MessageType::AuxCommand);
        assert_eq!(msg.aux_command(), AuxCommand::QueryVariantName);

        Ok(())
    }
}
