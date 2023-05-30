use crate::std;
use std::fmt;

use crate::{
    bool_enum, impl_default, impl_message_ops, impl_omnibus_nop_reply, len::START_DOWNLOAD_REPLY,
    MessageOps, MessageType,
};

pub mod index {
    pub const DATA3: usize = 6;
}

bool_enum!(
    DownloadReady,
    "Indicates whether the device is ready to enter the firmware downloading phase."
);

/// Host Start Download Reply
///
/// Data 3 contains the status of the device. If the Flash Download bit (denoted by 0x02) is not set, the
/// device is not yet in download mode and the Start Download command must be resent. When that bit is
/// set, the device is expecting the host to enter the downloading phase of the download process.
///
/// | STX  | LEN  | CTRL | DATA0 | DATA1 | DATA2 | DATA3   | DATA4 | DATA5 | ETX  | CHK  |
/// |------|------|------|-------|-------|-------|---------|-------|-------|------|------|
/// | 0x02 | 0x0B | 0x5n | nn    | nn    | nn    | 0x00/02 | nn    | nn    | 0x03 | zz   |
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StartDownloadReply {
    buf: [u8; START_DOWNLOAD_REPLY],
}

impl StartDownloadReply {
    /// Creates a new [StartDownloadReply] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; START_DOWNLOAD_REPLY],
        };

        msg.init();
        msg.set_message_type(MessageType::FirmwareDownload);

        msg
    }

    /// Gets whether the device is ready to enter the firmware download phase.
    pub fn download_ready(&self) -> DownloadReady {
        // Shift the Data 3 byte right 1 bit to convert to the bool_enum impl
        // 0x01 = 0x02 >> 1;
        ((self.buf[index::DATA3] & 0b10) >> 1).into()
    }

    /// Sets whether the device is ready to enter the firmware download phase.
    pub fn set_download_ready(&mut self, ready: DownloadReady) {
        let b: u8 = ready.into();
        // Shift the bool_enum value left 1 bit to convert to the Data 3 byte value
        // 0x02 = 0x01 << 1;
        self.buf[index::DATA3] = b << 1;
    }
}

impl_default!(StartDownloadReply);
impl_message_ops!(StartDownloadReply);
impl_omnibus_nop_reply!(StartDownloadReply);

impl fmt::Display for StartDownloadReply {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AckNak: {}, DeviceType: {}, MessageType: {}, DownloadReady: {}",
            self.acknak(),
            self.device_type(),
            self.message_type(),
            self.download_ready(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;

    #[test]
    #[rustfmt::skip]
    fn start_download_command_from_buf() -> Result<()> {
        let msg_bytes = [
            // STX | LEN | Message Type
            0x02, 0x0b, 0x50,
            // Data
            0x00, 0x00, 0x00, 0x02, 0x00, 0x00,
            // ETX | Checksum
            0x03, 0x59,
        ];

        let mut msg = StartDownloadReply::new();
        msg.from_buf(msg_bytes.as_ref())?;

        assert_eq!(msg.message_type(), MessageType::FirmwareDownload);
        assert_eq!(msg.download_ready(), DownloadReady::Set);

        let msg_bytes = [
            // STX | LEN | Message Type
            0x02, 0x0b, 0x50,
            // Data
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            // ETX | Checksum
            0x03, 0x5b,
        ];

        msg.from_buf(msg_bytes.as_ref())?;

        assert_eq!(msg.message_type(), MessageType::FirmwareDownload);
        assert_eq!(msg.download_ready(), DownloadReady::Unset);

        Ok(())
    }
}
