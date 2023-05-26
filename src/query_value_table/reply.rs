use crate::std;
use std::fmt;

use crate::{
    banknote::*, impl_default, impl_extended_ops, impl_message_ops, impl_omnibus_extended_reply,
    len::QUERY_VALUE_TABLE_REPLY, ExtendedCommand, ExtendedCommandOps, MessageOps, MessageType,
    OmnibusReplyOps,
};

/// Represents a denomination in non-extended mode.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BaseDenomination {
    /// Note value index reported starting with '1' and ending with '7'.
    note_index: usize,
    /// A three character ASCII currency code.
    iso_code: ISOCode,
    /// A three character ASCII decimal value.
    base_value: BaseValue,
    /// An ASCII coded sign (+/-) to be used with the exponent.
    sign: Sign,
    /// ASCII coded decimal value for the power of ten that the base is multiplied/divided.
    exponent: Exponent,
}

impl BaseDenomination {
    pub const LEN: usize = 10;

    /// Gets the index of the [BaseDenomination].
    pub fn note_index(&self) -> usize {
        self.note_index
    }

    /// Gets the ISO code of the [BaseDenomination].
    pub fn iso_code(&self) -> ISOCode {
        self.iso_code
    }

    /// Gets the base value of the [BaseDenomination].
    pub fn base_value(&self) -> BaseValue {
        self.base_value
    }

    /// Gets the sign of the [BaseDenomination].
    pub fn sign(&self) -> Sign {
        self.sign
    }

    /// Gets the exponent of the [BaseDenomination].
    pub fn exponent(&self) -> Exponent {
        self.exponent
    }

    /// Gets the full value of the [BaseDenomination].
    ///
    /// Convenience function for `base_value * 10^([+-]exponent)`.
    pub fn value(&self) -> f32 {
        let base_value: f32 = self.base_value.into();
        let exponent: f32 = self.exponent.into();

        match self.sign {
            Sign::Positive => base_value * 10f32.powf(exponent),
            Sign::Negative => base_value * 10f32.powf(-exponent),
        }
    }
}

impl From<&[u8]> for BaseDenomination {
    fn from(b: &[u8]) -> Self {
        Self {
            note_index: b[index::DENOM_INDEX] as usize,
            iso_code: b[index::DENOM_ISO..index::DENOM_ISO_END].as_ref().into(),
            base_value: b[index::DENOM_BASE_VALUE..index::DENOM_BASE_VALUE_END]
                .as_ref()
                .into(),
            sign: b[index::DENOM_SIGN].into(),
            exponent: b[index::DENOM_EXPONENT..index::DENOM_EXPONENT_END]
                .as_ref()
                .into(),
        }
    }
}

impl<const N: usize> From<[u8; N]> for BaseDenomination {
    fn from(b: [u8; N]) -> Self {
        b.as_ref().into()
    }
}

impl<const N: usize> From<&[u8; N]> for BaseDenomination {
    fn from(b: &[u8; N]) -> Self {
        b.as_ref().into()
    }
}

impl From<&BaseDenomination> for Banknote {
    fn from(b: &BaseDenomination) -> Self {
        Self::new(
            b.value(),
            b.iso_code(),
            NoteType::default(),
            NoteSeries::default(),
            NoteCompatibility::default(),
            NoteVersion::default(),
            BanknoteClassification::Genuine,
        )
    }
}

impl From<BaseDenomination> for Banknote {
    fn from(b: BaseDenomination) -> Banknote {
        (&b).into()
    }
}

impl From<&BaseDenomination> for NoteTableItem {
    fn from(b: &BaseDenomination) -> Self {
        NoteTableItem::new(b.note_index(), b.into())
    }
}

impl From<BaseDenomination> for NoteTableItem {
    fn from(b: BaseDenomination) -> Self {
        (&b).into()
    }
}

impl fmt::Display for BaseDenomination {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let open_brace = "{";
        let note_index = self.note_index;
        let iso_code = self.iso_code;
        let base_value: u16 = self.base_value.into();
        let sign: &str = self.sign.into();
        let exponent: u8 = self.exponent.into();
        let close_brace = "}";
        write!(f,
               "{open_brace}\"note_index\":{note_index}, \"iso_code\":{iso_code}, \"base_value\":{base_value}, \"sign\":{sign}, \"exponent\":{exponent}{close_brace}")
    }
}

