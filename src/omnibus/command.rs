use bitfield::bitfield;

use crate::{
    bool_enum, impl_default, impl_message_ops, impl_omnibus_command_ops,
    len::{FLASH_DATA_PACKET, OMNIBUS_COMMAND},
    FlashDownloadMessage, MessageOps, MessageType, StandardDenomination,
};

bitfield! {
    /// Operational Mode - Omnibus Command: data byte 1
    ///
    /// [OperationalMode] controls details about the operational mode of the device.
    /// It also contains the very important command bits that control what to do with a note in escrow.
    ///
    /// It is a bitfield representing the following settings:
    ///
    /// * Special Interrupt Mode: bit 0 (**Deprecated** **Obsolete** **Unimplemented**)
    /// * High Security Mode: bit 1 (**Deprecated**: Enabling is deprecated/unimplemented, defaults to high acceptance mode)
    /// * [OrientationControl]: bits 2..3
    /// * [EscrowMode]: bit 4
    /// * [DocumentStack]: bit 5
    /// * [DocumentReturn]: bit 6
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct OperationalMode(u8);
    u8;
    /// This field controls the acceptance of bank notes based on the orientation of those
    /// notes as they enter the device. Note that note orientations can also be controlled by
    /// a configuration coupon or on some models, “DIP” switches. In all cases, the most
    /// accommodating of the settings is used. See the Controlling the Orientation of Notes
    /// section (5.2.3) for more details on controlling the orientation of note acceptance.
    pub orientation_control, set_orientation_control: 3, 2;
    /// This mode determines how documents are handled after the documents have been
    /// validated. Note that documents that are unable to be validated are always rejected.
    /// This concept is discussed in detail in section 4.3.
    pub escrow_mode, set_escrow_mode: 4;
    /// If a document is in escrow, stack it in the cash box. Note that this
    /// command is only valid if Escrow mode is enabled and a document is in
    /// escrow. This command and the Document Return command are
    /// mutually exclusive.
    pub document_stack, set_document_stack: 5;
    /// If a document is in escrow, return it to the consumer. Note that this
    /// command is only valid if Escrow mode is enabled and a document is in
    /// escrow. This command and the Document Stack command are mutually
    /// exclusive.
    pub document_return, set_document_return: 6;
}

impl From<OperationalMode> for u8 {
    fn from(o: OperationalMode) -> Self {
        o.0
    }
}

impl From<&OperationalMode> for u8 {
    fn from(o: &OperationalMode) -> Self {
        o.0
    }
}

impl From<u8> for OperationalMode {
    fn from(b: u8) -> Self {
        Self(b & 0b111_1111)
    }
}

/// Controls which bill orientation is accepted
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OrientationControl {
    /// Accept notes fed right edge first, face up only
    OneWay = 0b00,
    /// Accept notes face up only
    TwoWay = 0b01,
    /// Accept notes fed any way
    FourWay = 0b10,
}

impl From<u8> for OrientationControl {
    fn from(b: u8) -> Self {
        match b & bitmask::ORIENTATION {
            0b00 => Self::OneWay,
            0b01 => Self::TwoWay,
            0b10 | 0b11 => Self::FourWay,
            _ => unreachable!("invalid OrientationControl"),
        }
    }
}

impl From<OrientationControl> for u8 {
    fn from(o: OrientationControl) -> Self {
        o as u8
    }
}

impl From<&OrientationControl> for u8 {
    fn from(o: &OrientationControl) -> Self {
        (*o).into()
    }
}

bool_enum!(
    EscrowMode,
    r"
 Determines how documents are handled after validation

 Unset:
     **Deprecated/Obsolete**: Escrow is disabled

 Set:
     Escrow mode is enabled
"
);

bool_enum!(
    DocumentStack,
    r"
 If a document is in escrow, stack it in the cash box

 This command is mutually exclusive with [DocumentReturn](DocumentReturn)

 Unset:
     No-op

 Set:
     Stack a document in the cash box. Only valid if escrow mode enabled
"
);

bool_enum!(
    DocumentReturn,
    r"
 If a document is in escrow, return it to the consumer

 This command is mutually exclusive with [DocumentStack](DocumentStack)

 Unset:
     No-op

 Set:
     Return a document to the consumer. Only valid if escrow mode enabled
"
);

