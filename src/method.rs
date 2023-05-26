use serde::{Deserialize, Serialize, Serializer};

use crate::std;
use std::fmt;

/// Method performed by the hardware
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Deserialize)]
#[serde(field_identifier, rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Method {
    /// Accept bills
    Accept,
    /// Stop current action
    Stop,
    /// Dispense bills
    Dispense,
    /// Stack bills
    Stack,
    /// Reject bills
    Reject,
    /// Get current status
    Status,
    /// Report escrow full
    EscrowFull,
    /// Reset the device
    Reset,
    /// Shutdown the socket connection
    Shutdown,
    /// Unknown method
    Unknown = 0xff,
}

impl Default for Method {
    fn default() -> Self {
        Self::Unknown
    }
}

impl Serialize for Method {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Self::Accept => serializer.serialize_unit_variant("Method", 0, "ACCEPT"),
            Self::Stop => serializer.serialize_unit_variant("Method", 1, "STOP"),
            Self::Dispense => serializer.serialize_unit_variant("Method", 2, "DISPENSE"),
            Self::Stack => serializer.serialize_unit_variant("Method", 3, "STACK"),
            Self::Reject => serializer.serialize_unit_variant("Method", 4, "REJECT"),
            Self::Status => serializer.serialize_unit_variant("Method", 5, "STATUS"),
            Self::EscrowFull => serializer.serialize_unit_variant("Method", 6, "ESCROW_FULL"),
            Self::Reset => serializer.serialize_unit_variant("Method", 7, "RESET"),
            Self::Shutdown => serializer.serialize_unit_variant("Method", 8, "SHUTDOWN"),
            Self::Unknown => serializer.serialize_unit_variant("Method", 0xff, "UNKNOWN"),
        }
    }
}

impl From<Method> for &'static str {
    fn from(m: Method) -> Self {
        match m {
            Method::Accept => "ACCEPT",
            Method::Stop => "STOP",
            Method::Dispense => "DISPENSE",
            Method::Stack => "STACK",
            Method::Reject => "REJECT",
            Method::Status => "STATUS",
            Method::EscrowFull => "ESCROW_FULL",
            Method::Reset => "RESET",
            Method::Shutdown => "SHUTDOWN",
            Method::Unknown => "UNKNOWN",
        }
    }
}

impl From<&Method> for &'static str {
    fn from(m: &Method) -> Self {
        (*m).into()
    }
}

impl From<&str> for Method {
    fn from(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "ACCEPT" => Self::Accept,
            "STOP" => Self::Stop,
            "DISPENSE" => Self::Dispense,
            "STACK" => Self::Stack,
            "REJECT" => Self::Reject,
            "STATUS" => Self::Status,
            "ESCROW_FULL" => Self::EscrowFull,
            "RESET" => Self::Reset,
            "SHUTDOWN" => Self::Shutdown,
            _ => Self::Unknown,
        }
    }
}

impl From<&[u8]> for Method {
    fn from(b: &[u8]) -> Self {
        std::str::from_utf8(b).unwrap_or("").into()
    }
}

impl<const N: usize> From<[u8; N]> for Method {
    fn from(b: [u8; N]) -> Self {
        b.as_ref().into()
    }
}

impl<const N: usize> From<&[u8; N]> for Method {
    fn from(b: &[u8; N]) -> Self {
        b.as_ref().into()
    }
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", <&'static str>::from(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{self, Result};

    #[test]
    fn test_method_serde() -> Result<()> {
        assert_eq!(serde_json::to_string(&Method::Accept)?, "\"ACCEPT\"");
        assert_eq!(serde_json::to_string(&Method::Stop)?, "\"STOP\"");
        assert_eq!(serde_json::to_string(&Method::Dispense)?, "\"DISPENSE\"");
        assert_eq!(serde_json::to_string(&Method::Stack)?, "\"STACK\"");
        assert_eq!(serde_json::to_string(&Method::Reject)?, "\"REJECT\"");
        assert_eq!(serde_json::to_string(&Method::Status)?, "\"STATUS\"");
        assert_eq!(serde_json::to_string(&Method::Shutdown)?, "\"SHUTDOWN\"");
        assert_eq!(
            serde_json::to_string(&Method::EscrowFull)?,
            "\"ESCROW_FULL\""
        );

        assert_eq!(
            serde_json::from_str::<Method>("\"ACCEPT\"")?,
            Method::Accept
        );
        assert_eq!(serde_json::from_str::<Method>("\"STOP\"")?, Method::Stop);
        assert_eq!(
            serde_json::from_str::<Method>("\"DISPENSE\"")?,
            Method::Dispense
        );
        assert_eq!(serde_json::from_str::<Method>("\"STACK\"")?, Method::Stack);
        assert_eq!(
            serde_json::from_str::<Method>("\"REJECT\"")?,
            Method::Reject
        );
        assert_eq!(
            serde_json::from_str::<Method>("\"STATUS\"")?,
            Method::Status
        );
        assert_eq!(
            serde_json::from_str::<Method>("\"ESCROW_FULL\"")?,
            Method::EscrowFull
        );
        assert_eq!(
            serde_json::from_str::<Method>("\"SHUTDOWN\"")?,
            Method::Shutdown
        );

        Ok(())
    }

    #[test]
    fn test_method_from_str() {
        // Check that upper-, lower-, and mixed-case method strings are parse correctly
        assert_eq!(Method::from("ACCEPT"), Method::Accept);
        assert_eq!(Method::from("accept"), Method::Accept);
        assert_eq!(Method::from("ACCept"), Method::Accept);

        assert_eq!(Method::from("STOP"), Method::Stop);
        assert_eq!(Method::from("stop"), Method::Stop);
        assert_eq!(Method::from("stOP"), Method::Stop);

        assert_eq!(Method::from("DISPENSE"), Method::Dispense);
        assert_eq!(Method::from("dispense"), Method::Dispense);
        assert_eq!(Method::from("disPENse"), Method::Dispense);

        assert_eq!(Method::from("STACK"), Method::Stack);
        assert_eq!(Method::from("stack"), Method::Stack);
        assert_eq!(Method::from("stACK"), Method::Stack);

        assert_eq!(Method::from("REJECT"), Method::Reject);
        assert_eq!(Method::from("reject"), Method::Reject);
        assert_eq!(Method::from("reJEct"), Method::Reject);

        assert_eq!(Method::from("STATUS"), Method::Status);
        assert_eq!(Method::from("status"), Method::Status);
        assert_eq!(Method::from("stAtus"), Method::Status);

        assert_eq!(Method::from("ESCROW_FULL"), Method::EscrowFull);
        assert_eq!(Method::from("escrow_full"), Method::EscrowFull);
        assert_eq!(Method::from("escrOW_fULl"), Method::EscrowFull);

        assert_eq!(Method::from("RESET"), Method::Reset);
        assert_eq!(Method::from("reset"), Method::Reset);
        assert_eq!(Method::from("reSet"), Method::Reset);

        assert_eq!(Method::from("SHUTDOWN"), Method::Shutdown);
        assert_eq!(Method::from("shutdown"), Method::Shutdown);
        assert_eq!(Method::from("SHutDown"), Method::Shutdown);

        assert_eq!(Method::from("UNKNOWN"), Method::Unknown);
        assert_eq!(Method::from("unknown"), Method::Unknown);
        assert_eq!(Method::from("UNknowN"), Method::Unknown);

        // All other strings should be parsed as unknown
        // Fuzz tests are better suited to exhaustively test random strings
        assert_eq!(Method::from("?@R@UI@(H"), Method::Unknown);
    }
}
