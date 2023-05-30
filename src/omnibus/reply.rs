use crate::std;
use std::fmt;

use crate::{
    banknote::*, impl_default, impl_from_for_omnibus_reply, impl_message_ops,
    impl_omnibus_reply_ops, len::OMNIBUS_REPLY, status::*, AdvancedBookmarkModeReply,
    ClearAuditDataRequestAck, ClearAuditDataRequestResults, ExtendedNoteInhibitsReplyAlt,
    ExtendedNoteReply, MessageOps, MessageType, NoteRetrievedEvent, NoteRetrievedReply,
    QueryApplicationIdReply, QueryApplicationPartNumberReply, QueryBootPartNumberReply,
    QueryDeviceCapabilitiesReply, QueryValueTableReply, QueryVariantIdReply, QueryVariantNameReply,
    QueryVariantPartNumberReply, SetEscrowTimeoutReply, StandardDenomination,
};

pub mod index {
    use crate::index::DATA;

    pub const DEVICE_STATE: usize = DATA;
    pub const DEVICE_STATUS: usize = DATA + 1;
    pub const EXCEPTION_STATUS: usize = DATA + 2;
    pub const MISC_DEVICE_STATE: usize = DATA + 3;
    pub const MODEL_NUMBER: usize = DATA + 4;
    pub const CODE_REVISION: usize = DATA + 5;
}

/// Omnibus Reply - (Type 2)
///
/// [OmnibusReply] represents a message sent from the device back to the hostl
///
/// The most common reply to an [OmnibusCommand](crate::OmnibusCommand) is the standard reply.
///
/// However, if barcode vouchers, extended note, extended coupon reporting is enabled in the standard
/// omnibus command, or if the unit is an SCR, then other reply formats are possible.
///
/// These replies are detailed in sections 7.5.1, 7.5.2, 7.5.4, and 7.5.15 respectively.
///
/// There are also other circumstances that may result in the device responding back
/// to the host with a different type of message. These special cases will be described in a future section
/// when the associated feature is enabled.
///
/// The Omnibus Reply is formatted as follows:
///
/// | Name  | STX  | LEN  | CTRL | Data 0 | Data 1 | Data 2 | Data 3 | Data 4 | Data 5 | ETX  | CHK |
/// |:------|:----:|:----:|:----:|:------:|:------:|:------:|:------:|:------:|:------:|:----:|:---:|
/// | Byte  | 0    | 1    | 2    | 4      | 5      | 6      | 7      | 8      | 9      | 10   | 11  |
/// | Value | 0x02 | 0x09 | 0x1n | nn     | nn     | nn     | nn     | nn     | nn     | 0x03 | zz  |
///
/// The data bytes are bitfields representing device information:
///
/// * Data byte 0: [DeviceState]
/// * Data byte 1: [DeviceStatus]
/// * Data byte 2: [ExceptionStatus]
/// * Data byte 3: [MiscDeviceState]
/// * Data byte 4: [ModelNumber]
/// * Data byte 5: [CodeRevision]
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct OmnibusReply {
    buf: [u8; OMNIBUS_REPLY],
}

impl OmnibusReply {
    /// Create a new OmnibusReply message
    pub fn new() -> Self {
        let mut message = Self {
            buf: [0u8; OMNIBUS_REPLY],
        };

        message.init();
        message.set_message_type(MessageType::OmnibusReply);

        message
    }
}

impl_from_for_omnibus_reply!(AdvancedBookmarkModeReply);
impl_from_for_omnibus_reply!(ClearAuditDataRequestAck);
impl_from_for_omnibus_reply!(ClearAuditDataRequestResults);
impl_from_for_omnibus_reply!(ExtendedNoteReply);
impl_from_for_omnibus_reply!(ExtendedNoteInhibitsReplyAlt);
impl_from_for_omnibus_reply!(NoteRetrievedReply);
impl_from_for_omnibus_reply!(NoteRetrievedEvent);
impl_from_for_omnibus_reply!(QueryValueTableReply);
impl_from_for_omnibus_reply!(SetEscrowTimeoutReply);
impl_from_for_omnibus_reply!(QueryBootPartNumberReply);
impl_from_for_omnibus_reply!(QueryApplicationPartNumberReply);
impl_from_for_omnibus_reply!(QueryVariantNameReply);
impl_from_for_omnibus_reply!(QueryVariantPartNumberReply);
impl_from_for_omnibus_reply!(QueryDeviceCapabilitiesReply);
impl_from_for_omnibus_reply!(QueryApplicationIdReply);
impl_from_for_omnibus_reply!(QueryVariantIdReply);

