use alloc::string::{String, ToString};

use crate::{
    std::{self, fmt},
    Currency,
};

pub type ISOCode = Currency;

/// A three character ASCII coded decimal value
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BaseValue(u16);

impl BaseValue {
    pub const LEN: usize = 3;

    pub const fn default() -> Self {
        Self(0)
    }
}

impl From<BaseValue> for u16 {
    fn from(b: BaseValue) -> Self {
        b.0
    }
}

impl From<&BaseValue> for u16 {
    fn from(b: &BaseValue) -> Self {
        (*b).into()
    }
}

impl From<BaseValue> for f32 {
    fn from(b: BaseValue) -> Self {
        b.0 as f32
    }
}

impl From<&BaseValue> for f32 {
    fn from(b: &BaseValue) -> Self {
        (*b).into()
    }
}

impl From<&[u8]> for BaseValue {
    fn from(b: &[u8]) -> Self {
        if b.len() < Self::LEN {
            Self(0)
        } else {
            // try to parse decimal value from the byte buffer
            // any failure results in a default (0) value
            let val = std::str::from_utf8(b[..Self::LEN].as_ref())
                .unwrap_or("0")
                .parse::<u16>()
                .unwrap_or(0);
            Self(val)
        }
    }
}

impl<const N: usize> From<[u8; N]> for BaseValue {
    fn from(b: [u8; N]) -> Self {
        b.as_ref().into()
    }
}

impl<const N: usize> From<&[u8; N]> for BaseValue {
    fn from(b: &[u8; N]) -> Self {
        b.as_ref().into()
    }
}

/// An ASCII  coded sign value for the Exponent.
/// This field is either a “+” or a “-“
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Sign {
    Positive,
    Negative,
}

impl Sign {
    pub const LEN: usize = 1;

    pub const fn default() -> Self {
        Self::Positive
    }
}

impl From<u8> for Sign {
    fn from(b: u8) -> Self {
        match b {
            b'+' => Self::Positive,
            b'-' => Self::Negative,
            _ => Self::Positive,
        }
    }
}

impl From<Sign> for &'static str {
    fn from(sign: Sign) -> Self {
        match sign {
            Sign::Negative => "-",
            Sign::Positive => "+",
        }
    }
}

/// ASCII coded decimal value for the power of ten
/// that the base is to either be multiplied by (if Sign
/// is “+”) or divided by (if Sign is “-“)
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Exponent(u8);

impl Exponent {
    pub const LEN: usize = 2;

    pub const fn default() -> Self {
        Self(1)
    }
}

impl From<u8> for Exponent {
    fn from(b: u8) -> Self {
        Self(b)
    }
}

impl From<Exponent> for u8 {
    fn from(e: Exponent) -> Self {
        e.0
    }
}

impl From<&Exponent> for u8 {
    fn from(e: &Exponent) -> Self {
        (*e).into()
    }
}

impl From<Exponent> for f32 {
    fn from(e: Exponent) -> Self {
        e.0 as f32
    }
}

impl From<&Exponent> for f32 {
    fn from(e: &Exponent) -> Self {
        (*e).into()
    }
}

impl From<&[u8]> for Exponent {
    fn from(b: &[u8]) -> Self {
        if b.len() < Exponent::LEN {
            Exponent(1)
        } else {
            let exp = std::str::from_utf8(b[..Exponent::LEN].as_ref())
                .unwrap_or("1")
                .parse::<u8>()
                .unwrap_or(1);
            Self(exp)
        }
    }
}

impl<const N: usize> From<[u8; N]> for Exponent {
    fn from(b: [u8; N]) -> Self {
        b.as_ref().into()
    }
}

impl<const N: usize> From<&[u8; N]> for Exponent {
    fn from(b: &[u8; N]) -> Self {
        b.as_ref().into()
    }
}

