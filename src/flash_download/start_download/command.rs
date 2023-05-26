use crate::{
    impl_default, impl_message_ops, len::START_DOWNLOAD_COMMAND, ExtendedNoteReporting, MessageOps,
    MessageType,
};

mod index {
    pub const DATA2: usize = 5;
}

/// Host Start Download Command
///
/// The host command shall start the device in Download mode and data 2 contains the option of either
/// being all zeros or having the value 0x10 depending on whether or not extended note mode is supported
/// with the device configuration. (See 4.2.2 for more details on extended note mode option).
///
/// | STX  | LEN  | CTRL | DATA0 | DATA1 | DATA2   | ETX  | CHK  |
/// |------|------|------|-------|-------|---------|------|------|
/// | 0x02 | 0x08 | 0x5n | 0x00  | 0x00  | 0x00/10 | 0x03 | zz   |
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StartDownloadCommand {
    buf: [u8; START_DOWNLOAD_COMMAND],
}

impl StartDownloadCommand {
    /// Creates a new [StartDownloadCommand] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; START_DOWNLOAD_COMMAND],
        };

        msg.init();
        msg.set_message_type(MessageType::FirmwareDownload);

        msg
    }

    /// Gets whether extended note mode is supported with the device configuration.
    pub fn extended_note(&self) -> ExtendedNoteReporting {
        // Shift the data byte right four bits (0x01 = 0x10 >> 4) to convert to the bool_enum impl
        (self.buf[index::DATA2] >> 4).into()
    }

    /// Sets whether extended note mode is supported with the device configuration.
    pub fn set_extended_note(&mut self, extended: ExtendedNoteReporting) {
        let b: u8 = extended.into();
        // Shift the data byte left four bits (0x10 = 0x01 << 4) to convert from the bool_enum impl
        self.buf[index::DATA2] = b << 4;
    }
}

impl_default!(StartDownloadCommand);
impl_message_ops!(StartDownloadCommand);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;

    #[test]
    #[rustfmt::skip]
    fn start_download_command_from_buf() -> Result<()> {
        let msg_bytes = [
            // STX | LEN | Message Type
            0x02, 0x08, 0x50,
            // Data
            0x00, 0x00, 0x10,
            // ETX | Checksum
            0x03, 0x48,
        ];

        let mut msg = StartDownloadCommand::new();
        msg.from_buf(msg_bytes.as_ref())?;

        assert_eq!(msg.message_type(), MessageType::FirmwareDownload);
        assert_eq!(msg.extended_note(), ExtendedNoteReporting::Set);

        let msg_bytes = [
            // STX | LEN | Message Type
            0x02, 0x08, 0x50,
            // Data
            0x00, 0x00, 0x00,
            // ETX | Checksum
            0x03, 0x58,
        ];

        msg.from_buf(msg_bytes.as_ref())?;

        assert_eq!(msg.message_type(), MessageType::FirmwareDownload);
        assert_eq!(msg.extended_note(), ExtendedNoteReporting::Unset);

        Ok(())
    }
}
