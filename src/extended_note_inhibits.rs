mod command;
mod reply;

pub use command::{
    index as extended_note_inhibits_command_index, EnableNote, SetExtendedNoteInhibits,
    SetExtendedNoteInhibitsCFSC, SetExtendedNoteInhibitsSC, CFSC_ENABLE_LEN, SC_ENABLE_LEN,
};
pub use reply::{
    index as extended_note_inhibits_reply_index, ExtendedNoteInhibitsReply,
    ExtendedNoteInhibitsReplyAlt,
};
