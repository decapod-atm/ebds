#[cfg(not(feature = "std"))]
use alloc::string::String;

use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};

use crate::std;
use std::fmt;

use crate::{
    cash::Currency,
    jsonrpc::{CLOSE_BRACE, OPEN_BRACE},
    status::{DeviceState, DeviceStateFlags},
};

pub const ENV_BAU_DEVICE: &str = "SERIAL_PATH_BAU";
pub const ENV_CDU_DEVICE: &str = "SERIAL_PATH_CPU";
pub const DEFAULT_BAU_DEV_PATH: &str = "/dev/bau";
pub const DEFAULT_CDU_DEV_PATH: &str = "/dev/cdu";

/// HardwareComponent is a list of possible hardware components used on the platform
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub enum HardwareComponent {
    /// Cash dispenser unit
    CDU,
    /// Electronic payment process
    EPP,
    /// Stores interface unit
    SIU,
    /// Remote pickup unit
    RPU,
    /// Magnetic card reader
    MCR,
    /// Bill acceptor unit
    BAU,
    /// Bill acceptor 2?
    BA2,
    /// Bar code scanner?
    BCS,
    /// Camera
    CAM,
    /// Universal power supply
    UPS,
}

impl HardwareComponent {
    pub const fn default() -> Self {
        Self::BAU
    }
}

impl From<HardwareComponent> for &'static str {
    fn from(h: HardwareComponent) -> Self {
        match h {
            HardwareComponent::CDU => "CDU",
            HardwareComponent::EPP => "EPP",
            HardwareComponent::SIU => "SIU",
            HardwareComponent::RPU => "RPU",
            HardwareComponent::MCR => "MCR",
            HardwareComponent::BAU => "BAU",
            HardwareComponent::BA2 => "BA2",
            HardwareComponent::BCS => "BCS",
            HardwareComponent::CAM => "CAM",
            HardwareComponent::UPS => "UPS",
        }
    }
}

impl From<&HardwareComponent> for &'static str {
    fn from(h: &HardwareComponent) -> Self {
        (*h).into()
    }
}

impl fmt::Display for HardwareComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", <&'static str>::from(self))
    }
}

/// HardwareState represents the different states hardware can be in
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Deserialize)]
#[serde(field_identifier, rename_all = "UPPERCASE")]
pub enum HardwareState {
    /// Everything is running properly
    OK,
    /// The hardware is missing
    Missing,
    /// The hardware emitted a warning
    Warning,
    /// The hardware emitted an error
    Error,
}

impl HardwareState {
    pub const fn default() -> Self {
        Self::OK
    }
}

impl Serialize for HardwareState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Self::OK => serializer.serialize_unit_variant("HardwareState", 0, "OK"),
            Self::Missing => serializer.serialize_unit_variant("HardwareState", 1, "MISSING"),
            Self::Warning => serializer.serialize_unit_variant("HardwareState", 2, "WARNING"),
            Self::Error => serializer.serialize_unit_variant("HardwareState", 3, "ERROR"),
        }
    }
}

impl From<DeviceState> for HardwareState {
    fn from(dev_state: DeviceState) -> Self {
        Self::from(DeviceStateFlags::from(dev_state))
    }
}

impl From<DeviceStateFlags> for HardwareState {
    fn from(dev_state: DeviceStateFlags) -> Self {
        match dev_state {
            DeviceStateFlags::Disconnected | DeviceStateFlags::CashBoxRemoved => Self::Missing,
            DeviceStateFlags::PowerUp
            | DeviceStateFlags::Initialize
            | DeviceStateFlags::Download
            | DeviceStateFlags::Idle
            | DeviceStateFlags::HostDisabled
            | DeviceStateFlags::BusyCalculation
            | DeviceStateFlags::Escrowed
            | DeviceStateFlags::Accepting
            | DeviceStateFlags::Stacking
            | DeviceStateFlags::Returning
            | DeviceStateFlags::Paused
            | DeviceStateFlags::Calibration
            | DeviceStateFlags::Dispensing
            | DeviceStateFlags::FloatingDown
            | DeviceStateFlags::IdleInEscrowSession
            | DeviceStateFlags::HostDisabledInEscrowSession
            | DeviceStateFlags::PatternRecovering => Self::OK,
            DeviceStateFlags::Cheated
            | DeviceStateFlags::StackerFull
            | DeviceStateFlags::TransportOpened
            | DeviceStateFlags::EscrowStorageFull
            | DeviceStateFlags::UnknownDocumentsDetected => Self::Warning,
            DeviceStateFlags::Jammed
            | DeviceStateFlags::Failure
            | DeviceStateFlags::Stalled
            | DeviceStateFlags::DisabledAndJammed
            | DeviceStateFlags::Disabled => Self::Error,
            _ => Self::Error,
        }
    }
}