mod index {
    use crate::{BaseDenomination, BaseValue, Exponent, ISOCode};

    pub const DENOM: usize = 10;
    pub const DENOM0: usize = DENOM;
    pub const DENOM1: usize = DENOM0 + BaseDenomination::LEN;
    pub const DENOM2: usize = DENOM1 + BaseDenomination::LEN;
    pub const DENOM3: usize = DENOM2 + BaseDenomination::LEN;
    pub const DENOM4: usize = DENOM3 + BaseDenomination::LEN;
    pub const DENOM5: usize = DENOM4 + BaseDenomination::LEN;
    pub const DENOM6: usize = DENOM5 + BaseDenomination::LEN;
    pub const DENOM6_END: usize = DENOM6 + BaseDenomination::LEN;

    pub const DENOM_INDEX: usize = 0;
    pub const DENOM_ISO: usize = 1;
    pub const DENOM_ISO_END: usize = DENOM_ISO + ISOCode::LEN;
    pub const DENOM_BASE_VALUE: usize = 4;
    pub const DENOM_BASE_VALUE_END: usize = DENOM_BASE_VALUE + BaseValue::LEN;
    pub const DENOM_SIGN: usize = 7;
    pub const DENOM_EXPONENT: usize = 8;
    pub const DENOM_EXPONENT_END: usize = DENOM_EXPONENT + Exponent::LEN;
}

/// Query Value Table - Reply (Subtype 0x06)
///
/// This message is a reply to [QueryValueTableCommand](crate::QueryValueTableCommand),
/// and contains the note table in the extended data.
///
/// The Query Value Table Reply is formatted as follows:
///
/// | Name  | STX  | LEN  | CTRL | Subtype | Data 0 | Data 1 | Data 2 | Data 3 | Data 4 | Data 5 | ETX  | CHK |
/// |:------|:----:|:----:|:----:|:-------:|:------:|:------:|:------:|:------:|:------:|:------:|:----:|:---:|
/// | Byte  | 0    | 1    | 2    | 3       | 4      | 5      | 6      | 7      | 8      | 9      | 10   | 11  |
/// | Value | 0x02 | 0x0C | 0x7n | 0x06    | nn     | nn     | nn     | nn     | nn     | nn     | 0x03 | zz  |
///
///
/// | Field      | Byte Offset | Field Description                                                                              | Sample Value<br>(2000 Yen) |
/// |:-----------|:-----------:|:----------------------------------------------------------------------------------------------:|:--------------------------:|
/// | Index      | 0           | Note Value Index reported starting with ‘1’ and ending with the final index ‘7’.               | 0x03                       |
/// | ISO Code   | 1..3        | A three character ASCII currency code. See ISO_4217 at the Wikipedia for details.              | “JPY”                      |
/// | Base Value | 4..6        | A three character ASCII coded decimal value.                                                   | “002”                      |
/// | Sign       | 7           | An ASCII coded sign value for the Exponent.<br>This field is either a “+” or a “-“.            | “+”                        |
/// | Exponent   | 8..9        | ASCII coded decimal value for the power of ten that the base is (multiplied “+”, divided “-“). | "03"                       |
///
/// In this example: Note Value = `002 x 10^03 = 2 x 1000 = ¥2000`.
///
/// If a Value Index does not have a corresponding denomination value, then all fields will be `0x00` following
/// the Value Index.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QueryValueTableReply {
    buf: [u8; QUERY_VALUE_TABLE_REPLY],
}

impl QueryValueTableReply {
    /// Creates a new [QueryValueTableReply].
    pub fn new() -> Self {
        let mut message = Self {
            buf: [0u8; QUERY_VALUE_TABLE_REPLY],
        };

        message.init();
        message.set_message_type(MessageType::Extended);
        message.set_extended_command(ExtendedCommand::QueryValueTable);

        message
    }

    /// Get the [BaseDenomination] 0.
    pub fn denom0(&self) -> BaseDenomination {
        self.buf[index::DENOM0..index::DENOM1].as_ref().into()
    }

