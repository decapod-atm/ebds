use crate::{
    impl_default, impl_extended_ops, impl_message_ops, impl_omnibus_extended_command,
    len::CLEAR_AUDIT_DATA_REQUEST, ExtendedCommand, ExtendedCommandOps, MessageOps, MessageType,
};

/// Clear Audit Data - Request (Subtype 0x1D)
///
/// The [ClearAuditDataRequest] command allows the host to perform a clear of the audit data on the SC Advanced.
///
/// This command will clear all audit information except for the lifetime audit section; these will
/// be protected and will not be cleared by this command.
///
/// The Clear Audit Data Request is formatted as follows:
///
/// | Name  | STX  | LEN  | CTRL | Subtype | Data 0 | Data 1 | Data 2 | ETX  | CHK |
/// |:------|:----:|:----:|:----:|:-------:|:------:|:------:|:------:|:----:|:---:|
/// | Byte  | 0    | 1    | 2    | 3       | 4      | 5      | 6      | 7    | 8   |
/// | Value | 0x02 | 0x09 | 0x7n | 0x1D    | nn     | nn     | nn     | 0x03 | zz  |
///
/// Since the command needs to clear large sections of memory, the command may take a few seconds to
/// complete.
pub struct ClearAuditDataRequest {
    buf: [u8; CLEAR_AUDIT_DATA_REQUEST],
}

impl ClearAuditDataRequest {
    pub fn new() -> Self {
        let mut message = Self {
            buf: [0u8; CLEAR_AUDIT_DATA_REQUEST],
        };

        message.init();
        message.set_message_type(MessageType::Extended);
        message.set_extended_command(ExtendedCommand::ClearAuditDataRequest);

        message
    }
}

impl_default!(ClearAuditDataRequest);
impl_message_ops!(ClearAuditDataRequest);
impl_extended_ops!(ClearAuditDataRequest);
impl_omnibus_extended_command!(ClearAuditDataRequest);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;

    #[test]
    #[rustfmt::skip]
    fn test_clear_audit_data_from_bytes() -> Result<()> {
        let msg_bytes = [
            // STX | LEN | Message type | Subtype
            0x02, 0x09, 0x70, 0x1d,
            // Data
            0x00, 0x00, 0x00,
            // ETX | Checksum
            0x03, 0x64,
        ];

        let mut msg = ClearAuditDataRequest::new();
        msg.from_buf(msg_bytes.as_ref())?;

        assert_eq!(msg.message_type(), MessageType::Extended);
        assert_eq!(
            msg.extended_command(),
            ExtendedCommand::ClearAuditDataRequest
        );

        Ok(())
    }
}
