use crate::std;

use crate::{
    error::{Error, Result},
    impl_default, impl_extended_ops, impl_message_ops, impl_omnibus_extended_command,
    len::SET_EXTENDED_NOTE_INHIBITS_BASE,
    ExtendedCommand, ExtendedCommandOps, ExtendedNoteReporting, MessageOps, MessageType,
    OmnibusCommandOps,
};

/// CFSC device extended note enable byte length, see section 7.5.3
pub const CFSC_ENABLE_LEN: usize = 8;
/// SC device extended note enable byte length, see section 7.5.3
pub const SC_ENABLE_LEN: usize = 19;

mod bitmask {
    pub const ENABLE_NOTE: u8 = 0b111_1111;
}

mod index {
    pub const ENABLE_NOTE: usize = 7;
}

bitfield! {
    /// Represents enabled notes in the extended note table.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct EnableNote(u8);
    u8;
    pub note1, set_note1: 0;
    pub note2, set_note2: 1;
    pub note3, set_note3: 2;
    pub note4, set_note4: 3;
    pub note5, set_note5: 4;
    pub note6, set_note6: 5;
    pub note7, set_note7: 6;
}

impl EnableNote {
    pub const LEN: usize = 7;

    /// Creates an [EnableNote] with no bits set.
    pub const fn none() -> Self {
        Self(0)
    }

    /// Creates an [EnableNote] with all bits set.
    pub const fn all() -> Self {
        Self(bitmask::ENABLE_NOTE)
    }

    /// Get the length of the [EnableNote] bitfield.
    pub const fn len() -> usize {
        Self::LEN
    }

    /// Sets an index to enable.
    ///
    /// Valid range is [1, 7] (inclusive).
    pub fn set_index(&mut self, index: usize) -> Result<()> {
        match index {
            1 => self.set_note1(true),
            2 => self.set_note2(true),
            3 => self.set_note3(true),
            4 => self.set_note4(true),
            5 => self.set_note5(true),
            6 => self.set_note6(true),
            7 => self.set_note7(true),
            _ => return Err(Error::failure("invalid enable index")),
        }
        Ok(())
    }
}

impl From<&[bool]> for EnableNote {
    fn from(b: &[bool]) -> Self {
        let mut inner = 0u8;
        // only allow a max of
        let end = std::cmp::min(b.len(), Self::len());
        for (i, &set) in b[..end].iter().enumerate() {
            let bit = if set { 1 } else { 0 };
            inner |= bit << i;
        }
        Self(inner)
    }
}

impl<const N: usize> From<&[bool; N]> for EnableNote {
    fn from(b: &[bool; N]) -> Self {
        b.as_ref().into()
    }
}

impl<const N: usize> From<[bool; N]> for EnableNote {
    fn from(b: [bool; N]) -> Self {
        (&b).into()
    }
}

impl From<u8> for EnableNote {
    fn from(b: u8) -> Self {
        Self(b & bitmask::ENABLE_NOTE)
    }
}

impl From<&EnableNote> for u8 {
    fn from(e: &EnableNote) -> u8 {
        e.0
    }
}

impl From<EnableNote> for u8 {
    fn from(e: EnableNote) -> u8 {
        (&e).into()
    }
}

