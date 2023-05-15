// ---------------------------------------------------------------------------
// Copyright:   (c) 2021 ff. Michael Amrhein (michael@adrhinum.de)
// License:     This program is part of a larger application. For license
//              details please read the file LICENSE.TXT provided together
//              with the application.
// ---------------------------------------------------------------------------
// $Source$
// $Revision$

#![doc = include_str ! ("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]
// activate some rustc lints
#![deny(non_ascii_idents)]
#![deny(unsafe_code)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(trivial_casts)]
#![warn(unused)]
#![allow(dead_code)]
// activate some clippy lints
#![warn(clippy::cast_possible_truncation)]
#![warn(clippy::cast_possible_wrap)]
#![warn(clippy::cast_precision_loss)]
#![warn(clippy::cast_sign_loss)]
#![warn(clippy::cognitive_complexity)]
#![warn(clippy::decimal_literal_representation)]
#![warn(clippy::enum_glob_use)]
#![warn(clippy::equatable_if_let)]
#![warn(clippy::fallible_impl_from)]
#![warn(clippy::if_not_else)]
#![warn(clippy::if_then_some_else_none)]
#![warn(clippy::implicit_clone)]
#![warn(clippy::integer_division)]
#![warn(clippy::manual_assert)]
#![warn(clippy::match_same_arms)]
// #![warn(clippy::mismatching_type_param_order)] TODO: enable when got stable
#![warn(clippy::missing_const_for_fn)]
#![warn(clippy::missing_errors_doc)]
#![warn(clippy::missing_panics_doc)]
#![warn(clippy::multiple_crate_versions)]
#![warn(clippy::multiple_inherent_impl)]
#![warn(clippy::must_use_candidate)]
#![warn(clippy::needless_pass_by_value)]
#![warn(clippy::print_stderr)]
#![warn(clippy::print_stdout)]
#![warn(clippy::semicolon_if_nothing_returned)]
#![warn(clippy::str_to_string)]
#![warn(clippy::string_to_string)]
#![warn(clippy::undocumented_unsafe_blocks)]
#![warn(clippy::unicode_not_nfc)]
#![warn(clippy::unimplemented)]
#![warn(clippy::unseparated_literal_suffix)]
#![warn(clippy::unused_self)]
#![warn(clippy::unwrap_in_result)]
#![warn(clippy::use_self)]
#![warn(clippy::used_underscore_binding)]
#![warn(clippy::wildcard_imports)]

extern crate alloc;

use alloc::{format, string::String};
#[doc(hidden)]
pub use core::cmp::Ordering;
#[doc(hidden)]
pub use core::fmt;
#[doc(hidden)]
pub use core::ops::{Add, Div, Mul, Sub};

pub use exchange::ExchangeRate;
use fpdec::{DivRounded, MulRounded, Round};
pub use iso_4217::Currency;
pub use quantities::{
    Amnt, AmountT, Dec, Decimal, Quantity, Rate, SIPrefix, Unit,
};

mod exchange;
mod iso_4217;