/// A single character binary field that encodes the
/// orientation of the bank note.
///
/// * 0x00 = Right Edge, Face Up
/// * 0x01 = Right Edge, Face Down
/// * 0x02 = Left Edge, Face Up
/// * 0x03 = Left Edge, Face Down
///
/// Note: In general, this field is only correct if the
/// Extended orientation bit is set in device
/// capabilities map.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BanknoteOrientation {
    RightEdgeFaceUp = 0x00,
    RightEdgeFaceDown = 0x01,
    LeftEdgeFaceUp = 0x02,
    LeftEdgeFaceDown = 0x03,
}

impl BanknoteOrientation {
    pub const LEN: usize = 1;
    pub const MASK: u8 = 0b11;

    pub const fn default() -> Self {
        Self::RightEdgeFaceUp
    }
}

impl From<u8> for BanknoteOrientation {
    fn from(b: u8) -> Self {
        match b & Self::MASK {
            0x00 => Self::RightEdgeFaceUp,
            0x01 => Self::RightEdgeFaceDown,
            0x02 => Self::LeftEdgeFaceUp,
            0x03 => Self::LeftEdgeFaceDown,
            // Computationally unreachable, but add the default case in the off chance the laws of
            // computation break
            //
            // Avoid the use of the `unreachable` macro because it panics if ever hit, and there is
            // a sane default here
            _ => Self::default(),
        }
    }
}

impl From<BanknoteOrientation> for &'static str {
    fn from(b: BanknoteOrientation) -> Self {
        match b {
            BanknoteOrientation::RightEdgeFaceUp => "Right edge face up",
            BanknoteOrientation::RightEdgeFaceDown => "Right edge face down",
            BanknoteOrientation::LeftEdgeFaceUp => "Left edge face up",
            BanknoteOrientation::LeftEdgeFaceDown => "Left edge face down",
        }
    }
}

impl From<&BanknoteOrientation> for &'static str {
    fn from(b: &BanknoteOrientation) -> Self {
        (*b).into()
    }
}

impl fmt::Display for BanknoteOrientation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bstr: &'static str = self.into();
        write!(f, "{bstr}")
    }
}

macro_rules! ascii_tuple_struct {
    ($name:ident, $base:tt, $doc:tt) => {
        #[doc = $doc]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub struct $name($base);

        impl $name {
            pub const LEN: usize = std::mem::size_of::<$base>();

            pub const fn default() -> Self {
                Self(0)
            }

            pub fn to_string(&self) -> String {
                std::str::from_utf8(&[self.0]).unwrap_or("").to_string()
            }
        }

        impl From<$base> for $name {
            fn from(b: $base) -> Self {
                Self(b)
            }
        }

        impl From<$name> for String {
            fn from(n: $name) -> String {
                n.to_string()
            }
        }

        impl From<&$name> for String {
            fn from(n: &$name) -> String {
                (*n).into()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.to_string())
            }
        }
    };
}

ascii_tuple_struct!(
    NoteType,
    u8,
    r"
 An ASCII letter that documents the note type.

 This corresponds to the data in the variant identity card.
"
);

ascii_tuple_struct!(
    NoteSeries,
    u8,
    r"
 An ASCII letter that documents the note series.

 This corresponds to the data in the variant identity card.
"
);

ascii_tuple_struct!(
    NoteCompatibility,
    u8,
    r"
 An ASCII letter that documents the revision of the recognition core used.

 This corresponds to the data in the variant identity card.
"
);

ascii_tuple_struct!(
    NoteVersion,
    u8,
    r"
 An ASCII letter that documents the version of the note's recognition criteria.

 This corresponds to the data in the variant identity card.
"
);

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BanknoteClassification {
    /// Sent for any the following:
    ///
    /// - In response to a Host Query Extended Note Specification Command (i.e. host requests a note table element).
    /// - In response to a note escrowed or stacked event while device is in extended note mode and classification is
    ///     - Supported by the device but disabled.
    ///     - NOT supported by the device.
    DisabledOrNotSupported = 0x00,
    /// Class 1 (unidentified banknote)
    Unidentified = 0x01,
    /// Class 2 (suspected counterfeit)
    SuspectedCounterfeit = 0x02,
    /// Class 3 (suspected zero value note)
    SuspectedZero = 0x03,
    /// Class 4 (genuine banknote)
    Genuine = 0x04,
}

