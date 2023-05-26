use crate::std;
use std::fmt;

use crate::{
    impl_default, impl_message_ops, impl_omnibus_nop_reply, len::QUERY_DEVICE_CAPABILITIES_REPLY,
    MessageOps, MessageType, CLOSE_BRACE, OPEN_BRACE,
};

mod index {
    pub const CAP0: usize = 3;
    pub const CAP1: usize = 4;
    pub const CAP2: usize = 5;
    pub const CAP3: usize = 6;
    pub const CAP4: usize = 7;
    pub const CAP5: usize = 8;
}

bitfield! {
    /// First set of device capabilties
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct Cap0(u8);
    u8;
    /// **OBSOLETE** Extended PUP mode is supported.
    pub extended_pup_mode, _: 0;
    /// Extended orientation handling is supported.
    pub extended_orientation, _: 1;
    /// [QueryApplicationId](crate::QueryApplicationIdCommand) and [QueryVariantId](crate::QueryVariantIdCommand) are supported
    pub application_and_variant_id, _: 2;
    /// QueryBNFStatus is supported.
    pub bnf_status, _: 3;
    /// Test documents are supported.
    pub test_documents, _: 4;
    /// Set Bezel is supported
    pub bezel, _: 5;
    /// Easitrax is supported (with Query Asset Number).
    pub easitrax, _: 6;
}

impl fmt::Display for Cap0 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{OPEN_BRACE}\"extended_pup_mode\":{},\"extended_orientation\":{},\"application_and_variant_id\":{},\"bnf_status\":{},\"test_documents\":{},\"bezel\":{},\"easitrax\":{}{CLOSE_BRACE}",
            self.extended_pup_mode(),
            self.extended_orientation(),
            self.application_and_variant_id(),
            self.bnf_status(),
            self.test_documents(),
            self.bezel(),
            self.easitrax(),
        )
    }
}

impl From<u8> for Cap0 {
    fn from(b: u8) -> Self {
        Self(b & 0b111_1111)
    }
}

impl From<&Cap0> for u8 {
    fn from(c: &Cap0) -> Self {
        c.0
    }
}

impl From<Cap0> for u8 {
    fn from(c: Cap0) -> Self {
        (&c).into()
    }
}

bitfield! {
    /// Second set of device capabilties
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct Cap1(u8);
    u8;
    /// Note Retrieved is supported.
    pub note_retrieved, _: 0;
    /// Advanced Bookmark mode is supported. (see 7.5.9)
    pub advanced_bookmark, _: 1;
    /// Device capable of ABDS download.
    pub abds_download, _: 2;
    /// Device supports Clear Audit Command. (see 7.5.23)
    pub clear_audit, _: 3;
    /// Multi-note escrow is supported.
    pub multi_note_escrow, _: 4;
    /// 32-bit Unix timestamp is supported.
    pub unix_timestamp_32bit, _: 5;
}

impl fmt::Display for Cap1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{OPEN_BRACE}\"note_retrieved\":{},\"advanced_bookmark\":{},\"abds_download\":{},\"clear_audit\":{},\"multi_note_escrow\":{},\"unix_timestamp_32bit\":{}{CLOSE_BRACE}",
            self.note_retrieved(),
            self.advanced_bookmark(),
            self.abds_download(),
            self.clear_audit(),
            self.multi_note_escrow(),
            self.unix_timestamp_32bit(),
        )
    }
}

impl From<u8> for Cap1 {
    fn from(b: u8) -> Self {
        Self(b & 0b11_1111)
    }
}

impl From<&Cap1> for u8 {
    fn from(c: &Cap1) -> Self {
        c.0
    }
}

impl From<Cap1> for u8 {
    fn from(c: Cap1) -> Self {
        (&c).into()
    }
}