#[doc = "UAE Dirham"]
pub const AED: Currency = Currency::AED;
#[doc = "Afghani"]
pub const AFN: Currency = Currency::AFN;
#[doc = "Lek"]
pub const ALL: Currency = Currency::ALL;
#[doc = "Armenian Dram"]
pub const AMD: Currency = Currency::AMD;
#[doc = "Netherlands Antillean Guilder"]
pub const ANG: Currency = Currency::ANG;
#[doc = "Kwanza"]
pub const AOA: Currency = Currency::AOA;
#[doc = "Argentine Peso"]
pub const ARS: Currency = Currency::ARS;
#[doc = "Australian Dollar"]
pub const AUD: Currency = Currency::AUD;
#[doc = "Aruban Florin"]
pub const AWG: Currency = Currency::AWG;
#[doc = "Azerbaijan Manat"]
pub const AZN: Currency = Currency::AZN;
#[doc = "Convertible Mark"]
pub const BAM: Currency = Currency::BAM;
#[doc = "Barbados Dollar"]
pub const BBD: Currency = Currency::BBD;
#[doc = "Taka"]
pub const BDT: Currency = Currency::BDT;
#[doc = "Bulgarian Lev"]
pub const BGN: Currency = Currency::BGN;
#[doc = "Bahraini Dinar"]
pub const BHD: Currency = Currency::BHD;
#[doc = "Burundi Franc"]
pub const BIF: Currency = Currency::BIF;
#[doc = "Bermudian Dollar"]
pub const BMD: Currency = Currency::BMD;
#[doc = "Brunei Dollar"]
pub const BND: Currency = Currency::BND;
#[doc = "Boliviano"]
pub const BOB: Currency = Currency::BOB;
#[doc = "Mvdol"]
pub const BOV: Currency = Currency::BOV;
#[doc = "Brazilian Real"]
pub const BRL: Currency = Currency::BRL;
#[doc = "Bahamian Dollar"]
pub const BSD: Currency = Currency::BSD;
#[doc = "Ngultrum"]
pub const BTN: Currency = Currency::BTN;
#[doc = "Pula"]
pub const BWP: Currency = Currency::BWP;
#[doc = "Belarusian Ruble"]
pub const BYN: Currency = Currency::BYN;
#[doc = "Belize Dollar"]
pub const BZD: Currency = Currency::BZD;
#[doc = "Canadian Dollar"]
pub const CAD: Currency = Currency::CAD;
#[doc = "Congolese Franc"]
pub const CDF: Currency = Currency::CDF;
#[doc = "WIR Euro"]
pub const CHE: Currency = Currency::CHE;
#[doc = "Swiss Franc"]
pub const CHF: Currency = Currency::CHF;
#[doc = "WIR Franc"]
pub const CHW: Currency = Currency::CHW;
#[doc = "Unidad de Fomento"]
pub const CLF: Currency = Currency::CLF;
#[doc = "Chilean Peso"]
pub const CLP: Currency = Currency::CLP;
#[doc = "Yuan Renminbi"]
pub const CNY: Currency = Currency::CNY;
#[doc = "Colombian Peso"]
pub const COP: Currency = Currency::COP;
#[doc = "Unidad de Valor Real"]
pub const COU: Currency = Currency::COU;
#[doc = "Costa Rican Colon"]
pub const CRC: Currency = Currency::CRC;
#[doc = "Peso Convertible"]
pub const CUC: Currency = Currency::CUC;
#[doc = "Cuban Peso"]
pub const CUP: Currency = Currency::CUP;
#[doc = "Cabo Verde Escudo"]
pub const CVE: Currency = Currency::CVE;
#[doc = "Czech Koruna"]
pub const CZK: Currency = Currency::CZK;
#[doc = "Djibouti Franc"]
pub const DJF: Currency = Currency::DJF;
#[doc = "Danish Krone"]
pub const DKK: Currency = Currency::DKK;
#[doc = "Dominican Peso"]
pub const DOP: Currency = Currency::DOP;
#[doc = "Algerian Dinar"]
pub const DZD: Currency = Currency::DZD;
#[doc = "Egyptian Pound"]
pub const EGP: Currency = Currency::EGP;
#[doc = "Nakfa"]
pub const ERN: Currency = Currency::ERN;
#[doc = "Ethiopian Birr"]
pub const ETB: Currency = Currency::ETB;
#[doc = "Euro"]
pub const EUR: Currency = Currency::EUR;
#[doc = "Fiji Dollar"]
pub const FJD: Currency = Currency::FJD;
#[doc = "Falkland Islands Pound"]
pub const FKP: Currency = Currency::FKP;
#[doc = "Pound Sterling"]
pub const GBP: Currency = Currency::GBP;
#[doc = "Lari"]
pub const GEL: Currency = Currency::GEL;
#[doc = "Ghana Cedi"]
pub const GHS: Currency = Currency::GHS;
#[doc = "Gibraltar Pound"]
pub const GIP: Currency = Currency::GIP;
#[doc = "Dalasi"]
pub const GMD: Currency = Currency::GMD;
#[doc = "Guinean Franc"]
pub const GNF: Currency = Currency::GNF;
#[doc = "Quetzal"]
pub const GTQ: Currency = Currency::GTQ;
#[doc = "Guyana Dollar"]
pub const GYD: Currency = Currency::GYD;
#[doc = "Hong Kong Dollar"]
pub const HKD: Currency = Currency::HKD;
#[doc = "Lempira"]
pub const HNL: Currency = Currency::HNL;
#[doc = "Kuna"]
pub const HRK: Currency = Currency::HRK;
#[doc = "Gourde"]
pub const HTG: Currency = Currency::HTG;
#[doc = "Forint"]
pub const HUF: Currency = Currency::HUF;
#[doc = "Rupiah"]
pub const IDR: Currency = Currency::IDR;
#[doc = "New Israeli Sheqel"]
pub const ILS: Currency = Currency::ILS;
#[doc = "Indian Rupee"]
pub const INR: Currency = Currency::INR;
#[doc = "Iraqi Dinar"]
pub const IQD: Currency = Currency::IQD;
#[doc = "Iranian Rial"]
pub const IRR: Currency = Currency::IRR;
#[doc = "Iceland Krona"]
pub const ISK: Currency = Currency::ISK;
#[doc = "Jamaican Dollar"]
pub const JMD: Currency = Currency::JMD;
#[doc = "Jordanian Dinar"]
pub const JOD: Currency = Currency::JOD;
#[doc = "Yen"]
pub const JPY: Currency = Currency::JPY;
#[doc = "Kenyan Shilling"]
pub const KES: Currency = Currency::KES;
#[doc = "Som"]
pub const KGS: Currency = Currency::KGS;
#[doc = "Riel"]
pub const KHR: Currency = Currency::KHR;
#[doc = "Comorian Franc "]
pub const KMF: Currency = Currency::KMF;
#[doc = "North Korean Won"]
pub const KPW: Currency = Currency::KPW;
#[doc = "Won"]
pub const KRW: Currency = Currency::KRW;
#[doc = "Kuwaiti Dinar"]
pub const KWD: Currency = Currency::KWD;
#[doc = "Cayman Islands Dollar"]
pub const KYD: Currency = Currency::KYD;
#[doc = "Tenge"]
pub const KZT: Currency = Currency::KZT;
#[doc = "Lao Kip"]
pub const LAK: Currency = Currency::LAK;
#[doc = "Lebanese Pound"]
pub const LBP: Currency = Currency::LBP;
#[doc = "Sri Lanka Rupee"]
pub const LKR: Currency = Currency::LKR;
#[doc = "Liberian Dollar"]
pub const LRD: Currency = Currency::LRD;
#[doc = "Loti"]
pub const LSL: Currency = Currency::LSL;
#[doc = "Libyan Dinar"]
pub const LYD: Currency = Currency::LYD;
#[doc = "Moroccan Dirham"]
pub const MAD: Currency = Currency::MAD;
#[doc = "Moldovan Leu"]
pub const MDL: Currency = Currency::MDL;
#[doc = "Malagasy Ariary"]
pub const MGA: Currency = Currency::MGA;
#[doc = "Denar"]
pub const MKD: Currency = Currency::MKD;
#[doc = "Kyat"]
pub const MMK: Currency = Currency::MMK;
#[doc = "Tugrik"]
pub const MNT: Currency = Currency::MNT;
#[doc = "Pataca"]
pub const MOP: Currency = Currency::MOP;
#[doc = "Ouguiya"]
pub const MRU: Currency = Currency::MRU;
#[doc = "Mauritius Rupee"]
pub const MUR: Currency = Currency::MUR;
#[doc = "Rufiyaa"]
pub const MVR: Currency = Currency::MVR;
#[doc = "Malawi Kwacha"]
pub const MWK: Currency = Currency::MWK;
#[doc = "Mexican Peso"]
pub const MXN: Currency = Currency::MXN;
#[doc = "Mexican Unidad de Inversion (UDI)"]
pub const MXV: Currency = Currency::MXV;
#[doc = "Malaysian Ringgit"]
pub const MYR: Currency = Currency::MYR;
#[doc = "Mozambique Metical"]
pub const MZN: Currency = Currency::MZN;
#[doc = "Namibia Dollar"]
pub const NAD: Currency = Currency::NAD;
#[doc = "Naira"]
pub const NGN: Currency = Currency::NGN;
#[doc = "Cordoba Oro"]
pub const NIO: Currency = Currency::NIO;
#[doc = "Norwegian Krone"]
pub const NOK: Currency = Currency::NOK;
#[doc = "Nepalese Rupee"]
pub const NPR: Currency = Currency::NPR;
#[doc = "New Zealand Dollar"]
pub const NZD: Currency = Currency::NZD;
#[doc = "Rial Omani"]
pub const OMR: Currency = Currency::OMR;
#[doc = "Balboa"]
pub const PAB: Currency = Currency::PAB;
#[doc = "Sol"]
pub const PEN: Currency = Currency::PEN;
#[doc = "Kina"]
pub const PGK: Currency = Currency::PGK;
#[doc = "Philippine Peso"]
pub const PHP: Currency = Currency::PHP;
#[doc = "Pakistan Rupee"]
pub const PKR: Currency = Currency::PKR;
#[doc = "Zloty"]
pub const PLN: Currency = Currency::PLN;
#[doc = "Guarani"]
pub const PYG: Currency = Currency::PYG;
#[doc = "Qatari Rial"]
pub const QAR: Currency = Currency::QAR;
#[doc = "Romanian Leu"]
pub const RON: Currency = Currency::RON;
#[doc = "Serbian Dinar"]
pub const RSD: Currency = Currency::RSD;
#[doc = "Russian Ruble"]
pub const RUB: Currency = Currency::RUB;
#[doc = "Rwanda Franc"]
pub const RWF: Currency = Currency::RWF;
#[doc = "Saudi Riyal"]
pub const SAR: Currency = Currency::SAR;
#[doc = "Solomon Islands Dollar"]
pub const SBD: Currency = Currency::SBD;
#[doc = "Seychelles Rupee"]
pub const SCR: Currency = Currency::SCR;
#[doc = "Sudanese Pound"]
pub const SDG: Currency = Currency::SDG;
#[doc = "Swedish Krona"]
pub const SEK: Currency = Currency::SEK;
#[doc = "Singapore Dollar"]
pub const SGD: Currency = Currency::SGD;
#[doc = "Saint Helena Pound"]
pub const SHP: Currency = Currency::SHP;
#[doc = "Leone"]
pub const SLE: Currency = Currency::SLE;
#[doc = "Leone"]
pub const SLL: Currency = Currency::SLL;
#[doc = "Somali Shilling"]
pub const SOS: Currency = Currency::SOS;
#[doc = "Surinam Dollar"]
pub const SRD: Currency = Currency::SRD;
#[doc = "South Sudanese Pound"]
pub const SSP: Currency = Currency::SSP;
#[doc = "Dobra"]
pub const STN: Currency = Currency::STN;
#[doc = "El Salvador Colon"]
pub const SVC: Currency = Currency::SVC;
#[doc = "Syrian Pound"]
pub const SYP: Currency = Currency::SYP;
#[doc = "Lilangeni"]
pub const SZL: Currency = Currency::SZL;
#[doc = "Baht"]
pub const THB: Currency = Currency::THB;
#[doc = "Somoni"]
pub const TJS: Currency = Currency::TJS;
#[doc = "Turkmenistan New Manat"]
pub const TMT: Currency = Currency::TMT;
#[doc = "Tunisian Dinar"]
pub const TND: Currency = Currency::TND;
#[doc = "Pa’anga"]
pub const TOP: Currency = Currency::TOP;
#[doc = "Turkish Lira"]
pub const TRY: Currency = Currency::TRY;
#[doc = "Trinidad and Tobago Dollar"]
pub const TTD: Currency = Currency::TTD;
#[doc = "New Taiwan Dollar"]
pub const TWD: Currency = Currency::TWD;
#[doc = "Tanzanian Shilling"]
pub const TZS: Currency = Currency::TZS;
#[doc = "Hryvnia"]
pub const UAH: Currency = Currency::UAH;
#[doc = "Uganda Shilling"]
pub const UGX: Currency = Currency::UGX;
#[doc = "US Dollar"]
pub const USD: Currency = Currency::USD;
#[doc = "US Dollar (Next day)"]
pub const USN: Currency = Currency::USN;
#[doc = "Uruguay Peso en Unidades Indexadas (UI)"]
pub const UYI: Currency = Currency::UYI;
#[doc = "Peso Uruguayo"]
pub const UYU: Currency = Currency::UYU;
#[doc = "Unidad Previsional"]
pub const UYW: Currency = Currency::UYW;
#[doc = "Uzbekistan Sum"]
pub const UZS: Currency = Currency::UZS;
#[doc = "Bolívar Soberano"]
pub const VED: Currency = Currency::VED;
#[doc = "Bolívar Soberano"]
pub const VES: Currency = Currency::VES;
#[doc = "Dong"]
pub const VND: Currency = Currency::VND;
#[doc = "Vatu"]
pub const VUV: Currency = Currency::VUV;
#[doc = "Tala"]
pub const WST: Currency = Currency::WST;
#[doc = "CFA Franc BEAC"]
pub const XAF: Currency = Currency::XAF;
#[doc = "East Caribbean Dollar"]
pub const XCD: Currency = Currency::XCD;
#[doc = "CFA Franc BCEAO"]
pub const XOF: Currency = Currency::XOF;
#[doc = "CFP Franc"]
pub const XPF: Currency = Currency::XPF;
#[doc = "Yemeni Rial"]
pub const YER: Currency = Currency::YER;
#[doc = "Rand"]
pub const ZAR: Currency = Currency::ZAR;
#[doc = "Zambian Kwacha"]
pub const ZMW: Currency = Currency::ZMW;
#[doc = "Zimbabwe Dollar"]
pub const ZWL: Currency = Currency::ZWL;

