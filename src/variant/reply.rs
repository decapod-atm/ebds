use crate::{
    index, inner_enum, len, std::fmt, AdvancedBookmarkModeReply, AuxCommand, Banknote,
    BaudRateChangeReply, ClearAuditDataRequestAck, ClearAuditDataRequestResults, Control,
    DocumentStatus, Error, ExtendedCommand, ExtendedNoteInhibitsReplyAlt, ExtendedNoteReply,
    FlashDownloadReply7bit, FlashDownloadReply8bit, MessageOps, MessageType, NoteRetrievedEvent,
    NoteRetrievedReply, OmnibusReply, OmnibusReplyOps, QueryApplicationIdReply,
    QueryApplicationPartNumberReply, QueryBootPartNumberReply, QueryDeviceCapabilitiesReply,
    QuerySoftwareCrcReply, QueryValueTableReply, QueryVariantIdReply, QueryVariantNameReply,
    QueryVariantPartNumberReply, Result, SetEscrowTimeoutReply, StartDownloadReply,
};

/// Message reply variants for message building.
#[derive(Debug, PartialEq)]
pub enum ReplyVariant {
    // Omnibus reply
    OmnibusReply(OmnibusReply),
    // Extended replies
    AdvancedBookmarkModeReply(AdvancedBookmarkModeReply),
    ClearAuditDataRequestAck(ClearAuditDataRequestAck),
    ClearAuditDataRequestResults(ClearAuditDataRequestResults),
    ExtendedNoteReply(ExtendedNoteReply),
    ExtendedNoteInhibitsReplyAlt(ExtendedNoteInhibitsReplyAlt),
    NoteRetrievedReply(NoteRetrievedReply),
    NoteRetrievedEvent(NoteRetrievedEvent),
    QueryValueTableReply(QueryValueTableReply),
    SetEscrowTimeoutReply(SetEscrowTimeoutReply),
    // Aux replies
    QuerySoftwareCrcReply(QuerySoftwareCrcReply),
    QueryBootPartNumberReply(QueryBootPartNumberReply),
    QueryApplicationPartNumberReply(QueryApplicationPartNumberReply),
    QueryVariantNameReply(QueryVariantNameReply),
    QueryVariantPartNumberReply(QueryVariantPartNumberReply),
    QueryDeviceCapabilitiesReply(QueryDeviceCapabilitiesReply),
    QueryApplicationIdReply(QueryApplicationIdReply),
    QueryVariantIdReply(QueryVariantIdReply),
    // Flash download replies
    BaudRateChangeReply(BaudRateChangeReply),
    FlashDownloadReply7bit(FlashDownloadReply7bit),
    FlashDownloadReply8bit(FlashDownloadReply8bit),
    StartDownloadReply(StartDownloadReply),
}

inner_enum!(ReplyVariant, AdvancedBookmarkModeReply);
inner_enum!(ReplyVariant, ClearAuditDataRequestAck);
inner_enum!(ReplyVariant, ClearAuditDataRequestResults);
inner_enum!(ReplyVariant, ExtendedNoteReply);
inner_enum!(ReplyVariant, ExtendedNoteInhibitsReplyAlt);
inner_enum!(ReplyVariant, NoteRetrievedReply);
inner_enum!(ReplyVariant, NoteRetrievedEvent);
inner_enum!(ReplyVariant, QueryValueTableReply);
inner_enum!(ReplyVariant, SetEscrowTimeoutReply);
inner_enum!(ReplyVariant, QuerySoftwareCrcReply);
inner_enum!(ReplyVariant, QueryBootPartNumberReply);
inner_enum!(ReplyVariant, QueryApplicationPartNumberReply);
inner_enum!(ReplyVariant, QueryVariantNameReply);
inner_enum!(ReplyVariant, QueryVariantPartNumberReply);
inner_enum!(ReplyVariant, QueryApplicationIdReply);
inner_enum!(ReplyVariant, QueryVariantIdReply);
inner_enum!(ReplyVariant, QueryDeviceCapabilitiesReply);
inner_enum!(ReplyVariant, BaudRateChangeReply);
inner_enum!(ReplyVariant, FlashDownloadReply7bit);
inner_enum!(ReplyVariant, FlashDownloadReply8bit);
inner_enum!(ReplyVariant, StartDownloadReply);

