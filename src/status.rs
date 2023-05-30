use crate::std::fmt;

use crate::{bool_enum, StandardDenomination, CLOSE_BRACE, OPEN_BRACE};

mod document_status;

pub use document_status::*;

bitfield! {
    /// DeviceState describes the current state of the device
    ///
    /// The variants describe the bitfield values of data byte 0 in OmnibusReply messages
    ///
    /// The variants ending in `-ing` are transient states, and should only be used for information
    /// purposes.
    ///
    /// The `Stacked`, `Returned`, and `Rejected` bits are mutually exclusive and will never be sent in
    /// the same message.
    ///
    /// DeviceState is a bitfield, representing the following:
    ///
    /// * [Idling]: bit 0
    /// * [Accepting]: bit 1
    /// * [EscrowedState]: bit 2
    /// * [Stacking]: bit 3
    /// * [StackedEvent]: bit 4
    /// * [Returning]: bit 5
    /// * [ReturnedEvent]: bit 6
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct DeviceState(u8);
    u8;
    /// The device is idling. Not processing a document.
    pub idling, set_idling: 0;
    /// The device is drawing in a document.
    pub accepting, set_accepting: 1;
    /// There is a valid document in escrow.
    pub escrowed_state, set_escrowed_state: 2;
    /// The device is stacking a document.
    pub stacking, set_stacking: 3;
    /// The device has stacked a document.
    pub stacked_event, set_stacked_event: 4;
    /// The device is returning a document to the customer.
    pub returning, set_returning: 5;
    /// The device has returned a document to the customer.
    pub returned_event, set_returned_event: 6;
}

impl DeviceState {
    /// Creates a [DeviceState] with no bits set.
    pub const fn none() -> Self {
        Self(0)
    }

    /// If all seven state bits are zero, the device is out of service.
    pub fn out_of_service(&self) -> bool {
        (self.0 & 0b111_1111) == 0
    }

    /// If all seven state bits are zero, the device is out of service.
    pub fn host_disabled(&self) -> bool {
        self.0 == DeviceStateFlags::HostDisabled as u8
    }
}

impl From<u8> for DeviceState {
    fn from(b: u8) -> Self {
        Self(b & 0b111_1111)
    }
}

impl From<DeviceState> for u8 {
    fn from(d: DeviceState) -> Self {
        (&d).into()
    }
}

impl From<&DeviceState> for u8 {
    fn from(d: &DeviceState) -> Self {
        d.0
    }
}

impl From<&mut DeviceState> for u8 {
    fn from(d: &mut DeviceState) -> Self {
        (&*d).into()
    }
}

impl fmt::Display for DeviceState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{OPEN_BRACE}idling:{}, accepting:{}, escrowed_state:{}, stacking:{}, stacked_event:{}, returning:{}, returned_event:{}{CLOSE_BRACE}",
            self.idling(),
            self.accepting(),
            self.escrowed_state(),
            self.stacking(),
            self.stacked_event(),
            self.returning(),
            self.returned_event(),
        )
    }
}

/// Values that represent device states
///
/// Represents semantic values for a combination of bitfield settings in [DeviceState].
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DeviceStateFlags {
    Disconnected = 0,
    PowerUp,
    Initialize,
    Download,
    Idle,
    HostDisabled,
    BusyCalculation,
    Escrowed,
    Accepting,
    Stacking,
    Returning,
    Cheated,
    Jammed,
    StackerFull,
    Paused,
    Calibration,
    Failure,
    Stalled,
    CashBoxRemoved,
    TransportOpened,
    Dispensing,
    FloatingDown,
    Disabled,
    EscrowStorageFull,
    IdleInEscrowSession,
    HostDisabledInEscrowSession,
    UnknownDocumentsDetected,
    PatternRecovering,
    DisabledAndJammed,
    Unknown = 0xff,
}

