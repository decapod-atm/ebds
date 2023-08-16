use crate::std::fmt;

use super::*;

/// Message reply variants for message building.
#[derive(Debug, PartialEq)]
pub enum MessageVariant {
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

impl MessageVariant {
    /// Validates the [MessageVariant] checksum.
    pub fn validate_checksum(&self) -> Result<()> {
        self.as_message().validate_checksum()
    }

    /// Gets the [MessageVariant] as a generic [MessageOps] implementation.
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

    /// Gets the [MessageVariant] as a mutable generic [MessageOps] implementation.
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

    /// Gets the [MessageVariant] as a generic [OmnibusReplyOps] implementation.
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

    /// Gets the [MessageVariant] as a mutable generic [OmnibusReplyOps] implementation.
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

    /// Gets the [MessageVariant] as an [AdvancedBookmarkModeReply].
    pub fn as_advanced_bookmark_mode_reply(&self) -> Result<&AdvancedBookmarkModeReply> {
        match self {
            Self::AdvancedBookmarkModeReply(msg) => Ok(msg),
            _ => Err(Error::failure(format!(
                "Expected AdvancedBookmarkModeReply, MessageVariant is {self}"
            ))),
        }
    }

    /// Consumes and converts the [MessageVariant] into an [AdvancedBookmarkModeReply].
    pub fn into_advanced_bookmark_mode_reply(self) -> Result<AdvancedBookmarkModeReply> {
        match self {
            Self::AdvancedBookmarkModeReply(msg) => Ok(msg),
            _ => Err(Error::failure(format!(
                "Expected AdvancedBookmarkModeReply, MessageVariant is {self}"
            ))),
        }
    }

    /// Gets whether the [MessageVariant] is an [AdvancedBookmarkModeReply].
    pub fn is_advanced_bookmark_mode_reply(&self) -> bool {
        matches!(self, Self::AdvancedBookmarkModeReply(_))
    }

    /// Gets the [MessageVariant] as an [QuerySoftwareCrcReply].
    pub fn as_query_software_crc_reply(&self) -> Result<&QuerySoftwareCrcReply> {
        match self {
            Self::QuerySoftwareCrcReply(msg) => Ok(msg),
            _ => Err(Error::failure(format!(
                "Expected QuerySoftwareCrcReply, MessageVariant is {self}"
            ))),
        }
    }

    /// Consumes and converts the [MessageVariant] into an [QuerySoftwareCrcReply].
    pub fn into_query_software_crc_reply(self) -> Result<QuerySoftwareCrcReply> {
        match self {
            Self::QuerySoftwareCrcReply(msg) => Ok(msg),
            _ => Err(Error::failure(format!(
                "Expected QuerySoftwareCrcReply, MessageVariant is {self}"
            ))),
        }
    }

    /// Gets whether the [MessageVariant] is an [QuerySoftwareCrcReply].
    pub fn is_query_software_crc_reply(&self) -> bool {
        matches!(self, Self::QuerySoftwareCrcReply(_))
    }

    /// Gets the [MessageVariant] as an [QueryBootPartNumberReply].
    pub fn as_query_boot_part_number_reply(&self) -> Result<&QueryBootPartNumberReply> {
        match self {
            Self::QueryBootPartNumberReply(msg) => Ok(msg),
            _ => Err(Error::failure(format!(
                "Expected QueryBootPartNumberReply, MessageVariant is {self}"
            ))),
        }
    }

    /// Consumes and converts the [MessageVariant] into an [QueryBootPartNumberReply].
    pub fn into_query_boot_part_number_reply(self) -> Result<QueryBootPartNumberReply> {
        match self {
            Self::QueryBootPartNumberReply(msg) => Ok(msg),
            _ => Err(Error::failure(format!(
                "Expected QueryBootPartNumberReply, MessageVariant is {self}"
            ))),
        }
    }