impl BanknoteClassification {
    pub const fn default() -> Self {
        Self::DisabledOrNotSupported
    }
}

impl From<u8> for BanknoteClassification {
    fn from(b: u8) -> Self {
        match b {
            0x00 => Self::DisabledOrNotSupported,
            0x01 => Self::Unidentified,
            0x02 => Self::SuspectedCounterfeit,
            0x03 => Self::SuspectedZero,
            0x04 => Self::Genuine,
            _ => {
                log::trace!("Unknown banknote classification: 0x{b:x}");
                Self::default()
            }
        }
    }
}

impl From<BanknoteClassification> for &'static str {
    fn from(b: BanknoteClassification) -> Self {
        match b {
            BanknoteClassification::DisabledOrNotSupported => "Disabled or not supported",
            BanknoteClassification::Unidentified => "Unidentified",
            BanknoteClassification::SuspectedCounterfeit => "Suspected counterfeit",
            BanknoteClassification::SuspectedZero => "Suspected zero",
            BanknoteClassification::Genuine => "Genuine",
        }
    }
}

impl From<&BanknoteClassification> for &'static str {
    fn from(b: &BanknoteClassification) -> Self {
        (*b).into()
    }
}

impl fmt::Display for BanknoteClassification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", <&'static str>::from(self))
    }
}

/// The banknote value
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Banknote {
    /// The banknote value.
    pub(crate) value: f32,
    /// The banknote ISO code.
    pub(crate) iso_code: ISOCode,
    /// The banknote type.
    pub(crate) note_type: NoteType,
    /// The banknote series.
    pub(crate) note_series: NoteSeries,
    /// The banknote compatibility.
    pub(crate) note_compatibility: NoteCompatibility,
    /// The banknote version.
    pub(crate) note_version: NoteVersion,
    /// The banknote classification, see [BanknoteClassification].
    pub(crate) banknote_classification: BanknoteClassification,
}

impl Banknote {
    pub const fn new(
        value: f32,
        iso_code: ISOCode,
        note_type: NoteType,
        note_series: NoteSeries,
        note_compatibility: NoteCompatibility,
        note_version: NoteVersion,
        banknote_classification: BanknoteClassification,
    ) -> Self {
        Self {
            value,
            iso_code,
            note_type,
            note_series,
            note_compatibility,
            note_version,
            banknote_classification,
        }
    }

    pub const fn default() -> Self {
        Self {
            value: 0f32,
            iso_code: ISOCode::new(),
            note_type: NoteType::default(),
            note_series: NoteSeries::default(),
            note_compatibility: NoteCompatibility::default(),
            note_version: NoteVersion::default(),
            banknote_classification: BanknoteClassification::default(),
        }
    }

    /// Get the banknote value.
    pub fn value(&self) -> f32 {
        self.value
    }

    /// Set the banknote value.
    pub fn set_value(&mut self, value: u32) {
        self.value = value as f32;
    }

    /// Sets the [Banknote] value, consumes and returns the [Banknote].
    pub fn with_value(mut self, value: u32) -> Self {
        self.set_value(value);
        self
    }

    /// Get the banknote ISO code.
    pub fn iso_code(&self) -> ISOCode {
        self.iso_code
    }

    /// Set the banknote ISO code.
    pub fn set_iso_code(&mut self, iso_code: ISOCode) {
        self.iso_code = iso_code;
    }

    /// Get the banknote type.
    pub fn note_type(&self) -> NoteType {
        self.note_type
    }

    /// Set the banknote type.
    pub fn set_note_type(&mut self, note_type: NoteType) {
        self.note_type = note_type;
    }