/// Set Extended Note Inhibits - Request (Subtype 0x03)
///
/// This command is used to control the acceptance of bank notes on a note type basis. It is only used when
/// the device is running in extended note mode (section 4.2.2).
///
/// The generic parameter is the sum of
/// [SET_EXTENDED_NOTE_INHIBITS_BASE](crate::len::SET_EXTENDED_NOTE_INHIBITS_BASE),
/// and the number of enable note bytes (either [CFSC_ENABLE_LEN] or [SC_ENABLE_LEN]).
///
/// The Set Extended Note Inhibits is formatted as follows:
///
/// | Name  | STX  | LEN  | CTRL | Subtype | Data 0 | Data 1 | Data 2 | Enable 1 | ...    | Enable N | ETX    | CHK    |
/// |:------|:----:|:----:|:----:|:-------:|:------:|:------:|:------:|:--------:|:------:|:--------:|:------:|:------:|
/// | Byte  | 0    | 1    | 2    | 3       | 4      | 5      | 6      | 7        | ...    | LL - 3   | LL - 2 | LL - 1 |
/// | Value | 0x02 | LL   | 0x7n | 0x03    | nn     | nn     | nn     | nn       | nn     | nn       | 0x03   | zz     |
///
/// | **CFSC** |
/// |:--------:|
///
/// Supports up to 50 denomination types, therefore the command requires 8 extended data bytes.
///
/// This will make the message length 0x11:
///
/// | Byte     | Bit 6   | Bit 5   | Bit 4   | Bit 3   | Bit 2   | Bit 1   | Bit 0   |
/// |:---------|:-------:|:-------:|:-------:|:-------:|:-------:|:-------:|:-------:|
/// | Enable 1 | Note 7  | Note 6  | Note 5  | Note 4  | Note 3  | Note 2  | Note 1  |
/// | Enable 2 | Note 14 | Note 13 | Note 12 | Note 11 | Note 10 | Note 9  | Note 8  |
/// | Enable 3 | Note 21 | Note 20 | Note 19 | Note 18 | Note 17 | Note 16 | Note 15 |
/// | Enable 4 | Note 28 | Note 27 | Note 26 | Note 25 | Note 24 | Note 23 | Note 22 |
/// | Enable 5 | Note 35 | Note 34 | Note 33 | Note 32 | Note 31 | Note 30 | Note 29 |
/// | Enable 6 | Note 42 | Note 41 | Note 40 | Note 39 | Note 38 | Note 37 | Note 36 |
/// | Enable 7 | Note 49 | Note 48 | Note 47 | Note 46 | Note 45 | Note 44 | Note 43 |
/// | Enable 8 | -       | -       | -       | -       | -       | -       | Note 50 |
///
/// | **SC Adv** | **SCR** |
/// |:----------:|:-------:|
///
/// Supports up to 128 denomination types, therefore the command requires 19 extended
/// data bytes.
///
/// This will make the message length 0x1C:
///
/// | Byte      | Bit 6    | Bit 5    | Bit 4    | Bit 3    | Bit 2    | Bit 1    | Bit 0    |
/// |:----------|:--------:|:--------:|:--------:|:--------:|:--------:|:--------:|:--------:|
/// | Enable 1  | Note 7   | Note 6   | Note 5   | Note 4   | Note 3   | Note 2   | Note 1   |
/// | Enable 2  | Note 14  | Note 13  | Note 12  | Note 11  | Note 10  | Note 9   | Note 8   |
/// | Enable 3  | Note 21  | Note 20  | Note 19  | Note 18  | Note 17  | Note 16  | Note 15  |
/// | Enable 4  | Note 28  | Note 27  | Note 26  | Note 25  | Note 24  | Note 23  | Note 22  |
/// | Enable 5  | Note 35  | Note 34  | Note 33  | Note 32  | Note 31  | Note 30  | Note 29  |
/// | Enable 6  | Note 42  | Note 41  | Note 40  | Note 39  | Note 38  | Note 37  | Note 36  |
/// | Enable 7  | Note 49  | Note 48  | Note 47  | Note 46  | Note 45  | Note 44  | Note 43  |
/// | Enable 8  | Note 56  | Note 55  | Note 54  | Note 53  | Note 52  | Note 51  | Note 50  |
/// | Enable 9  | Note 63  | Note 62  | Note 61  | Note 60  | Note 59  | Note 58  | Note 57  |
/// | Enable 10 | Note 70  | Note 69  | Note 68  | Note 67  | Note 66  | Note 65  | Note 64  |
/// | Enable 11 | Note 77  | Note 76  | Note 75  | Note 74  | Note 73  | Note 72  | Note 71  |
/// | Enable 12 | Note 84  | Note 83  | Note 82  | Note 81  | Note 80  | Note 79  | Note 78  |
/// | Enable 13 | Note 91  | Note 90  | Note 89  | Note 88  | Note 87  | Note 86  | Note 85  |
/// | Enable 14 | Note 98  | Note 97  | Note 98  | Note 97  | Note 96  | Note 95  | Note 94  |
/// | Enable 15 | Note 105 | Note 104 | Note 103 | Note 102 | Note 101 | Note 100 | Note 99  |
/// | Enable 16 | Note 112 | Note 111 | Note 110 | Note 109 | Note 108 | Note 107 | Note 106 |
/// | Enable 17 | Note 119 | Note 118 | Note 117 | Note 116 | Note 115 | Note 114 | Note 113 |
/// | Enable 18 | Note 126 | Note 125 | Note 124 | Note 123 | Note 122 | Note 121 | Note 120 |
/// | Enable 19 | -        | -        | -        | -        | -        | Note 128 | Note 127 |
///
/// If the bit equals 1 then the note is enabled.
pub struct SetExtendedNoteInhibits<const M: usize, const N: usize> {
    buf: [u8; M],
}

impl<const M: usize, const N: usize> SetExtendedNoteInhibits<M, N> {
    /// The length of enable note bytes (N - [SET_EXTENDED_NOTE_INHIBITS_BASE]).
    pub const ENABLE_NOTE_LEN: usize = N;