impl From<u8> for DeviceStateFlags {
    fn from(f: u8) -> Self {
        match f {
            0 => Self::Disconnected,
            1 => Self::PowerUp,
            2 => Self::Initialize,
            3 => Self::Download,
            4 => Self::Idle,
            5 => Self::HostDisabled,
            6 => Self::BusyCalculation,
            7 => Self::Escrowed,
            8 => Self::Accepting,
            9 => Self::Stacking,
            10 => Self::Returning,
            11 => Self::Cheated,
            12 => Self::Jammed,
            13 => Self::StackerFull,
            14 => Self::Paused,
            15 => Self::Calibration,
            16 => Self::Failure,
            17 => Self::Stalled,
            18 => Self::CashBoxRemoved,
            19 => Self::TransportOpened,
            20 => Self::Dispensing,
            21 => Self::FloatingDown,
            22 => Self::Disabled,
            23 => Self::EscrowStorageFull,
            24 => Self::IdleInEscrowSession,
            25 => Self::HostDisabledInEscrowSession,
            26 => Self::UnknownDocumentsDetected,
            27 => Self::PatternRecovering,
            28 => Self::DisabledAndJammed,
            _ => Self::Unknown,
        }
    }
}

impl From<&DeviceState> for DeviceStateFlags {
    fn from(s: &DeviceState) -> Self {
        Self::from(s.0)
    }
}

impl From<&mut DeviceState> for DeviceStateFlags {
    fn from(s: &mut DeviceState) -> Self {
        Self::from(&*s)
    }
}

impl From<DeviceState> for DeviceStateFlags {
    fn from(s: DeviceState) -> Self {
        Self::from(&s)
    }
}

impl From<&DeviceStateFlags> for &'static str {
    fn from(d: &DeviceStateFlags) -> Self {
        match *d {
            DeviceStateFlags::Disconnected => "Disconnected",
            DeviceStateFlags::PowerUp => "Power up",
            DeviceStateFlags::Initialize => "Initialize",
            DeviceStateFlags::Download => "Download",
            DeviceStateFlags::Idle => "Idle",
            DeviceStateFlags::HostDisabled => "Host disabled",
            DeviceStateFlags::BusyCalculation => "Busy calculation",
            DeviceStateFlags::Escrowed => "Escrowed",
            DeviceStateFlags::Accepting => "Accepting",
            DeviceStateFlags::Stacking => "Stacking",
            DeviceStateFlags::Returning => "Returning",
            DeviceStateFlags::Cheated => "Cheated",
            DeviceStateFlags::Jammed => "Jammed",
            DeviceStateFlags::StackerFull => "Stacker full",
            DeviceStateFlags::Paused => "Paused",
            DeviceStateFlags::Calibration => "Calibration",
            DeviceStateFlags::Failure => "Failure",
            DeviceStateFlags::Stalled => "Stalled",
            DeviceStateFlags::CashBoxRemoved => "CashBox removed",
            DeviceStateFlags::TransportOpened => "Transport opened",
            DeviceStateFlags::Dispensing => "Dispensing",
            DeviceStateFlags::FloatingDown => "Floating down",
            DeviceStateFlags::Disabled => "Disabled",
            DeviceStateFlags::EscrowStorageFull => "Escrow storage full",
            DeviceStateFlags::IdleInEscrowSession => "Idle in escrow session",
            DeviceStateFlags::HostDisabledInEscrowSession => "Host disabled in escrow session",
            DeviceStateFlags::UnknownDocumentsDetected => "Unknown documents detected",
            DeviceStateFlags::PatternRecovering => "Pattern recovering",
            DeviceStateFlags::DisabledAndJammed => "Disabled and jammed",
            DeviceStateFlags::Unknown => "Unknown",
        }
    }
}

impl From<DeviceStateFlags> for &'static str {
    fn from(d: DeviceStateFlags) -> Self {
        (&d).into()
    }
}

impl fmt::Display for DeviceStateFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", <&'static str>::from(self))
    }
}

bool_enum!(Idling, r"The device is idling. Not processing a document.");
bool_enum!(Accepting, r"The device is drawing in a document.");
bool_enum!(EscrowedState, r"There is a valid document in escrow.");
bool_enum!(Stacking, r"The device is stacking a document.");
bool_enum!(StackedEvent, r"The device has stacked a document.");
bool_enum!(
    Returning,
    r"The device is returning a document to the customer."
);
bool_enum!(
    ReturnedEvent,
    r"The device has returned a document to the customer."
);