impl ReplyVariant {
    /// Validates the [ReplyVariant] checksum.
    pub fn validate_checksum(&self) -> Result<()> {
        self.as_message().validate_checksum()
    }

    /// Gets the [ReplyVariant] as a generic [MessageOps] implementation.
    pub fn as_message(&self) -> &dyn MessageOps {
        match self {
            Self::AdvancedBookmarkModeReply(msg) => msg,
            Self::ClearAuditDataRequestAck(msg) => msg,
            Self::ClearAuditDataRequestResults(msg) => msg,
            Self::ExtendedNoteReply(msg) => msg,
            Self::ExtendedNoteInhibitsReplyAlt(msg) => msg,
            Self::NoteRetrievedReply(msg) => msg,
            Self::NoteRetrievedEvent(msg) => msg,
            Self::OmnibusReply(msg) => msg,
            Self::QueryValueTableReply(msg) => msg,
            Self::SetEscrowTimeoutReply(msg) => msg,
            Self::QuerySoftwareCrcReply(msg) => msg,
            Self::QueryBootPartNumberReply(msg) => msg,
            Self::QueryApplicationPartNumberReply(msg) => msg,
            Self::QueryVariantNameReply(msg) => msg,
            Self::QueryVariantPartNumberReply(msg) => msg,
            Self::QueryDeviceCapabilitiesReply(msg) => msg,
            Self::QueryApplicationIdReply(msg) => msg,
            Self::QueryVariantIdReply(msg) => msg,
            Self::BaudRateChangeReply(msg) => msg,
            Self::FlashDownloadReply7bit(msg) => msg,
            Self::FlashDownloadReply8bit(msg) => msg,
            Self::StartDownloadReply(msg) => msg,
        }
    }

    /// Gets the [ReplyVariant] as a mutable generic [MessageOps] implementation.
    pub fn as_message_mut(&mut self) -> &mut dyn MessageOps {
        match self {
            Self::AdvancedBookmarkModeReply(msg) => msg,
            Self::ClearAuditDataRequestAck(msg) => msg,
            Self::ClearAuditDataRequestResults(msg) => msg,
            Self::ExtendedNoteReply(msg) => msg,
            Self::ExtendedNoteInhibitsReplyAlt(msg) => msg,
            Self::NoteRetrievedReply(msg) => msg,
            Self::NoteRetrievedEvent(msg) => msg,
            Self::OmnibusReply(msg) => msg,
            Self::QueryValueTableReply(msg) => msg,
            Self::SetEscrowTimeoutReply(msg) => msg,
            Self::QuerySoftwareCrcReply(msg) => msg,
            Self::QueryBootPartNumberReply(msg) => msg,
            Self::QueryApplicationPartNumberReply(msg) => msg,
            Self::QueryVariantNameReply(msg) => msg,
            Self::QueryVariantPartNumberReply(msg) => msg,
            Self::QueryDeviceCapabilitiesReply(msg) => msg,
            Self::QueryApplicationIdReply(msg) => msg,
            Self::QueryVariantIdReply(msg) => msg,
            Self::BaudRateChangeReply(msg) => msg,
            Self::FlashDownloadReply7bit(msg) => msg,
            Self::FlashDownloadReply8bit(msg) => msg,
            Self::StartDownloadReply(msg) => msg,
        }
    }