bitfield! {
    /// Third set of device capabilties
    ///
    /// Note: **SCR Classification** If banknote classification is supported (i.e. Cap Byte 3, bit 1 is set), all denomination
    /// recycling bits will be set to 0.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct Cap2(u8);
    u8;
    /// 1 Denomination recycling is supported.
    pub one_denom_recycling, _: 0;
    /// 2 Denomination recycling is supported.
    pub two_denom_recycling, _: 1;
    /// 3 Denomination recycling is supported.
    pub three_denom_recycling, _: 2;
    /// 4 Denomination recycling is supported.
    pub four_denom_recycling, _: 3;
    /// **Retail Only** Improperly Seated Head Detection is supported.
    pub improperly_seated_head_detection, _: 4;
    /// **SCR** Host Controlled Recycler Inventory (Mixed Denomintion Recycling) is supported.
    pub mixed_denom_recycling, _: 6;
}

impl fmt::Display for Cap2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{OPEN_BRACE}\"one_denom_recycling\":{},\"two_denom_recycling\":{},\"three_denom_recycling\":{},\"four_denom_recycling\":{},\"improperly_seated_head_detection\":{},\"mixed_denom_recycling\":{}{CLOSE_BRACE}",
            self.one_denom_recycling(),
            self.two_denom_recycling(),
            self.three_denom_recycling(),
            self.four_denom_recycling(),
            self.improperly_seated_head_detection(),
            self.mixed_denom_recycling(),
        )
    }
}

impl From<u8> for Cap2 {
    fn from(b: u8) -> Self {
        Self(b & 0b101_1111)
    }
}

impl From<&Cap2> for u8 {
    fn from(c: &Cap2) -> Self {
        c.0
    }
}

impl From<Cap2> for u8 {
    fn from(c: Cap2) -> Self {
        (&c).into()
    }
}

bitfield! {
    /// Fourth set of device capabilties
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct Cap3(u8);
    u8;
    /// Customer Configuration Options Set/Query (Msg 6 Subtypes 0x25 and 0x26) are supported.
    pub customer_config, _: 0;
    /// **SCR Classification** Banknote classification is supported.
    pub banknote_classification, _: 1;
}

impl fmt::Display for Cap3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{OPEN_BRACE}\"customer_config\":{},\"banknote_classification\":{}{CLOSE_BRACE}",
            self.customer_config(),
            self.banknote_classification(),
        )
    }
}

impl From<u8> for Cap3 {
    fn from(b: u8) -> Self {
        Self(b & 0x000_0011)
    }
}

impl From<&Cap3> for u8 {
    fn from(c: &Cap3) -> Self {
        c.0
    }
}

impl From<Cap3> for u8 {
    fn from(c: Cap3) -> Self {
        (&c).into()
    }
}

bitfield! {
    /// Fifth set of device capabilties
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct Cap4(u8);
    u8;
    /// RFU
    pub reserved, _: 6, 0;
}

impl fmt::Display for Cap4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{OPEN_BRACE}\"reserved\": 0b{:08b}{CLOSE_BRACE}",
            self.reserved(),
        )
    }
}

impl From<u8> for Cap4 {
    fn from(b: u8) -> Self {
        Self(b & 0b111_1111)
    }
}

impl From<&Cap4> for u8 {
    fn from(c: &Cap4) -> Self {
        c.0
    }
}

impl From<Cap4> for u8 {
    fn from(c: Cap4) -> Self {
        (&c).into()
    }
}

bitfield! {
    /// Sixth set of device capabilties
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct Cap5(u8);
    u8;
    /// RFU
    pub reserved, _: 6, 0;
}

impl fmt::Display for Cap5 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{OPEN_BRACE}\"reserved\": 0b{:08b}{CLOSE_BRACE}",
            self.reserved()
        )
    }
}

impl From<u8> for Cap5 {
    fn from(b: u8) -> Self {
        Self(b & 0b111_1111)
    }
}

impl From<&Cap5> for u8 {
    fn from(c: &Cap5) -> Self {
        c.0
    }
}

impl From<Cap5> for u8 {
    fn from(c: Cap5) -> Self {
        (&c).into()
    }
}

