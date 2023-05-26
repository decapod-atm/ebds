#[cfg(not(feature = "std"))]
use alloc::string::String;

use crate::std;
use std::fmt;

/// Cash denominations
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Denomination {
    Zero = 0,
    One = 1,
    Two = 2,
    Five = 5,
    Ten = 10,
    Twenty = 20,
    Fifty = 50,
    Hundred = 100,
    TwoHundred = 200,
    FiveHundred = 500,
    Thousand = 1000,
    TwoThousand = 2000,
    FiveThousand = 5000,
    TenThousand = 10_000,
    TwentyThousand = 20_000,
    FiftyThousand = 50_000,
    HundredThousand = 100_000,
}

impl From<Denomination> for &'static str {
    fn from(d: Denomination) -> Self {
        match d {
            Denomination::Zero => "Zero",
            Denomination::One => "One",
            Denomination::Two => "Two",
            Denomination::Five => "Five",
            Denomination::Ten => "Ten",
            Denomination::Twenty => "Twenty",
            Denomination::Fifty => "Fifty",
            Denomination::Hundred => "Hundred",
            Denomination::TwoHundred => "Two hundred",
            Denomination::FiveHundred => "Five hundred",
            Denomination::Thousand => "Thousand",
            Denomination::TwoThousand => "Two thousand",
            Denomination::FiveThousand => "Five thousand",
            Denomination::TenThousand => "Ten thousand",
            Denomination::TwentyThousand => "Twenty thousand",
            Denomination::FiftyThousand => "Fifty thousand",
            Denomination::HundredThousand => "Hundred thousand",
        }
    }
}

impl From<&Denomination> for &'static str {
    fn from(d: &Denomination) -> Self {
        (*d).into()
    }
}

impl fmt::Display for Denomination {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", <&'static str>::from(self))
    }
}

impl From<u32> for Denomination {
    fn from(digit: u32) -> Self {
        match digit {
            0 => Self::Zero,
            1 => Self::One,
            2 => Self::Two,
            5 => Self::Five,
            10 => Self::Ten,
            20 => Self::Twenty,
            50 => Self::Fifty,
            100 => Self::Hundred,
            200 => Self::TwoHundred,
            500 => Self::FiveHundred,
            1000 => Self::Thousand,
            2000 => Self::TwoThousand,
            5000 => Self::FiveThousand,
            10_000 => Self::TenThousand,
            20_000 => Self::TwentyThousand,
            50_000 => Self::FiftyThousand,
            100_000 => Self::HundredThousand,
            _ => Self::Zero,
        }
    }
}

impl From<Denomination> for u32 {
    fn from(d: Denomination) -> Self {
        d as u32
    }
}

bitfield! {
    /// Enable/disable note denominations while in base note mode
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct StandardDenomination(u8);
    u8;
    pub one, set_one: 0;
    pub two, set_two: 1;
    pub three, set_three: 2;
    pub four, set_four: 3;
    pub five, set_five: 4;
    pub six, set_six: 5;
    pub seven, set_seven: 6;
}

mod bitmask {
    pub const DENOMINATION: u8 = 0b111_1111;
}

impl StandardDenomination {
    /// Creates a [Denomination] with all denominations set.
    pub const fn all() -> Self {
        Self(bitmask::DENOMINATION)
    }

    /// Creates a [Denomination] with no denominations set.
    pub const fn none() -> Self {
        Self(0)
    }

    /// Converts from the [ExceptionStatus](crate::status::ExceptionStatus) `note_value` field.
    pub const fn from_note_value(note_value: u8) -> Self {
        match note_value {
            0b000 => Self::none(),
            0b001..=0b111 => Self(1 << (note_value - 1)),
            _ => Self::none(),
        }
    }

    /// Sets all denomintations.
    pub fn set_all(&mut self) {
        self.0 |= bitmask::DENOMINATION;
    }

    /// Inverts all the denomination bits.
    pub fn set_inverted(&mut self) {
        self.0 ^= bitmask::DENOMINATION;
    }

    /// Inverts all the denomination bits.
    pub fn invert(&self) -> Self {
        Self(self.0 ^ bitmask::DENOMINATION)
    }
}

fn denom_delimiter(has_denom: bool) -> &'static str {
    if has_denom {
        ","
    } else {
        ""
    }
}

impl fmt::Display for StandardDenomination {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut dis = String::new();
        let mut has_denom = false;

        if self.one() {
            dis += "Denom1";
            has_denom = true;
        }
        if self.two() {
            dis = dis + denom_delimiter(has_denom) + "Denom2";
            has_denom = true;
        }
        if self.three() {
            dis = dis + denom_delimiter(has_denom) + "Denom3";
            has_denom = true;
        }
        if self.four() {
            dis = dis + denom_delimiter(has_denom) + "Denom4";
            has_denom = true;
        }
        if self.five() {
            dis = dis + denom_delimiter(has_denom) + "Denom5";
            has_denom = true;
        }
        if self.six() {
            dis = dis + denom_delimiter(has_denom) + "Denom6";
            has_denom = true;
        }
        if self.seven() {
            dis = dis + denom_delimiter(has_denom) + "Denom7";
            has_denom = true;
        }

        if has_denom {
            write!(f, "{}", dis)
        } else {
            write!(f, "None")
        }
    }
}

impl From<StandardDenomination> for u8 {
    fn from(d: StandardDenomination) -> Self {
        d.0
    }
}

impl From<&StandardDenomination> for u8 {
    fn from(d: &StandardDenomination) -> Self {
        d.0
    }
}

impl From<u8> for StandardDenomination {
    fn from(b: u8) -> Self {
        Self(b & bitmask::DENOMINATION)
    }
}

impl From<StandardDenominationFlag> for StandardDenomination {
    fn from(f: StandardDenominationFlag) -> Self {
        Self(f as u8)
    }
}

/// Bit flags for [StandardDenomination]s.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StandardDenominationFlag {
    Denom1 = 0b000_0001,
    Denom2 = 0b000_0010,
    Denom3 = 0b000_0100,
    Denom4 = 0b000_1000,
    Denom5 = 0b001_0000,
    Denom6 = 0b010_0000,
    Denom7 = 0b100_0000,
    Zero = 0b000_0000,
}

impl StandardDenominationFlag {
    pub const fn default() -> Self {
        Self::Zero
    }
}

impl From<StandardDenomination> for StandardDenominationFlag {
    fn from(d: StandardDenomination) -> Self {
        // matches lowest value first
        if d.one() {
            Self::Denom1
        } else if d.two() {
            Self::Denom2
        } else if d.three() {
            Self::Denom3
        } else if d.four() {
            Self::Denom4
        } else if d.five() {
            Self::Denom5
        } else if d.six() {
            Self::Denom6
        } else if d.seven() {
            Self::Denom7
        } else {
            Self::Zero
        }
    }
}
