use crate::{
    impl_aux_ops, impl_default, impl_message_ops, len::SOFT_RESET, AuxCommand, AuxCommandOps,
    MessageOps, MessageType,
};

pub mod index {
    pub const DATA: usize = 3;
    pub const DATA_END: usize = DATA + 2;
}

/// Acceptor Soft Reset: (Subtype 0x7F)
///
/// This command is used to reset the device. There is not necessarily a reply to this command, but some
/// data may be sent by the device. The host system should ignore all data sent by the device for at least
/// one second. Further, the device may take as much as fifteen seconds to return to normal operation after
/// being reset and the host should poll, once per second, for at least fifteen seconds until the device
/// replies.
///
/// The acceptor soft reset command takes the form:
///
/// | Name  | STX  | LEN  | CTRL | Data A | Data B | Command | ETX  | CHK |
/// |:------|:----:|:----:|:----:|:------:|:------:|:-------:|:----:|:---:|
/// | Byte  | 0    | 1    | 2    | 3      | 4      | 5       | 6    | 7   |
/// | Value | 0x02 | 0x08 | 0x6n | 0x7F   | 0x7F   | 0x7F    | 0x03 | zz  |
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SoftReset {
    buf: [u8; SOFT_RESET],
}

impl SoftReset {
    /// Creates a new [SoftReset] message.
    pub fn new() -> Self {
        let mut message = Self {
            buf: [0u8; SOFT_RESET],
        };

        message.init();
        message.set_message_type(MessageType::AuxCommand);
        message.set_aux_command(AuxCommand::SoftReset);
        message.set_reset_data();

        message
    }

    fn set_reset_data(&mut self) {
        self.buf[index::DATA..index::DATA_END].copy_from_slice([0x7f, 0x7f].as_ref());
    }
}

impl_default!(SoftReset);
impl_message_ops!(SoftReset);
impl_aux_ops!(SoftReset);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;

    #[test]
    #[rustfmt::skip]
    fn test_soft_reset_from_bytes() -> Result<()> {
        let msg_bytes = [
            // STX | LEN | Message type | Subtype
            0x02, 0x08, 0x60,
            // Data
            0x7f, 0x7f,
            // Command
            0x7f,
            // ETX | Checksum
            0x03, 0x17,
        ];

        let mut msg = SoftReset::new();
        msg.from_buf(msg_bytes.as_ref())?;

        assert_eq!(msg.message_type(), MessageType::AuxCommand);
        assert_eq!(msg.aux_command(), AuxCommand::SoftReset);

        Ok(())
    }
}
