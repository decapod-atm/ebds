#[cfg(not(feature = "std"))]
use alloc::string::String;

use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};

use crate::std;
use std::fmt;

use crate::{
    banknote::NoteTableItem,
    denomination::{Denomination, StandardDenomination, StandardDenominationFlag},
    method::Method,
    OPEN_BRACE, CLOSE_BRACE,
};

/// Container for cash inserted into the bill acceptor
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Deserialize)]
pub struct CashInsertionEvent {
    pub event: Method,
    pub amount: u32,
}

impl CashInsertionEvent {
    pub const fn new(event: Method, amount: u32) -> Self {
        Self { event, amount }
    }

    pub fn event(&self) -> Method {
        self.event
    }

    pub fn amount(&self) -> u32 {
        self.amount
    }
}

impl Default for CashInsertionEvent {
    fn default() -> Self {
        Self {
            event: Method::Accept,
            amount: 0,
        }
    }
}

impl fmt::Display for CashInsertionEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{OPEN_BRACE}\"event\":\"{}\",\"amount\":{}{CLOSE_BRACE}",
            self.event, self.amount
        )
    }
}

impl Serialize for CashInsertionEvent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut cash_insertion = serializer.serialize_struct("CashInsertionEvent", 2)?;

        cash_insertion.serialize_field("event", &self.event)?;
        cash_insertion.serialize_field("amount", &self.amount)?;

        cash_insertion.end()
    }
}

/// ISO 4217 codes: <https://en.wikipedia.org/wiki/ISO_4217>
///
/// Developers: add more codes as needed
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Deserialize)]
#[serde(field_identifier, rename_all = "UPPERCASE")]
pub enum Currency {
    /// Australian dollar
    AUD = 36,
    /// Armenian dram
    AMD = 51,
    /// Canadian dollar
    CAD = 124,
    /// Japanese yen
    JPY = 392,
    /// Euro
    EUR = 978,
    /// Great British pound
    GBP = 826,
    /// Mexican peso
    MXN = 484,
    /// Chinese renminbi
    CNY = 156,
    /// United States dollar
    USD = 840,
    /// No currency
    XXX = 999,
}

impl Currency {
    /// The length of the ASCII string, not the internal representation.
    pub const LEN: usize = 3;

    /// Gets the [Denomination] based on [Currency] and a base [StandardDenomination].
    pub fn denomination_value_base(&self, denom: StandardDenomination) -> Denomination {
        let denom_flag: StandardDenominationFlag = denom.into();
        match *self {
            Currency::USD => match denom_flag {
                StandardDenominationFlag::Denom1 => Denomination::One,
                StandardDenominationFlag::Denom2 => Denomination::Two,
                StandardDenominationFlag::Denom3 => Denomination::Five,
                StandardDenominationFlag::Denom4 => Denomination::Ten,
                StandardDenominationFlag::Denom5 => Denomination::Twenty,
                StandardDenominationFlag::Denom6 => Denomination::Fifty,
                StandardDenominationFlag::Denom7 => Denomination::Hundred,
                _ => Denomination::Zero,
            },
            Currency::CAD => match denom_flag {
                StandardDenominationFlag::Denom2 => Denomination::Five,
                StandardDenominationFlag::Denom3 => Denomination::Ten,
                StandardDenominationFlag::Denom4 => Denomination::Twenty,
                StandardDenominationFlag::Denom5 => Denomination::Fifty,
                StandardDenominationFlag::Denom6 => Denomination::Hundred,
                _ => Denomination::Zero,
            },
            Currency::GBP => match denom_flag {
                StandardDenominationFlag::Denom1 => Denomination::One,
                StandardDenominationFlag::Denom2 => Denomination::Five,
                StandardDenominationFlag::Denom3 => Denomination::Ten,
                StandardDenominationFlag::Denom4 => Denomination::Twenty,
                StandardDenominationFlag::Denom5 => Denomination::Fifty,
                _ => Denomination::Zero,
            },
            Currency::AMD => match denom_flag {
                StandardDenominationFlag::Denom1 => Denomination::Thousand,
                StandardDenominationFlag::Denom2 => Denomination::TwoThousand,
                StandardDenominationFlag::Denom3 => Denomination::FiveThousand,
                StandardDenominationFlag::Denom4 => Denomination::TenThousand,
                StandardDenominationFlag::Denom5 => Denomination::TwentyThousand,
                StandardDenominationFlag::Denom6 => Denomination::FiftyThousand,
                StandardDenominationFlag::Denom7 => Denomination::HundredThousand,
                _ => Denomination::Zero,
            },
            _ => Denomination::Zero,
        }
    }

