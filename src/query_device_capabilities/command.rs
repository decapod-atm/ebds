use crate::std;
use std::fmt;

use crate::{
    impl_aux_ops, impl_default, impl_message_ops, impl_omnibus_command_ops,
    len::QUERY_DEVICE_CAPABILITIES_COMMAND, AuxCommand, AuxCommandOps, MessageOps, MessageType,
};

/// Query Device Capabilities - Command (Subtype 0x0D)
///
/// This command is used to query the device capabilities. In general, this command should only be sent to
/// devices that have indicated support by setting the DeviceCaps bit in a standard poll reply (see section
/// 7.1.2.4.)
///
/// The Query Device Capabilities Command is formatted as follows:
///
/// | Name  | STX  | LEN  | CTRL | Data A | Data B | Command | ETX  | CHK |
/// |:------|:----:|:----:|:----:|:------:|:------:|:-------:|:----:|:---:|
/// | Byte  | 0    | 1    | 2    | 3      | 4      | 5       | 6    | 7   |
/// | Value | 0x02 | 0x08 | 0x6n | 0x00   | 0x00   | 0x0D    | 0x03 | zz  |
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QueryDeviceCapabilitiesCommand {
    buf: [u8; QUERY_DEVICE_CAPABILITIES_COMMAND],
}

impl QueryDeviceCapabilitiesCommand {
    /// Create a new [QueryDeviceCapabilitiesCommand] message
    pub fn new() -> Self {
        let mut message = Self {
            buf: [0u8; QUERY_DEVICE_CAPABILITIES_COMMAND],
        };

        message.init();
        message.set_message_type(MessageType::AuxCommand);
        message.set_aux_command(AuxCommand::QueryDeviceCapabilities);

        message
    }
}

impl_default!(QueryDeviceCapabilitiesCommand);
impl_message_ops!(QueryDeviceCapabilitiesCommand);
impl_aux_ops!(QueryDeviceCapabilitiesCommand);
impl_omnibus_command_ops!(QueryDeviceCapabilitiesCommand);

impl fmt::Display for QueryDeviceCapabilitiesCommand {
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
    fn test_query_device_capabilities_command_from_buf() -> Result<()> {
        let msg_bytes = [
            // STX | LEN | Message Type
            0x02, 0x08, 0x60,
            // Data
            0x00, 0x00,
            // Command
            0x0d,
            // ETX | Checksum
            0x03, 0x65,
        ];

        let mut msg = QueryDeviceCapabilitiesCommand::new();
        msg.from_buf(msg_bytes.as_ref())?;

        assert_eq!(msg.message_type(), MessageType::AuxCommand);
        assert_eq!(msg.aux_command(), AuxCommand::QueryDeviceCapabilities);

        Ok(())
    }
}
