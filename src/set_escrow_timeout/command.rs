use crate::{
    impl_default, impl_extended_ops, impl_message_ops, impl_omnibus_extended_command,
    len::SET_ESCROW_TIMEOUT_COMMAND, ExtendedCommand, ExtendedCommandOps, MessageOps, MessageType,
};

mod index {
    pub const NOTES: usize = 7;
    pub const BARCODES: usize = 8;
}

const TIMEOUT_MASK: u8 = 0x7f;

/// This command is generally used to set the escrow timeout of the device. However, it can also serve an
/// alternative function in reporting a special coupon if that mode is enabled (Section 7.1.1.3).
///
/// This command is generally used to set the escrow timeout of the device. However, it can also serve an
/// alternative function in reporting a special coupon if that mode is enabled (Section 7.1.1.3).
/// SCR Classification When classification mode is enabled in the device, the device will suppress the
/// EBDS escrow timeout. This means that if communications are lost with the host, then the SCR will keep
/// the notes at escrow until communications are restored and the host makes the escrow decision.
///
/// The Set Escrow command is formatted as follows:
///
/// | Name  | STX  | LEN  | CTRL | Subtype | Data 0 | Data 1 | Data 2 | Notes | Barcode | ETX  | CHK |
/// |:------|:----:|:----:|:----:|:-------:|:------:|:------:|:------:|:-----:|:-------:|:----:|:---:|
/// | Byte  | 0    | 1    | 2    | 3       | 4      | 5      | 6      | 7     | 8       | 9    | 10  |
/// | Value | 0x02 | 0x0B | 0x7n | 0x04    | nn     | nn     | nn     | nn    | nn      | 0x03 | zz  |
pub struct SetEscrowTimeoutCommand {
    buf: [u8; SET_ESCROW_TIMEOUT_COMMAND],
}

impl SetEscrowTimeoutCommand {
    /// Creates a new [SetEscrowTimeoutCommand].
    pub fn new() -> Self {
        let mut message = Self {
            buf: [0u8; SET_ESCROW_TIMEOUT_COMMAND],
        };

        message.init();
        message.set_message_type(MessageType::Extended);
        message.set_extended_command(ExtendedCommand::SetEscrowTimeout);

        message
    }

    /// Gets the timeout for bank notes.
    pub fn notes_timeout(&self) -> u8 {
        self.buf[index::NOTES] & TIMEOUT_MASK
    }

    /// Sets the timeout for bank notes.
    ///
    /// 1-127 sets the timeout in seconds.
    ///
    /// 0 disables the timeout.
    pub fn set_notes_timeout(&mut self, secs: u8) {
        self.buf[index::NOTES] = secs & TIMEOUT_MASK;
    }

    /// Sets the timeout for barcodes.
    ///
    /// 1-127 sets the timeout in seconds.
    ///
    /// 0 disables the timeout.
    pub fn set_barcodes_timeout(&mut self, secs: u8) {
        self.buf[index::BARCODES] = secs & TIMEOUT_MASK;
    }

    /// Gets the timeout for barcodes.
    pub fn barcodes_timeout(&self) -> u8 {
        self.buf[index::BARCODES] & TIMEOUT_MASK
    }
}

impl_default!(SetEscrowTimeoutCommand);
impl_message_ops!(SetEscrowTimeoutCommand);
impl_extended_ops!(SetEscrowTimeoutCommand);
impl_omnibus_extended_command!(SetEscrowTimeoutCommand);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;

    #[test]
    #[rustfmt::skip]
    fn test_set_escrow_timeout_command_from_bytes() -> Result<()> {
        let msg_bytes = [
            // STX | LEN | Message type | Subtype
            0x02, 0x0b, 0x70, 0x04,
            // Data
            0x00, 0x00, 0x00,
            // Notes
            0x01,
            // Barcode
            0x02,
            // ETX | Checksum
            0x03, 0x7c,
        ];

        let mut msg = SetEscrowTimeoutCommand::new();
        msg.from_buf(msg_bytes.as_ref())?;

        assert_eq!(msg.message_type(), MessageType::Extended);
        assert_eq!(msg.extended_command(), ExtendedCommand::SetEscrowTimeout);
        assert_eq!(msg.notes_timeout(), 1);
        assert_eq!(msg.barcodes_timeout(), 2);

        Ok(())
    }
}