    /// Gets the [Denomination] based on [Currency] and an extended [NoteTableItem].
    pub fn denomination_value_extended(&self, note: &NoteTableItem) -> Denomination {
        let code: &'static str = note.banknote().iso_code().into();
        let curr_str: &'static str = self.into();

        if curr_str == code {
            Denomination::from(note.banknote().value() as u32)
        } else {
            Denomination::Zero
        }
    }

    /// Creates the default variant for [Currency].
    pub const fn default() -> Self {
        Self::XXX
    }
}

impl From<&str> for Currency {
    fn from(currency: &str) -> Self {
        match currency {
            "USD" => Self::USD,
            "CNY" => Self::CNY,
            "GBP" => Self::GBP,
            "JPY" => Self::JPY,
            "EUR" => Self::EUR,
            "AUD" => Self::AUD,
            "CAD" => Self::CAD,
            "MXN" => Self::MXN,
            "AMD" => Self::AMD,
            _ => Self::XXX,
        }
    }
}

impl From<&String> for Currency {
    fn from(currency: &String) -> Self {
        currency.as_str().into()
    }
}

impl From<String> for Currency {
    fn from(currency: String) -> Self {
        (&currency).into()
    }
}

impl From<Currency> for &'static str {
    fn from(c: Currency) -> &'static str {
        match c {
            Currency::USD => "USD",
            Currency::CNY => "CNY",
            Currency::GBP => "GBP",
            Currency::JPY => "JPY",
            Currency::EUR => "EUR",
            Currency::AUD => "AUD",
            Currency::CAD => "CAD",
            Currency::MXN => "MXN",
            Currency::AMD => "AMD",
            _ => "XXX",
        }
    }
}

impl From<&Currency> for &'static str {
    fn from(c: &Currency) -> Self {
        (*c).into()
    }
}

impl From<&[u8]> for Currency {
    fn from(b: &[u8]) -> Self {
        if b.len() < Self::LEN {
            Self::default()
        } else {
            let iso = std::str::from_utf8(b[..Self::LEN].as_ref()).unwrap_or("XXX");
            iso.into()
        }
    }
}

impl<const N: usize> From<[u8; N]> for Currency {
    fn from(b: [u8; N]) -> Self {
        b.as_ref().into()
    }
}

impl<const N: usize> From<&[u8; N]> for Currency {
    fn from(b: &[u8; N]) -> Self {
        b.as_ref().into()
    }
}

impl From<u16> for Currency {
    fn from(b: u16) -> Self {
        match b {
            36 => Currency::AUD,
            51 => Currency::AMD,
            124 => Currency::CAD,
            392 => Currency::JPY,
            978 => Currency::EUR,
            826 => Currency::GBP,
            484 => Currency::MXN,
            156 => Currency::CNY,
            840 => Currency::USD,
            _ => Currency::XXX,
        }
    }
}

impl From<Currency> for u16 {
    fn from(c: Currency) -> Self {
        c as u16
    }
}

impl From<&Currency> for u16 {
    fn from(c: &Currency) -> Self {
        (*c).into()
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", <&'static str>::from(self))
    }
}

impl Serialize for Currency {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Self::USD => serializer.serialize_unit_variant("Currency", 0, "USD"),
            Self::CNY => serializer.serialize_unit_variant("Currency", 1, "CNY"),
            Self::GBP => serializer.serialize_unit_variant("Currency", 2, "GBP"),
            Self::JPY => serializer.serialize_unit_variant("Currency", 3, "JPY"),
            Self::EUR => serializer.serialize_unit_variant("Currency", 4, "EUR"),
            Self::AUD => serializer.serialize_unit_variant("Currency", 5, "AUD"),
            Self::CAD => serializer.serialize_unit_variant("Currency", 6, "CAD"),
            Self::MXN => serializer.serialize_unit_variant("Currency", 7, "MXN"),
            Self::AMD => serializer.serialize_unit_variant("Currency", 8, "AMD"),
            Self::XXX => serializer.serialize_unit_variant("Currency", 9, "XXX"),
        }
    }
}

/// Bill acceptor configuration
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub struct BillAcceptorConfig {
    /// The currency being accepted
    currency: Currency,
}

impl BillAcceptorConfig {
    /// Create a new BillAcceptorConfig
    pub const fn new(currency: Currency) -> Self {
        Self { currency }
    }