/// Represents a money amount, i.e. the combination of a numerical value and a
/// money unit, aka. currency.
///
/// Instances of `Money` can be created by:
/// * calling fn `Money::new`, providing a numerical amount and a `Currency`, or
/// * multiplying a numerical amount and a `Currency`.
///
/// The given amount is rounded to the number of fractional decimal digits
/// defined by the `Currency` unit (see `Currency.minor_unit()`.
///
/// Examples:
///
/// ```rust
/// # use moneta::{Dec, Decimal, Money, Quantity, EUR};
/// let m = Money::new(Dec!(38.5), EUR);
/// assert_eq!(m.to_string(), "38.50 EUR");
/// let m = Dec!(38.497) * EUR;
/// assert_eq!(m.to_string(), "38.50 EUR");
/// ```
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize, ::serde::Serialize))]
pub struct Money {
    amount: AmountT,
    unit: Currency,
}

impl Quantity for Money {
    type UnitType = Currency;

    /// Returns a new instance of `Money`.
    ///
    /// The given amount is rounded to the number of fractional decimal digits
    /// defined by the `Currency` unit.
    #[inline]
    #[allow(clippy::cast_possible_wrap)]
    fn new(amount: AmountT, unit: Self::UnitType) -> Self {
        Self {
            amount: amount.round(unit.minor_unit() as i8),
            unit,
        }
    }

