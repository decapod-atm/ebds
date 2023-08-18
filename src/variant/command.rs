use crate::{
    index, inner_enum, len, std::fmt, AuxCommandOps, Control, Error, ExtendedCommandOps,
    MessageOps, MessageType, OmnibusCommand, QueryApplicationPartNumberCommand,
    QueryBootPartNumberCommand, QueryDeviceCapabilitiesCommand, QueryExtendedNoteSpecification,
    QueryValueTableCommand, QueryVariantNameCommand, QueryVariantPartNumberCommand, Result,
    SetExtendedNoteInhibitsCFSC, SetExtendedNoteInhibitsSC, SoftReset, CFSC_ENABLE_LEN,
};

/// Represents variants of an EBDS command message.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CommandVariant {
    // Omnibus command
    OmnibusCommand(OmnibusCommand),
    // Extended commands
    QueryValueTableCommand(QueryValueTableCommand),
    QueryExtendedNoteSpecification(QueryExtendedNoteSpecification),
    SetExtendedNoteInhibitsCFSC(SetExtendedNoteInhibitsCFSC),
    SetExtendedNoteInhibitsSC(SetExtendedNoteInhibitsSC),
    // Aux commands
    QueryBootPartNumberCommand(QueryBootPartNumberCommand),
    QueryApplicationPartNumberCommand(QueryApplicationPartNumberCommand),
    QueryDeviceCapabilitiesCommand(QueryDeviceCapabilitiesCommand),
    QueryVariantPartNumberCommand(QueryVariantPartNumberCommand),
    QueryVariantNameCommand(QueryVariantNameCommand),
    SoftReset(SoftReset),
}

impl CommandVariant {
    /// Gets the [CommandVariant] as a generic [MessageOps] trait object.
    pub fn as_message(&self) -> &dyn MessageOps {
        match self {
            Self::OmnibusCommand(cmd) => cmd,
            Self::QueryExtendedNoteSpecification(cmd) => cmd,
            Self::SetExtendedNoteInhibitsCFSC(cmd) => cmd,
            Self::SetExtendedNoteInhibitsSC(cmd) => cmd,
            Self::QueryValueTableCommand(cmd) => cmd,
            Self::QueryBootPartNumberCommand(cmd) => cmd,
            Self::QueryApplicationPartNumberCommand(cmd) => cmd,
            Self::QueryDeviceCapabilitiesCommand(cmd) => cmd,
            Self::QueryVariantPartNumberCommand(cmd) => cmd,
            Self::QueryVariantNameCommand(cmd) => cmd,
            Self::SoftReset(cmd) => cmd,
        }
    }

    /// Gets the [CommandVariant] as a generic [AuxCommandOps] trait object.
    pub fn as_aux_command(&self) -> Result<&dyn AuxCommandOps> {
        match self {
            Self::QueryBootPartNumberCommand(cmd) => Ok(cmd),
            Self::QueryApplicationPartNumberCommand(cmd) => Ok(cmd),
            Self::QueryDeviceCapabilitiesCommand(cmd) => Ok(cmd),
            Self::QueryVariantPartNumberCommand(cmd) => Ok(cmd),
            Self::QueryVariantNameCommand(cmd) => Ok(cmd),
            Self::SoftReset(cmd) => Ok(cmd),
            _ => Err(Error::failure(format!(
                "invalid variant, expected AuxCommand, have: {self}"
            ))),
        }
    }

    /// Gets the [CommandVariant] as a generic [ExtendedCommandOps] trait object.
    pub fn as_extended_command(&self) -> Result<&dyn ExtendedCommandOps> {
        match self {
            Self::QueryValueTableCommand(cmd) => Ok(cmd),
            Self::QueryExtendedNoteSpecification(cmd) => Ok(cmd),
            Self::SetExtendedNoteInhibitsCFSC(cmd) => Ok(cmd),
            Self::SetExtendedNoteInhibitsSC(cmd) => Ok(cmd),
            _ => Err(Error::failure(format!(
                "invalid variant, expected ExtendedCommand, have: {self}"
            ))),
        }
    }
}

inner_enum!(CommandVariant, OmnibusCommand);
inner_enum!(CommandVariant, QueryValueTableCommand);
inner_enum!(CommandVariant, QueryExtendedNoteSpecification);
inner_enum!(CommandVariant, SetExtendedNoteInhibitsCFSC);
inner_enum!(CommandVariant, SetExtendedNoteInhibitsSC);
inner_enum!(CommandVariant, QueryBootPartNumberCommand);
inner_enum!(CommandVariant, QueryApplicationPartNumberCommand);
inner_enum!(CommandVariant, QueryVariantPartNumberCommand);
inner_enum!(CommandVariant, QueryVariantNameCommand);
inner_enum!(CommandVariant, SoftReset);

impl TryFrom<&[u8]> for CommandVariant {
    type Error = Error;