    /// Gets whether the [MessageVariant] is an [QueryBootPartNumberReply].
    pub fn is_query_boot_part_number_reply(&self) -> bool {
        matches!(self, Self::QueryBootPartNumberReply(_))
    }

    /// Gets the [MessageVariant] as an [QueryApplicationPartNumberReply].
    pub fn as_query_application_part_number_reply(
        &self,
    ) -> Result<&QueryApplicationPartNumberReply> {
        match self {
            Self::QueryApplicationPartNumberReply(msg) => Ok(msg),
            _ => Err(Error::failure(format!(
                "Expected QueryApplicationPartNumberReply, MessageVariant is {self}"
            ))),
        }
    }

    /// Consumes and converts the [MessageVariant] into an [QueryApplicationPartNumberReply].
    pub fn into_query_application_part_number_reply(
        self,
    ) -> Result<QueryApplicationPartNumberReply> {
        match self {
            Self::QueryApplicationPartNumberReply(msg) => Ok(msg),
            _ => Err(Error::failure(format!(
                "Expected QueryApplicationPartNumberReply, MessageVariant is {self}"
            ))),
        }
    }

    /// Gets whether the [MessageVariant] is an [QueryApplicationPartNumberReply].
    pub fn is_query_application_part_number_reply(&self) -> bool {
        matches!(self, Self::QueryApplicationPartNumberReply(_))
    }

    /// Gets the [MessageVariant] as an [QueryVariantNameReply].
    pub fn as_query_variant_name_reply(&self) -> Result<&QueryVariantNameReply> {
        match self {
            Self::QueryVariantNameReply(msg) => Ok(msg),
            _ => Err(Error::failure(format!(
                "Expected QueryVariantNameReply, MessageVariant is {self}"
            ))),
        }
    }

    /// Consumes and converts the [MessageVariant] into an [QueryVariantNameReply].
    pub fn into_query_variant_name_reply(self) -> Result<QueryVariantNameReply> {
        match self {
            Self::QueryVariantNameReply(msg) => Ok(msg),
            _ => Err(Error::failure(format!(
                "Expected QueryVariantNameReply, MessageVariant is {self}"
            ))),
        }
    }

    /// Gets whether the [MessageVariant] is an [QueryVariantNameReply].
    pub fn is_query_variant_name_reply(&self) -> bool {
        matches!(self, Self::QueryVariantNameReply(_))
    }

    /// Gets the [MessageVariant] as an [QueryVariantPartNumberReply].
    pub fn as_query_variant_part_number_reply(&self) -> Result<&QueryVariantPartNumberReply> {
        match self {
            Self::QueryVariantPartNumberReply(msg) => Ok(msg),
            _ => Err(Error::failure(format!(
                "Expected QueryVariantPartNumberReply, MessageVariant is {self}"
            ))),
        }
    }

    /// Consumes and converts the [MessageVariant] into an [QueryVariantPartNumberReply].
    pub fn into_query_variant_part_number_reply(self) -> Result<QueryVariantPartNumberReply> {
        match self {
            Self::QueryVariantPartNumberReply(msg) => Ok(msg),
            _ => Err(Error::failure(format!(
                "Expected QueryVariantPartNumberReply, MessageVariant is {self}"
            ))),
        }
    }

    /// Gets whether the [MessageVariant] is an [QueryVariantPartNumberReply].
    pub fn is_query_variant_part_number_reply(&self) -> bool {
        matches!(self, Self::QueryVariantPartNumberReply(_))
    }

    /// Gets the [MessageVariant] as an [QueryDeviceCapabilitiesReply].
    pub fn as_query_device_capabilities_reply(&self) -> Result<&QueryDeviceCapabilitiesReply> {
        match self {
            Self::QueryDeviceCapabilitiesReply(msg) => Ok(msg),
            _ => Err(Error::failure(format!(
                "Expected QueryDeviceCapabilitiesReply, MessageVariant is {self}"
            ))),
        }
    }