    /// The numerical amount of `self`.
    #[inline(always)]
    fn amount(&self) -> AmountT {
        self.amount
    }

    /// The `Currency` unit of `self`.
    #[inline(always)]
    fn unit(&self) -> Self::UnitType {
        self.unit
    }
}

impl Eq for Money {}

impl PartialEq<Self> for Money {
    /// Two `Money` instances are equal, if they have the same `Currency` unit
    /// and their numerical amounts are equal.
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        <Self as Quantity>::eq(self, other)
    }
}

impl PartialOrd for Money {
    /// Returns the numerical order of the amounts of the compared instances, if
    /// their `Currency` units are identical, otherwise `None`.
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        <Self as Quantity>::partial_cmp(self, other)
    }
}

impl Add<Self> for Money {
    type Output = Self;

    /// Returns the sum of the two operands.
    ///
    /// Example:
    ///
    /// ```rust
    /// # use moneta::{Dec, Decimal, Money, Quantity, EUR};
    /// let x = Money::new(Dec!(38.5), EUR);
    /// let y = Dec!(8.397) * EUR;
    /// let z = x + y;
    /// assert_eq!(z.to_string(), "46.90 EUR")
    /// ```
    ///
    /// ### Panics
    /// The function panics in the following cases:
    /// * The operands have different `Currency` units.
    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        <Self as Quantity>::add(self, rhs)
    }
}