    fn try_from(val: &[u8]) -> Result<Self> {
        let (len, max) = (val.len(), len::MAX_MESSAGE);

        if len > max {
            Err(Error::failure(format!(
                "invalid command length, have: {len}, maximum: {max}"
            )))
        } else {
            let raw_msg_type = val[index::CONTROL];
            let msg_type = MessageType::from(Control::from(raw_msg_type).message_type());
            log::trace!("Command message type: {msg_type}");

            match msg_type {
                MessageType::OmnibusCommand => {
                    let mut cmd = OmnibusCommand::new();
                    cmd.from_buf(val)?;
                    Ok(Self::OmnibusCommand(cmd))
                }
                MessageType::AuxCommand => {
                    use crate::aux_command::{index as aux_index, AuxCommand};

                    let aux_len = len::AUX_COMMAND;

                    if len < aux_len {
                        Err(Error::failure(format!(
                            "invalid AuxCommand length, have: {len}, expected: {aux_len}"
                        )))
                    } else {
                        let raw_aux_type = val[aux_index::COMMAND];
                        let aux_type = AuxCommand::from(raw_aux_type);

                        match aux_type {
                            AuxCommand::QueryBootPartNumber => {
                                let mut cmd = QueryBootPartNumberCommand::new();
                                cmd.from_buf(val)?;
                                Ok(Self::QueryBootPartNumberCommand(cmd))
                            }
                            AuxCommand::QueryApplicationPartNumber => {
                                let mut cmd = QueryApplicationPartNumberCommand::new();
                                cmd.from_buf(val)?;
                                Ok(Self::QueryApplicationPartNumberCommand(cmd))
                            }
                            AuxCommand::QueryVariantPartNumber => {
                                let mut cmd = QueryVariantPartNumberCommand::new();
                                cmd.from_buf(val)?;
                                Ok(Self::QueryVariantPartNumberCommand(cmd))
                            }
                            AuxCommand::QueryVariantName => {
                                let mut cmd = QueryVariantNameCommand::new();
                                cmd.from_buf(val)?;
                                Ok(Self::QueryVariantNameCommand(cmd))
                            }
                            AuxCommand::QueryDeviceCapabilities => {
                                let mut cmd = QueryDeviceCapabilitiesCommand::new();
                                cmd.from_buf(val)?;
                                Ok(Self::QueryDeviceCapabilitiesCommand(cmd))
                            }
                            AuxCommand::SoftReset => {
                                let mut cmd = SoftReset::new();
                                cmd.from_buf(val)?;
                                Ok(Self::SoftReset(cmd))
                            }
                            _ => Err(Error::failure(format!(
                                "invalid AuxCommand message type: {aux_type}, raw: {raw_aux_type}"
                            ))),
                        }
                    }
                }
                MessageType::Extended => {
                    use crate::extended_command::{index as ext_index, ExtendedCommand};

                    let min_ext_len = 8;

                    if len < min_ext_len {
                        Err(Error::failure(format!(
                            "invalid ExtendedCommand length, have: {len}, minimum: {min_ext_len}"
                        )))
                    } else {
                        let raw_ext_type = val[ext_index::SUBTYPE];
                        let ext_type = ExtendedCommand::from(raw_ext_type);

                        match ext_type {
                            ExtendedCommand::QueryValueTable => {
                                let mut cmd = QueryValueTableCommand::new();
                                cmd.from_buf(val)?;
                                Ok(Self::QueryValueTableCommand(cmd))
                            }
                            ExtendedCommand::ExtendedNoteSpecification => {
                                let mut cmd = QueryExtendedNoteSpecification::new();
                                cmd.from_buf(val)?;
                                Ok(Self::QueryExtendedNoteSpecification(cmd))
                            }
                            ExtendedCommand::SetExtendedNoteInhibits => {
                                if len.saturating_sub(len::SET_EXTENDED_NOTE_INHIBITS_BASE) <= CFSC_ENABLE_LEN {
                                    let mut cmd = SetExtendedNoteInhibitsCFSC::new();
                                    cmd.from_buf(val)?;
                                    Ok(Self::SetExtendedNoteInhibitsCFSC(cmd))
                                } else {
                                    let mut cmd = SetExtendedNoteInhibitsSC::new();
                                    cmd.from_buf(val)?;
                                    Ok(Self::SetExtendedNoteInhibitsSC(cmd))
                                }
                            }
                            _ => Err(Error::failure(format!("invalid ExtendedCommand message type: {ext_type}, raw: {raw_ext_type}"))),
                        }
                    }
                }
                _ => Err(Error::failure(format!(
                    "invalid command message type: {msg_type}, raw: {raw_msg_type}"
                ))),
            }
        }
    }
}

impl fmt::Display for CommandVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandVariant::OmnibusCommand(cmd) => write!(f, "OmnibusCommand({cmd})"),
            CommandVariant::QueryValueTableCommand(cmd) => {
                write!(f, "QueryValueTableCommand({cmd})")
            }
            CommandVariant::QueryExtendedNoteSpecification(cmd) => {
                write!(f, "QueryExtendedNoteSpecification({cmd})")
            }
            CommandVariant::SetExtendedNoteInhibitsCFSC(cmd) => {
                write!(f, "SetExtendedNoteInhibitsCFSC({cmd})")
            }
            CommandVariant::SetExtendedNoteInhibitsSC(cmd) => {
                write!(f, "SetExtendedNoteInhibitsSC({cmd})")
            }
            CommandVariant::QueryBootPartNumberCommand(cmd) => {
                write!(f, "QueryBootPartNumberCommand({cmd})")
            }
            CommandVariant::QueryApplicationPartNumberCommand(cmd) => {
                write!(f, "QueryApplicationPartNumberCommand({cmd})")
            }
            CommandVariant::QueryVariantPartNumberCommand(cmd) => {
                write!(f, "QueryVariantPartNumberCommand({cmd})")
            }
            CommandVariant::QueryVariantNameCommand(cmd) => {
                write!(f, "QueryVariantNameCommand({cmd})")
            }
            CommandVariant::QueryDeviceCapabilitiesCommand(cmd) => {
                write!(f, "QueryDeviceCapabilitiesCommand({cmd})")
            }
            CommandVariant::SoftReset(cmd) => {
                write!(f, "SoftReset({cmd})")
            }
        }
    }
}