bitfield! {
    /// DeviceStatus contains non-state related status of the device.
    ///
    /// DeviceStatus is a bitfield, representing the following:
    ///
    /// * [Cheated]: bit 0
    /// * [Rejected]: bit 1
    /// * [Jammed]: bit 2
    /// * [StackerFull]: bit 3
    /// * [CassetteAttached]: bit 4
    /// * [Paused]: bit 5
    /// * [Calibration]: bit 6
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct DeviceStatus(u8);
    u8;
    /// The device has detected conditions consistent with an attempt to fraud the system.
    pub cheated, set_cheated: 0;
    /// The document presented to the device could not be validated and was returned to the customer.
    pub rejected, set_rejected: 1;
    /// The path is blocked and the device has been unable to resolve the issue.
    /// Intervention is required.
    pub jammed, set_jammed: 2;
    /// The cash box is full of documents and no more may be accepted.
    /// The device will be out of service until the issue is corrected.
    pub stacker_full, set_stacker_full: 3;
    /// If unset, the cash box has been removed. No documents may be accepted.
    /// The device is out of service until the issue is corrected.
    ///
    /// If set, the cash box is attached to the device.
    pub cassette_attached, set_cassette_attached: 4;
    /// The customer is attempting to feed another note while the previous
    /// note is still being processed. The customer must remove the note to
    /// permit processing to continue.
    pub paused, set_paused: 5;
    /// It is possible to field calibrate devices. In general, due to advances in processes
    /// used in manufacturing and continuous self-calibration, this is not needed.
    /// Calibrating a device with an incorrect document will greatly reduce
    /// performance. For more information on field calibration please refer to section 4.7.
    ///
    /// If unset, the device is in normal operation.
    ///
    /// If set, the device is in calibration mode. Intervention is required to feed a
    /// special calibration document into the device.
    pub calibration, set_calibration: 6;
}

impl fmt::Display for DeviceStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{OPEN_BRACE}\"cheated\":{},\"rejected\":{},\"jammed\":{},\"stacker_full\":{},\"cassette_attached\":{},\"paused\":{},\"calibration\":{}{CLOSE_BRACE}",
            self.cheated(),
            self.rejected(),
            self.jammed(),
            self.stacker_full(),
            self.cassette_attached(),
            self.paused(),
            self.calibration(),
        )
    }
}

impl DeviceStatus {
    /// Creates a [DeviceStatus] with no set bits.
    pub const fn none() -> Self {
        Self(0)
    }
}

impl From<u8> for DeviceStatus {
    fn from(b: u8) -> Self {
        Self(b & 0b111_1111)
    }
}

impl From<DeviceStatus> for u8 {
    fn from(d: DeviceStatus) -> Self {
        d.0
    }
}

impl From<&DeviceStatus> for u8 {
    fn from(d: &DeviceStatus) -> Self {
        d.0
    }
}

/// Values that represent the cash box status
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CashBoxStatus {
    Attached = 0,
    Removed = 1,
    Full = 2,
    Unknown = 0xff,
}

impl CashBoxStatus {
    pub const fn default() -> Self {
        Self::Attached
    }
}

impl From<bool> for CashBoxStatus {
    fn from(b: bool) -> Self {
        match b {
            true => Self::Attached,
            false => Self::Removed,
        }
    }
}

impl From<u8> for CashBoxStatus {
    fn from(b: u8) -> Self {
        match b {
            0 => Self::Attached,
            1 => Self::Removed,
            2 => Self::Full,
            _ => Self::Unknown,
        }
    }
}

impl From<&CashBoxStatus> for u8 {
    fn from(c: &CashBoxStatus) -> Self {
        (*c) as u8
    }
}

impl From<CashBoxStatus> for u8 {
    fn from(c: CashBoxStatus) -> Self {
        (&c).into()
    }
}

impl From<CashBoxStatus> for &'static str {
    fn from(c: CashBoxStatus) -> Self {
        match c {
            CashBoxStatus::Attached => "attached",
            CashBoxStatus::Removed => "removed",
            CashBoxStatus::Full => "full",
            CashBoxStatus::Unknown => "unknown",
        }
    }
}

impl From<&CashBoxStatus> for &'static str {
    fn from(c: &CashBoxStatus) -> Self {
        (*c).into()
    }
}

impl fmt::Display for CashBoxStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", <&'static str>::from(self))
    }
}

