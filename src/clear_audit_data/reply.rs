use crate::std;
use std::fmt;

use crate::{
    bool_enum, impl_extended_ops, impl_message_ops, impl_omnibus_extended_reply,
    len::{CLEAR_AUDIT_DATA_REQUEST_ACK, CLEAR_AUDIT_DATA_REQUEST_RESULTS},
    ExtendedCommand, ExtendedCommandOps, MessageOps, MessageType, OmnibusReplyOps,
};

bool_enum!(
    ClearAuditAckNak,
    r"Whether the device is able to perform the Clear Audit Data Request."
);

bool_enum!(
    ClearAuditPassFail,
    r"Whether the device successfully proccessed the Clear Audit Data Request."
);

pub mod index {
    pub const ACKNAK: usize = 10;
    pub const PASS_FAIL: usize = 10;
}

/// Clear Audit Data - Request Acknowledgement (Subtype 0x1D)
///
/// The [ClearAuditDataRequestAck] reply is an immediate response to a request to perform a clear of the audit data on the SC Advanced.
///
/// Since the command needs to clear large sections of memory, the command may take a few seconds to
/// complete.
///
/// The device will inform the host that the operation is complete by posting back a completion
/// response at later time. However, the device will post an acknowledgement of the host request immediately
///
/// The Clear Audit Data Request Acknowledgement is formatted as follows:
///
/// | Name  | STX  | LEN  | CTRL | Subtype | Data 0 | Data 1 | Data 2 | Data 3 | Data 4 | Data 5 | ACK/NAK | ETX  | CHK |
/// |:------|:----:|:----:|:----:|:-------:|:------:|:------:|:------:|:------:|:------:|:------:|:-------:|:----:|:---:|
/// | Byte  | 0    | 1    | 2    | 3       | 4      | 5      | 6      | 7      | 8      | 9      | 10      | 11   | 12  |
/// | Value | 0x02 | 0x0D | 0x7n | 0x1D    | nn     | nn     | nn     | nn     | nn     | nn     | 0x00/01 | 0x03 | zz  |
///
/// If for any reason the device is unable to honor the host request, a NAK will be sent to the host
/// (represented by 0x00 for byte 10). The device will NAK the host in the following situations:
///
/// * Device is in power up mode.
/// * A current transaction underway. If a document has been inserted and is being processed, the
/// device will NAK all Clear Audit Data Request
/// * Device is in calibration mode.
/// * The device is currently servicing another type 7 message request.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ClearAuditDataRequestAck {
    buf: [u8; CLEAR_AUDIT_DATA_REQUEST_ACK],
}

impl ClearAuditDataRequestAck {
    pub fn new() -> Self {
        let mut message = Self {
            buf: [0u8; CLEAR_AUDIT_DATA_REQUEST_ACK],
        };

        message.init();
        message.set_message_type(MessageType::Extended);
        message.set_extended_command(ExtendedCommand::ClearAuditDataRequest);

        message
    }

    /// Gets the ACKNAK data field.
    ///
    /// If for any reason the device is unable to honor the host request, a NAK will be sent to the host
    /// (represented by 0x00 for byte 10). The device will NAK the host in the following situations:
    ///
    /// - Device is in power up mode.
    /// - A current transaction underway. If a document has been inserted and is being processed, the device will NAK all Clear Audit Data Request
    /// - Device is in calibration mode.
    /// - The device is currently servicing another type 7 message request.
    pub fn audit_acknak(&self) -> ClearAuditAckNak {
        self.buf[index::ACKNAK].into()
    }
}

impl_message_ops!(ClearAuditDataRequestAck);
impl_omnibus_extended_reply!(ClearAuditDataRequestAck);
impl_extended_ops!(ClearAuditDataRequestAck);

impl fmt::Display for ClearAuditDataRequestAck {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AckNak: {}, DeviceType: {}, MessageType: {}, Subtype: {}, DeviceState: {}, DeviceStatus: {}, ExceptionStatus: {}, MiscDeviceState: {}, ModelNumber: {}, CodeRevision: {}, AuditAckNak: {}",
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
            self.audit_acknak(),
        )
    }
}

