use crate::{
    banknote::NoteTableItem,
    denomination::{Denomination, StandardDenomination, StandardDenominationFlag},
    Currency,
};

pub trait CurrencyDenomination {
    /// Gets the [Denomination] based on [Currency] and a base [StandardDenomination].
    fn denomination_value_base(&self, denom: StandardDenomination) -> Denomination;

    /// Gets the [Denomination] based on [Currency] and an extended [NoteTableItem].
    fn denomination_value_extended(&self, note: &NoteTableItem) -> Denomination;
}

impl CurrencyDenomination for Currency {
    fn denomination_value_base(&self, denom: StandardDenomination) -> Denomination {
        let denom_flag: StandardDenominationFlag = denom.into();

        match self {
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

    fn denomination_value_extended(&self, note: &NoteTableItem) -> Denomination {
        let code = <&str>::from(note.banknote().iso_code());
        let curr_str = <&str>::from(self);

        if curr_str == code {
            Denomination::from(note.banknote().value() as u32)
        } else {
            Denomination::Zero
        }
    }
}