bool_enum!(
    Cheated,
    r"The device has detected conditions consistent with an attempt to fraud the system."
);
bool_enum!(
    Rejected,
    r"The document presented to the device could not be validated and was returned to the customer."
);
bool_enum!(
    Jammed,
    r"
 The path is blocked and the device has been unable to resolve the issue.

 Intervention is required.
"
);
bool_enum!(
    StackerFull,
    r"
 The cash box is full of documents and no more may be accepted.

 The device will be out of service until the issue is corrected.
"
);
bool_enum!(
    CassetteAttached,
    r"
 Unset: the cash box has been removed. No documents may be accepted.
 The device is out of service until the issue is corrected.

 Set: the cash box is attached to the device.
"
);
bool_enum!(
    Paused,
    r"
 The customer is attempting to feed another note while the previous
 note is still being processed. The customer must remove the note to
 permit processing to continue.
"
);
bool_enum!(
    Calibration,
    r"
 It is possible to field calibrate devices. In general, due to advances in processes
 used in manufacturing and continuous self-calibration, this is not needed.
 Calibrating a device with an incorrect document will greatly reduce
 performance. For more information on field calibration please refer to section 4.7.

 Unset: the device is in normal operation.

 Set: the device is in calibration mode. Intervention is required to feed a
 special calibration document into the device.
"
);

bitfield! {
    /// ExceptionStatus contains additional information on exceptional statuses as well as the reporting of the note value. (Non-extended mode only.)
    ///
    /// ExceptionStatus is a bitfield, representing the following:
    ///
    /// * [PowerUp]: bit 0
    /// * [InvalidCommand]: bit 1
    /// * [Failure]: bit 2
    /// * [NoteValue](crate::denomination::StandardDenomination): bit 3..5
    /// * [TranportOpen]: bit 6
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct ExceptionStatus(u8);
    u8;
    /// Unset (0): The device is operating normally (Power up process is complete)
    /// Set (1): The device has been powered up. It is performing its initialization
    ///     routine, and not yet ready to accept documents.
    pub power_up, set_power_up: 0;
    /// The device received an invalid command.
    pub invalid_command, set_invalid_command: 1;
    /// The device has encountered a problem and is out of service.
    /// Intervention is required.
    pub failure, set_failure: 2;
    /// The non-extended note value field. This field is valid when the device is in non-
    /// extended mode and either the escrow or stacked bits are set.
    /// (See Omnibus Reply – Data Byte 0 for details of those events)
    ///
    /// 000 - Unknown/No credit
    /// 001 - Denom1
    /// 010 - Denom2
    /// 011 - Denom3
    /// 100 - Denom4
    /// 101 - Denom5
    /// 110 - Denom6
    /// 111 - Denom7
    pub note_value, set_note_value: 5, 3;
    /// Unset (0) - Note path access is closed
    /// Set (1) - Note path access is opened (vault, door, or both).
    ///
    /// **WARNING**: This bit will also be reported one time upon power up if the note path
    /// was opened while the unit was powered down.
    pub transport_open, set_transport_open: 6;
}

impl ExceptionStatus {
    /// Create an [ExceptionStatus] with no bits set.
    pub const fn none() -> Self {
        Self(0)
    }
}

impl fmt::Display for ExceptionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{OPEN_BRACE}\"power_up\":{},\"invalid_command\":{},\"note_value\":\"{}\",\"transport_open\":{}{CLOSE_BRACE}",
            self.power_up(),
            self.invalid_command(),
            StandardDenomination::from_note_value(self.note_value()),
            self.transport_open(),
        )
    }
}

impl From<u8> for ExceptionStatus {
    fn from(b: u8) -> Self {
        Self(b & 0b111_1111)
    }
}

impl From<ExceptionStatus> for u8 {
    fn from(e: ExceptionStatus) -> Self {
        e.0
    }
}

impl From<&ExceptionStatus> for u8 {
    fn from(e: &ExceptionStatus) -> Self {
        e.0
    }
}

bool_enum!(
    PowerUpStatus,
    r"
 The power up status of the device.
 
 Unset: The device is operating normally (Power up process is complete)

 Set: The device has been powered up. It is performing its initialization routine, and not yet ready to accept documents.
"
);
bool_enum!(
    InvalidCommand,
    r"Whether the device received an invalid command."
);
bool_enum!(
    Failure,
    r"
 The device has encountered a problem and is out of service.

 Intervention is required.
"
);
bool_enum!(
    TransportOpen,
    r"
 Whether the device transport is open.

 Unset: Note path access is closed

 Set: Note path access is opened (vault, door, or both).

 **WARNING**: This bit will also be reported one time upon power up if the note path was opened while the unit was powered down. 
"
);