/// Clear Audit Data - Request Results (Subtype 0x1D)
///
/// The [ClearAuditDataRequestResults] reply contains the results of the clear audit data process on the SC Advanced.
///
/// If the device ACKs the original request, it will process that request and issue a completion response.
///
/// This response will be issued as a reply to a general host omnibus command. The message will contain a data
/// byte that will tell the host if the operation passed or failed.
///
/// The Clear Audit Data Request Results is formatted as follows:
///
/// | Name  | STX  | LEN  | CTRL | Subtype | Data 0 | Data 1 | Data 2 | Data 3 | Data 4 | Data 5 | Pass/Fail | ETX  | CHK |
/// |:------|:----:|:----:|:----:|:-------:|:------:|:------:|:------:|:------:|:------:|:------:|:---------:|:----:|:---:|
/// | Byte  | 0    | 1    | 2    | 3       | 4      | 5      | 6      | 7      | 8      | 9      | 10        | 11   | 12  |
/// | Value | 0x02 | 0x0D | 0x7n | 0x1D    | nn     | nn     | nn     | nn     | nn     | nn     | 0x00/01   | 0x03 | zz  |
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ClearAuditDataRequestResults {
    buf: [u8; CLEAR_AUDIT_DATA_REQUEST_RESULTS],
}

impl ClearAuditDataRequestResults {
    pub fn new() -> Self {
        let mut message = Self {
            buf: [0u8; CLEAR_AUDIT_DATA_REQUEST_RESULTS],
        };

        message.init();
        message.set_message_type(MessageType::Extended);
        message.set_extended_command(ExtendedCommand::ClearAuditDataRequest);

        message
    }

    /// Gets the Pass/Fail data field.
    pub fn pass_fail(&self) -> ClearAuditPassFail {
        self.buf[index::PASS_FAIL].into()
    }
}

impl_message_ops!(ClearAuditDataRequestResults);
impl_omnibus_extended_reply!(ClearAuditDataRequestResults);
impl_extended_ops!(ClearAuditDataRequestResults);

impl fmt::Display for ClearAuditDataRequestResults {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AckNak: {}, DeviceType: {}, MessageType: {}, Subtype: {}, DeviceState: {}, DeviceStatus: {}, ExceptionStatus: {}, MiscDeviceState: {}, ModelNumber: {}, CodeRevision: {}, PassFail: {}",
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
            self.pass_fail(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;

    #[test]
    #[rustfmt::skip]
    fn test_clear_audit_data_request_ack_from_bytes() -> Result<()> {
        let msg_bytes = [
            // STX | LEN | Message type | Subtype
            0x02, 0x0d, 0x70, 0x1d,
            // Data
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            // ACK/NAK
            0x01,
            // ETX | Checksum
            0x03, 0x61,
        ];

        let mut msg = ClearAuditDataRequestAck::new();
        msg.from_buf(msg_bytes.as_ref())?;

        assert_eq!(msg.message_type(), MessageType::Extended);
        assert_eq!(
            msg.extended_command(),
            ExtendedCommand::ClearAuditDataRequest
        );
        assert_eq!(msg.audit_acknak(), ClearAuditAckNak::Set);

        let msg_bytes = [
            // STX | LEN | Message type | Subtype
            0x02, 0x0d, 0x70, 0x1d,
            // Data
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            // ACK/NAK
            0x00,
            // ETX | Checksum
            0x03, 0x60,
        ];

        msg.from_buf(msg_bytes.as_ref())?;

        assert_eq!(msg.message_type(), MessageType::Extended);
        assert_eq!(
            msg.extended_command(),
            ExtendedCommand::ClearAuditDataRequest
        );
        assert_eq!(msg.audit_acknak(), ClearAuditAckNak::Unset);

        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn test_clear_audit_data_request_results_from_bytes() -> Result<()> {
        let msg_bytes = [
            // STX | LEN | Message type | Subtype
            0x02, 0x0d, 0x70, 0x1d,
            // Data
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            // Pass/Fail
            0x11,
            // ETX | Checksum
            0x03, 0x71,
        ];

        let mut msg = ClearAuditDataRequestResults::new();
        msg.from_buf(msg_bytes.as_ref())?;

        assert_eq!(msg.message_type(), MessageType::Extended);
        assert_eq!(
            msg.extended_command(),
            ExtendedCommand::ClearAuditDataRequest
        );
        assert_eq!(msg.pass_fail(), ClearAuditPassFail::Set);

        let msg_bytes = [
            // STX | LEN | Message type | Subtype
            0x02, 0x0d, 0x70, 0x1d,
            // Data
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            // Pass/Fail
            0x10,
            // ETX | Checksum
            0x03, 0x70,
        ];

        msg.from_buf(msg_bytes.as_ref())?;

        assert_eq!(msg.message_type(), MessageType::Extended);
        assert_eq!(
            msg.extended_command(),
            ExtendedCommand::ClearAuditDataRequest
        );
        assert_eq!(msg.pass_fail(), ClearAuditPassFail::Unset);

        Ok(())
    }
}
