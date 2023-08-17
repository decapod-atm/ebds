use crate::std;
use std::fmt;

use crate::{
    impl_extended_ops, impl_message_ops, impl_omnibus_extended_reply,
    len::SET_ESCROW_TIMEOUT_REPLY, ExtendedCommand, ExtendedCommandOps, MessageOps, MessageType,
    OmnibusReplyOps,
};

/// This command is generally used to set the escrow timeout of the device. However, it can also serve an
/// alternative function in reporting a special coupon if that mode is enabled (Section 7.1.1.3).
///
/// The Notes and Barcode fields set the timeout for bank notes and barcodes in seconds. This is a value
/// from 1 through 127 seconds, or zero to disable the timeout. By default, both timeouts are disabled in
/// most software implementations.
///
/// The reply contains no extended data.
///
/// The Set Escrow reply is formatted as follows:
///
/// | Name  | STX  | LEN  | CTRL | Subtype | Data 0 | Data 1 | Data 2 | Data 3 | Data 4 | Data 5 | ETX  | CHK |
/// |:------|:----:|:----:|:----:|:-------:|:------:|:------:|:------:|:------:|:------:|:------:|:----:|:---:|
/// | Byte  | 0    | 1    | 2    | 3       | 4      | 5      | 6      | 7      | 8      | 9      | 10   | 11  |
/// | Value | 0x02 | 0x0C | 0X7n | 0x04    | nn     | nn     | nn     | nn     | nn     | nn     | 0x03 | zz  |
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SetEscrowTimeoutReply {
    buf: [u8; SET_ESCROW_TIMEOUT_REPLY],
}

impl SetEscrowTimeoutReply {
    /// Creates a new [SetEscrowTimeoutReply].
    pub fn new() -> Self {
        let mut message = Self {
            buf: [0u8; SET_ESCROW_TIMEOUT_REPLY],
        };

        message.init();
        message.set_message_type(MessageType::Extended);
        message.set_extended_command(ExtendedCommand::SetEscrowTimeout);

        message
    }
}

impl_message_ops!(SetEscrowTimeoutReply);
impl_extended_ops!(SetEscrowTimeoutReply);
impl_omnibus_extended_reply!(SetEscrowTimeoutReply);

impl fmt::Display for SetEscrowTimeoutReply {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "
            AckNak: {},
            DeviceType: {},
            MessageType: {},
            Subtype: {},
            DeviceState: {},
            DeviceStatus: {},
            ExceptionStatus: {},
            MiscDeviceState: {},
            ModelNumber: {},
            CodeRevision: {},
            ",
            self.acknak(),
            self.device_type(),
            self.message_type(),
            self.extended_command(),
            self.device_state(),
            self.device_status(),
            self.exception_status(),
            self.misc_device_state(),
            self.model_number(),
            self.code_revision(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;

    #[test]
    #[rustfmt::skip]
    fn test_set_escrow_timeout_reply_from_bytes() -> Result<()> {
        let msg_bytes = [
            // STX | LEN | Message type | Subtype
            0x02, 0x0c, 0x70, 0x04,
            // Data
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            // ETX | Checksum
            0x03, 0x78,
        ];

        let mut msg = SetEscrowTimeoutReply::new();
        msg.from_buf(msg_bytes.as_ref())?;

        assert_eq!(msg.message_type(), MessageType::Extended);
        assert_eq!(msg.extended_command(), ExtendedCommand::SetEscrowTimeout);

        Ok(())
    }
}