    /// Consumes and converts the [MessageVariant] into an [QueryDeviceCapabilitiesReply].
    pub fn into_query_device_capabilities_reply(self) -> Result<QueryDeviceCapabilitiesReply> {
        match self {
            Self::QueryDeviceCapabilitiesReply(msg) => Ok(msg),
            _ => Err(Error::failure(format!(
                "Expected QueryDeviceCapabilitiesReply, MessageVariant is {self}"
            ))),
        }
    }

    /// Gets whether the [MessageVariant] is an [QueryDeviceCapabilitiesReply].
    pub fn is_query_device_capabilities_reply(&self) -> bool {
        matches!(self, Self::QueryDeviceCapabilitiesReply(_))
    }

    /// Gets the [MessageVariant] as an [QueryApplicationIdReply].
    pub fn as_query_application_id_reply(&self) -> Result<&QueryApplicationIdReply> {
        match self {
            Self::QueryApplicationIdReply(msg) => Ok(msg),
            _ => Err(Error::failure(format!(
                "Expected QueryApplicationIdReply, MessageVariant is {self}"
            ))),
        }
    }

    /// Consumes and converts the [MessageVariant] into an [QueryApplicationIdReply].
    pub fn into_query_application_id_reply(self) -> Result<QueryApplicationIdReply> {
        match self {
            Self::QueryApplicationIdReply(msg) => Ok(msg),
            _ => Err(Error::failure(format!(
                "Expected QueryApplicationIdReply, MessageVariant is {self}"
            ))),
        }
    }

    /// Gets whether the [MessageVariant] is an [QueryApplicationIdReply].
    pub fn is_query_application_id_reply(&self) -> bool {
        matches!(self, Self::QueryApplicationIdReply(_))
    }

    /// Gets the [MessageVariant] as an [QueryVariantIdReply].
    pub fn as_query_variant_id_reply(&self) -> Result<&QueryVariantIdReply> {
        match self {
            Self::QueryVariantIdReply(msg) => Ok(msg),
            _ => Err(Error::failure(format!(
                "Expected QueryVariantIdReply, MessageVariant is {self}"
            ))),
        }
    }

    /// Consumes and converts the [MessageVariant] into an [QueryVariantIdReply].
    pub fn into_query_variant_id_reply(self) -> Result<QueryVariantIdReply> {
        match self {
            Self::QueryVariantIdReply(msg) => Ok(msg),
            _ => Err(Error::failure(format!(
                "Expected QueryVariantIdReply, MessageVariant is {self}"
            ))),
        }
    }

    /// Gets whether the [MessageVariant] is an [QueryVariantIdReply].
    pub fn is_query_variant_id_reply(&self) -> bool {
        matches!(self, Self::QueryVariantIdReply(_))
    }

    /// Gets the [MessageVariant] as an [NoteRetrievedReply].
    pub fn as_note_retrieved_reply(&self) -> Result<&NoteRetrievedReply> {
        match self {
            Self::NoteRetrievedReply(msg) => Ok(msg),
            _ => Err(Error::failure(format!(
                "Expected NoteRetrievedReply, MessageVariant is {self}"
            ))),
        }
    }

    /// Consumes and converts the [MessageVariant] into an [NoteRetrievedReply].
    pub fn into_note_retrieved_reply(self) -> Result<NoteRetrievedReply> {
        match self {
            Self::NoteRetrievedReply(msg) => Ok(msg),
            _ => Err(Error::failure(format!(
                "Expected NoteRetrievedReply, MessageVariant is {self}"
            ))),
        }
    }

    /// Gets whether the [MessageVariant] is an [NoteRetrievedReply].
    pub fn is_note_retrieved_reply(&self) -> bool {
        matches!(self, Self::NoteRetrievedReply(_))
    }