    /// Get the banknote series.
    pub fn note_series(&self) -> NoteSeries {
        self.note_series
    }

    /// Set the banknote series.
    pub fn set_note_series(&mut self, note_series: NoteSeries) {
        self.note_series = note_series;
    }

    /// Get the banknote compatibility.
    pub fn note_compatibility(&self) -> NoteCompatibility {
        self.note_compatibility
    }

    /// Set the banknote compatibility.
    pub fn set_note_compatibility(&mut self, note_compatibility: NoteCompatibility) {
        self.note_compatibility = note_compatibility;
    }

    /// Get the banknote version.
    pub fn note_version(&self) -> NoteVersion {
        self.note_version
    }

    /// Set the banknote version.
    pub fn set_note_version(&mut self, note_version: NoteVersion) {
        self.note_version = note_version;
    }

    /// Get the banknote classification.
    pub fn banknote_classification(&self) -> BanknoteClassification {
        self.banknote_classification
    }

    /// Set the banknote classification.
    pub fn set_banknote_classification(&mut self, banknote_classification: BanknoteClassification) {
        self.banknote_classification = banknote_classification;
    }
}

impl fmt::Display for Banknote {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Value: {} ISO Code: {} Note Type: {} Note Series: {} Note Compatibility: {} Note Version: {} Banknote Classification: {}",
            self.value as u64,
            self.iso_code,
            self.note_type,
            self.note_series,
            self.note_compatibility,
            self.note_version,
            self.banknote_classification,
        )
    }
}

/// Extended note table item
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NoteTableItem {
    pub(crate) note_index: usize,
    pub(crate) banknote: Banknote,
}

impl NoteTableItem {
    /// Creates a new [NoteTableItem].
    pub const fn new(note_index: usize, banknote: Banknote) -> Self {
        Self {
            note_index,
            banknote,
        }
    }

    /// Creates a default (null) [NoteTableItem]
    pub const fn default() -> Self {
        Self {
            note_index: 0,
            banknote: Banknote::default(),
        }
    }

    /// Get whether the [NoteTableItem] is null, indicating the end of the note table
    pub fn is_null(&self) -> bool {
        self == &Self::default()
    }

    /// Get the note index.
    pub fn note_index(&self) -> usize {
        self.note_index
    }

    /// Set the note index.
    pub fn set_note_index(&mut self, note_index: usize) {
        self.note_index = note_index;
    }

    /// Get a reference to the banknote.
    pub fn banknote(&self) -> &Banknote {
        &self.banknote
    }

    /// Get a mutable reference to the banknote.
    pub fn banknote_mut(&mut self) -> &mut Banknote {
        &mut self.banknote
    }

    /// Set the banknote.
    pub fn set_banknote(&mut self, banknote: Banknote) {
        self.banknote = banknote;
    }
}