impl fmt::Display for OmnibusReply {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AckNak: {}, DeviceType: {}, MessageType: {}, DeviceState: {}, DeviceStatus: {}, ExceptionStatus: {}, MiscDeviceState: {}, ModelNumber: {}, CodeRevision: {}",
            self.acknak(),
            self.device_type(),
            self.message_type(),
            self.device_state(),
            self.device_status(),
            self.exception_status(),
            self.misc_device_state(),
            self.model_number(),
            self.code_revision(),
        )
    }
}

pub trait OmnibusReplyOps: MessageOps {
    /// Get the device state data field
    fn device_state(&self) -> DeviceState {
        self.buf()[index::DEVICE_STATE].into()
    }

    /// Set the device state data field
    fn set_device_state(&mut self, device_state: DeviceState) {
        self.buf_mut()[index::DEVICE_STATE] = device_state.into();
    }

    /// Get the idling device state data field
    fn idling(&self) -> Idling {
        self.device_state().idling().into()
    }

    /// Get the idling device state data field
    fn set_idling(&mut self, idling: Idling) {
        let mut state = self.device_state();
        state.set_idling(idling.into());
        self.set_device_state(state);
    }

    /// Get the accepting device state data field
    fn accepting(&self) -> Accepting {
        self.device_state().accepting().into()
    }

    /// Get the accepting device state data field
    fn set_accepting(&mut self, accepting: Accepting) {
        let mut state = self.device_state();
        state.set_accepting(accepting.into());
        self.set_device_state(state);
    }

    /// Get the escrowed state device state data field
    fn escrowed_state(&self) -> EscrowedState {
        self.device_state().escrowed_state().into()
    }

    /// Get the escrowed state device state data field
    fn set_escrowed_state(&mut self, escrowed_state: EscrowedState) {
        let mut state = self.device_state();
        state.set_escrowed_state(escrowed_state.into());
        self.set_device_state(state);
    }

    /// Get the stacking device state data field
    fn stacking(&self) -> Stacking {
        self.device_state().stacking().into()
    }

    /// Get the stacking device state data field
    fn set_stacking(&mut self, stacking: Stacking) {
        let mut state = self.device_state();
        state.set_stacking(stacking.into());
        self.set_device_state(state);
    }

    /// Get the stacked event device state data field
    fn stacked_event(&self) -> StackedEvent {
        self.device_state().stacked_event().into()
    }

    /// Get the stacked event device state data field
    fn set_stacked_event(&mut self, stacked_event: StackedEvent) {
        let mut state = self.device_state();
        state.set_stacked_event(stacked_event.into());
        self.set_device_state(state);
    }

    /// Get the returning device state data field
    fn returning(&self) -> Returning {
        self.device_state().returning().into()
    }

    /// Get the returning device state data field
    fn set_returning(&mut self, returning: Returning) {
        let mut state = self.device_state();
        state.set_returning(returning.into());
        self.set_device_state(state);
    }

    /// Get the returned event device state data field
    fn returned_event(&self) -> ReturnedEvent {
        self.device_state().returned_event().into()
    }

    /// Get the returned event device state data field
    fn set_returned_event(&mut self, returned_event: ReturnedEvent) {
        let mut state = self.device_state();
        state.set_returned_event(returned_event.into());
        self.set_device_state(state);
    }

    /// Get the device status data field
    fn device_status(&self) -> DeviceStatus {
        self.buf()[index::DEVICE_STATUS].into()
    }

    fn set_device_status(&mut self, device_status: DeviceStatus) {
        self.buf_mut()[index::DEVICE_STATUS] = device_status.into();
    }

    /// Get the cheated device status data field
    fn cheated(&self) -> Cheated {
        self.device_status().cheated().into()
    }

    /// Set the cheated device status data field
    fn set_cheated(&mut self, cheated: Cheated) {
        let mut status = self.device_status();
        status.set_cheated(cheated.into());
        self.set_device_status(status);
    }

    /// Get the rejected device status data field
    fn rejected(&self) -> Rejected {
        self.device_status().rejected().into()
    }

    /// Set the rejected device status data field
    fn set_rejected(&mut self, rejected: Rejected) {
        let mut status = self.device_status();
        status.set_rejected(rejected.into());
        self.set_device_status(status);
    }

    /// Get the jammed device status data field
    fn jammed(&self) -> Jammed {
        self.device_status().jammed().into()
    }