    /// Gets the [ReplyVariant] as a generic [OmnibusReplyOps] implementation.
    pub fn as_omnibus_reply(&self) -> &dyn OmnibusReplyOps {
        match self {
            Self::AdvancedBookmarkModeReply(msg) => msg,
            Self::ClearAuditDataRequestAck(msg) => msg,
            Self::ClearAuditDataRequestResults(msg) => msg,
            Self::ExtendedNoteReply(msg) => msg,
            Self::ExtendedNoteInhibitsReplyAlt(msg) => msg,
            Self::NoteRetrievedReply(msg) => msg,
            Self::NoteRetrievedEvent(msg) => msg,
            Self::OmnibusReply(msg) => msg,
            Self::QueryValueTableReply(msg) => msg,
            Self::SetEscrowTimeoutReply(msg) => msg,
            Self::QuerySoftwareCrcReply(msg) => msg,
            Self::QueryBootPartNumberReply(msg) => msg,
            Self::QueryApplicationPartNumberReply(msg) => msg,
            Self::QueryVariantNameReply(msg) => msg,
            Self::QueryVariantPartNumberReply(msg) => msg,
            Self::QueryDeviceCapabilitiesReply(msg) => msg,
            Self::QueryApplicationIdReply(msg) => msg,
            Self::QueryVariantIdReply(msg) => msg,
            Self::BaudRateChangeReply(msg) => msg,
            Self::FlashDownloadReply7bit(msg) => msg,
            Self::FlashDownloadReply8bit(msg) => msg,
            Self::StartDownloadReply(msg) => msg,
        }
    }

    /// Gets the [ReplyVariant] as a mutable generic [OmnibusReplyOps] implementation.
    pub fn as_omnibus_reply_mut(&mut self) -> &mut dyn OmnibusReplyOps {
        match self {
            Self::AdvancedBookmarkModeReply(msg) => msg,
            Self::ClearAuditDataRequestAck(msg) => msg,
            Self::ClearAuditDataRequestResults(msg) => msg,
            Self::ExtendedNoteReply(msg) => msg,
            Self::ExtendedNoteInhibitsReplyAlt(msg) => msg,
            Self::NoteRetrievedReply(msg) => msg,
            Self::NoteRetrievedEvent(msg) => msg,
            Self::OmnibusReply(msg) => msg,
            Self::QueryValueTableReply(msg) => msg,
            Self::SetEscrowTimeoutReply(msg) => msg,
            Self::QuerySoftwareCrcReply(msg) => msg,
            Self::QueryBootPartNumberReply(msg) => msg,
            Self::QueryApplicationPartNumberReply(msg) => msg,
            Self::QueryVariantNameReply(msg) => msg,
            Self::QueryVariantPartNumberReply(msg) => msg,
            Self::QueryDeviceCapabilitiesReply(msg) => msg,
            Self::QueryApplicationIdReply(msg) => msg,
            Self::QueryVariantIdReply(msg) => msg,
            Self::BaudRateChangeReply(msg) => msg,
            Self::FlashDownloadReply7bit(msg) => msg,
            Self::FlashDownloadReply8bit(msg) => msg,
            Self::StartDownloadReply(msg) => msg,
        }
    }

    pub fn into_omnibus_reply(self) -> OmnibusReply {
        match self {
            Self::AdvancedBookmarkModeReply(msg) => msg.into(),
            Self::ClearAuditDataRequestAck(msg) => msg.into(),
            Self::ClearAuditDataRequestResults(msg) => msg.into(),
            Self::ExtendedNoteReply(msg) => msg.into(),
            Self::ExtendedNoteInhibitsReplyAlt(msg) => msg.into(),
            Self::NoteRetrievedReply(msg) => msg.into(),
            Self::NoteRetrievedEvent(msg) => msg.into(),
            Self::OmnibusReply(msg) => msg,
            Self::QueryValueTableReply(msg) => msg.into(),
            Self::SetEscrowTimeoutReply(msg) => msg.into(),
            Self::QueryBootPartNumberReply(msg) => msg.into(),
            Self::QueryApplicationPartNumberReply(msg) => msg.into(),
            Self::QueryVariantNameReply(msg) => msg.into(),
            Self::QueryVariantPartNumberReply(msg) => msg.into(),
            Self::QueryDeviceCapabilitiesReply(msg) => msg.into(),
            Self::QueryApplicationIdReply(msg) => msg.into(),
            Self::QueryVariantIdReply(msg) => msg.into(),
            _ => OmnibusReply::new(),
        }
    }