impl Sub<Self> for Money {
    type Output = Self;

    /// Returns the difference of the two operands.
    ///
    /// Example:
    ///
    /// ```rust
    /// # use moneta::{Dec, Decimal, Money, Quantity, EUR};
    /// let x = Money::new(Dec!(38.5), EUR);
    /// let y = Dec!(8.397) * EUR;
    /// let z = x - y;
    /// assert_eq!(z.to_string(), "30.10 EUR")
    /// ```
    ///
    /// ### Panics
    /// The function panics in the following cases:
    /// * The operands have different `Currency` units.
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        <Self as Quantity>::sub(self, rhs)
    }
}

impl Div<Self> for Money {
    type Output = AmountT;

    /// Returns the quotient of the two operands' amounts.
    ///
    /// Example:
    ///
    /// ```rust
    /// # use moneta::{Dec, Decimal, Money, Quantity, EUR};
    /// let x = Money::new(Dec!(63.75), EUR);
    /// let y = Dec!(8.5) * EUR;
    /// let z = x / y;
    /// assert_eq!(z.to_string(), "7.5")
    /// ```
    ///
    /// ### Panics
    /// The function panics in the following cases:
    /// * The operands have different `Currency` units.
    #[inline(always)]
    fn div(self, rhs: Self) -> Self::Output {
        <Self as Quantity>::div(self, rhs)
    }
}

