use crate::std;
use std::fmt;

use crate::MessageOps;

/// Extended Commands (Type 7): The extended commands utilize message type 7 to provide functionality outside of the standard
/// omnibus commands. The use of message type 7 is complicated by the fact that it can be used by either
/// the host or device at anytime.
///
/// Developers: add additional types from the specification as needed
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ExtendedCommand {
    ExtendedBarcodeReply = 0x1,
    ExtendedNoteSpecification = 0x2,
    SetExtendedNoteInhibits = 0x3,
    SetEscrowTimeout = 0x4,
    QueryValueTable = 0x6,
    NoteRetrieved = 0xb,
    AdvancedBookmark = 0xd,
    ClearAuditDataRequest = 0x1d,
    Reserved = 0xff,
}

impl From<u8> for ExtendedCommand {
    fn from(b: u8) -> Self {
        match b {
            0x1 => ExtendedCommand::ExtendedBarcodeReply,
            0x2 => ExtendedCommand::ExtendedNoteSpecification,
            0x3 => ExtendedCommand::SetExtendedNoteInhibits,
            0x4 => ExtendedCommand::SetEscrowTimeout,
            0x6 => ExtendedCommand::QueryValueTable,
            0xb => ExtendedCommand::NoteRetrieved,
            0xd => ExtendedCommand::AdvancedBookmark,
            0x1d => ExtendedCommand::ClearAuditDataRequest,
            // Missing values are either specified and unneeded, or unspecified and RFU
            _ => ExtendedCommand::Reserved,
        }
    }
}

impl From<ExtendedCommand> for u8 {
    fn from(e: ExtendedCommand) -> Self {
        e as u8
    }
}

impl From<&ExtendedCommand> for u8 {
    fn from(e: &ExtendedCommand) -> Self {
        (*e).into()
    }
}

impl From<ExtendedCommand> for &'static str {
    fn from(e: ExtendedCommand) -> Self {
        match e {
            ExtendedCommand::ExtendedBarcodeReply => "ExtendedBarcodeReply",
            ExtendedCommand::ExtendedNoteSpecification => "ExtendedNoteSpecification",
            ExtendedCommand::SetExtendedNoteInhibits => "SetExtendedNoteInhibits",
            ExtendedCommand::SetEscrowTimeout => "SetEscrowTimeout / ExtendedCoupon",
            ExtendedCommand::QueryValueTable => "QueryValueTable",
            ExtendedCommand::NoteRetrieved => "NoteRetrieved",
            ExtendedCommand::AdvancedBookmark => "AdvancedBookmark",
            ExtendedCommand::ClearAuditDataRequest => "ClearAuditDataRequest",
            ExtendedCommand::Reserved => "Reserved",
        }
    }
}

impl From<&ExtendedCommand> for &'static str {
    fn from(e: &ExtendedCommand) -> Self {
        (*e).into()
    }
}

impl fmt::Display for ExtendedCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", <&'static str>::from(self))
    }
}

pub mod index {
    pub const SUBTYPE: usize = 3;
}

pub trait ExtendedCommandOps: MessageOps {
    /// Get the extended command sub-type
    fn extended_command(&self) -> ExtendedCommand {
        self.buf()[index::SUBTYPE].into()
    }

    /// Set the extended command sub-type
    fn set_extended_command(&mut self, ext_cmd: ExtendedCommand) {
        self.buf_mut()[index::SUBTYPE] = ext_cmd.into();
    }
}
