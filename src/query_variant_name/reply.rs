use crate::std;
use std::fmt;

use crate::{
    impl_default, impl_message_ops, impl_omnibus_nop_reply, len::QUERY_VARIANT_NAME_REPLY,
    MessageOps, MessageType,
};

mod index {
    pub const DATA: usize = 3;
}

/// Query Variant Name - Reply (Subtype 0x08)
///
/// Represents the currency variant name currently in use by the firmware.
///
/// The data returned by the device takes the form of an ASCII string that is either 32 bytes long or is
/// terminated by a non-printable character (`0x00`).
///
/// The Query Variant Name Reply is formatted as follows:
///
/// | Name  | STX  | LEN  | CTRL | Data 0 | Data 1 | ... | Data 31 | ETX  | CHK |
/// |:------|:----:|:----:|:----:|:------:|:------:|:---:|:-------:|:----:|:---:|
/// | Byte  | 0    | 1    | 2    | 3      | 4      | ... | 34      | 35   | 36  |
/// | Value | 0x02 | 0x25 | 0x6n | nn     | nn     | nn  | nn      | 0x03 | zz  |
///
/// The names of the currencies supported are represented as three character ISO codes. If more than one
/// currency is supported, they are separated by underscore `_` characters. For example `USD_CAD` would
/// signify a mixed U.S.A./Canadian bill set.
///
/// For further information on currency descriptors, please see <https://en.wikipedia.org/wiki/ISO_4217>.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QueryVariantNameReply {
    buf: [u8; QUERY_VARIANT_NAME_REPLY],
}

impl QueryVariantNameReply {
    /// Creates a new [QueryVariantNameReply]
    pub fn new() -> Self {
        let mut message = Self {
            buf: [0u8; QUERY_VARIANT_NAME_REPLY],
        };

        message.init();
        message.set_message_type(MessageType::AuxCommand);

        message
    }

    /// Gets the variant name from the [QueryVariantNameReply].
    pub fn variant_name(&self) -> &str {
        let etx_index = self.etx_index();
        let buf = self.buf();

        let name = std::str::from_utf8(buf[index::DATA..etx_index].as_ref()).unwrap_or("Unknown");
        let end = if let Some(i) = name.find('\0') {
            i
        } else {
            name.len()
        };

        &name[..end]
    }
}

impl_default!(QueryVariantNameReply);
impl_message_ops!(QueryVariantNameReply);
impl_omnibus_nop_reply!(QueryVariantNameReply);

impl fmt::Display for QueryVariantNameReply {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AckNak: {}, DeviceType: {}, MessageType: {}, VariantName: {}, Checksum: {}",
            self.acknak(),
            self.device_type(),
            self.message_type(),
            self.variant_name(),
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
    fn test_query_variant_name_reply_from_buf() -> Result<()> {
        // Variant name - short
        let msg_bytes = [
            // STX | LEN | Message Type
            0x02, 0x25, 0x60,
            // Data (name in ASCII)
            b'S', b'C', b'N', b'L', b' ',
            b'6', b'6', b'7', b'0', b'R',
            b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0',
            b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0',
            // ETX | Checksum
            0x03, 0x22,
        ];

        let mut msg = QueryVariantNameReply::new();
        msg.from_buf(msg_bytes.as_ref())?;

        assert_eq!(msg.message_type(), MessageType::AuxCommand);

        assert_eq!(msg.variant_name(), "SCNL 6670R");

        // Variant name - full
        let msg_bytes = [
            // STX | LEN | Message Type
            0x02, 0x25, 0x60,
            // Data (name in ASCII)
            b'S', b'C', b'N', b'L', b' ',
            b'6', b'6', b'7', b'0', b'R', b' ',
            b'P', b'l', b'u', b's', b' ',
            b's', b'o', b'm', b'e', b' ',
            b'l', b'o', b'n', b'g', b' ',
            b's', b'u', b'f', b'f', b'i', b'x',
            // ETX | Checksum
            0x03, 0x11,
        ];

        let mut msg = QueryVariantNameReply::new();
        msg.from_buf(msg_bytes.as_ref())?;

        assert_eq!(msg.message_type(), MessageType::AuxCommand);

        assert_eq!(msg.variant_name(), "SCNL 6670R Plus some long suffix");

        Ok(())
    }
}