    /// Set the jammed device status data field
    fn set_jammed(&mut self, jammed: Jammed) {
        let mut status = self.device_status();
        status.set_jammed(jammed.into());
        self.set_device_status(status);
    }

    /// Get the stacker full device status data field
    fn stacker_full(&self) -> StackerFull {
        self.device_status().stacker_full().into()
    }

    /// Set the stacker full device status data field
    fn set_stacker_full(&mut self, stacker_full: StackerFull) {
        let mut status = self.device_status();
        status.set_stacker_full(stacker_full.into());
        self.set_device_status(status);
    }

    /// Get the cassette attached device status data field
    fn cassette_attached(&self) -> CassetteAttached {
        self.device_status().cassette_attached().into()
    }

    /// Set the cassette attached device status data field
    fn set_cassette_attached(&mut self, cassette_attached: CassetteAttached) {
        let mut status = self.device_status();
        status.set_cassette_attached(cassette_attached.into());
        self.set_device_status(status);
    }

    /// Get the status of the cash box
    fn cash_box_status(&self) -> CashBoxStatus {
        let status = self.device_status();

        if status.stacker_full() {
            CashBoxStatus::Full
        } else if status.cassette_attached() {
            CashBoxStatus::Attached
        } else {
            CashBoxStatus::Removed
        }
    }

    /// Get the paused device status data field
    fn paused(&self) -> Paused {
        self.device_status().paused().into()
    }

    /// Set the paused device status data field
    fn set_paused(&mut self, paused: Paused) {
        let mut status = self.device_status();
        status.set_paused(paused.into());
        self.set_device_status(status);
    }

    /// Get the calibration in progress device status data field
    fn calibration(&self) -> Calibration {
        self.device_status().calibration().into()
    }

    /// Set the calibration in progress device status data field
    fn set_calibration(&mut self, calibration: Calibration) {
        let mut status = self.device_status();
        status.set_calibration(calibration.into());
        self.set_device_status(status);
    }

    /// Get the exception status data field
    fn exception_status(&self) -> ExceptionStatus {
        self.buf()[index::EXCEPTION_STATUS].into()
    }

    fn set_exception_status(&mut self, exception_status: ExceptionStatus) {
        self.buf_mut()[index::EXCEPTION_STATUS] = exception_status.into();
    }

    /// Get the power up status data field
    fn power_up(&self) -> PowerUpStatus {
        self.exception_status().power_up().into()
    }

    /// Set the power up status data field
    fn set_power_up(&mut self, power_up: PowerUpStatus) {
        let mut ex = self.exception_status();
        ex.set_power_up(power_up.into());
        self.set_exception_status(ex);
    }

    /// Get the invalid command data field
    fn invalid_command(&self) -> InvalidCommand {
        self.exception_status().invalid_command().into()
    }

    /// Set the invalid command data field
    fn set_invalid_command(&mut self, invalid_command: InvalidCommand) {
        let mut ex = self.exception_status();
        ex.set_invalid_command(invalid_command.into());
        self.set_exception_status(ex);
    }

    /// Get the failure data field
    fn failure(&self) -> Failure {
        self.exception_status().failure().into()
    }

    /// Set the failure data field
    fn set_failure(&mut self, failure: Failure) {
        let mut ex = self.exception_status();
        ex.set_failure(failure.into());
        self.set_exception_status(ex);
    }

    /// Get the note value data field
    fn note_value(&self) -> StandardDenomination {
        self.exception_status().note_value().into()
    }

    /// Set the note value data field
    fn set_note_value(&mut self, note_value: StandardDenomination) {
        let mut ex = self.exception_status();
        ex.set_note_value(note_value.into());
        self.set_exception_status(ex);
    }

    /// Get the transport open data field
    fn transport_open(&self) -> TransportOpen {
        self.exception_status().transport_open().into()
    }

    /// Set the transport open data field
    fn set_transport_open(&mut self, transport_open: TransportOpen) {
        let mut ex = self.exception_status();
        ex.set_transport_open(transport_open.into());
        self.set_exception_status(ex);
    }

    /// Get the miscellaneous device status data field
    fn misc_device_state(&self) -> MiscDeviceState {
        self.buf()[index::MISC_DEVICE_STATE].into()
    }

    fn set_misc_device_state(&mut self, misc_device_state: MiscDeviceState) {
        self.buf_mut()[index::MISC_DEVICE_STATE] = misc_device_state.into();
    }

    /// Get the stalled data field
    fn stalled(&self) -> Stalled {
        self.misc_device_state().stalled().into()
    }

