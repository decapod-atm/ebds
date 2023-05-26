use crate::{
    CodeRevision, DeviceState, DeviceStatus, ExceptionStatus, ExtendedCommand, MessageOps,
    MiscDeviceState, ModelNumber,
};

mod index {
    pub const SUBTYPE: usize = 3;
    pub const DEVICE_STATE: usize = SUBTYPE + 1;
    pub const DEVICE_STATUS: usize = SUBTYPE + 2;
    pub const EXCEPTION_STATUS: usize = SUBTYPE + 3;
    pub const MISC_DEVICE_STATE: usize = SUBTYPE + 4;
    pub const MODEL_NUMBER: usize = SUBTYPE + 5;
    pub const CODE_REVISION: usize = SUBTYPE + 6;
}

pub trait ExtendedReplyOps: MessageOps {
    /// Get the extended command sub-type
    fn extended_command(&self) -> ExtendedCommand {
        self.buf()[index::SUBTYPE].into()
    }

    /// Set the extended command sub-type
    fn set_extended_command(&mut self, ext_cmd: ExtendedCommand) {
        self.buf_mut()[index::SUBTYPE] = ext_cmd.into();
    }

    /// Get the device state data field
    fn device_state(&self) -> DeviceState {
        self.buf()[index::DEVICE_STATE].into()
    }

    /// Set the device state data field
    fn set_device_state(&mut self, device_state: DeviceState) {
        self.buf_mut()[index::DEVICE_STATE] = device_state.into();
    }

    /// Get the device status data field
    fn device_status(&self) -> DeviceStatus {
        self.buf()[index::DEVICE_STATUS].into()
    }

    /// Set the device status data field
    fn set_device_status(&mut self, device_status: DeviceStatus) {
        self.buf_mut()[index::DEVICE_STATUS] = device_status.into();
    }

    /// Get the exception status data field
    fn exception_status(&self) -> ExceptionStatus {
        self.buf()[index::EXCEPTION_STATUS].into()
    }

    /// Set the exception status data field
    fn set_exception_status(&mut self, exception_status: ExceptionStatus) {
        self.buf_mut()[index::EXCEPTION_STATUS] = exception_status.into();
    }

    /// Get the miscellaneous device status data field
    fn misc_device_state(&self) -> MiscDeviceState {
        self.buf()[index::MISC_DEVICE_STATE].into()
    }

    /// Set the miscellaneous device status data field
    fn set_misc_device_state(&mut self, misc_device_state: MiscDeviceState) {
        self.buf_mut()[index::MISC_DEVICE_STATE] = misc_device_state.into();
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