impl From<HardwareState> for &'static str {
    fn from(h: HardwareState) -> Self {
        match h {
            HardwareState::OK => "OK",
            HardwareState::Missing => "MISSING",
            HardwareState::Warning => "WARNING",
            HardwareState::Error => "ERROR",
        }
    }
}

impl From<&HardwareState> for &'static str {
    fn from(h: &HardwareState) -> Self {
        (*h).into()
    }
}

impl fmt::Display for HardwareState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <&'static str>::from(self))
    }
}

/// BillAcceptorStatusDetails represents detailed information about the bill acceptor hardware
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct BillAcceptorStatusDetails {
    // Was the cashbox removed, or has the field not been set
    cashbox_removed: Option<bool>,
    // The firmware version
    firmware_version: Option<String>,
    // ISO string for the type of currency
    currency: Option<Currency>,
    // Is the bill acceptor jammed, or has the field not been set
    jammed: Option<bool>,
}

impl BillAcceptorStatusDetails {
    /// Create a new BillAcceptorStatusDetails
    pub const fn new(
        cashbox_removed: Option<bool>,
        firmware_version: Option<String>,
        currency: Option<Currency>,
        jammed: Option<bool>,
    ) -> Self {
        Self {
            cashbox_removed,
            firmware_version,
            currency,
            jammed,
        }
    }

    pub const fn default() -> Self {
        Self {
            cashbox_removed: None,
            firmware_version: None,
            currency: None,
            jammed: None,
        }
    }

    /// Builder function to include cashbox removed status
    pub fn with_cashbox_removed(mut self, cashbox_removed: bool) -> Self {
        self.cashbox_removed = Some(cashbox_removed);
        self
    }

    /// Builder function to include firmware version
    pub fn with_firmware_version(mut self, firmware_version: &str) -> Self {
        self.firmware_version = Some(firmware_version.into());
        self
    }

    /// Builder function to include currency status
    pub fn with_currency(mut self, currency: Currency) -> Self {
        self.currency = Some(currency);
        self
    }

    /// Builder function to include jammed status
    pub fn with_jammed(mut self, jammed: bool) -> Self {
        self.jammed = Some(jammed);
        self
    }

    /// Get whether the cashbox is removed
    ///
    /// If none is set, returns false
    pub fn cashbox_removed(&self) -> bool {
        if let Some(ret) = self.cashbox_removed {
            ret
        } else {
            false
        }
    }

    /// Set whether the cashbox is removed
    pub fn set_cashbox_removed(&mut self, removed: bool) {
        self.cashbox_removed = Some(removed);
    }

    /// Unset the cashbox removed status
    pub fn unset_cashbox_removed(&mut self) {
        self.cashbox_removed = None;
    }

    /// Get the firmware version
    ///
    /// If none is set, returns the empty string
    pub fn firmware_version(&self) -> &str {
        if let Some(ret) = self.firmware_version.as_ref() {
            ret
        } else {
            ""
        }
    }

    /// Set the firmware version
    pub fn set_firmware_version<S>(&mut self, version: S)
    where
        S: Into<String>,
    {
        self.firmware_version = Some(version.into());
    }

    /// Unset the firmware version
    pub fn unset_firmware_version(&mut self) {
        self.firmware_version = None
    }

    /// Get the BAU currency
    ///
    /// If none is set, returns the default currency (USD)
    pub fn currency(&self) -> Currency {
        if let Some(ret) = self.currency {
            ret
        } else {
            Currency::USD
        }
    }

    /// Set the BAU currency
    pub fn set_currency(&mut self, currency: Currency) {
        self.currency = Some(currency);
    }

    /// Unset the BAU currency
    pub fn unset_currency(&mut self) {
        self.currency = None;
    }

    /// Get whether the BAU is jammed
    ///
    /// If none is set, returns false
    pub fn jammed(&self) -> bool {
        if let Some(ret) = self.jammed {
            ret
        } else {
            false
        }
    }

    /// Set whether the BAU is jammed
    pub fn set_jammed(&mut self, jammed: bool) {
        self.jammed = Some(jammed);
    }

    /// Unset the jammed status
    pub fn unset_jammed(&mut self) {
        self.jammed = None;
    }
}

