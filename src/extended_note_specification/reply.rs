use crate::std;
use std::fmt;

use crate::{
    banknote::*, impl_default, impl_extended_ops, impl_message_ops, impl_omnibus_extended_reply,
    len::EXTENDED_NOTE_REPLY, status::*, ExtendedCommand, ExtendedCommandOps, MessageOps,
    MessageType, OmnibusReplyOps,
};

impl From<&ExtendedNoteReply> for Banknote {
    fn from(reply: &ExtendedNoteReply) -> Self {
        let base_value: f32 = reply.base_value().into();
        let exponent: f32 = reply.exponent().into();

        let value = match reply.sign() {
            Sign::Positive => base_value * 10f32.powf(exponent),
            Sign::Negative => base_value * 10f32.powf(-exponent),
        };

        Self::new(
            value,
            reply.iso_code(),
            reply.note_type(),
            reply.note_series(),
            reply.note_compatibility(),
            reply.note_version(),
            reply.banknote_classification(),
        )
    }
}

impl From<ExtendedNoteReply> for Banknote {
    fn from(reply: ExtendedNoteReply) -> Self {
        Self::from(&reply)
    }
}

impl From<&ExtendedNoteReply> for NoteTableItem {
    fn from(e: &ExtendedNoteReply) -> NoteTableItem {
        Self::new(e.note_index(), e.into())
    }
}

impl From<ExtendedNoteReply> for NoteTableItem {
    fn from(e: ExtendedNoteReply) -> NoteTableItem {
        (&e).into()
    }
}

impl From<&ExtendedNoteReply> for DocumentStatus {
    fn from(reply: &ExtendedNoteReply) -> Self {
        let status = DocumentStatus::default().with_standard_denomination(reply.note_value());

        match reply.banknote_classification() {
            BanknoteClassification::Genuine | BanknoteClassification::DisabledOrNotSupported => {
                status.with_accepted_note_table_item(AcceptedNoteTableItem::new(
                    reply.into(),
                    reply.orientation(),
                ))
            }
            _ => status,
        }
    }
}

pub mod index {
    use super::{BaseValue, Exponent, ISOCode};

    pub const EXTENDED: usize = 10;
    pub const NOTE_INDEX: usize = EXTENDED;
    pub const ISO_CODE: usize = EXTENDED + 1;
    pub const ISO_CODE_END: usize = ISO_CODE + ISOCode::LEN;
    pub const BASE_VALUE: usize = EXTENDED + 4;
    pub const BASE_VALUE_END: usize = BASE_VALUE + BaseValue::LEN;
    pub const SIGN: usize = EXTENDED + 7;
    pub const EXPONENT: usize = EXTENDED + 8;
    pub const EXPONENT_END: usize = EXPONENT + Exponent::LEN;
    pub const ORIENTATION: usize = EXTENDED + 10;
    pub const NOTE_TYPE: usize = EXTENDED + 11;
    pub const NOTE_SERIES: usize = EXTENDED + 12;
    pub const NOTE_COMPATIBILITY: usize = EXTENDED + 13;
    pub const NOTE_VERSION: usize = EXTENDED + 14;
    pub const BANKNOTE_CLASSIFICATION: usize = EXTENDED + 15;
}