impl fmt::Display for Money {
    /// Returns a formatted string representation of `self`.
    ///
    /// The given format spec is applied to the numerial amount of `self` and
    /// the resulting string is appended by a blank and the symbol of the
    /// `Currency` unit of `self`.
    ///
    /// Example:
    ///
    /// ```rust
    /// # use moneta::{Dec, Decimal, Money, Quantity, EUR};
    /// let m = Money::new(Dec!(38.5), EUR);
    /// assert_eq!(format!("{:>+12.2}", m), "  +38.50 EUR");
    /// ```
    fn fmt(&self, form: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tmp: String;
        let amnt_non_neg = self.amount().is_positive();
        let abs_amnt = self.amount().abs();
        if let Some(prec) = form.precision() {
            tmp = format!("{:.*} {}", prec, abs_amnt, self.unit());
        } else {
            tmp = format!(
                "{:.*} {}",
                self.unit().minor_unit() as usize,
                abs_amnt,
                self.unit()
            );
        }
        form.pad_integral(amnt_non_neg, "", &tmp)
    }
}

impl Mul<Money> for AmountT {
    type Output = Money;

    /// Returns a new instance of `Money`, with the same `Currency` unit as
    /// `rhs` and with an amount equal to `self * rhs.amount()`, rounded to the
    /// number of fractional digits defined by `rhs.unit()`.
    ///
    /// Example:
    ///
    /// ```rust
    /// # use moneta::{Dec, Decimal, Quantity, HKD};
    /// let x = Dec!(-1.5);
    /// let y = Dec!(730.04) * HKD;
    /// let z = x * y;
    /// assert_eq!(z.amount(), Dec!(-1095.06));
    /// assert_eq!(z.unit(), HKD);
    /// ```
    #[inline(always)]
    fn mul(self, rhs: Money) -> Self::Output {
        Self::Output::new(
            self.mul_rounded(rhs.amount(), rhs.unit().minor_unit()),
            rhs.unit(),
        )
    }
}

impl Mul<AmountT> for Money {
    type Output = Self;