    /// Gets the [MessageVariant] as an [NoteRetrievedEvent].
    pub fn as_note_retrieved_event(&self) -> Result<&NoteRetrievedEvent> {
        match self {
            Self::NoteRetrievedEvent(msg) => Ok(msg),
            _ => Err(Error::failure(format!(
                "Expected NoteRetrievedEvent, MessageVariant is {self}"
            ))),
        }
    }

    /// Consumes and converts the [MessageVariant] into an [NoteRetrievedEvent].
    pub fn into_note_retrieved_event(self) -> Result<NoteRetrievedEvent> {
        match self {
            Self::NoteRetrievedEvent(msg) => Ok(msg),
            _ => Err(Error::failure(format!(
                "Expected NoteRetrievedEvent, MessageVariant is {self}"
            ))),
        }
    }

    /// Gets whether the [MessageVariant] is an [NoteRetrievedEvent].
    pub fn is_note_retrieved_event(&self) -> bool {
        matches!(self, Self::NoteRetrievedEvent(_))
    }

    /// Gets the [MessageVariant] as an [ExtendedNoteReply].
    pub fn as_extended_note_reply(&self) -> Result<&ExtendedNoteReply> {
        match self {
            Self::ExtendedNoteReply(msg) => Ok(msg),
            _ => Err(Error::failure(format!(
                "Expected ExtendedNoteReply, MessageVariant is {self}"
            ))),
        }
    }

    /// Consumes and converts the [MessageVariant] into an [ExtendedNoteReply].
    pub fn into_extended_note_reply(self) -> Result<ExtendedNoteReply> {
        match self {
            Self::ExtendedNoteReply(msg) => Ok(msg),
            _ => Err(Error::failure(format!(
                "Expected ExtendedNoteReply, MessageVariant is {self}"
            ))),
        }
    }

    /// Gets whether the [MessageVariant] is an [ExtendedNoteReply].
    pub fn is_extended_note_reply(&self) -> bool {
        matches!(self, Self::ExtendedNoteReply(_))
    }

    /// Gets the [MessageVariant] as a [ClearAuditDataRequestAck] message.
    pub fn as_clear_audit_data_request_ack(&self) -> Result<&ClearAuditDataRequestAck> {
        match self {
            Self::ClearAuditDataRequestAck(msg) => Ok(msg),
            _ => Err(Error::failure(format!(
                "Expected ClearAuditDataRequestAck, MessageVariant is {self}"
            ))),
        }
    }

    /// Gets the [MessageVariant] as a [ClearAuditDataRequestResults] message.
    pub fn as_clear_audit_data_request_results(&self) -> Result<&ClearAuditDataRequestResults> {
        match self {
            Self::ClearAuditDataRequestResults(msg) => Ok(msg),
            _ => Err(Error::failure(format!(
                "Expected ClearAuditDataRequestResults, MessageVariant is {self}"
            ))),
        }
    }

    /// Consumes and converts the [MessageVariant] into a [ClearAuditDataRequestResults] message.
    pub fn into_clear_audit_data_request_results(self) -> Result<ClearAuditDataRequestResults> {
        match self {
            Self::ClearAuditDataRequestResults(msg) => Ok(msg),
            _ => Err(Error::failure(format!(
                "Expected ClearAuditDataRequestResults, MessageVariant is {self}"
            ))),
        }
    }

    /// Gets the [MessageVariant] as an [QueryValueTableReply].
    pub fn as_query_value_table_reply(&self) -> Result<&QueryValueTableReply> {
        match self {
            Self::QueryValueTableReply(msg) => Ok(msg),
            _ => Err(Error::failure(format!(
                "Expected QueryValueTableReply, MessageVariant is {self}"
            ))),
        }
    }

    /// Consumes and converts the [MessageVariant] into an [QueryValueTableReply].
    pub fn into_query_value_table_reply(self) -> Result<QueryValueTableReply> {
        match self {
            Self::QueryValueTableReply(msg) => Ok(msg),
            _ => Err(Error::failure(format!(
                "Expected QueryValueTableReply, MessageVariant is {self}"
            ))),
        }
    }