/// Extended Note Specification - Reply (Subtype 0x02)
///
/// ExtendedNoteReply represents a message sent from the device back to the host
///
/// The reply contains 18 additional bytes of data that describe the bank note in great detail. This message
//// can be sent from the device for two reasons:
///
/// * Response to a host’s query extended note command
/// * Device is running in extended note mode and a valid banknote has either reached escrow or
/// been stacked.
///
/// The Extended Note Reply is formatted as follows:
///
/// | Name  | STX  | LEN  | CTRL | Subtype | Data 0 | Data 1 | Data 2 | Data 3 | Data 4 | Data 5 | ExtData 0 | ... | ExtData 17 | ETX  | CHK |
/// |:------|:----:|:----:|:----:|:-------:|:------:|:------:|:------:|:------:|:------:|:------:|:---------:|:---:|:----------:|:----:|:---:|
/// | Byte  | 0    | 1    | 2    | 3       | 4      | 5      | 6      | 7      | 8      | 9      | 10        | ... | 27         | 28   | 29  |
/// | Value | 0x02 | 0x1E | 0x7n | 0x03    | nn     | nn     | nn     | nn     | nn     | nn     | nn        | nn  | nn         | 0x03 | zz  |
///
/// If this is a reply to the Host Query Command, the index will match the same value sent from the host.
/// Otherwise, the index value is not used and set to 0x00 for any escrowed or stacked notes.
///
/// | Field                   | Byte Offset | Field Description                                    | Sample Value <br> (2000 Yen Note)  |
/// |:------------------------|:-----------:|:-----------------------------------------------------|:----------------------------------:|
/// | Index                   | 0           | Not used for escrow or stacked notes                 | 0x00                               |
/// | ISO Code                | 1..3        | A three character ASCII currecny code,<br> see <https://en.wikipedia.org/wiki/ISO_4217> | "JPY" |
/// | Base Value              | 4..6        | A three character ASCII coded decimal value          | "002"                              |
/// | Sign                    | 7           | An ASCII coded sign value for the Exponent ("+"/"-") | "+"                                |
/// | Exponent                | 8..9        | An ASCII coded decimal power of ten to [multiply "+", divide "-"] the Base Value | "03"   |
/// | Orientation             | 10          | A single character binary field that encodes the orientation of the bank note.<br><br>0x00 = Right Edge, Face Up<br>0x01 = Right Edge, Face Down<br>0x02 = Left Edge, Face Up<br>0x03 = Left Edge, Face Down<br><br>Note: in general this field is only correct if the Extended Orientation bit is set in the device capabilities map.| 0x00 |
/// | Type                    | 11          | An ASCII letter that documents the note type.<br>This corresponds to the data in the variant identity card. | "A" |
/// | Series                  | 12          | An ASCII letter that documents the note series.<br>This corresponds to the data in the variant identity card. | "A" |
/// | Compatibility           | 13          | An ASCII letter that documents the revision of the compatibility core used.<br>This corresponds to the data in the variant identity card. | "B" |
/// | Version                 | 14          | An ASCII letter that documents the version of the note's recognition criteria.<br>This corresponds to the data in the variant identity card. | "A" |
/// | Banknote Classification | 15          | 0x00 = Sent for any of the following:<ul><li>In response to a Host Query Extended Note Specification Command (i.e. host requests a note table element).</li><li>In response to a note escrowed or stacked event while device is in extended note mode and classification is:<ul><li>Supported by the device but disabled.</li><li>NOT supported by the device.</li></ul></li></ul><br>**SC Adv Classification**<br>**SCR Classification**<br>0x01 = Class 1 (unidentified banknote)<br>0x02 = Class 2 (suspected counterfeit)<br>0x03 = Class 3 (suspected zero value note)<br>0x04 = Class 4 (genuine banknote) | 0x00 |
/// | Reserved                | 16..17      | Bytes reserved for future use                         | N/A                             |
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ExtendedNoteReply {
    buf: [u8; EXTENDED_NOTE_REPLY],
}

impl ExtendedNoteReply {
    /// Create a new ExtendedNoteReply message
    pub fn new() -> Self {
        let mut message = Self {
            buf: [0u8; EXTENDED_NOTE_REPLY],
        };

        message.init();
        message.set_message_type(MessageType::Extended);
        message.set_extended_command(ExtendedCommand::ExtendedNoteSpecification);

        message
    }

    /// Get the note index (invalid for escrowed and stacked notes).
    pub fn note_index(&self) -> usize {
        self.buf[index::NOTE_INDEX] as usize
    }

    /// Get the ISO 4217 code
    pub fn iso_code(&self) -> ISOCode {
        self.buf[index::ISO_CODE..index::ISO_CODE_END]
            .as_ref()
            .into()
    }

    /// Get the note's base value
    pub fn base_value(&self) -> BaseValue {
        self.buf[index::BASE_VALUE..index::BASE_VALUE_END]
            .as_ref()
            .into()
    }

    /// Get the note's sign
    pub fn sign(&self) -> Sign {
        self.buf[index::SIGN].into()
    }

    /// Get the note's exponent
    pub fn exponent(&self) -> Exponent {
        self.buf[index::EXPONENT..index::EXPONENT_END]
            .as_ref()
            .into()
    }

    /// Get the note's orientation
    pub fn orientation(&self) -> BanknoteOrientation {
        self.buf[index::ORIENTATION].into()
    }

    /// Get the note's type
    pub fn note_type(&self) -> NoteType {
        self.buf[index::NOTE_TYPE].into()
    }

    /// Get the note's series
    pub fn note_series(&self) -> NoteSeries {
        self.buf[index::NOTE_SERIES].into()
    }