bitfield! {
    /// MiscDeviceState contains miscellaneous device state fields.
    ///
    /// MiscDeviceState is a bitfield, representing the following:
    ///
    /// * [Stalled]: bit 0
    /// * [FlashDownload]: bit 1
    /// * [PreStack]: bit 2
    /// * [RawBarcode]: bit 3
    /// * [DeviceCapabilities]: bit 4
    /// * [Disabled]: bit 5
    /// * Reserved: bit 6
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct MiscDeviceState(u8);
    u8;
    /// The device is stalled.
    pub stalled, set_stalled: 0;
    /// A flash download is ready to commence. The host may begin send
    /// download records. See section 4.8 for details.
    pub flash_download, set_flash_download: 1;
    /// **Deprecated**: This bit indicates that the document has reached a
    /// point in the stacking process where it can no longer be retrieved.
    pub pre_stack, set_pre_stack: 2;
    /// **Gaming only**: whether 24 character barcodes will be converted to 18 characters
    pub raw_barcode, set_raw_barcode: 3;
    /// Whether the Query Device Capabilities command is supported
    pub device_capabilities, set_device_capabilities: 4;
    /// Unset (0): SCR device enabled
    /// Set (1): SCR device disabled
    pub disabled, set_disabled: 5;
    /// Reserved
    pub reserved, _: 6;
}

impl fmt::Display for MiscDeviceState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{OPEN_BRACE}\"stalled\":{},\"flash_download\":{},\"pre_stack\":{},\"raw_barcode\":{},\"device_capabilities\":{},\"disabled\":{}{CLOSE_BRACE}",
            self.stalled(),
            self.flash_download(),
            self.pre_stack(),
            self.raw_barcode(),
            self.device_capabilities(),
            self.disabled(),
        )
    }
}

impl From<u8> for MiscDeviceState {
    fn from(b: u8) -> Self {
        Self(b & 0b11_1111)
    }
}

impl From<MiscDeviceState> for u8 {
    fn from(m: MiscDeviceState) -> Self {
        m.0
    }
}

impl From<&MiscDeviceState> for u8 {
    fn from(m: &MiscDeviceState) -> Self {
        m.0
    }
}

bool_enum!(Stalled, r" The device is stalled.");
bool_enum!(
    FlashDownload,
    r"
 A flash download is ready to commence. The host may begin send
 download records. See section 4.8 for details.
"
);
bool_enum!(
    PreStack,
    r"
 **Deprecated**: This bit indicates that the document has reached a
 point in the stacking process where it can no longer be retrieved.
"
);
bool_enum!(
    RawBarcode,
    r" **Gaming only**: whether 24 character barcodes will be converted to 18 characters"
);
bool_enum!(
    DeviceCapabilities,
    r" Whether the Query Device Capabilities command is supported."
);
bool_enum!(
    Disabled,
    r"
 Whether the SCR device is disabled.

 Unset: SCR device enabled

 Set: SCR device disabled
"
);