    /// Set the stalled data field
    fn set_stalled(&mut self, stalled: Stalled) {
        let mut misc = self.misc_device_state();
        misc.set_stalled(stalled.into());
        self.set_misc_device_state(misc);
    }

    /// Get the flash download data field
    fn flash_download(&self) -> FlashDownload {
        self.misc_device_state().flash_download().into()
    }

    /// Set the flash download data field
    fn set_flash_download(&mut self, flash_download: FlashDownload) {
        let mut misc = self.misc_device_state();
        misc.set_flash_download(flash_download.into());
        self.set_misc_device_state(misc);
    }

    /// Get the pre-stack data field
    fn pre_stack(&self) -> PreStack {
        self.misc_device_state().pre_stack().into()
    }

    /// Set the pre-stack data field
    fn set_pre_stack(&mut self, pre_stack: PreStack) {
        let mut misc = self.misc_device_state();
        misc.set_pre_stack(pre_stack.into());
        self.set_misc_device_state(misc);
    }

    /// Get the raw barcode data field
    fn raw_barcode(&self) -> RawBarcode {
        self.misc_device_state().raw_barcode().into()
    }

    /// Set the raw barcode data field
    fn set_raw_barcode(&mut self, raw_barcode: RawBarcode) {
        let mut misc = self.misc_device_state();
        misc.set_raw_barcode(raw_barcode.into());
        self.set_misc_device_state(misc);
    }

    /// Get the device capabilities data field
    fn device_capabilities(&self) -> DeviceCapabilities {
        self.misc_device_state().device_capabilities().into()
    }

    /// Set the device capabilities data field
    fn set_device_capabilities(&mut self, device_capabilities: DeviceCapabilities) {
        let mut misc = self.misc_device_state();
        misc.set_device_capabilities(device_capabilities.into());
        self.set_misc_device_state(misc);
    }

    /// Get the disabled data field
    fn disabled(&self) -> Disabled {
        self.misc_device_state().disabled().into()
    }

    /// Set the disabled data field
    fn set_disabled(&mut self, disabled: Disabled) {
        let mut misc = self.misc_device_state();
        misc.set_disabled(disabled.into());
        self.set_misc_device_state(misc);
    }

    /// Get the model number data field
    fn model_number(&self) -> ModelNumber {
        self.buf()[index::MODEL_NUMBER].into()
    }

    /// Set the model number data field
    fn set_model_number(&mut self, model_number: ModelNumber) {
        self.buf_mut()[index::MODEL_NUMBER] = model_number.into();
    }

    /// Get the code revision data field
    fn code_revision(&self) -> CodeRevision {
        self.buf()[index::CODE_REVISION].into()
    }

    /// Set the code revision data field
    fn set_code_revision(&mut self, code_revision: CodeRevision) {
        self.buf_mut()[index::CODE_REVISION] = code_revision.into()
    }
}

impl_default!(OmnibusReply);
impl_message_ops!(OmnibusReply);
impl_omnibus_reply_ops!(OmnibusReply);

impl From<&OmnibusReply> for Banknote {
    fn from(reply: &OmnibusReply) -> Self {
        use crate::bau_currency;

        let note_value = bau_currency().denomination_value_base(reply.note_value());
        Self::default().with_value(note_value.into())
    }
}

impl From<&dyn OmnibusReplyOps> for DocumentStatus {
    fn from(reply: &dyn OmnibusReplyOps) -> Self {
        Self::default().with_standard_denomination(reply.note_value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;

    #[test]
    #[rustfmt::skip]
    fn test_omnibus_reply_from_buf() -> Result<()> {
        let msg_bytes = [
            // STX | LEN | Message Type
            0x02, 0x0b, 0x20,
            // Data
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            // ETX | Checksum
            0x03, 0x2b,
        ];

        let mut msg = OmnibusReply::new();
        msg.from_buf(msg_bytes.as_ref())?;

        assert_eq!(msg.message_type(), MessageType::OmnibusReply);
        assert_eq!(msg.device_state(), DeviceState::from(0));
        assert_eq!(msg.device_status(), DeviceStatus::from(0));
        assert_eq!(msg.exception_status(), ExceptionStatus::from(0));
        assert_eq!(msg.misc_device_state(), MiscDeviceState::from(0));
        assert_eq!(msg.model_number(), ModelNumber::from(0));
        assert_eq!(msg.code_revision(), CodeRevision::from(0));

        Ok(())
    }
}