    /// Returns a new instance of `Money`, with the same `Currency` unit as
    /// `self` and with an amount equal to `self.amount() * rhs`, rounded to the
    /// number of fractional digits defined by `self.unit()`.
    #[inline(always)]
    fn mul(self, rhs: AmountT) -> Self::Output {
        Self::Output::new(
            self.amount().mul_rounded(rhs, self.unit().minor_unit()),
            self.unit(),
        )
    }
}

impl Div<AmountT> for Money {
    type Output = Self;

    /// Returns a new instance of `Money`, with the same `Currency` unit as
    /// `self` and with an amount equal to `self.amount() / rhs`, rounded to the
    /// number of fractional digits defined by `self.unit()`.
    #[inline(always)]
    fn div(self, rhs: AmountT) -> Self::Output {
        Self::Output::new(
            self.amount().div_rounded(rhs, self.unit().minor_unit()),
            self.unit(),
        )
    }
}

impl<TQ: Quantity> Mul<Rate<TQ, Self>> for Money {
    type Output = TQ;

    /// Returns an instance of `TQ` eqivalent to `self` according to `rhs`.
    ///
    /// Example:
    ///
    /// ```rust
    /// # use moneta::{Dec, Decimal, Money, Quantity, USD};
    /// # use quantities::prelude::*;
    /// # #[quantity]
    /// # #[ref_unit(Kilogram, "kg", KILO, "Reference unit of quantity `Mass`")]
    /// # #[unit(Gram, "g", NONE, 0.001, "0.001·kg")]
    /// # struct Mass {}
    /// let d = Dec!(7.5) * USD;
    /// let r = Rate::<Mass, Money>::new(Dec!(3.5), KILOGRAM, Dec!(10), USD);
    /// let m = d * r;
    /// assert_eq!(m, Dec!(2.625) * KILOGRAM);
    /// ```
    ///
    /// ### Panics
    /// The function panics in the following cases:
    /// * `rhs.per_unit()` is not identical to `self.unit()`
    fn mul(self, rhs: Rate<TQ, Self>) -> Self::Output {
        let amnt: AmountT =
            (self / rhs.per_unit().as_qty()) / rhs.per_unit_multiple();
        Self::Output::new(amnt * rhs.term_amount(), rhs.term_unit())
    }
}

impl<PQ: Quantity> Div<Rate<Self, PQ>> for Money {
    type Output = PQ;

    /// Returns an instance of `PQ` eqivalent to `self` according to `rhs`.
    ///
    /// Example:
    ///
    /// ```rust
    /// # use moneta::{Dec, Decimal, Money, Quantity, USD};
    /// # use quantities::prelude::*;
    /// # #[quantity]
    /// # #[ref_unit(Kilogram, "kg", KILO, "Reference unit of quantity `Mass`")]
    /// # #[unit(Gram, "g", NONE, 0.001, "0.001·kg")]
    /// # struct Mass {}
    /// let d = Dec!(7.5) * USD;
    /// let r = Rate::<Money, Mass>::new(Dec!(3), USD, Dec!(10), KILOGRAM);
    /// let m = d / r;
    /// assert_eq!(m, Dec!(25) * KILOGRAM);
    /// ```
    ///
    /// ### Panics
    /// The function panics in the following cases:
    /// * `rhs.term_unit()` is not identical to `self.unit()`
    fn div(self, rhs: Rate<Self, PQ>) -> Self::Output {
        let amnt: AmountT =
            (self / rhs.term_unit().as_qty()) / rhs.term_amount();
        Self::Output::new(amnt * rhs.per_unit_multiple(), rhs.per_unit())
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;

    use super::*;

    #[test]
    fn test_money() {
        let amnt: AmountT = Amnt!(27.95);
        let m = amnt * EUR;
        assert_eq!(m.amount, amnt);
        assert_eq!(m.unit, Currency::EUR);
        assert_eq!(m.to_string(), "27.95 EUR");
    }

    #[test]
    #[allow(clippy::cast_possible_wrap)]
    fn test_money_exceed_minor_unit() {
        let amnt: AmountT = Amnt!(7.905);
        let m = amnt * EUR;
        assert_eq!(m.amount, amnt.round(EUR.minor_unit() as i8));
    }
}