    pub fn currency(&self) -> Currency {
        self.currency
    }
}

impl Default for BillAcceptorConfig {
    fn default() -> Self {
        Self {
            currency: Currency::USD,
        }
    }
}

impl fmt::Display for BillAcceptorConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{OPEN_BRACE}\"currency\":\"{}\"{CLOSE_BRACE}", self.currency)
    }
}

/// Request to dispense bills
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub struct DispenseRequest {
    /// Amount to dispense
    amount: u32,
    /// Currency to dispense
    currency: Currency,
}

impl DispenseRequest {
    /// Create a new DispenseRequest
    pub const fn new(amount: u32, currency: Currency) -> Self {
        Self { amount, currency }
    }
}

impl fmt::Display for DispenseRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{OPEN_BRACE}\"amount\":{},\"currency\":\"{}\"{CLOSE_BRACE}",
            self.amount, self.currency
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{self, Result};

    #[test]
    fn test_cash_insertion_event_serde() -> Result<()> {
        let insertion_event = CashInsertionEvent {
            event: Method::Dispense,
            amount: 42,
        };
        let expected = "{\"event\":\"DISPENSE\",\"amount\":42}";

        assert_eq!(serde_json::to_string(&insertion_event)?, expected);
        assert_eq!(
            serde_json::from_str::<CashInsertionEvent>(expected)?,
            insertion_event
        );

        Ok(())
    }

    #[test]
    fn test_currency_serde() -> Result<()> {
        assert_eq!(serde_json::to_string(&Currency::USD)?, "\"USD\"");
        assert_eq!(serde_json::to_string(&Currency::CNY)?, "\"CNY\"");
        assert_eq!(serde_json::to_string(&Currency::GBP)?, "\"GBP\"");
        assert_eq!(serde_json::to_string(&Currency::JPY)?, "\"JPY\"");
        assert_eq!(serde_json::to_string(&Currency::AUD)?, "\"AUD\"");
        assert_eq!(serde_json::to_string(&Currency::EUR)?, "\"EUR\"");
        assert_eq!(serde_json::to_string(&Currency::CAD)?, "\"CAD\"");
        assert_eq!(serde_json::to_string(&Currency::MXN)?, "\"MXN\"");
        assert_eq!(serde_json::to_string(&Currency::AMD)?, "\"AMD\"");
        assert_eq!(serde_json::to_string(&Currency::XXX)?, "\"XXX\"");

        assert_eq!(serde_json::from_str::<Currency>("\"USD\"")?, Currency::USD);
        assert_eq!(serde_json::from_str::<Currency>("\"CNY\"")?, Currency::CNY);
        assert_eq!(serde_json::from_str::<Currency>("\"GBP\"")?, Currency::GBP);
        assert_eq!(serde_json::from_str::<Currency>("\"JPY\"")?, Currency::JPY);
        assert_eq!(serde_json::from_str::<Currency>("\"EUR\"")?, Currency::EUR);
        assert_eq!(serde_json::from_str::<Currency>("\"AUD\"")?, Currency::AUD);
        assert_eq!(serde_json::from_str::<Currency>("\"CAD\"")?, Currency::CAD);
        assert_eq!(serde_json::from_str::<Currency>("\"MXN\"")?, Currency::MXN);
        assert_eq!(serde_json::from_str::<Currency>("\"AMD\"")?, Currency::AMD);
        assert_eq!(serde_json::from_str::<Currency>("\"XXX\"")?, Currency::XXX);

        assert_eq!(Currency::from("WHaT_a_W3ird_CurRen$Y"), Currency::XXX);

        Ok(())
    }

    #[test]
    fn test_bill_acceptor_config_serde() -> Result<()> {
        let acceptor_cfg = BillAcceptorConfig {
            currency: Currency::USD,
        };
        let expected = "{\"currency\":\"USD\"}";

        assert_eq!(serde_json::to_string(&acceptor_cfg)?, expected);
        assert_eq!(
            serde_json::from_str::<BillAcceptorConfig>(expected)?,
            acceptor_cfg
        );

        Ok(())
    }

    #[test]
    fn test_dispense_request_serde() -> Result<()> {
        let dispense_req = DispenseRequest {
            amount: 42,
            currency: Currency::USD,
        };
        let expected = "{\"amount\":42,\"currency\":\"USD\"}";

        assert_eq!(serde_json::to_string(&dispense_req)?, expected);
        assert_eq!(
            serde_json::from_str::<DispenseRequest>(expected)?,
            dispense_req
        );

        Ok(())
    }
}
