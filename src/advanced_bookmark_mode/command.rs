use crate::{
    bool_enum, impl_default, impl_extended_ops, impl_message_ops, impl_omnibus_extended_command,
    len::ADVANCED_BOOKMARK_MODE_COMMAND, ExtendedCommand, ExtendedCommandOps, MessageOps,
    MessageType,
};

pub mod index {
    pub const STATUS: usize = 7;
}

bool_enum!(
    AdvancedBookmarkStatus,
    r"Whether the Advance Bookmark Mode is enabled on the device"
);

/// Advanced Bookmark Mode - Command (Subtype 0x0D)
///
/// The Advanced bookmark message is used to enable/disable a special mode of the device. This is
/// available on newer firmware. When a device is in advanced bookmark mode, it will allow the very next
/// document to be processed by the host even if it does not have any value. If the document meets certain
/// size restrictions, the host can decide to accept or return the document. If stacked, the document will be
/// reported as “No Value.”
///
/// To prevent accidentally stacking a valid document (banknote or barcode), the device automatically
/// reject all validated documents.
///
/// **WARNING** Even though the device will auto reject a valid note in this mode, there is no guarantee that
/// the device will reject ALL valid notes in this mode because the device may possibly not detect the
/// document as a valid banknote; the burden is placed on the host implementation to ensure no miscounts
/// occur when using this message. This mode is designed for attended services and allows the attendant to
/// insert a slip or receipt that should be placed with the currency.
///
/// The device will automatically leave the mode if any of the following conditions are met:
///
/// * Power is lost
/// * Host informs the device to leave the mode.
/// * Calibration mode is entered.
/// * A document is stacked
///
/// The most important one is the last one: “A document is stacked.” This means the mode does not persist
/// and will only be enabled for a single document. Once this document is stacked, the device will resume
/// normal operations.
///
/// If this mode is enabled along with the normal bookmark mode, then advanced bookmark mode takes
/// precedence; all valid notes and barcodes will automatically be rejected.
///
/// The host must enable this feature every time a custom bookmark is to be accepted. This is done with the
/// following message. (The same message structure can also be used to disable the feature).
///
/// The Advanced Bookmark Mode Command is formatted as follows:
///
/// | Name  | STX  | LEN  | CTRL | Subtype | Data 0 | Data 1 | Data 2 | Status  | ETX  | CHK |
/// |:------|:----:|:----:|:----:|:-------:|:------:|:------:|:------:|:-------:|:----:|:---:|
/// | Byte  | 0    | 1    | 2    | 3       | 4      | 5      | 6      | 7       | 8    | 9   |
/// | Value | 0x02 | 0x0B | 0x7n | 0x04    | nn     | nn     | nn     | 0x00/01 | 0x03 | zz  |
///
/// The Status byte tells the device to enable (0x01) or disable (0x00) the mode.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AdvancedBookmarkModeCommand {
    buf: [u8; ADVANCED_BOOKMARK_MODE_COMMAND],
}

impl AdvancedBookmarkModeCommand {
    /// Creates a new [AdvancedBookmarkModeCommand].
    pub fn new() -> Self {
        let mut message = Self {
            buf: [0u8; ADVANCED_BOOKMARK_MODE_COMMAND],
        };

        message.init();
        message.set_message_type(MessageType::Extended);
        message.set_extended_command(ExtendedCommand::AdvancedBookmark);

        message
    }

    /// Gets the status of Advanced Bookmark Mode.
    pub fn status(&self) -> AdvancedBookmarkStatus {
        self.buf[index::STATUS].into()
    }

    /// Sets the status of Advanced Bookmark Mode.
    pub fn set_status(&mut self, status: AdvancedBookmarkStatus) {
        self.buf[index::STATUS] = status.into();
    }
}

impl_default!(AdvancedBookmarkModeCommand);
impl_message_ops!(AdvancedBookmarkModeCommand);
impl_extended_ops!(AdvancedBookmarkModeCommand);
impl_omnibus_extended_command!(AdvancedBookmarkModeCommand);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;

    #[test]
    #[rustfmt::skip]
    fn test_advanced_bookmark_command_from_bytes() -> Result<()> {
        let msg_bytes = [
            // STX | LEN | Message type | Subtype
            0x02, 0x0a, 0x70, 0x0d,
            // Data
            0x00, 0x00, 0x00, 0x01,
            // ETX | Checksum
            0x03, 0x76,
        ];

        let mut msg = AdvancedBookmarkModeCommand::new();
        msg.from_buf(msg_bytes.as_ref())?;

        assert_eq!(msg.message_type(), MessageType::Extended);
        assert_eq!(msg.extended_command(), ExtendedCommand::AdvancedBookmark);
        assert_eq!(msg.status(), AdvancedBookmarkStatus::Set);

        let msg_bytes = [
            // STX | LEN | Message type | Subtype
            0x02, 0x0a, 0x70, 0x0d,
            // Data
            0x00, 0x00, 0x00, 0x00,
            // ETX | Checksum
            0x03, 0x77,
        ];

        msg.from_buf(msg_bytes.as_ref())?;

        assert_eq!(msg.message_type(), MessageType::Extended);
        assert_eq!(msg.extended_command(), ExtendedCommand::AdvancedBookmark);
        assert_eq!(msg.status(), AdvancedBookmarkStatus::Unset);

        Ok(())
    }
}