impl fmt::Display for BillAcceptorStatusDetails {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{OPEN_BRACE}")?;
        if let Some(ret) = self.cashbox_removed {
            write!(f, "\"cashbox_removed\":{}", ret)?;
        }
        if let Some(ret) = self.firmware_version.as_ref() {
            write!(f, ", \"firmware_version\":\"{}\"", ret)?;
        }
        if let Some(ret) = self.currency {
            write!(f, ", \"currency\":\"{}\"", ret)?;
        }
        if let Some(ret) = self.jammed {
            write!(f, ", \"jammed\":{}", ret)?;
        }
        write!(f, "{CLOSE_BRACE}")
    }
}

impl Serialize for BillAcceptorStatusDetails {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut bau_status = serializer.serialize_struct("BillAcceptorStatusDetails", 4)?;

        bau_status.serialize_field("cashbox_removed", &self.cashbox_removed)?;
        bau_status.serialize_field("firmware_version", &self.firmware_version)?;
        bau_status.serialize_field("currency", &self.currency)?;
        bau_status.serialize_field("jammed", &self.jammed)?;

        bau_status.end()
    }
}

/// HardwareStatusDetails represents the type of hardware details are provided for in
/// HardwareStatus
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub enum HardwareStatusDetails {
    NULL,
    BAU,
}

impl Default for HardwareStatusDetails {
    fn default() -> Self {
        Self::NULL
    }
}

impl From<HardwareStatusDetails> for &'static str {
    fn from(h: HardwareStatusDetails) -> Self {
        match h {
            HardwareStatusDetails::NULL => "NULL",
            HardwareStatusDetails::BAU => "BAU",
        }
    }
}

impl From<&HardwareStatusDetails> for &'static str {
    fn from(h: &HardwareStatusDetails) -> &'static str {
        (*h).into()
    }
}

impl fmt::Display for HardwareStatusDetails {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", <&'static str>::from(self))
    }
}

/// HardwareStatus represents basics information about the current status of hardware
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct HardwareStatus {
    // Hardware component type
    component: HardwareComponent,
    // Current state of the hardware
    state: HardwareState,
    // Basic description of the hardware status
    description: String,
    // Details of the BAU status
    details: BillAcceptorStatusDetails,
}

impl HardwareStatus {
    pub const fn new(
        component: HardwareComponent,
        state: HardwareState,
        description: String,
        details: BillAcceptorStatusDetails,
    ) -> Self {
        Self {
            component,
            state,
            description,
            details,
        }
    }

    pub const fn default() -> Self {
        Self {
            component: HardwareComponent::default(),
            state: HardwareState::default(),
            description: String::new(),
            details: BillAcceptorStatusDetails::default(),
        }
    }

    /// Get the hardware component
    pub fn component(&self) -> HardwareComponent {
        self.component
    }

    /// Set the hardware component
    pub fn set_component(&mut self, component: HardwareComponent) {
        self.component = component;
    }

    /// Get the hardware state
    pub fn state(&self) -> HardwareState {
        self.state
    }

    /// Set the hardware state
    pub fn set_state(&mut self, state: HardwareState) {
        self.state = state;
    }

    /// Set the "worst" state for the HardwareStatus
    /// (e.g. [HardwareState::Error](HardwareState) takes precedence over [HardwareState::OK](HardwareState))
    pub fn set_priority_state(&mut self, proposed: HardwareState) {
        if (proposed as u32) > (self.state as u32) {
            self.state = proposed;
        }
    }

    /// Get the hardware description
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Set the hardware description
    pub fn set_description<S>(&mut self, description: S)
    where
        S: Into<String>,
    {
        self.description = description.into();
    }

    /// Get the BAU status details
    pub fn details(&self) -> &BillAcceptorStatusDetails {
        &self.details
    }

    /// Get a mutable reference to the BAU status details
    pub fn details_mut(&mut self) -> &mut BillAcceptorStatusDetails {
        &mut self.details
    }

    /// Set the BAU status details
    pub fn set_details(&mut self, details: BillAcceptorStatusDetails) {
        self.details = details;
    }
}

