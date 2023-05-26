use crate::std;
use std::fmt;

#[cfg(not(feature = "std"))]
use alloc::string::String;

/// Check digit for the [ProjectNumber].
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CheckDigit(u8);

impl CheckDigit {
    /// The length (in bytes) of the [CheckDigit].
    pub const LEN: usize = 1;

    /// Converts [CheckDigit] to a u8.
    pub const fn as_u8(&self) -> u8 {
        self.0
    }
}

impl From<u8> for CheckDigit {
    /// Parse the byte as an ASCII string.
    ///
    /// If a direct conversion from a u8 is wanted, use:
    ///
    /// let num = 0x0u8;
    /// let _ = CheckDigit(num);
    fn from(b: u8) -> Self {
        let digit = std::str::from_utf8(&[b])
            .unwrap_or("")
            .parse::<u8>()
            .unwrap_or(0xff);

        Self(digit)
    }
}

impl From<CheckDigit> for u8 {
    fn from(c: CheckDigit) -> Self {
        c.0
    }
}

impl From<&CheckDigit> for u8 {
    fn from(c: &CheckDigit) -> Self {
        (*c).into()
    }
}

/// The Application Part Number type.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PartNumberType {
    Type1 = 1,
    Type2 = 2,
    Variant = 3,
    Unknown = 0x00,
}

/// The part number is composed of a project number (5-6 digits) and version number (3 digits) with an
/// optional Check sum digit in the middle. Please see the following table for expected values.
///
/// | Project Number (5-6 bytes) | Check Digit (0-1 bytes) | Version (3 bytes) | Description |
/// |----------------------------|-------------------------|-------------------|-------------|
/// | 28000...28599              |                         |                   | Type 1 Application Part Number (Requires Check Digit) **CFSC Only** |
/// | 286000...289999            |                         |                   | Type 2 Application Part Number (No Check Digit) |
/// |                            | 0...9                   |                   | Check digit (Not applicable for Type 2 Application Part Numbers) **CFSC Only** |
/// |                            |                         | 000...999         | Formatted as V1.23 |
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProjectNumber {
    number: u32,
    check_digit: CheckDigit,
    part_type: PartNumberType,
}

impl ProjectNumber {
    /// The length (in bytes) of the [ProjectNumber].
    ///
    /// The length represents the ASCII string length, not the internal representation.
    pub const LEN: usize = 6;
    /// The length (in bytes) of a Type 1 Application Part Number.
    pub const TYPE_1_LEN: usize = 5;
    /// The length (in bytes) of a Type 2 Application Part Number.
    pub const TYPE_2_LEN: usize = 6;
    /// The length (in bytes) of a Variant Application Part Number.
    pub const VARIANT_LEN: usize = 5;
    /// The index of the Checksum Digit (invalid for Type 2 Application Part Number).
    pub const CHECK_DIGIT_IDX: usize = 5;

    /// Creates a Type 1 [ProjectNumber] from a number and [CheckDigit].
    pub const fn type1(number: u32, check_digit: CheckDigit) -> Self {
        Self {
            number,
            check_digit,
            part_type: PartNumberType::Type1,
        }
    }

    /// Creates a Type 2 [ProjectNumber].
    pub const fn type2(number: u32) -> Self {
        Self {
            number,
            check_digit: CheckDigit(0xff),
            part_type: PartNumberType::Type2,
        }
    }

    /// Creates a Variant [ProjectNumber] from a number and [CheckDigit].
    pub const fn variant(number: u32, check_digit: CheckDigit) -> Self {
        Self {
            number,
            check_digit,
            part_type: PartNumberType::Variant,
        }
    }

    /// Creates a zeroed [ProjectNumber].
    pub const fn zero() -> Self {
        Self {
            number: 0,
            check_digit: CheckDigit(0x00),
            part_type: PartNumberType::Unknown,
        }
    }

    /// Gets the application part number.
    pub const fn number(&self) -> u32 {
        self.number
    }

    /// Gets the check digit.
    pub const fn check_digit(&self) -> CheckDigit {
        self.check_digit
    }

    /// Gets the [PartNumberType].
    pub const fn part_type(&self) -> PartNumberType {
        self.part_type
    }
}

impl From<&[u8]> for ProjectNumber {
    fn from(b: &[u8]) -> Self {
        if b.len() < Self::LEN {
            return Self::zero();
        }

        let type1: u32 = std::str::from_utf8(b[..Self::TYPE_1_LEN].as_ref())
            .unwrap_or("")
            .parse::<u32>()
            .unwrap_or(0);

        let type2: u32 = std::str::from_utf8(b[..Self::TYPE_2_LEN].as_ref())
            .unwrap_or("")
            .parse::<u32>()
            .unwrap_or(0);

        if (28_000..=28_599).contains(&type1) {
            Self::type1(type1, CheckDigit::from(b[Self::CHECK_DIGIT_IDX]))
        } else if (49000..=49999).contains(&type1) || (51_000..=52_999).contains(&type1) {
            Self::variant(type1, CheckDigit::from(b[Self::CHECK_DIGIT_IDX]))
        } else if (286_000..=289_999).contains(&type2) {
            Self::type2(type2)
        } else {
            Self::zero()
        }
    }
}

impl fmt::Display for ProjectNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.part_type() {
            PartNumberType::Type1 => write!(
                f,
                "{} check digit: {}",
                self.number,
                self.check_digit.as_u8()
            ),
            PartNumberType::Type2 => write!(f, "{}", self.number),
            PartNumberType::Variant => write!(
                f,
                "{} check digit: {}",
                self.number,
                self.check_digit.as_u8()
            ),
            PartNumberType::Unknown => write!(f, "Unknown"),
        }
    }
}