bitfield! {
    /// Configuration - Omnibus Command: data byte 2
    ///
    /// [Configuration] contains configuration settings for the device.
    ///
    /// It is a bitfield representing the following settings:
    ///
    /// * [NoPush]: bit 0
    /// * [Barcode]: bit 1
    /// * [PowerUp]: bits 2..3
    /// * [ExtendedNoteReporting]: bit 4
    /// * [ExtendedCouponReporting]: bit 5
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct Configuration(u8);
    u8;
    pub no_push, set_no_push: 0;
    pub barcode, set_barcode: 1;
    pub power_up, set_power_up: 3, 2;
    pub extended_note, set_extended_note: 4;
    pub extended_coupon, set_extended_coupon: 5;
}

impl From<Configuration> for u8 {
    fn from(c: Configuration) -> Self {
        c.0
    }
}

impl From<&Configuration> for u8 {
    fn from(c: &Configuration) -> Self {
        c.0
    }
}

impl From<u8> for Configuration {
    fn from(b: u8) -> Self {
        Self(b & 0b11_1111)
    }
}

bool_enum!(
    NoPush,
    r"
 There are times when the device is unable to give credit for a note due to a
 problem in transporting the document, and the document cannot be returned
 to the customer. In these cases, this bit determines how such documents should
 be handled.

 Unset:
     Recommended: Push non-credit notes into the stacker, and continue operating.

 Set:
     Do not push non-credit notes. Stall the device with the document still in the path. A
     manager/technician level intervention is required.
"
);

bool_enum!(
    Barcode,
    r"
 A barcode voucher is a document with a unique bar-coded identity number
 encoded into it. These identity numbers are referenced against an external
 database by the host to determine the validity and value of the voucher.
 Notes: Barcode vouchers must be inserted “face up” with CFSC devices.

 Unset: 
     Barcode vouchers are disabled.

 Set:
     Barcode vouchers are enabled.
"
);

/// Values that represent acceptor device power up  policy.
/// That define device behavior on power up with bill in trace.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PowerUp {
    /// Post - Escrow : The procedure will complete and the document will be stacked. However, no value will be reported to the host.
    A = 0b00,
    /// Escrow : The device will go out of order and hold the document at the escrow position.
    /// Post - Escrow : The procedure will complete and the document will be stacked.
    B = 0b01,
    /// Escrow : The device will go out of order and hold the document at the escrow position.
    /// Post - Escrow : The procedure will complete and the document will be stacked.However, no value will be reported to the host.
    C = 0b10,
    Reserved = 0b11,
}

impl From<u8> for PowerUp {
    fn from(b: u8) -> Self {
        match b & bitmask::POWER_UP {
            0b00 => Self::A,
            0b01 => Self::B,
            0b10 => Self::B,
            _ => Self::Reserved,
        }
    }
}

bool_enum!(
    ExtendedNoteReporting,
    r"
 Whether to use extended note reporting for bank note values.

 Unset:
     Use non-extended note reporting. Notes are reported as the generic Denom1 through 7.

 Set:
     Use extended note reporting. Notes are reported to the host via the Extended Omnibus.

 - Note Reply packets. See Section 7.5.2 (Subtype 0x02)
 Extended Note Specification Message for details.

 - Notes are enabled / inhibited individually via the Set Note
 Inhibits command. See (Subtype 0x03) Set Extended Note Inhibits for details.

 - This bit is also associated with enabling / disabling the device.
 See Section 4.9 Disabling the Device and Inhibiting Notes.
"
);

bool_enum!(
    ExtendedCouponReporting,
    r"
 Handling for MEI/CPI coupon vouchers.

 Unset:
     Recommended: No special handling of generic coupons. MEI™ Generic Coupons (if
     supported) are reported the same as a bank note of the same value.
     Free vend coupons are not supported.

 Set:
     Enable detailed reporting of MEI Generic Coupons. The host
     receives details on the type and identification of generic coupons fed
     into the device. See the (Subtype 0x04) Set Escrow Timeout /
     Extended Coupon response (7.5.4) for more details about the device’s
     message when a coupon is detected.
"
);

pub mod index {
    use crate::index::DATA;

