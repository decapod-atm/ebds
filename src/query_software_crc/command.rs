use crate::{
    impl_aux_ops, impl_default, impl_message_ops, len::QUERY_SOFTWARE_CRC_COMMAND, AuxCommand,
    AuxCommandOps, MessageOps, MessageType,
};

/// Query Software CRC - Command (Subtype 0x00)
///
/// | **S2K** | **CFSC** | **SC Adv** | **SCR** |
/// |:-------:|:--------:|:----------:|:-------:|
///
/// This command is used to query the device for the 16 bit CRC of the flash contents.
///
/// The Query Software CRC Command is formatted as follows:
///
/// | Name  | STX  | LEN  | CTRL | Data A | Data B | Command | ETX  | CHK |
/// |:------|:----:|:----:|:----:|:------:|:------:|:-------:|:----:|:---:|
/// | Byte  | 0    | 1    | 2    | 3      | 4      | 5       | 6    | 7   |
/// | Value | 0x02 | 0x08 | 0x6n | 0x00   | 0x00   | 0x00    | 0x03 | zz  |
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QuerySoftwareCrcCommand {
    buf: [u8; QUERY_SOFTWARE_CRC_COMMAND],
}

impl QuerySoftwareCrcCommand {
    pub fn new() -> Self {
        let mut message = Self {
            buf: [0u8; QUERY_SOFTWARE_CRC_COMMAND],
        };

        message.init();
        message.set_message_type(MessageType::AuxCommand);
        message.set_aux_command(AuxCommand::QuerySoftwareCrc);

        message
    }
}

impl_default!(QuerySoftwareCrcCommand);
impl_message_ops!(QuerySoftwareCrcCommand);
impl_aux_ops!(QuerySoftwareCrcCommand);