bitfield! {
    /// ModelNumber contains the model number identification of the device. The following tables show how the
    /// device model can be obtained depending on the known device types.
    ///
    /// | Bit # | Name         | Value | Description                                    |
    /// |-------|--------------|-------|------------------------------------------------|
    /// | 0..6  | Model Number | nn    | A value that represents the mode of the device |
    ///
    /// **S2K**
    ///
    /// | Hex  | Decimal | ASCII | Product                                                     |
    /// |:----:|:-------:|:-----:|:------------------------------------------------------------|
    /// | 0x41 | 65      | **A** | AE2600 Gen2D, Australia                                     |
    /// | 0x42 | 66      | **B** | AE2800 Gen2D, Russia                                        |
    /// | 0x43 | 67      | **C** | AE2600 Gen2D, Canada                                        |
    /// | 0x44 | 68      | **D** | AE2800 Gen2D, Euro                                          |
    /// | 0x45 | 68      | **E** | Reserved (VN2300, US Economy)                               |
    /// | 0x46 | 70      | **F** | Reserved (VN2600 Gen 2B, Gen 2D, China)                     |
    /// | 0x47 | 71      | **G** | Reserved (AE2800 Gen2D, Argentina)                          |
    /// | 0x48 | 72      | **H** | AE2400, US Economy                                          |
    /// | 0x49 | 73      | **I** | AE2600 Gen2D, Vietnam                                       |
    /// | 0x4A | 74      | **J** | VN2600 Gen2D, Colombian                                     |
    /// | 0x4B | 75      | **K** | VN2600 Gen2D, Ukraine                                       |
    /// | 0x4C | 76      | **L** | AE2400 Gen2C, US Low Cost                                   |
    /// | 0x4D | 77      | **M** | AE2800 Gen2D, Mexico                                        |
    /// | 0x4E | 78      | **N** | AE2400 Gen2D, US                                            |
    /// | 0x4F | 79      | **O** | VN2600 Gen2D, Philippines (Green revision)                  |
    /// | 0x50 | 80      | **P** | AE2600 Gen2B, Gen2C, Gen2D, US Premium                      |
    /// | 0x51 | 81      | **Q** | VN2600 Gen2D, Philippines (Red revision - **DISCONTINUED**) |
    /// | 0x52 | 82      | **R** | VN2500, US VER1 Reference                                   |
    /// | 0x53 | 83      | **S** | AE2600 Gen2D, Saudi                                         |
    /// | 0x54 | 84      | **T** | RFU                                                         |
    /// | 0x55 | 85      | **U** | VN2500 Gen2C, US                                            |
    /// | 0x56 | 86      | **V** | VN2500, US VER2 Reference                                   |
    /// | 0x57 | 87      | **W** | AE2800 Gen2D, Brazil                                        |
    /// | 0x58 | 88      | **X** | AE2800 Gen2D, US Expanded                                   |
    /// | 0x59 | 89      | **Y** | RFU                                                         |
    /// | 0x5A | 90      | **Z** | AE2600 Gen2D, Indonesia                                     |
    /// | 0x68 | 104     | **h** | AE2600 Gen2D, Croatia                                       |
    /// | 0x69 | 105     | **i** | AE2600 Gen2D, Israel                                        |
    ///
    /// **SC** **SC Adv** **SCR**
    ///
    /// | Hex  | Decimal | ASCII | Product                                                     |
    /// |:----:|:-------:|:-----:|:------------------------------------------------------------|
    /// | 0x54 | 84      | **T** | Cashflow SC83<br>Cashflow SC85<br>SC Advance 83 (SCN83)<br>SC Advance 85 (SCN85)<br>SCR83 |
    /// | 0x55 | 85      | **U** | Cashflow SC66<br>SC Advance 66 (SCN66)                      |
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct ModelNumber(u8);
    u8;
    pub model_number, set_model_number: 6, 0;
}

#[cfg(feature = "s2k")]
impl fmt::Display for ModelNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let model_str: &'static str = S2kModelNumber::from(self).into();
        write!(f, "\"{}\"", model_str)
    }
}

#[cfg(feature = "sc")]
impl fmt::Display for ModelNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let model_str: &'static str = SCModelNumber::from(self).into();
        write!(f, "\"{}\"", model_str)
    }
}

#[cfg(not(any(feature = "s2k", feature = "sc")))]
impl fmt::Display for ModelNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:x}", self.0)
    }
}

impl From<u8> for ModelNumber {
    fn from(b: u8) -> Self {
        Self(b & 0b111_1111)
    }
}

impl From<ModelNumber> for u8 {
    fn from(m: ModelNumber) -> Self {
        m.0
    }
}

impl From<&ModelNumber> for u8 {
    fn from(m: &ModelNumber) -> Self {
        m.0
    }
}

/// Model numbers for S2K machines
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S2kModelNumber {
    AE2600Australia = 0x41,
    AE2800Russia = 0x42,
    AE2600Canada = 0x43,
    AE2800Euro = 0x44,
    ReservedVN2300 = 0x45,
    ReservedVN2600 = 0x46,
    ReservedAE2800 = 0x47,
    AE2400USEconomy = 0x48,
    AE2600Vietnam = 0x49,
    VN2600Colombia = 0x4a,
    VN2600Ukraine = 0x4b,
    AE2400USLowCost = 0x4c,
    AE2800Mexico = 0x4d,
    AE2400US = 0x4e,
    VN2600PhilippinesGreen = 0x4f,
    AE2600USPremium = 0x50,
    VN2600PhilippinesRed = 0x51,
    VN2500USReferenceV1 = 0x52,
    AE2600Saudi = 0x53,
    Reserved0 = 0x54,
    VN2500US = 0x55,
    VN2500USReferenceV2 = 0x56,
    AE2800Brazil = 0x57,
    AE2800USExpanded = 0x58,
    Reserved1 = 0x59,
    AE2600Indonesia = 0x5a,
    AE2600Croatia = 0x68,
    AE2600Israel = 0x69,
    Reserved = 0xff,
}

