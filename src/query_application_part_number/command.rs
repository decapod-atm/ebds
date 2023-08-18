use crate::{
    impl_aux_ops, impl_message_ops, len::QUERY_APPLICATION_PART_NUMBER_COMMAND, std::fmt,
    AuxCommand, AuxCommandOps, MessageOps, MessageType,
};

/// Query Application Part Number - Command (Subtype 0x07)
///
/// This command is used to return the software part number of the boot component of the device
/// firmware.
///
/// The Query Application Part Number Command is formatted as follows:
///
/// | Name  | STX  | LEN  | CTRL | Data A | Data B | Command | ETX  | CHK |
/// |:------|:----:|:----:|:----:|:------:|:------:|:-------:|:----:|:---:|
/// | Byte  | 0    | 1    | 2    | 3      | 4      | 5       | 6    | 7   |
/// | Value | 0x02 | 0x08 | 0x6n | 0x00   | 0x00   | 0x07    | 0x03 | zz  |
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct QueryApplicationPartNumberCommand {
    buf: [u8; QUERY_APPLICATION_PART_NUMBER_COMMAND],
}

impl QueryApplicationPartNumberCommand {
    pub fn new() -> Self {
        let mut message = Self {
            buf: [0u8; QUERY_APPLICATION_PART_NUMBER_COMMAND],
        };

        message.init();
        message.set_message_type(MessageType::AuxCommand);
        message.set_aux_command(AuxCommand::QueryApplicationPartNumber);

        message
    }
}

impl_message_ops!(QueryApplicationPartNumberCommand);
impl_aux_ops!(QueryApplicationPartNumberCommand);

impl fmt::Display for QueryApplicationPartNumberCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""acknak": {}, "#, self.acknak())?;
        write!(f, r#""device_type": {}, "#, self.device_type())?;
        write!(f, r#""message_type": {}, "#, self.message_type())?;
        write!(f, r#""aux_command": {}"#, self.aux_command())?;
        write!(f, "}}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;

    #[test]
    #[rustfmt::skip]
    fn test_query_application_part_number_command_from_buf() -> Result<()> {
        let msg_bytes = [
            // STX | LEN | Message Type
            0x02, 0x08, 0x60,
            // Data
            0x00, 0x00,
            // Command
            0x07,
            // ETX | Checksum
            0x03, 0x6f,
        ];

        let mut msg = QueryApplicationPartNumberCommand::new();
        msg.from_buf(msg_bytes.as_ref())?;

        assert_eq!(msg.message_type(), MessageType::AuxCommand);
        assert_eq!(msg.aux_command(), AuxCommand::QueryApplicationPartNumber);

        Ok(())
    }
}