/// Query Device Capabilities - Reply (Subtype 0x0D)
///
/// This the reply for the query of device capabilities:
/// [QueryDeviceCapabilitiesCommand](crate::QueryDeviceCapabilitiesCommand).
///
/// The Query Device Capabilities Reply is formatted as follows:
///
/// | Name  | STX  | LEN  | CTRL | Cap 0 | Cap 1 | Cap 2 | Cap 3 | Cap 4 | Cap 5 | ETX  | CHK |
/// |:------|:----:|:----:|:----:|:-----:|:-----:|:-----:|:-----:|:-----:|:-----:|:----:|:---:|
/// | Byte  | 0    | 1    | 2    | 3     | 4     | 5     | 6     | 7     | 8     | 9    | 10  |
/// | Value | 0x02 | 0x0B | 0x6n | nn    | nn    | nn    | nn    | nn    | nn    | 0x03 | zz  |
///
/// The `Cap` fields are bitfields describing device capabilities:
///
/// * [Cap0]
/// * [Cap1]
/// * [Cap2]
/// * [Cap3]
/// * [Cap4] - RFU
/// * [Cap5] - RFU
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QueryDeviceCapabilitiesReply {
    buf: [u8; QUERY_DEVICE_CAPABILITIES_REPLY],
}

impl QueryDeviceCapabilitiesReply {
    /// Create a new [QueryDeviceCapabilitiesReply] message
    pub fn new() -> Self {
        let mut message = Self {
            buf: [0u8; QUERY_DEVICE_CAPABILITIES_REPLY],
        };

        message.init();
        message.set_message_type(MessageType::AuxCommand);

        message
    }

    /// Get the first set of capabilities
    pub fn cap0(&self) -> Cap0 {
        self.buf[index::CAP0].into()
    }

    /// Get the second set of capabilities
    pub fn cap1(&self) -> Cap1 {
        self.buf[index::CAP1].into()
    }

    /// Get the third set of capabilities
    pub fn cap2(&self) -> Cap2 {
        self.buf[index::CAP2].into()
    }

    /// Get the fourth set of capabilities
    pub fn cap3(&self) -> Cap3 {
        self.buf[index::CAP3].into()
    }

    /// Get the fifth set of capabilities
    pub fn cap4(&self) -> Cap4 {
        self.buf[index::CAP4].into()
    }

    /// Get the sixth set of capabilities
    pub fn cap5(&self) -> Cap5 {
        self.buf[index::CAP5].into()
    }
}

impl_default!(QueryDeviceCapabilitiesReply);
impl_message_ops!(QueryDeviceCapabilitiesReply);
impl_omnibus_nop_reply!(QueryDeviceCapabilitiesReply);

impl fmt::Display for QueryDeviceCapabilitiesReply {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AckNak: {}, DeviceType: {}, MessageType: {}, Capability set 0: {}, Capability set 1: {}, Capability set 2: {}, Capability set 3: {}, Capability set 4: {}, Capability set 5: {}",
            self.acknak(),
            self.device_type(),
            self.message_type(),
            self.cap0(),
            self.cap1(),
            self.cap2(),
            self.cap3(),
            self.cap4(),
            self.cap5(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;

    #[test]
    #[rustfmt::skip]
    fn test_query_device_capabilities_reply_from_buf() -> Result<()> {
        let msg_bytes = [
            // STX | LEN | Message Type
            0x02, 0x0b, 0x60,
            // Capabilities
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            // ETX | Checksum
            0x03, 0x6b,
        ];

        let mut msg = QueryDeviceCapabilitiesReply::new();
        msg.from_buf(msg_bytes.as_ref())?;

        assert_eq!(msg.message_type(), MessageType::AuxCommand);

        assert_eq!(msg.cap0(), Cap0::from(0));
        assert_eq!(msg.cap1(), Cap1::from(0));
        assert_eq!(msg.cap2(), Cap2::from(0));
        assert_eq!(msg.cap3(), Cap3::from(0));
        assert_eq!(msg.cap4(), Cap4::from(0));
        assert_eq!(msg.cap5(), Cap5::from(0));

        Ok(())
    }
}