/// The Boot Version number.
///
/// Formatted as the 3-digit ASCII string divided by one hundred.
///
/// Example:
///
/// ```rust
/// # use ebds::PartVersion;
/// let version = PartVersion::from(b"123");
/// let formatted_version = version.as_string();
/// assert_eq!(formatted_version, "V1.23");
/// ```
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PartVersion(u16);

impl PartVersion {
    /// The length (in bytes) of the [PartVersion].
    ///
    /// The length represents the ASCII string length, not the internal representation.
    pub const LEN: usize = 3;

    pub fn as_string(&self) -> String {
        format!("{self}")
    }
}

impl From<&[u8]> for PartVersion {
    fn from(b: &[u8]) -> Self {
        let version = std::str::from_utf8(b)
            .unwrap_or("")
            .parse::<u16>()
            .unwrap_or(0);

        if version > 999 {
            Self(0)
        } else {
            Self(version)
        }
    }
}

impl<const N: usize> From<[u8; N]> for PartVersion {
    fn from(b: [u8; N]) -> Self {
        b.as_ref().into()
    }
}

impl<const N: usize> From<&[u8; N]> for PartVersion {
    fn from(b: &[u8; N]) -> Self {
        b.as_ref().into()
    }
}

impl fmt::Display for PartVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "V{:.2}", (self.0 as f32) / 100f32)
    }
}

/// The boot part number from the device firmware.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BootPartNumber {
    project_number: ProjectNumber,
    version: PartVersion,
}

impl BootPartNumber {
    /// The length (in bytes) of the [BootPartNumber].
    pub const LEN: usize = 9;

    /// Creates a new [BootPartNumber].
    pub const fn new(project_number: ProjectNumber, version: PartVersion) -> Self {
        Self {
            project_number,
            version,
        }
    }

    pub const fn default() -> Self {
        Self {
            project_number: ProjectNumber::zero(),
            version: PartVersion(0),
        }
    }
}

impl From<&[u8]> for BootPartNumber {
    fn from(b: &[u8]) -> Self {
        if b.len() < Self::LEN {
            Self::default()
        } else {
            let len = std::cmp::min(b.len(), Self::LEN);

            let project_number: ProjectNumber = b[..ProjectNumber::LEN].as_ref().into();
            let version: PartVersion = b[ProjectNumber::LEN..len].as_ref().into();

            Self {
                project_number,
                version,
            }
        }
    }
}

impl fmt::Display for BootPartNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Project number: {}, Version: {}",
            self.project_number, self.version
        )
    }
}

/// The application part number from the device firmware.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ApplicationPartNumber {
    project_number: ProjectNumber,
    version: PartVersion,
}

impl ApplicationPartNumber {
    /// The length (in bytes) of the [ApplicationPartNumber].
    pub const LEN: usize = 9;

    /// Creates a new [ApplicationPartNumber].
    pub const fn new(project_number: ProjectNumber, version: PartVersion) -> Self {
        Self {
            project_number,
            version,
        }
    }

    pub const fn default() -> Self {
        Self {
            project_number: ProjectNumber::zero(),
            version: PartVersion(0),
        }
    }
}

impl From<&[u8]> for ApplicationPartNumber {
    fn from(b: &[u8]) -> Self {
        if b.len() < Self::LEN {
            Self::default()
        } else {
            let len = std::cmp::min(b.len(), Self::LEN);

            let project_number: ProjectNumber = b[..ProjectNumber::LEN].as_ref().into();
            let version: PartVersion = b[ProjectNumber::LEN..len].as_ref().into();

            Self {
                project_number,
                version,
            }
        }
    }
}

impl fmt::Display for ApplicationPartNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Project number: {}, Version: {}",
            self.project_number, self.version
        )
    }
}

/// The part number is composed of a project number (5-6 digits) and version number (3 digits) with an
/// optional Check sum digit in the middle. Please see the following table for expected values.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VariantPartNumber {
    project_number: ProjectNumber,
    version: PartVersion,
}

impl VariantPartNumber {
    /// The length (in bytes) of the [VariantPartNumber].
    pub const LEN: usize = 9;

    /// Creates a new [VariantPartNumber].
    pub const fn new(project_number: ProjectNumber, version: PartVersion) -> Self {
        Self {
            project_number,
            version,
        }
    }

    pub const fn default() -> Self {
        Self {
            project_number: ProjectNumber::zero(),
            version: PartVersion(0),
        }
    }
}

impl From<&[u8]> for VariantPartNumber {
    fn from(b: &[u8]) -> Self {
        if b.len() < Self::LEN {
            Self::default()
        } else {
            let len = std::cmp::min(b.len(), Self::LEN);

            let project_number: ProjectNumber = b[..ProjectNumber::LEN].as_ref().into();
            let version: PartVersion = b[ProjectNumber::LEN..len].as_ref().into();

            Self {
                project_number,
                version,
            }
        }
    }
}

impl fmt::Display for VariantPartNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Project number: {}, Version: {}",
            self.project_number, self.version
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boot_version_parsing() {
        let version = PartVersion::from(b"123");
        let formatted_version = version.as_string();
        assert_eq!(formatted_version, "V1.23");

        let version = PartVersion::from(b"23");
        let formatted_version = version.as_string();
        assert_eq!(formatted_version, "V0.23");

        let version = PartVersion::from(b"3");
        let formatted_version = version.as_string();
        assert_eq!(formatted_version, "V0.03");

        let version = PartVersion::from(b"");
        let formatted_version = version.as_string();
        assert_eq!(formatted_version, "V0.00");

        // Number is out of range
        let version = PartVersion::from(b"999888777");
        let formatted_version = version.as_string();
        assert_eq!(formatted_version, "V0.00");
    }
}
