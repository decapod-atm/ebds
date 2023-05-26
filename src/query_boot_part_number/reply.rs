use crate::std;
use std::fmt;

use crate::{
    impl_default, impl_message_ops, impl_omnibus_nop_reply, len::QUERY_BOOT_PART_NUMBER_REPLY,
    BootPartNumber, MessageOps, MessageType, PartVersion, ProjectNumber,
};

mod index {
    pub const PROJECT_NUM: usize = 3;
    pub const VERSION: usize = 9;
}

/// Query Boot Part Number - Reply (Subtype 0x06)
///
/// This is the response to a query for the software part number of the boot component of the device firmware.
///
/// The data returned by the device takes the form of an ASCII string that is 9 bytes long.
///
/// The Query Boot Part Number Reply is formatted as follows:
///
/// | Name  | STX  | LEN  | CTRL | Data 0 | Data 1 | ...  | Data 8 | ETX  | CHK |
/// |:------|:----:|:----:|:----:|:------:|:------:|:----:|:------:|:----:|:---:|
/// | Byte  | 0    | 1    | 2    | 3      | 4      | ...  | 11     | 12   | 13  |
/// | Value | 0x02 | 0x0E | 0x6n | nn     | nn     | nn   | nn     | 0x03 | zz  |
///
/// The part number is composed of a project number (5-6 digits) and version number (3 digits) with an
/// optional Check sum digit in the middle.
///
/// See [ProjectNumber](crate::ProjectNumber) for formatting details.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QueryBootPartNumberReply {
    buf: [u8; QUERY_BOOT_PART_NUMBER_REPLY],
}

impl QueryBootPartNumberReply {
    /// Creates a new [QueryBootPartNumberReply].
    pub fn new() -> Self {
        let mut message = Self {
            buf: [0u8; QUERY_BOOT_PART_NUMBER_REPLY],
        };

        message.init();
        message.set_message_type(MessageType::AuxCommand);

        message
    }

    /// Gets the [BootPartNumber].
    pub fn boot_part_number(&self) -> BootPartNumber {
        self.buf[index::PROJECT_NUM..self.etx_index()]
            .as_ref()
            .into()
    }

    /// Gets the [ProjectNumber] parsed from the raw byte buffer.
    ///
    /// On invalid ranges, returns a zeroed [ProjectNumber].
    pub fn project_number(&self) -> ProjectNumber {
        self.buf[index::PROJECT_NUM..index::VERSION].as_ref().into()
    }

    /// Gets the [PartVersion].
    pub fn version(&self) -> PartVersion {
        self.buf[index::VERSION..self.etx_index()].as_ref().into()
    }
}

impl_default!(QueryBootPartNumberReply);
impl_message_ops!(QueryBootPartNumberReply);
impl_omnibus_nop_reply!(QueryBootPartNumberReply);

impl fmt::Display for QueryBootPartNumberReply {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AckNak: {}, DeviceType: {}, MessageType: {}, BootPartNumber: {}, ProjectNumber: {}, Version: {}",
            self.acknak(),
            self.device_type(),
            self.message_type(),
            self.boot_part_number(),
            self.project_number(),
            self.version(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CheckDigit;
    use crate::Result;

    #[test]
    #[rustfmt::skip]
    fn test_query_boot_part_number_reply_from_buf() -> Result<()> {
        // Type 1 Boot Part Number
        let msg_bytes = [
            // STX | LEN | Message Type
            0x02, 0x0e, 0x60,
            // Project number (in ASCII)
            b'2', b'8', b'0', b'0', b'0',
            // Check Digit (in ASCII)
            b'0',
            // Version (in ASCII)
            b'1', b'2', b'3',
            // ETX | Checksum
            0x03, 0x54,
        ];

        let mut msg = QueryBootPartNumberReply::new();
        msg.from_buf(msg_bytes.as_ref())?;

        assert_eq!(msg.message_type(), MessageType::AuxCommand);

        let exp_project_number = ProjectNumber::type1(28_000, CheckDigit::from(b'0'));
        let exp_part_version = PartVersion::from(b"123");
        let exp_boot_part_number = BootPartNumber::new(exp_project_number, exp_part_version);

        assert_eq!(msg.boot_part_number(), exp_boot_part_number);
        assert_eq!(msg.project_number(), exp_project_number);
        assert_eq!(msg.version(), exp_part_version);
        assert_eq!(msg.version().as_string().as_str(), "V1.23");

        // Type 2 Boot Part Number
        let msg_bytes = [
            // STX | LEN | Message Type
            0x02, 0x0e, 0x60,
            // Project number (in ASCII)
            b'2', b'8', b'6', b'0', b'0', b'0',
            // Version (in ASCII)
            b'1', b'2', b'3',
            // ETX | Checksum
            0x03, 0x52,
        ];

        msg.from_buf(msg_bytes.as_ref())?;

        assert_eq!(msg.message_type(), MessageType::AuxCommand);

        let exp_project_number = ProjectNumber::type2(286_000);
        let exp_part_version = PartVersion::from(b"123");
        let exp_boot_part_number = BootPartNumber::new(exp_project_number, exp_part_version);

        assert_eq!(msg.boot_part_number(), exp_boot_part_number);
        assert_eq!(msg.project_number(), exp_project_number);
        assert_eq!(msg.version(), exp_part_version);
        assert_eq!(msg.version().as_string().as_str(), "V1.23");

        Ok(())
    }
}