    /// Converts a [ReplyVariant] into a [Banknote].
    pub fn into_banknote(&self) -> Result<Banknote> {
        match self {
            Self::ExtendedNoteReply(msg) => Ok(msg.into()),
            Self::OmnibusReply(msg) => Ok(msg.into()),
            _ => Err(Error::failure(format!("ReplyVariant->Banknote conversion only implemented for ExtendedNoteReply, have: {self}"))), 
        }
    }

    /// Converts the [ReplyVariant] into a [DocumentStatus].
    pub fn document_status(&self) -> DocumentStatus {
        if let Ok(extended_note) = self.as_extended_note_reply() {
            extended_note.into()
        } else {
            self.as_omnibus_reply().into()
        }
    }

    /// Parses an Auxilliary command response from the provided buffer and command type.
    ///
    /// The command is provided to provide a better heuristic of what the response should be.
    /// This is necessary because Aux commands do not include a subtype byte in the response like
    /// Extended commands. Also, mutltiple response types share the same length, so the command
    /// type from the sent message is the best guess for what the response should be.
    pub fn from_aux_buf(buf: &[u8], command: AuxCommand) -> Result<Self> {
        let msg_len = buf.len();

        if !(len::MIN_MESSAGE..=len::MAX_MESSAGE).contains(&msg_len) {
            return Err(Error::failure("invalid message length"));
        }

        let msg_type: MessageType = Control::from(buf[index::CONTROL]).message_type().into();
        let exp_msg_type = MessageType::AuxCommand;

        if msg_type != exp_msg_type {
            return Err(Error::failure(
                "invalid message type: {msg_type}, expected: {exp_msg_type}",
            ));
        }

        match command {
            AuxCommand::QuerySoftwareCrc => {
                let mut msg = QuerySoftwareCrcReply::new();
                msg.from_buf(buf)?;
                Ok(Self::QuerySoftwareCrcReply(msg))
            }
            AuxCommand::QueryBootPartNumber => {
                let mut msg = QueryBootPartNumberReply::new();
                msg.from_buf(buf)?;
                Ok(Self::QueryBootPartNumberReply(msg))
            }
            AuxCommand::QueryApplicationPartNumber => {
                let mut msg = QueryApplicationPartNumberReply::new();
                msg.from_buf(buf)?;
                Ok(Self::QueryApplicationPartNumberReply(msg))
            }
            AuxCommand::QueryVariantName => {
                let mut msg = QueryVariantNameReply::new();
                msg.from_buf(buf)?;
                Ok(Self::QueryVariantNameReply(msg))
            }
            AuxCommand::QueryVariantPartNumber => {
                let mut msg = QueryVariantPartNumberReply::new();
                msg.from_buf(buf)?;
                Ok(Self::QueryVariantPartNumberReply(msg))
            }
            AuxCommand::QueryDeviceCapabilities => {
                let mut msg = QueryDeviceCapabilitiesReply::new();
                msg.from_buf(buf)?;
                Ok(Self::QueryDeviceCapabilitiesReply(msg))
            }
            AuxCommand::QueryApplicationId => {
                let mut msg = QueryApplicationIdReply::new();
                msg.from_buf(buf)?;
                Ok(Self::QueryApplicationIdReply(msg))
            }
            AuxCommand::QueryVariantId => {
                let mut msg = QueryVariantIdReply::new();
                msg.from_buf(buf)?;
                Ok(Self::QueryVariantIdReply(msg))
            }
            _ => Err(Error::failure("invalid AuxCommand reply type")),
        }
    }