    /// Gets the [MessageVariant] as an [SetEscrowTimeoutReply].
    pub fn as_set_escrow_timeout_reply(&self) -> Result<&SetEscrowTimeoutReply> {
        match self {
            Self::SetEscrowTimeoutReply(msg) => Ok(msg),
            _ => Err(Error::failure(format!(
                "Expected SetEscrowTimeoutReply, MessageVariant is {self}"
            ))),
        }
    }

    /// Consumes and converts the [MessageVariant] into an [SetEscrowTimeoutReply].
    pub fn into_set_escrow_timeout_reply(self) -> Result<SetEscrowTimeoutReply> {
        match self {
            Self::SetEscrowTimeoutReply(msg) => Ok(msg),
            _ => Err(Error::failure(format!(
                "Expected SetEscrowTimeoutReply, MessageVariant is {self}"
            ))),
        }
    }

    /// Gets the [MessageVariant] as an [ExtendedNoteInhibitsReplyAlt].
    pub fn as_extended_note_inhibits_reply(&self) -> Result<&ExtendedNoteInhibitsReplyAlt> {
        match self {
            Self::ExtendedNoteInhibitsReplyAlt(msg) => Ok(msg),
            _ => Err(Error::failure(format!(
                "Expected ExtendedNoteInhibitsReplyAlt, MessageVariant is {self}"
            ))),
        }
    }

    /// Consumes and converts the [MessageVariant] into an [ExtendedNoteInhibitsReplyAlt].
    pub fn into_extended_note_inhibits_reply(self) -> Result<ExtendedNoteInhibitsReplyAlt> {
        match self {
            Self::ExtendedNoteInhibitsReplyAlt(msg) => Ok(msg),
            _ => Err(Error::failure(format!(
                "Expected ExtendedNoteInhibitsReplyAlt, MessageVariant is {self}"
            ))),
        }
    }

    /// Gets whether the [MessageVariant] is an [ExtendedNoteInhibitsReplyAlt].
    pub fn is_extended_note_inhibits_reply(&self) -> bool {
        matches!(self, Self::ExtendedNoteInhibitsReplyAlt(_))
    }

    /// Gets the [MessageVariant] as an [BaudRateChangeReply].
    pub fn as_baud_rate_change_reply(&self) -> Result<&BaudRateChangeReply> {
        match self {
            Self::BaudRateChangeReply(msg) => Ok(msg),
            _ => Err(Error::failure(format!(
                "Expected BaudRateChangeReply, MessageVariant is {self}"
            ))),
        }
    }

    /// Consumes and converts the [MessageVariant] into an [BaudRateChangeReply].
    pub fn into_baud_rate_change_reply(self) -> Result<BaudRateChangeReply> {
        match self {
            Self::BaudRateChangeReply(msg) => Ok(msg),
            _ => Err(Error::failure(format!(
                "Expected BaudRateChangeReply, MessageVariant is {self}"
            ))),
        }
    }

    /// Gets whether the [MessageVariant] is an [BaudRateChangeReply].
    pub fn is_baud_rate_change_reply(&self) -> bool {
        matches!(self, Self::BaudRateChangeReply(_))
    }

    /// Gets the [MessageVariant] as an [StartDownloadReply].
    pub fn as_start_download_reply(&self) -> Result<&StartDownloadReply> {
        match self {
            Self::StartDownloadReply(msg) => Ok(msg),
            _ => Err(Error::failure(format!(
                "Expected StartDownloadReply, MessageVariant is {self}"
            ))),
        }
    }

    /// Consumes and converts the [MessageVariant] into an [StartDownloadReply].
    pub fn into_start_download_reply(self) -> Result<StartDownloadReply> {
        match self {
            Self::StartDownloadReply(msg) => Ok(msg),
            _ => Err(Error::failure(format!(
                "Expected StartDownloadReply, MessageVariant is {self}"
            ))),
        }
    }

