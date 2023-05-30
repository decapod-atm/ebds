use crate::std;
use std::fmt;

use crate::{
    impl_aux_ops, impl_default, impl_message_ops, impl_omnibus_nop_reply,
    len::QUERY_SOFTWARE_CRC_REPLY, seven_bit_u16, u16_seven_bit, AuxCommand, AuxCommandOps,
    MessageOps, MessageType,
};

pub mod index {
    pub const CRC_BEGIN: usize = 3;
    pub const CRC_END: usize = 7;
}

/// Query Software CRC - Reply (Subtype 0x00)
///
/// | **S2K** | **CFSC** | **SC Adv** | **SCR** |
/// |:-------:|:--------:|:----------:|:-------:|
///
/// The Query Software CRC Reply is formatted as follows:
///
/// | Name  | STX  | LEN  | CTRL | CRC 1 | CRC 2 | CRC 3 | CRC 4 | N/A  | N/A  | ETX  | CHK |
/// |:------|:----:|:----:|:----:|:-----:|:-----:|:-----:|:-----:|:----:|:----:|:----:|:---:|
/// | Byte  | 0    | 1    | 2    | 3     | 4     | 5     | 6     | 7    | 8    | 9    | 10  |
/// | Value | 0x02 | 0x08 | 0x6n | 0x0n  | 0x0n  | 0x0n  | 0x0n  | 0x00 | 0x00 | 0x03 | zz  |
///
/// The 16 bit CRC data is sent in bytes 3 through 6, four bits at a time.
///
/// This may be extracted as shown below:
///
/// Example:
///
/// ```rust
/// # use ebds::MessageOps;
/// let mut reply_crc = ebds::QuerySoftwareCrcReply::new();
///
/// let exp_crc = 0x1234;
/// reply_crc.set_crc(exp_crc);
///
/// let crc_bytes = reply_crc.buf();
///
/// let hi = ((crc_bytes[3] & 0xf) << 4) | (crc_bytes[4] & 0xf);
/// let lo = ((crc_bytes[5] & 0xf) << 4) | (crc_bytes[6] & 0xf);
///
/// let crc = u16::from_be_bytes([hi, lo]);
///
/// assert_eq!(exp_crc, crc);
/// ```
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QuerySoftwareCrcReply {
    buf: [u8; QUERY_SOFTWARE_CRC_REPLY],
}

impl QuerySoftwareCrcReply {
    pub fn new() -> Self {
        let mut message = Self {
            buf: [0u8; QUERY_SOFTWARE_CRC_REPLY],
        };

        message.init();
        message.set_message_type(MessageType::AuxCommand);
        message.set_aux_command(AuxCommand::QuerySoftwareCrc);

        message
    }

    /// Gets the CRC-16 from the [QuerySoftwareCrcReply].
    pub fn crc(&self) -> u16 {
        seven_bit_u16(self.buf[index::CRC_BEGIN..index::CRC_END].as_ref())
    }

    /// Sets the CRC-16 from the [QuerySoftwareCrcReply].
    pub fn set_crc(&mut self, crc: u16) {
        self.buf[index::CRC_BEGIN..index::CRC_END].copy_from_slice(u16_seven_bit(crc).as_ref());
    }
}

impl_default!(QuerySoftwareCrcReply);
impl_message_ops!(QuerySoftwareCrcReply);
impl_omnibus_nop_reply!(QuerySoftwareCrcReply);
impl_aux_ops!(QuerySoftwareCrcReply);

impl fmt::Display for QuerySoftwareCrcReply {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AckNak: {}, DeviceType: {}, MessageType: {}, CRC: {}",
            self.acknak(),
            self.device_type(),
            self.message_type(),
            self.crc(),
        )
    }
}