    /// Contructs a [ReplyVariant] from a buffer
    pub fn from_buf(buf: &[u8]) -> Result<Self> {
        let msg_len = buf.len();

        if !(len::MIN_MESSAGE..=len::MAX_MESSAGE).contains(&msg_len) {
            return Err(Error::failure("invalid message length"));
        }

        let control = Control::from(buf[index::CONTROL]);
        let msg_type = MessageType::from(control.message_type());

        match msg_type {
            MessageType::OmnibusReply => {
                let mut msg = OmnibusReply::new();
                msg.from_buf(buf)?;
                Ok(Self::OmnibusReply(msg))
            }
            MessageType::FirmwareDownload => match msg_len {
                len::BAUD_CHANGE_REPLY => {
                    let mut msg = BaudRateChangeReply::new();
                    msg.from_buf(buf)?;
                    Ok(Self::BaudRateChangeReply(msg))
                }
                len::START_DOWNLOAD_REPLY => {
                    let mut msg = StartDownloadReply::new();
                    msg.from_buf(buf)?;
                    Ok(Self::StartDownloadReply(msg))
                }
                len::FLASH_DOWNLOAD_REPLY_7BIT => {
                    let mut msg = FlashDownloadReply7bit::new();
                    msg.from_buf(buf)?;
                    Ok(Self::FlashDownloadReply7bit(msg))
                }
                len::FLASH_DOWNLOAD_REPLY_8BIT => {
                    let mut msg = FlashDownloadReply8bit::new();
                    msg.from_buf(buf)?;
                    Ok(Self::FlashDownloadReply8bit(msg))
                }
                _ => Err(Error::failure(format!(
                    "unsupported FirmwareDownload reply message length: {msg_len}"
                ))),
            },
            MessageType::Extended => {
                let raw_sub_type = buf[index::EXT_SUBTYPE];
                let sub_type = ExtendedCommand::from(raw_sub_type);
                match sub_type {
                    ExtendedCommand::ClearAuditDataRequest => {
                        let cad_reply_diff = buf[10];
                        if cad_reply_diff == 0x00 || cad_reply_diff == 0x01 {
                            // Acknowledgement will have a 0x00 or 0x01 value in the 10th index
                            let mut msg = ClearAuditDataRequestAck::new();
                            msg.from_buf(buf)?;
                            Ok(Self::ClearAuditDataRequestAck(msg))
                        } else if cad_reply_diff == 0x10 || cad_reply_diff == 0x11 {
                            // Results will have a 0x10 or 0x11 value in the 10th index
                            let mut msg = ClearAuditDataRequestResults::new();
                            msg.from_buf(buf)?;
                            Ok(Self::ClearAuditDataRequestResults(msg))
                        } else {
                            Err(Error::failure("invalid ClearAuditDataRequest reply type"))
                        }
                    }
                    ExtendedCommand::ExtendedNoteSpecification => {
                        let mut msg = ExtendedNoteReply::new();
                        msg.from_buf(buf)?;
                        Ok(Self::ExtendedNoteReply(msg))
                    }
                    ExtendedCommand::SetExtendedNoteInhibits => {
                        let mut msg = ExtendedNoteInhibitsReplyAlt::new();
                        msg.from_buf(buf)?;
                        Ok(Self::ExtendedNoteInhibitsReplyAlt(msg))
                    }
                    ExtendedCommand::QueryValueTable => {
                        let mut msg = QueryValueTableReply::new();
                        msg.from_buf(buf)?;
                        Ok(Self::QueryValueTableReply(msg))
                    }
                    ExtendedCommand::NoteRetrieved => {
                        use crate::note_retrieved::reply::index as nr_index;

                        if buf.len() < len::NOTE_RETRIEVED_REPLY {
                            Err(Error::failure(
                                "invalid message length for a NoteRetrieved reply",
                            ))
                        } else {
                            let acknak_event = buf[nr_index::ACKNAK];
                            match acknak_event {
                                0x00 | 0x01 => {
                                    let mut msg = NoteRetrievedReply::new();
                                    msg.from_buf(buf)?;
                                    Ok(Self::NoteRetrievedReply(msg))
                                }
                                0x7f => {
                                    let mut msg = NoteRetrievedEvent::new();
                                    msg.from_buf(buf)?;
                                    Ok(Self::NoteRetrievedEvent(msg))
                                }
                                _ => Err(Error::failure(
                                    "invalid AckNak/Event value: 0x{acknak_event:x}",
                                )),
                            }
                        }
                    }
                    ExtendedCommand::AdvancedBookmark => {
                        let mut msg = AdvancedBookmarkModeReply::new();
                        msg.from_buf(buf)?;
                        Ok(Self::AdvancedBookmarkModeReply(msg))
                    }
                    _ => Err(Error::failure(format!(
                        "unsupported extended message type: {sub_type}, raw: 0x{raw_sub_type:x}"
                    ))),
                }
            }
            // AuxCommands currently unsupported due to no reliable way to determine reply types
            // without access to the command type.
            _ => Err(Error::failure(format!(
                "expected Omnibus or Extended reply types, received: {msg_type}"
            ))),
        }
    }
}

