use crate::std;
use std::fmt;

use crate::{
    impl_default, impl_message_ops, impl_omnibus_nop_reply, len::QUERY_VARIANT_ID_REPLY,
    MessageOps, MessageType, PartVersion, ProjectNumber, VariantPartNumber,
};

pub mod index {
    pub const PROJECT_NUM: usize = 3;
    pub const VERSION: usize = 9;
}

/// Query Variant ID - Reply (Subtype 0x0F)
///
/// The part number is composed of a project number (5-6 digits) and version number (3 digits) with an
/// optional Check sum digit in the middle.
///
/// The Query Variant ID Reply is formatted as follows:
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
pub struct QueryVariantIdReply {
    buf: [u8; QUERY_VARIANT_ID_REPLY],
}

impl QueryVariantIdReply {
    /// Creates a new [QueryVariantIdReply].
    pub fn new() -> Self {
        let mut message = Self {
            buf: [0u8; QUERY_VARIANT_ID_REPLY],
        };

        message.init();
        message.set_message_type(MessageType::AuxCommand);

        message
    }

    /// Gets the [VariantPartNumber].
    pub fn variant_part_number(&self) -> VariantPartNumber {
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

impl_default!(QueryVariantIdReply);
impl_message_ops!(QueryVariantIdReply);
impl_omnibus_nop_reply!(QueryVariantIdReply);

impl fmt::Display for QueryVariantIdReply {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AckNak: {}, DeviceType: {}, MessageType: {}, VariantPartNumber: {}, ProjectNumber: {}, Version: {}",
            self.acknak(),
            self.device_type(),
            self.message_type(),
            self.variant_part_number(),
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
    fn test_query_variant_id_reply_from_buf() -> Result<()> {
        // Type 1 Variant Part Number
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

        let mut msg = QueryVariantIdReply::new();
        msg.from_buf(msg_bytes.as_ref())?;

        assert_eq!(msg.message_type(), MessageType::AuxCommand);

        let exp_project_number = ProjectNumber::type1(28_000, CheckDigit::from(b'0'));
        let exp_part_version = PartVersion::from(b"123");
        let exp_variant_part_number = VariantPartNumber::new(exp_project_number, exp_part_version);

        assert_eq!(msg.variant_part_number(), exp_variant_part_number);
        assert_eq!(msg.project_number(), exp_project_number);
        assert_eq!(msg.version(), exp_part_version);
        assert_eq!(msg.version().as_string().as_str(), "V1.23");

        // Type 2 Variant Part Number
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
        let exp_variant_part_number = VariantPartNumber::new(exp_project_number, exp_part_version);

        assert_eq!(msg.variant_part_number(), exp_variant_part_number);
        assert_eq!(msg.project_number(), exp_project_number);
        assert_eq!(msg.version(), exp_part_version);
        assert_eq!(msg.version().as_string().as_str(), "V1.23");

        Ok(())
    }
}