    /// Get the [BaseDenomination] 1.
    pub fn denom1(&self) -> BaseDenomination {
        self.buf[index::DENOM1..index::DENOM2].as_ref().into()
    }

    /// Get the [BaseDenomination] 2.
    pub fn denom2(&self) -> BaseDenomination {
        self.buf[index::DENOM2..index::DENOM3].as_ref().into()
    }

    /// Get the [BaseDenomination] 3.
    pub fn denom3(&self) -> BaseDenomination {
        self.buf[index::DENOM3..index::DENOM4].as_ref().into()
    }

    /// Get the [BaseDenomination] 4.
    pub fn denom4(&self) -> BaseDenomination {
        self.buf[index::DENOM4..index::DENOM5].as_ref().into()
    }

    /// Get the [BaseDenomination] 5.
    pub fn denom5(&self) -> BaseDenomination {
        self.buf[index::DENOM5..index::DENOM6].as_ref().into()
    }

    /// Get the [BaseDenomination] 6.
    pub fn denom6(&self) -> BaseDenomination {
        self.buf[index::DENOM6..index::DENOM6_END].as_ref().into()
    }
}

impl_default!(QueryValueTableReply);
impl_message_ops!(QueryValueTableReply);
impl_omnibus_extended_reply!(QueryValueTableReply);
impl_extended_ops!(QueryValueTableReply);