    /// Get the note's compatibility
    pub fn note_compatibility(&self) -> NoteCompatibility {
        self.buf[index::NOTE_COMPATIBILITY].into()
    }

    /// Get the note's version
    pub fn note_version(&self) -> NoteVersion {
        self.buf[index::NOTE_VERSION].into()
    }

    /// Get the note's banknote classification
    pub fn banknote_classification(&self) -> BanknoteClassification {
        self.buf[index::BANKNOTE_CLASSIFICATION].into()
    }

    /// Check if the reply is null
    pub fn is_null(&self) -> bool {
        let mut res = true;
        self.buf[index::EXTENDED..self.etx_index()]
            .iter()
            .for_each(|&b| {
                if res && b != 0 {
                    res = false
                }
            });
        res
    }
}

impl_default!(ExtendedNoteReply);
impl_message_ops!(ExtendedNoteReply);
impl_extended_ops!(ExtendedNoteReply);
impl_omnibus_extended_reply!(ExtendedNoteReply);

impl fmt::Display for ExtendedNoteReply {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AckNak: {}, DeviceType: {}, MessageType: {}, DeviceState: {}, DeviceStatus: {}, ExceptionStatus: {}, MiscDeviceState: {}, ModelNumber: {}, CodeRevision: {}, Banknote: {}",
            self.acknak(),
            self.device_type(),
            self.message_type(),
            self.device_state(),
            self.device_status(),
            self.exception_status(),
            self.misc_device_state(),
            self.model_number(),
            self.code_revision(),
            Banknote::from(self),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;

    #[test]
    #[rustfmt::skip]
    fn test_extended_note_reply_from_bytes() -> Result<()> {
        let mut msg_bytes = [
            // STX | LEN | Message type | Subtype
            0x02, 0x1e, 0x70, 0x02,
            // Data
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            // Index
            0x00,
            // ISO code
            0x00, 0x00, 0x00,
            // Base value
            0x00, 0x00, 0x00,
            // Sign
            0x00,
            // Exponent
            0x00, 0x00,
            // Orientation
            0x00,
            // Note type
            0x00,
            // Note series
            0x00,
            // Note compatibility
            0x00,
            // Note version
            0x00,
            // Banknote classification
            0x00,
            // Reserved
            0x00, 0x00,
            // ETX | Checksum
            0x03, 0x36,
        ];

        msg_bytes[index::ISO_CODE..index::ISO_CODE_END].copy_from_slice(b"JPY".as_ref());
        msg_bytes[index::BASE_VALUE..index::BASE_VALUE_END].copy_from_slice(b"002".as_ref());
        msg_bytes[index::SIGN] = b'+';
        msg_bytes[index::EXPONENT..index::EXPONENT_END].copy_from_slice(b"03".as_ref());
        msg_bytes[index::ORIENTATION] = BanknoteOrientation::RightEdgeFaceUp as u8;
        msg_bytes[index::NOTE_TYPE] = b'A';
        msg_bytes[index::NOTE_SERIES] = b'A';
        msg_bytes[index::NOTE_COMPATIBILITY] = b'B';
        msg_bytes[index::NOTE_VERSION] = b'A';
        msg_bytes[index::BANKNOTE_CLASSIFICATION] = BanknoteClassification::DisabledOrNotSupported as u8;

        let mut msg = ExtendedNoteReply::new();
        msg.from_buf(msg_bytes.as_ref())?;

        assert_eq!(msg.message_type(), MessageType::Extended);
        assert_eq!(msg.extended_command(), ExtendedCommand::ExtendedNoteSpecification);
        assert_eq!(msg.note_index(), 0);
        assert_eq!(msg.iso_code(), ISOCode::JPY);
        assert_eq!(msg.base_value(), BaseValue::from(b"002"));
        assert_eq!(msg.sign(), Sign::Positive);
        assert_eq!(msg.exponent(), Exponent::from(b"03"));
        assert_eq!(msg.orientation(), BanknoteOrientation::RightEdgeFaceUp);
        assert_eq!(msg.note_type(), NoteType::from(b'A'));
        assert_eq!(msg.note_series(), NoteSeries::from(b'A'));
        assert_eq!(msg.note_compatibility(), NoteCompatibility::from(b'B'));
        assert_eq!(msg.note_version(), NoteVersion::from(b'A'));
        assert_eq!(msg.banknote_classification(), BanknoteClassification::DisabledOrNotSupported);

        assert_eq!(Banknote::from(msg).value(), 2000.0);

        Ok(())
    }
}