    pub const DENOMINATION: usize = DATA;
    pub const OPERATIONAL_MODE: usize = DATA + 1;
    pub const CONFIGURATION: usize = DATA + 2;
}

mod bitmask {
    pub const ORIENTATION: u8 = 0b11;
    pub const POWER_UP: u8 = 0b11;
}

/// Omnibus Command - (Type 1)
///
/// The concept behind the omnibus command is simple. The host sends a packet to the device with
/// virtually everything needed to control a bill acceptor, and the device responds with a packet with
/// virtually everything needed by the host. Thus in theory, only one command is needed. In practice, the
/// sophistication of the command set long ago reached the point where it was not feasible to fit in all the
/// data all the time. Thus the auxiliary and extended commands were created. Despite this, the omnibus
/// command remains the very core of EBDS and the most frequently used command.
///
/// The Omnibus Command is formatted as follows:
///
/// | Name  | STX  | LEN  | CTRL | Data 0 | Data 1 | Data 2 | ETX  | CHK |
/// |:------|:----:|:----:|:----:|:------:|:------:|:------:|:----:|:---:|
/// | Byte  | 0    | 1    | 2    | 4      | 5      | 6      | 7    | 8   |
/// | Value | 0x02 | 0x09 | 0x1n | nn     | nn     | nn     | 0x03 | zz  |
///
/// The data may vary, and is represented by the `nn`.
///
/// Finally, the checksum is denoted with a `zz`.
///
/// This convention will be used throughout the documentation.
///
/// The data bytes are bitfields used to represent:
///
/// * Data byte 0: the [StandardDenomination]s enabled by the device
/// * Data byte 1: the [OperationalMode] settings
/// * Data byte 2: the [Configuration] settings
///
/// For more detailed information about the meaning of the data fields, see the EBDS Protocol
/// Specification: sections 7.1.1.[1-3].
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OmnibusCommand {
    buf: [u8; OMNIBUS_COMMAND],
}

impl OmnibusCommand {
    /// Create a new OmnibusCommand message
    pub fn new() -> Self {
        let mut command = Self {
            buf: [0u8; OMNIBUS_COMMAND],
        };

        command.init();
        command.set_message_type(MessageType::OmnibusCommand);
        command.set_escrow_mode(EscrowMode::Set);

        command
    }
}

pub trait OmnibusCommandOps: MessageOps {
    /// Get the denomination data byte (data byte 0)
    fn denomination(&self) -> StandardDenomination {
        self.buf()[index::DENOMINATION].into()
    }

    /// Set the denomination data byte
    fn set_denomination(&mut self, denomination: StandardDenomination) {
        self.buf_mut()[index::DENOMINATION] = denomination.into();
    }

    /// Get the operational mode data byte (data byte 1)
    fn operational_mode(&self) -> OperationalMode {
        self.buf()[index::OPERATIONAL_MODE].into()
    }

    /// Set the operational mode data byte (data byte 1)
    fn set_operational_mode(&mut self, op: OperationalMode) {
        self.buf_mut()[index::OPERATIONAL_MODE] = op.into();
    }

    /// Get the orientation control data field
    fn orientation_control(&self) -> OrientationControl {
        self.operational_mode().orientation_control().into()
    }

    /// Set the orientation control data field
    fn set_orientation_control(&mut self, orientation: OrientationControl) {
        let mut op = self.operational_mode();
        op.set_orientation_control(orientation as u8);
        self.set_operational_mode(op);
    }

    /// Get the escrow mode data field
    fn escrow_mode(&self) -> EscrowMode {
        self.operational_mode().escrow_mode().into()
    }

    /// Set the escrow mode data field
    fn set_escrow_mode(&mut self, escrow_mode: EscrowMode) {
        let mut op = self.operational_mode();
        op.set_escrow_mode(escrow_mode.into());
        self.set_operational_mode(op);
    }

    /// Get the document stack data field
    fn document_stack(&self) -> DocumentStack {
        self.operational_mode().document_stack().into()
    }

    /// Set the document stack data field
    fn set_document_stack(&mut self, document_stack: DocumentStack) {
        let mut op = self.operational_mode();
        op.set_document_stack(document_stack.into());
        self.set_operational_mode(op);
    }