impl fmt::Display for QueryValueTableReply {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AckNak: {}, DeviceType: {}, MessageType: {}, Subtype: {}, DeviceState: {}, DeviceStatus: {}, ExceptionStatus: {}, MiscDeviceState: {}, ModelNumber: {}, CodeRevision: {}, Denom0: {}, Denom1: {}, Denom2: {}, Denom3: {}, Denom4: {}, Denom5: {}, Denom6: {}",
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
            self.denom0(),
            self.denom1(),
            self.denom2(),
            self.denom3(),
            self.denom4(),
            self.denom5(),
            self.denom6(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;

    #[test]
    #[rustfmt::skip]
    fn test_query_value_table_reply_from_bytes() -> Result<()> {
        let msg_bytes = [
            // STX | LEN | Message type | Subtype
            0x02, 0x52, 0x70, 0x06,
            // Data
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            // Denom 0
            0x01, b'U', b'S', b'D', b'0', b'0', b'1', b'+', b'0', b'0', 
            // Denom 1
            0x02, b'U', b'S', b'D', b'0', b'0', b'2', b'+', b'0', b'0', 
            // Denom 2
            0x03, b'U', b'S', b'D', b'0', b'0', b'5', b'+', b'0', b'0', 
            // Denom 3
            0x04, b'U', b'S', b'D', b'0', b'0', b'1', b'+', b'0', b'1', 
            // Denom 4
            0x05, b'U', b'S', b'D', b'0', b'0', b'2', b'+', b'0', b'1', 
            // Denom 5
            0x06, b'U', b'S', b'D', b'0', b'0', b'5', b'+', b'0', b'1', 
            // Denom 6
            0x07, b'U', b'S', b'D', b'0', b'0', b'1', b'+', b'0', b'2', 
            // ETX | Checksum
            0x03, 0x7f,
        ];

        let mut msg = QueryValueTableReply::new();
        msg.from_buf(msg_bytes.as_ref())?;

        assert_eq!(msg.message_type(), MessageType::Extended);
        assert_eq!(msg.extended_command(), ExtendedCommand::QueryValueTable);

        let exp_denom0 = BaseDenomination::from([0x01, b'U', b'S', b'D', b'0', b'0', b'1', b'+', b'0', b'0']);
        let exp_denom1 = BaseDenomination::from([0x02, b'U', b'S', b'D', b'0', b'0', b'2', b'+', b'0', b'0']);
        let exp_denom2 = BaseDenomination::from([0x03, b'U', b'S', b'D', b'0', b'0', b'5', b'+', b'0', b'0']);
        let exp_denom3 = BaseDenomination::from([0x04, b'U', b'S', b'D', b'0', b'0', b'1', b'+', b'0', b'1']);
        let exp_denom4 = BaseDenomination::from([0x05, b'U', b'S', b'D', b'0', b'0', b'2', b'+', b'0', b'1']);
        let exp_denom5 = BaseDenomination::from([0x06, b'U', b'S', b'D', b'0', b'0', b'5', b'+', b'0', b'1']);
        let exp_denom6 = BaseDenomination::from([0x07, b'U', b'S', b'D', b'0', b'0', b'1', b'+', b'0', b'2']);

        assert_eq!(msg.denom0(), exp_denom0);
        assert_eq!(msg.denom0().note_index(), 1);
        assert_eq!(msg.denom0().iso_code(), ISOCode::USD);
        assert_eq!(msg.denom0().base_value(), BaseValue::from(b"001"));
        assert_eq!(msg.denom0().sign(), Sign::Positive);
        assert_eq!(msg.denom0().exponent(), Exponent::from(b"00"));
        assert_eq!(Banknote::from(msg.denom0()).value(), 1.0);

        assert_eq!(msg.denom1(), exp_denom1);
        assert_eq!(msg.denom1().note_index(), 2);
        assert_eq!(msg.denom1().iso_code(), ISOCode::USD);
        assert_eq!(msg.denom1().base_value(), BaseValue::from(b"002"));
        assert_eq!(msg.denom1().sign(), Sign::Positive);
        assert_eq!(msg.denom1().exponent(), Exponent::from(b"00"));
        assert_eq!(Banknote::from(msg.denom1()).value(), 2.0);

        assert_eq!(msg.denom2(), exp_denom2);
        assert_eq!(msg.denom2().note_index(), 3);
        assert_eq!(msg.denom2().iso_code(), ISOCode::USD);
        assert_eq!(msg.denom2().base_value(), BaseValue::from(b"005"));
        assert_eq!(msg.denom2().sign(), Sign::Positive);
        assert_eq!(msg.denom2().exponent(), Exponent::from(b"00"));
        assert_eq!(Banknote::from(msg.denom2()).value(), 5.0);

        assert_eq!(msg.denom3(), exp_denom3);
        assert_eq!(msg.denom3().note_index(), 4);
        assert_eq!(msg.denom3().iso_code(), ISOCode::USD);
        assert_eq!(msg.denom3().base_value(), BaseValue::from(b"001"));
        assert_eq!(msg.denom3().sign(), Sign::Positive);
        assert_eq!(msg.denom3().exponent(), Exponent::from(b"01"));
        assert_eq!(Banknote::from(msg.denom3()).value(), 10.0);

        assert_eq!(msg.denom4(), exp_denom4);
        assert_eq!(msg.denom4().note_index(), 5);
        assert_eq!(msg.denom4().iso_code(), ISOCode::USD);
        assert_eq!(msg.denom4().base_value(), BaseValue::from(b"002"));
        assert_eq!(msg.denom4().sign(), Sign::Positive);
        assert_eq!(msg.denom4().exponent(), Exponent::from(b"01"));
        assert_eq!(Banknote::from(msg.denom4()).value(), 20.0);

        assert_eq!(msg.denom5(), exp_denom5);
        assert_eq!(msg.denom5().note_index(), 6);
        assert_eq!(msg.denom5().iso_code(), ISOCode::USD);
        assert_eq!(msg.denom5().base_value(), BaseValue::from(b"005"));
        assert_eq!(msg.denom5().sign(), Sign::Positive);
        assert_eq!(msg.denom5().exponent(), Exponent::from(b"01"));
        assert_eq!(Banknote::from(msg.denom5()).value(), 50.0);

        assert_eq!(msg.denom6(), exp_denom6);
        assert_eq!(msg.denom6().note_index(), 7);
        assert_eq!(msg.denom6().iso_code(), ISOCode::USD);
        assert_eq!(msg.denom6().base_value(), BaseValue::from(b"001"));
        assert_eq!(msg.denom6().sign(), Sign::Positive);
        assert_eq!(msg.denom6().exponent(), Exponent::from(b"02"));
        assert_eq!(Banknote::from(msg.denom6()).value(), 100.0);

        Ok(())
    }
}