impl fmt::Display for HardwareStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let component = self.component();
        let state = self.state();
        let description = self.description();
        let details = self.details();

        write!(f, "{OPEN_BRACE}\"component\":\"{component}\",\"state\":{state},\"description\":\"{description}\",\"details\":{details}{CLOSE_BRACE}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{self, Result};

    #[test]
    fn test_hardware_component_serde() -> Result<()> {
        assert_eq!(serde_json::to_string(&HardwareComponent::CDU)?, "\"CDU\"");
        assert_eq!(serde_json::to_string(&HardwareComponent::EPP)?, "\"EPP\"");
        assert_eq!(serde_json::to_string(&HardwareComponent::SIU)?, "\"SIU\"");
        assert_eq!(serde_json::to_string(&HardwareComponent::RPU)?, "\"RPU\"");
        assert_eq!(serde_json::to_string(&HardwareComponent::MCR)?, "\"MCR\"");
        assert_eq!(serde_json::to_string(&HardwareComponent::BAU)?, "\"BAU\"");
        assert_eq!(serde_json::to_string(&HardwareComponent::BA2)?, "\"BA2\"");
        assert_eq!(serde_json::to_string(&HardwareComponent::BCS)?, "\"BCS\"");
        assert_eq!(serde_json::to_string(&HardwareComponent::CAM)?, "\"CAM\"");
        assert_eq!(serde_json::to_string(&HardwareComponent::UPS)?, "\"UPS\"");

        assert_eq!(
            serde_json::from_str::<HardwareComponent>("\"CDU\"")?,
            HardwareComponent::CDU
        );
        assert_eq!(
            serde_json::from_str::<HardwareComponent>("\"EPP\"")?,
            HardwareComponent::EPP
        );
        assert_eq!(
            serde_json::from_str::<HardwareComponent>("\"SIU\"")?,
            HardwareComponent::SIU
        );
        assert_eq!(
            serde_json::from_str::<HardwareComponent>("\"RPU\"")?,
            HardwareComponent::RPU
        );
        assert_eq!(
            serde_json::from_str::<HardwareComponent>("\"MCR\"")?,
            HardwareComponent::MCR
        );
        assert_eq!(
            serde_json::from_str::<HardwareComponent>("\"BAU\"")?,
            HardwareComponent::BAU
        );
        assert_eq!(
            serde_json::from_str::<HardwareComponent>("\"BA2\"")?,
            HardwareComponent::BA2
        );
        assert_eq!(
            serde_json::from_str::<HardwareComponent>("\"BCS\"")?,
            HardwareComponent::BCS
        );
        assert_eq!(
            serde_json::from_str::<HardwareComponent>("\"CAM\"")?,
            HardwareComponent::CAM
        );
        assert_eq!(
            serde_json::from_str::<HardwareComponent>("\"UPS\"")?,
            HardwareComponent::UPS
        );

        Ok(())
    }

    #[test]
    fn test_hardware_state_serde() -> Result<()> {
        assert_eq!(serde_json::to_string(&HardwareState::OK)?, "\"OK\"");
        assert_eq!(
            serde_json::to_string(&HardwareState::Missing)?,
            "\"MISSING\""
        );
        assert_eq!(
            serde_json::to_string(&HardwareState::Warning)?,
            "\"WARNING\""
        );
        assert_eq!(serde_json::to_string(&HardwareState::Error)?, "\"ERROR\"");

        Ok(())
    }

    #[test]
    fn test_bill_acceptor_status_details_serde() -> Result<()> {
        let bau_status_filled = BillAcceptorStatusDetails {
            cashbox_removed: Some(true),
            firmware_version: Some("version-1.0".into()),
            currency: Some(Currency::USD),
            jammed: Some(false),
        };

        let expected = "{\"cashbox_removed\":true,\"firmware_version\":\"version-1.0\",\"currency\":\"USD\",\"jammed\":false}";

        assert_eq!(serde_json::to_string(&bau_status_filled)?, expected);

        let des_filled: BillAcceptorStatusDetails = serde_json::from_str(expected)?;

        assert_eq!(des_filled, bau_status_filled);

        let bau_status_sparse = BillAcceptorStatusDetails {
            cashbox_removed: None,
            firmware_version: Some("version-1.0".into()),
            currency: Some(Currency::USD),
            jammed: None,
        };

        let expected = "{\"cashbox_removed\":null,\"firmware_version\":\"version-1.0\",\"currency\":\"USD\",\"jammed\":null}";

        assert_eq!(serde_json::to_string(&bau_status_sparse)?, expected);

        let sparse_json = "{\"firmware_version\":\"version-1.0\",\"currency\":\"USD\"}";
        let des_sparse: BillAcceptorStatusDetails = serde_json::from_str(sparse_json)?;
        assert_eq!(des_sparse, bau_status_sparse);

        Ok(())
    }
}

#[cfg(feature = "std")]
/// Get the device path from the environment, or return the default path
pub fn get_device_path(env_key: &str, default_path: &str) -> String {
    std::env::var(env_key).unwrap_or(default_path.into())
}

#[cfg(not(feature = "std"))]
pub fn get_device_path(_env_key: &str, default_path: &str) -> String {
    default_path.into()
}