    /// Get the document return data field
    fn document_return(&self) -> DocumentReturn {
        self.operational_mode().document_return().into()
    }

    /// Set the document return data field
    fn set_document_return(&mut self, document_return: DocumentReturn) {
        let mut op = self.operational_mode();
        op.set_document_return(document_return.into());
        self.set_operational_mode(op);
    }

    /// Get the device configuration setting data byte (data byte 2)
    fn configuration(&self) -> Configuration {
        self.buf()[index::CONFIGURATION].into()
    }

    /// Set the device configuration setting data byte (data byte 2)
    fn set_configuration(&mut self, cfg: Configuration) {
        self.buf_mut()[index::CONFIGURATION] = cfg.into();
    }

    /// Get the no push data field
    fn no_push(&self) -> NoPush {
        self.configuration().no_push().into()
    }

    /// Set the no push data field
    fn set_no_push(&mut self, no_push: NoPush) {
        let mut cfg = self.configuration();
        cfg.set_no_push(no_push.into());
        self.set_configuration(cfg);
    }

    /// Get the barcode data field
    fn barcode(&self) -> Barcode {
        self.configuration().barcode().into()
    }

    /// Set the barcode data field
    fn set_barcode(&mut self, barcode: Barcode) {
        let mut cfg = self.configuration();
        cfg.set_barcode(barcode.into());
        self.set_configuration(cfg);
    }

    /// Get the power up data field
    fn power_up(&self) -> PowerUp {
        self.configuration().power_up().into()
    }

    /// Set the power up data field
    fn set_power_up(&mut self, power_up: PowerUp) {
        let mut cfg = self.configuration();
        cfg.set_power_up(power_up as u8);
        self.set_configuration(cfg);
    }

    /// Get the extended note reporting data field
    fn extended_note(&self) -> ExtendedNoteReporting {
        self.configuration().extended_note().into()
    }

    /// Set the extended note data field
    fn set_extended_note(&mut self, extended_note: ExtendedNoteReporting) {
        let mut cfg = self.configuration();
        cfg.set_extended_note(extended_note.into());
        self.set_configuration(cfg);
    }

    /// Get the extended coupon reporting data field
    fn extended_coupon(&self) -> ExtendedCouponReporting {
        self.configuration().extended_coupon().into()
    }

    /// Set the extended coupon data field
    fn set_extended_coupon(&mut self, extended_coupon: ExtendedCouponReporting) {
        let mut cfg = self.configuration();
        cfg.set_extended_coupon(extended_coupon.into());
        self.set_configuration(cfg);
    }
}

impl_default!(OmnibusCommand);
impl_message_ops!(OmnibusCommand);
impl_omnibus_command_ops!(OmnibusCommand);

// Implements FlashDownloadMessage to allow using OmnibusCommand in
// `AcceptorDeviceHandle::poll_flash_download`
impl FlashDownloadMessage<FLASH_DATA_PACKET> for OmnibusCommand {
    fn is_initial_poll(&self) -> bool {
        true
    }

    fn packet_number(&self) -> u16 {
        0xffff
    }

    fn set_packet_number(&mut self, _n: u16) {}

    fn increment_packet_number(&mut self) -> u16 {
        0xffff
    }

    fn data(&self) -> [u8; FLASH_DATA_PACKET] {
        [0u8; FLASH_DATA_PACKET]
    }

    fn data_ref(&self) -> &[u8] {
        self.buf()
    }

    fn set_data(&mut self, _data: &[u8]) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;

    #[test]
    #[rustfmt::skip]
    fn test_omnibus_command_from_buf() -> Result<()> {
        let msg_bytes = [
            // STX | LEN | Message Type
            0x02, 0x08, 0x10,
            // Data
            0x7f, 0x00, 0x00,
            // ETX | Checksum
            0x03, 0x67,
        ];

        let mut msg = OmnibusCommand::new();
        msg.from_buf(msg_bytes.as_ref())?;

        assert_eq!(msg.message_type(), MessageType::OmnibusCommand);
        assert_eq!(msg.denomination(), StandardDenomination::all());
        assert_eq!(msg.operational_mode(), OperationalMode::from(0));
        assert_eq!(msg.configuration(), Configuration::from(0));

        Ok(())
    }
}