impl fmt::Display for NoteTableItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Index: {} {}", self.note_index, self.banknote)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iso_code() {
        let iso_aud = ISOCode::AUD;
        let iso_amd = ISOCode::AMD;
        let iso_cad = ISOCode::CAD;
        let iso_eur = ISOCode::EUR;
        let iso_gbp = ISOCode::GBP;
        let iso_mxn = ISOCode::MXN;
        let iso_cny = ISOCode::CNY;
        let iso_usd = ISOCode::USD;
        let iso_xxx = ISOCode::XXX;

        assert_eq!(<&str>::from(iso_aud), "AUD");
        assert_eq!(<&str>::from(iso_amd), "AMD");
        assert_eq!(<&str>::from(iso_cad), "CAD");
        assert_eq!(<&str>::from(iso_eur), "EUR");
        assert_eq!(<&str>::from(iso_gbp), "GBP");
        assert_eq!(<&str>::from(iso_mxn), "MXN");
        assert_eq!(<&str>::from(iso_cny), "CNY");
        assert_eq!(<&str>::from(iso_usd), "USD");
        assert_eq!(<&str>::from(iso_xxx), "XXX");

        assert_eq!(ISOCode::from("AUD"), iso_aud);
        assert_eq!(ISOCode::from("AMD"), iso_amd);
        assert_eq!(ISOCode::from("CAD"), iso_cad);
        assert_eq!(ISOCode::from("EUR"), iso_eur);
        assert_eq!(ISOCode::from("GBP"), iso_gbp);
        assert_eq!(ISOCode::from("MXN"), iso_mxn);
        assert_eq!(ISOCode::from("CNY"), iso_cny);
        assert_eq!(ISOCode::from("USD"), iso_usd);
        assert_eq!(ISOCode::from("XXX"), iso_xxx);
        assert_eq!(ISOCode::from(""), iso_xxx);

        assert_eq!(ISOCode::from(b"AUD"), iso_aud);
        assert_eq!(ISOCode::from(b"AMD"), iso_amd);
        assert_eq!(ISOCode::from(b"CAD"), iso_cad);
        assert_eq!(ISOCode::from(b"EUR"), iso_eur);
        assert_eq!(ISOCode::from(b"GBP"), iso_gbp);
        assert_eq!(ISOCode::from(b"MXN"), iso_mxn);
        assert_eq!(ISOCode::from(b"CNY"), iso_cny);
        assert_eq!(ISOCode::from(b"USD"), iso_usd);
        assert_eq!(ISOCode::from(b"XXX"), iso_xxx);
        assert_eq!(ISOCode::from(b""), iso_xxx);
    }

    #[test]
    fn test_base_value() {
        let base_value = BaseValue(42);

        assert_eq!(u16::from(base_value), 42);
        assert_eq!(f32::from(base_value), 42.0);
        assert_eq!(BaseValue::from(b"042"), base_value);

        // Check that values that are too short get parsed as the default value
        for i in 0..=u8::MAX {
            assert_eq!(BaseValue::from([i]), BaseValue::default());

            for j in 0..=u8::MAX {
                assert_eq!(BaseValue::from([i, j]), BaseValue::default());
            }
        }

        // Check that values that are too long only parse the first BaseValue::LEN bytes
        assert_eq!(BaseValue::from(b"042f"), base_value);
    }

    #[test]
    fn test_sign() {
        let sign_pos = Sign::Positive;
        let sign_neg = Sign::Negative;

        assert_eq!(Sign::from(b'+'), sign_pos);
        assert_eq!(Sign::from(b'-'), sign_neg);

        for b in 0..=u8::MAX {
            if b != b'-' {
                assert_eq!(Sign::from(b), sign_pos);
            }
        }

        assert_eq!(<&'static str>::from(sign_pos), "+");
        assert_eq!(<&'static str>::from(sign_neg), "-");
    }

    #[test]
    fn test_exponent() {
        let exp_max = Exponent(99);
        let exp_def = Exponent(1);
        let exp_min = Exponent(0);

        assert_eq!(Exponent::default(), exp_def);
        assert_eq!(Exponent::from(b"99"), exp_max);
        assert_eq!(Exponent::from(b"00"), exp_min);

        // Check that values that are too short are parsed as the default value
        assert_eq!(Exponent::from([]), exp_def);
        for i in 0..=u8::MAX {
            // Check that values that are too short are parsed as the default value
            assert_eq!(Exponent::from([i]), exp_def);

            for j in 0..=u8::MAX {
                if i == b'+' && (b'0'..=b'9').contains(&j) {
                    assert_eq!(
                        Exponent::from([i, j]),
                        Exponent(std::str::from_utf8(&[j]).unwrap().parse::<u8>().unwrap())
                    );
                } else if !(b'0'..=b'9').contains(&i) || !(b'0'..=b'9').contains(&j) {
                    // Check that values outside the valid range are parsed as the default value
                    assert_eq!(
                        Exponent::from([i, j]),
                        exp_def,
                        "i: {i}, j: {j}, string({})",
                        std::str::from_utf8(&[i, j]).unwrap()
                    );
                }
            }
        }
    }

    #[test]
    fn test_banknote_orientation() {
        assert_eq!(
            BanknoteOrientation::from(0x00),
            BanknoteOrientation::RightEdgeFaceUp
        );
        assert_eq!(
            BanknoteOrientation::from(0x01),
            BanknoteOrientation::RightEdgeFaceDown
        );
        assert_eq!(
            BanknoteOrientation::from(0x02),
            BanknoteOrientation::LeftEdgeFaceUp
        );
        assert_eq!(
            BanknoteOrientation::from(0x03),
            BanknoteOrientation::LeftEdgeFaceDown
        );

        // Check that values outside the base range are parsed as their masked values
        for i in 0x04..=u8::MAX {
            assert_eq!(
                BanknoteOrientation::from(i),
                BanknoteOrientation::from(i & BanknoteOrientation::MASK)
            );
        }
    }

    #[test]
    fn test_banknote_classification() {
        assert_eq!(
            BanknoteClassification::from(0x00),
            BanknoteClassification::DisabledOrNotSupported
        );
        assert_eq!(
            BanknoteClassification::from(0x01),
            BanknoteClassification::Unidentified
        );
        assert_eq!(
            BanknoteClassification::from(0x02),
            BanknoteClassification::SuspectedCounterfeit
        );
        assert_eq!(
            BanknoteClassification::from(0x03),
            BanknoteClassification::SuspectedZero
        );
        assert_eq!(
            BanknoteClassification::from(0x04),
            BanknoteClassification::Genuine
        );

        for i in 0x05..=u8::MAX {
            assert_eq!(
                BanknoteClassification::from(i),
                BanknoteClassification::default()
            );
        }
    }

    #[test]
    fn test_ascii_tuples() {
        let ascii_table = [
            b' ', b'!', b'"', b'#', b'$', b'%', b'&', b'\'', b'(', b')', b'*', b'+', b',', b'-',
            b'.', b'/', b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b':', b';',
            b'<', b'=', b'>', b'?', b'@', b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I',
            b'J', b'K', b'L', b'M', b'N', b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W',
            b'X', b'Y', b'Z', b'[', b'\\', b']', b'^', b'_', b'`', b'a', b'b', b'c', b'd', b'e',
            b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's',
            b't', b'u', b'v', b'w', b'x', b'y', b'z', b'{', b'|', b'}', b'~',
        ];

        for c in 0..=u8::MAX {
            if (32..=126).contains(&c) {
                let ascii_index = (c - 32) as usize;
                let ascii_val = ascii_table[ascii_index];

                // Check that printable ASCII characters are parsed as non-empty strings
                assert_eq!(NoteType::from(c).to_string().as_bytes()[0], ascii_val);
                assert_eq!(NoteSeries::from(c).to_string().as_bytes()[0], ascii_val);
                assert_eq!(
                    NoteCompatibility::from(c).to_string().as_bytes()[0],
                    ascii_val
                );
                assert_eq!(NoteVersion::from(c).to_string().as_bytes()[0], ascii_val);
            } else if c < 128 {
                // Check that non-printable ASCII characters are parsed as their unicode values
                assert!(!NoteType::from(c).to_string().is_empty());
                assert!(!NoteSeries::from(c).to_string().is_empty());
                assert!(!NoteCompatibility::from(c).to_string().is_empty());
                assert!(!NoteVersion::from(c).to_string().is_empty());
            } else {
                // Check that all other values are parsed as an empty string
                assert!(NoteType::from(c).to_string().is_empty());
                assert!(NoteSeries::from(c).to_string().is_empty());
                assert!(NoteCompatibility::from(c).to_string().is_empty());
                assert!(NoteVersion::from(c).to_string().is_empty());
            }
        }
    }
}