impl From<u8> for S2kModelNumber {
    fn from(b: u8) -> Self {
        match b {
            0x41 => Self::AE2600Australia,
            0x42 => Self::AE2800Russia,
            0x43 => Self::AE2600Canada,
            0x44 => Self::AE2800Euro,
            0x45 => Self::ReservedVN2300,
            0x46 => Self::ReservedVN2600,
            0x47 => Self::ReservedAE2800,
            0x48 => Self::AE2400USEconomy,
            0x49 => Self::AE2600Vietnam,
            0x4a => Self::VN2600Colombia,
            0x4b => Self::VN2600Ukraine,
            0x4c => Self::AE2400USLowCost,
            0x4d => Self::AE2800Mexico,
            0x4e => Self::AE2400US,
            0x4f => Self::VN2600PhilippinesGreen,
            0x50 => Self::AE2600USPremium,
            0x51 => Self::VN2600PhilippinesRed,
            0x52 => Self::VN2500USReferenceV1,
            0x53 => Self::AE2600Saudi,
            0x54 => Self::Reserved0,
            0x55 => Self::VN2500US,
            0x56 => Self::VN2500USReferenceV2,
            0x57 => Self::AE2800Brazil,
            0x58 => Self::AE2800USExpanded,
            0x59 => Self::Reserved1,
            0x5a => Self::AE2600Indonesia,
            0x68 => Self::AE2600Croatia,
            0x69 => Self::AE2600Israel,
            _ => Self::Reserved,
        }
    }
}

impl From<ModelNumber> for S2kModelNumber {
    fn from(b: ModelNumber) -> Self {
        b.0.into()
    }
}

impl From<&ModelNumber> for S2kModelNumber {
    fn from(b: &ModelNumber) -> Self {
        (*b).into()
    }
}

impl From<S2kModelNumber> for &'static str {
    fn from(s: S2kModelNumber) -> Self {
        match s {
            S2kModelNumber::AE2600Australia => "AE2600 Gen2D, Australia",
            S2kModelNumber::AE2800Russia => "AE2800 Gen2D, Russia",
            S2kModelNumber::AE2600Canada => "AE2600 Gen2D, Canada",
            S2kModelNumber::AE2800Euro => "AE2800 Gen2D, Euro",
            S2kModelNumber::ReservedVN2300 => "Reserved (VN2300, US Economy)",
            S2kModelNumber::ReservedVN2600 => "Reserved (VN2600 Gen2B, Gen2D, China)",
            S2kModelNumber::ReservedAE2800 => "Reserved (AE2800 Gen2D, Argentina)",
            S2kModelNumber::AE2400USEconomy => "AE2400, US Economy",
            S2kModelNumber::AE2600Vietnam => "AE2600 Gen2D, Vietnam",
            S2kModelNumber::VN2600Colombia => "VN2600 Gen2D, Colombian",
            S2kModelNumber::VN2600Ukraine => "VN2600 Gen2D, Ukraine",
            S2kModelNumber::AE2400USLowCost => "AE2400 Gen2C, US Low Cost",
            S2kModelNumber::AE2800Mexico => "AE2800 Gen2D, Mexico",
            S2kModelNumber::AE2400US => "AE2400 Gen2D, US",
            S2kModelNumber::VN2600PhilippinesGreen => "VN2600 Gen2D, Philippines (Green revision)",
            S2kModelNumber::AE2600USPremium => "AE2600 Gen2B, Gen2C, Gen2D, US Premium",
            S2kModelNumber::VN2600PhilippinesRed => {
                "VN2600 Gen 2D, Philippines (Red revision - DISCONTINUED)"
            }
            S2kModelNumber::VN2500USReferenceV1 => "VN2500, US VER1 Reference",
            S2kModelNumber::AE2600Saudi => "AE2600 Gen2D, Saudi",
            S2kModelNumber::Reserved0 => "RFU",
            S2kModelNumber::VN2500US => "VN2500 Gen2C, US",
            S2kModelNumber::VN2500USReferenceV2 => "VN2500, US VER2 Reference",
            S2kModelNumber::AE2800Brazil => "AE2800 Gen2D, Brazil",
            S2kModelNumber::AE2800USExpanded => "AE2800 Gen2D, US Expanded",
            S2kModelNumber::Reserved1 => "RFU",
            S2kModelNumber::AE2600Indonesia => "AE2600 Gen2D, Indonesia",
            S2kModelNumber::AE2600Croatia => "AE2600 Gen2D, Croatia",
            S2kModelNumber::AE2600Israel => "AE2600 Gen2D, Israel",
            S2kModelNumber::Reserved => "Reserved",
        }
    }
}