impl fmt::Display for ReplyVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AdvancedBookmarkModeReply(msg) => write!(f, "AdvancedBookmarkModeReply({msg})"),
            Self::ClearAuditDataRequestAck(msg) => write!(f, "ClearAuditDataRequestAck({msg})"),
            Self::ClearAuditDataRequestResults(msg) => {
                write!(f, "ClearAuditDataRequestResults({msg})")
            }
            Self::ExtendedNoteReply(msg) => write!(f, "ExtendedNoteReply({msg})"),
            Self::ExtendedNoteInhibitsReplyAlt(msg) => {
                write!(f, "ExtendedNoteInhibitsReplyAlt({msg})")
            }
            Self::NoteRetrievedReply(msg) => write!(f, "NoteRetrievedReply({msg})"),
            Self::NoteRetrievedEvent(msg) => write!(f, "NoteRetrievedEvent({msg})"),
            Self::OmnibusReply(msg) => write!(f, "OmnibusReply({msg})"),
            Self::QueryValueTableReply(msg) => write!(f, "QueryValueTableReply({msg})"),
            Self::SetEscrowTimeoutReply(msg) => write!(f, "SetEscrowTimeoutReply({msg})"),
            Self::QuerySoftwareCrcReply(msg) => write!(f, "QuerySoftwareCrcReply({msg})"),
            Self::QueryBootPartNumberReply(msg) => write!(f, "QueryBootPartNumberReply({msg})"),
            Self::QueryApplicationPartNumberReply(msg) => {
                write!(f, "QueryApplicationPartNumberReply({msg})")
            }
            Self::QueryVariantNameReply(msg) => write!(f, "QueryVariantNameReply({msg})"),
            Self::QueryVariantPartNumberReply(msg) => {
                write!(f, "QueryVariantPartNumberReply({msg})")
            }
            Self::QueryDeviceCapabilitiesReply(msg) => {
                write!(f, "QueryDeviceCapabilitiesReply({msg})")
            }
            Self::QueryApplicationIdReply(msg) => write!(f, "QueryApplicationIdReply({msg})"),
            Self::QueryVariantIdReply(msg) => write!(f, "QueryVariantIdReply({msg})"),
            Self::BaudRateChangeReply(msg) => write!(f, "BaudRateChangeReply({msg})"),
            Self::FlashDownloadReply7bit(msg) => write!(f, "FlashDownloadReply7bit({msg})"),
            Self::FlashDownloadReply8bit(msg) => write!(f, "FlashDownloadReply8bit({msg})"),
            Self::StartDownloadReply(msg) => write!(f, "StartDownloadReply({msg})"),
        }
    }
}
