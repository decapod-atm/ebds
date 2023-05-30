use crate::std;
use std::fmt;

use crate::MessageOps;

/// Auxilliary Commmands (Type 6): The Auxiliary Commands are used to provide functionality outside the scope of the Omnibus command
/// in the previous sections. These commands can be specific to a certain code base, so be sure to check the
/// compatibility icons before each section.

/// Developers: add additional types from the specification as needed
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AuxCommand {
    QuerySoftwareCrc = 0x00,
    QueryBootPartNumber = 0x06,
    QueryApplicationPartNumber = 0x07,
    QueryVariantName = 0x08,
    QueryVariantPartNumber = 0x09,
    QueryDeviceCapabilities = 0x0d,
    QueryApplicationId = 0x0e,
    QueryVariantId = 0x0f,
    SoftReset = 0x7f,
    Reserved = 0xff,
}

impl From<u8> for AuxCommand {
    fn from(b: u8) -> Self {
        match b {
            0x00 => Self::QuerySoftwareCrc,
            0x06 => Self::QueryBootPartNumber,
            0x07 => Self::QueryApplicationPartNumber,
            0x08 => Self::QueryVariantName,
            0x09 => Self::QueryVariantPartNumber,
            0x0d => Self::QueryDeviceCapabilities,
            0x0e => Self::QueryApplicationId,
            0x0f => Self::QueryVariantId,
            0x7f => Self::SoftReset,
            _ => Self::Reserved,
        }
    }
}

impl From<AuxCommand> for &'static str {
    fn from(a: AuxCommand) -> Self {
        match a {
            AuxCommand::QuerySoftwareCrc => "QuerySoftwareCrc",
            AuxCommand::QueryBootPartNumber => "QueryBootPartNumber",
            AuxCommand::QueryApplicationPartNumber => "QueryApplicationPartNumber",
            AuxCommand::QueryVariantName => "QueryVariantName",
            AuxCommand::QueryVariantPartNumber => "QueryVariantPartNumber",
            AuxCommand::QueryDeviceCapabilities => "QueryDeviceCapabilities",
            AuxCommand::QueryApplicationId => "QueryApplicationId",
            AuxCommand::QueryVariantId => "QueryVariantId",
            AuxCommand::SoftReset => "SoftReset",
            AuxCommand::Reserved => "Reserved",
        }
    }
}

impl From<&AuxCommand> for &'static str {
    fn from(a: &AuxCommand) -> Self {
        (*a).into()
    }
}

impl From<AuxCommand> for u8 {
    fn from(a: AuxCommand) -> Self {
        a as u8
    }
}

impl From<&AuxCommand> for u8 {
    fn from(a: &AuxCommand) -> Self {
        (*a).into()
    }
}

impl fmt::Display for AuxCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", <&'static str>::from(self))
    }
}

pub mod index {
    pub const COMMAND: usize = 5;
}

pub trait AuxCommandOps: MessageOps {
    /// Gets the auxilliary command sub-type.
    fn aux_command(&self) -> AuxCommand {
        assert!(self.buf().len() > index::COMMAND);

        self.buf()[index::COMMAND].into()
    }

    /// Sets the auxilliary command sub-type.
    fn set_aux_command(&mut self, aux_command: AuxCommand) {
        self.buf_mut()[index::COMMAND] = aux_command.into();
    }
}