impl From<&S2kModelNumber> for &'static str {
    fn from(s: &S2kModelNumber) -> &'static str {
        (*s).into()
    }
}

/// Model numbers for SC, SC Advanced, and SCR machines
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCModelNumber {
    SC8385 = 0x54,
    SC66 = 0x55,
    Reserved = 0xff,
}

impl From<SCModelNumber> for &'static str {
    fn from(s: SCModelNumber) -> Self {
        match s {
            SCModelNumber::SC8385 => "Cashflow SC83 / Cashflow SC85 / SC Advance 83 (SCN83) / SC Advance 85 (SCN85) / SCR83",
            SCModelNumber::SC66 => "Cashflow SC66 / SC Advance 66 (SCN66)",
            _ => "Reserved",
        }
    }
}

impl From<&SCModelNumber> for &'static str {
    fn from(s: &SCModelNumber) -> Self {
        (*s).into()
    }
}

impl From<u8> for SCModelNumber {
    fn from(b: u8) -> Self {
        match b {
            0x54 => Self::SC8385,
            0x55 => Self::SC66,
            _ => Self::Reserved,
        }
    }
}

impl From<ModelNumber> for SCModelNumber {
    fn from(b: ModelNumber) -> Self {
        b.0.into()
    }
}

impl From<&ModelNumber> for SCModelNumber {
    fn from(b: &ModelNumber) -> Self {
        (*b).into()
    }
}

bitfield! {
    /// CodeRevision contains the code revision identifier. However, the version number of the code is not
    /// sufficient to identify that software. This is because different software parts use independent version
    /// numbers. Version numbers are only useful for comparing firmware from the same software part.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct CodeRevision(u8);
    u8;
    /// The version number of the firmware code in the device. This may be coded as:
    ///
    ///- CFSC , SC Adv , SCR : A seven bit binary value with an implied
    ///     divide by 10. (versions 0.0 through 12.7). Example (0x23 = 35
    ///     decimal. 35/10 = version 3.50)
    ///
    ///- S2K : A 3 and 4 digit BCD value with an implied divide by 10.
    ///     (versions 0.0 through 7.9). Ignoring the most significant bit,
    ///     the next 3 bits make up major build (x111 = 7). Last 4 bits
    ///     make up the minor revision (1001 = 9)
    pub code_revision, set_code_revision: 6, 0;
}

impl From<u8> for CodeRevision {
    fn from(b: u8) -> Self {
        Self(b & 0b111_1111)
    }
}

impl From<CodeRevision> for u8 {
    fn from(c: CodeRevision) -> Self {
        c.0
    }
}

impl From<&CodeRevision> for u8 {
    fn from(c: &CodeRevision) -> Self {
        c.0
    }
}

#[cfg(feature = "s2k")]
impl fmt::Display for CodeRevision {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", S2kCodeRevision(*self))
    }
}

#[cfg(feature = "sc")]
impl fmt::Display for CodeRevision {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", SCCodeRevision(*self))
    }
}

/// Wrapper around [CodeRevision](CodeRevision) for S2K-variant devices
pub struct S2kCodeRevision(CodeRevision);

impl fmt::Display for S2kCodeRevision {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.0 .0 & 0b111_0000 >> 4, self.0 .0 & 0b1111)
    }
}

/// Wrapper around [CodeRevision](CodeRevision) for SC-variant devices
pub struct SCCodeRevision(CodeRevision);

impl fmt::Display for SCCodeRevision {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", (self.0 .0 & 0b111_1111) as f32 / 10f32)
    }
}