    /// Creates a new [SetExtendedNoteInhibits] message.
    pub fn new() -> Self {
        assert!(
            M == SET_EXTENDED_NOTE_INHIBITS_BASE + CFSC_ENABLE_LEN
                || M == SET_EXTENDED_NOTE_INHIBITS_BASE + SC_ENABLE_LEN
        );

        let mut message = Self { buf: [0u8; M] };

        message.init();
        message.set_message_type(MessageType::Extended);
        message.set_extended_note(ExtendedNoteReporting::Set);
        message.set_extended_command(ExtendedCommand::SetExtendedNoteInhibits);

        message
    }

    /// Get the table of enabled note bytes.
    pub fn enabled_notes(&self) -> [EnableNote; N] {
        let mut ret = [EnableNote::none(); N];

        for (&note, set_note) in self.buf
            [index::ENABLE_NOTE..index::ENABLE_NOTE + Self::ENABLE_NOTE_LEN]
            .iter()
            .zip(ret.iter_mut())
        {
            *set_note = EnableNote::from(note);
        }

        ret
    }

    /// Sets the enable note bytes
    ///
    /// Example: `notes[0]` sets `Enable 1`, `notes[1]` sets `Enable 2` etc.
    ///
    /// Note: maximum of [ENABLE_NOTE_LEN](Self::ENABLE_NOTE_LEN) [EnableNote]s can be set, any extra supplied are ignored.
    pub fn set_enabled_notes(&mut self, notes: &[EnableNote]) {
        let max_len = std::cmp::min(notes.len(), Self::ENABLE_NOTE_LEN);

        for (i, note) in notes[..max_len].iter().enumerate() {
            self.buf[index::ENABLE_NOTE + i] = note.into();
        }
    }
}

pub const CFSC_ENABLE_FULL_LEN: usize = SET_EXTENDED_NOTE_INHIBITS_BASE + CFSC_ENABLE_LEN;
pub const SC_ENABLE_FULL_LEN: usize = SET_EXTENDED_NOTE_INHIBITS_BASE + SC_ENABLE_LEN;

pub type SetExtendedNoteInhibitsCFSC =
    SetExtendedNoteInhibits<CFSC_ENABLE_FULL_LEN, CFSC_ENABLE_LEN>;
pub type SetExtendedNoteInhibitsSC = SetExtendedNoteInhibits<SC_ENABLE_FULL_LEN, SC_ENABLE_LEN>;

impl_default!(SetExtendedNoteInhibits, M, N);
impl_message_ops!(SetExtendedNoteInhibits, M, N);
impl_extended_ops!(SetExtendedNoteInhibits, M, N);
impl_omnibus_extended_command!(SetExtendedNoteInhibits, M, N);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;

    #[test]
    #[rustfmt::skip]
    fn test_query_set_extended_note_inhibits_from_bytes() -> Result<()> {

        // CFSC note table
        let msg_bytes = [
            // STX | LEN | Message type | Subtype
            0x02, 0x11, 0x70, 0x03,
            // Data
            0x00, 0x00, 0x00,
            // Enable
            0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            // ETX | Checksum
            0x03, 0x63,
        ];

        let mut msg = SetExtendedNoteInhibitsCFSC::new();
        msg.from_buf(msg_bytes.as_ref())?;

        assert_eq!(msg.message_type(), MessageType::Extended);
        assert_eq!(msg.extended_command(), ExtendedCommand::SetExtendedNoteInhibits);

        let exp_enabled = [
            EnableNote::from(1), EnableNote::none(), EnableNote::none(), EnableNote::none(),
            EnableNote::none(), EnableNote::none(), EnableNote::none(),  EnableNote::none(),
        ];

        assert_eq!(msg.enabled_notes(), exp_enabled);

        // SC note table
        let msg_bytes = [
            // STX | LEN | Message type | Subtype
            0x02, 0x1c, 0x70, 0x03,
            // Data
            0x00, 0x00, 0x00,
            // Enable
            0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00,
            // ETX | Checksum
            0x03, 0x6e,
        ];

        let mut msg = SetExtendedNoteInhibitsSC::new();
        msg.from_buf(msg_bytes.as_ref())?;

        assert_eq!(msg.message_type(), MessageType::Extended);
        assert_eq!(msg.extended_command(), ExtendedCommand::SetExtendedNoteInhibits);

        let exp_enabled = [
            EnableNote::from(1), EnableNote::none(), EnableNote::none(), EnableNote::none(),
            EnableNote::none(), EnableNote::none(), EnableNote::none(),  EnableNote::none(),
            EnableNote::none(), EnableNote::none(), EnableNote::none(),  EnableNote::none(),
            EnableNote::none(), EnableNote::none(), EnableNote::none(),  EnableNote::none(),
            EnableNote::none(), EnableNote::none(), EnableNote::none(),
        ];

        assert_eq!(msg.enabled_notes(), exp_enabled);

        Ok(())
    }
}