    /// Gets whether the [MessageVariant] is an [StartDownloadReply].
    pub fn is_start_download_reply(&self) -> bool {
        matches!(self, Self::StartDownloadReply(_))
    }

    /// Gets the [MessageVariant] as an [FlashDownloadReply].
    pub fn as_flash_download_reply(&self) -> Result<&dyn FlashDownloadReply> {
        match self {
            Self::FlashDownloadReply7bit(msg) => Ok(msg),
            Self::FlashDownloadReply8bit(msg) => Ok(msg),
            _ => Err(Error::failure(format!(
                "Expected FlashDownloadReply, MessageVariant is {self}"
            ))),
        }
    }

    /// Gets the [MessageVariant] as an [FlashDownloadReply7bit].
    pub fn as_flash_download_reply_7bit(&self) -> Result<&FlashDownloadReply7bit> {
        match self {
            Self::FlashDownloadReply7bit(msg) => Ok(msg),
            _ => Err(Error::failure(format!(
                "Expected FlashDownloadReply, MessageVariant is {self}"
            ))),
        }
    }

    /// Gets the [MessageVariant] as an [FlashDownloadReply7bit].
    pub fn as_flash_download_reply_8bit(&self) -> Result<&FlashDownloadReply8bit> {
        match self {
            Self::FlashDownloadReply8bit(msg) => Ok(msg),
            _ => Err(Error::failure(format!(
                "Expected FlashDownloadReply, MessageVariant is {self}"
            ))),
        }
    }

    /// Consumes and converts the [MessageVariant] into an [FlashDownloadReply7bit].
    pub fn into_flash_download_reply_7bit(self) -> Result<FlashDownloadReply7bit> {
        match self {
            Self::FlashDownloadReply7bit(msg) => Ok(msg),
            _ => Err(Error::failure(format!(
                "Expected FlashDownloadReply, MessageVariant is {self}"
            ))),
        }
    }

    /// Consumes and converts the [MessageVariant] into an [FlashDownloadReply8bit].
    pub fn into_flash_download_reply_8bit(self) -> Result<FlashDownloadReply8bit> {
        match self {
            Self::FlashDownloadReply8bit(msg) => Ok(msg),
            _ => Err(Error::failure(format!(
                "Expected FlashDownloadReply, MessageVariant is {self}"
            ))),
        }
    }

    /// Gets whether the [MessageVariant] is an [FlashDownloadReply].
    pub fn is_flash_download_reply(&self) -> bool {
        matches!(
            self,
            Self::FlashDownloadReply7bit(_) | Self::FlashDownloadReply8bit(_)
        )
    }

    /// Gets whether the [MessageVariant] is an [FlashDownloadReply7bit].
    pub fn is_flash_download_reply_7bit(&self) -> bool {
        matches!(self, Self::FlashDownloadReply7bit(_))
    }

    /// Gets whether the [MessageVariant] is an [FlashDownloadReply8bit].
    pub fn is_flash_download_reply_8bit(&self) -> bool {
        matches!(self, Self::FlashDownloadReply8bit(_))
    }

    /// Converts a [MessageVariant] into a [Banknote](hal_common::banknote::Banknote).
    pub fn into_banknote(&self) -> Result<Banknote> {
        match self {
            Self::ExtendedNoteReply(msg) => Ok(msg.into()),
            Self::OmnibusReply(msg) => Ok(msg.into()),
            _ => Err(Error::failure(format!("MessageVariant->Banknote conversion only implemented for ExtendedNoteReply, have: {self}"))), 
        }
    }

    /// Converts the [MessageVariant] into a [DocumentStatus].
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

    /// Contructs a [MessageVariant] from a buffer
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

impl fmt::Display for MessageVariant {
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
