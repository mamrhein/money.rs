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

pub use currency::Currency;
use currency::CurrencyRegistry;
use currency_info::CurrencyKeyError;
pub use exchange::ExchangeRate;
use fpdec::{DivRounded, MulRounded, Round};
pub use iso_4217::ISOCurrency;
pub use quantities::{
    Amnt, AmountT, Dec, Decimal, Quantity, Rate, SIPrefix, Unit,
};

mod currency;
mod currency_info;
mod exchange;
mod iso_4217;

static CURRENCY_REGISTRY: CurrencyRegistry = CurrencyRegistry::new();

impl Currency {
    /// Returns a newly registered `Currency` with the given attributes.
    #[inline]
    pub fn new(
        symbol: &str,
        name: &str,
        minor_units: u8,
    ) -> Result<Currency, CurrencyKeyError> {
        CURRENCY_REGISTRY.register_currency(symbol, name, minor_units)
    }

    /// Returns the minor unit of `self`.
    #[inline]
    pub fn minor_unit(&self) -> u8 {
        CURRENCY_REGISTRY.get_currency_info(self).minor_unit
    }
}

impl Unit for Currency {
    type QuantityType = Money;

    #[inline]
    fn iter() -> impl Iterator<Item = Self> {
        CURRENCY_REGISTRY.get_registered_currencies().into_iter()
    }

    /// Returns `Some(curr)` where `curr.symbol()` == `symbol`, or `None` if
    /// there is no such currency.
    #[must_use]
    fn from_symbol(symbol: &str) -> Option<Self> {
        CURRENCY_REGISTRY.get_currency_from_symbol(symbol)
    }

    #[inline]
    fn name(&self) -> String {
        CURRENCY_REGISTRY.get_currency_info(self).name
    }

    #[inline]
    fn symbol(&self) -> String {
        CURRENCY_REGISTRY.get_currency_info(self).symbol
    }

    #[inline(always)]
    fn si_prefix(&self) -> Option<SIPrefix> {
        None
    }
}
//
// impl FromStr for Currency {
//     type Err = CurrencyKeyError;
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match CURRENCY_REGISTRY.get_currency_from_symbol(s) {
//             Some(c) => Ok(c),
//             None => Err(CurrencyKeyError(
//                 "No currency with symbol '{s}' registered.".to_string(),
//             )),
//         }
//     }
// }

impl fmt::Display for Currency {
    fn fmt(&self, form: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as Unit>::fmt(self, form)
    }
}

impl From<ISOCurrency> for Currency {
    fn from(curr: ISOCurrency) -> Self {
        match Currency::from_symbol(&curr.symbol()) {
            Some(c) => c,
            None => {
                Currency::new(curr.symbol(), curr.name(), curr.minor_unit())
                    .unwrap()
            }
        }
    }
}

impl Mul<Currency> for AmountT {
    type Output = Money;

    #[inline(always)]
    fn mul(self, rhs: Currency) -> Self::Output {
        Money::new(self, rhs)
    }
}

impl Mul<&Currency> for AmountT {
    type Output = Money;

    #[inline(always)]
    fn mul(self, rhs: &Currency) -> Self::Output {
        Money::new(self, *rhs)
    }
}

impl Mul<AmountT> for Currency {
    type Output = Money;

    #[inline(always)]
    fn mul(self, rhs: AmountT) -> Self::Output {
        Money::new(rhs, self)
    }
}

impl Mul<AmountT> for &Currency {
    type Output = Money;

    #[inline(always)]
    fn mul(self, rhs: AmountT) -> Self::Output {
        Money::new(rhs, *self)
    }
}

#[doc = "UAE Dirham"]
pub const AED: Currency = Currency::from_iso_curr(ISOCurrency::AED);
#[doc = "Afghani"]
pub const AFN: Currency = Currency::from_iso_curr(ISOCurrency::AFN);
#[doc = "Lek"]
pub const ALL: Currency = Currency::from_iso_curr(ISOCurrency::ALL);
#[doc = "Armenian Dram"]
pub const AMD: Currency = Currency::from_iso_curr(ISOCurrency::AMD);
#[doc = "Netherlands Antillean Guilder"]
pub const ANG: Currency = Currency::from_iso_curr(ISOCurrency::ANG);
#[doc = "Kwanza"]
pub const AOA: Currency = Currency::from_iso_curr(ISOCurrency::AOA);
#[doc = "Argentine Peso"]
pub const ARS: Currency = Currency::from_iso_curr(ISOCurrency::ARS);
#[doc = "Australian Dollar"]
pub const AUD: Currency = Currency::from_iso_curr(ISOCurrency::AUD);
#[doc = "Aruban Florin"]
pub const AWG: Currency = Currency::from_iso_curr(ISOCurrency::AWG);
#[doc = "Azerbaijan Manat"]
pub const AZN: Currency = Currency::from_iso_curr(ISOCurrency::AZN);
#[doc = "Convertible Mark"]
pub const BAM: Currency = Currency::from_iso_curr(ISOCurrency::BAM);
#[doc = "Barbados Dollar"]
pub const BBD: Currency = Currency::from_iso_curr(ISOCurrency::BBD);
#[doc = "Taka"]
pub const BDT: Currency = Currency::from_iso_curr(ISOCurrency::BDT);
#[doc = "Bulgarian Lev"]
pub const BGN: Currency = Currency::from_iso_curr(ISOCurrency::BGN);
#[doc = "Bahraini Dinar"]
pub const BHD: Currency = Currency::from_iso_curr(ISOCurrency::BHD);
#[doc = "Burundi Franc"]
pub const BIF: Currency = Currency::from_iso_curr(ISOCurrency::BIF);
#[doc = "Bermudian Dollar"]
pub const BMD: Currency = Currency::from_iso_curr(ISOCurrency::BMD);
#[doc = "Brunei Dollar"]
pub const BND: Currency = Currency::from_iso_curr(ISOCurrency::BND);
#[doc = "Boliviano"]
pub const BOB: Currency = Currency::from_iso_curr(ISOCurrency::BOB);
#[doc = "Mvdol"]
pub const BOV: Currency = Currency::from_iso_curr(ISOCurrency::BOV);
#[doc = "Brazilian Real"]
pub const BRL: Currency = Currency::from_iso_curr(ISOCurrency::BRL);
#[doc = "Bahamian Dollar"]
pub const BSD: Currency = Currency::from_iso_curr(ISOCurrency::BSD);
#[doc = "Ngultrum"]
pub const BTN: Currency = Currency::from_iso_curr(ISOCurrency::BTN);
#[doc = "Pula"]
pub const BWP: Currency = Currency::from_iso_curr(ISOCurrency::BWP);
#[doc = "Belarusian Ruble"]
pub const BYN: Currency = Currency::from_iso_curr(ISOCurrency::BYN);
#[doc = "Belize Dollar"]
pub const BZD: Currency = Currency::from_iso_curr(ISOCurrency::BZD);
#[doc = "Canadian Dollar"]
pub const CAD: Currency = Currency::from_iso_curr(ISOCurrency::CAD);
#[doc = "Congolese Franc"]
pub const CDF: Currency = Currency::from_iso_curr(ISOCurrency::CDF);
#[doc = "WIR Euro"]
pub const CHE: Currency = Currency::from_iso_curr(ISOCurrency::CHE);
#[doc = "Swiss Franc"]
pub const CHF: Currency = Currency::from_iso_curr(ISOCurrency::CHF);
#[doc = "WIR Franc"]
pub const CHW: Currency = Currency::from_iso_curr(ISOCurrency::CHW);
#[doc = "Unidad de Fomento"]
pub const CLF: Currency = Currency::from_iso_curr(ISOCurrency::CLF);
#[doc = "Chilean Peso"]
pub const CLP: Currency = Currency::from_iso_curr(ISOCurrency::CLP);
#[doc = "Yuan Renminbi"]
pub const CNY: Currency = Currency::from_iso_curr(ISOCurrency::CNY);
#[doc = "Colombian Peso"]
pub const COP: Currency = Currency::from_iso_curr(ISOCurrency::COP);
#[doc = "Unidad de Valor Real"]
pub const COU: Currency = Currency::from_iso_curr(ISOCurrency::COU);
#[doc = "Costa Rican Colon"]
pub const CRC: Currency = Currency::from_iso_curr(ISOCurrency::CRC);
#[doc = "Peso Convertible"]
pub const CUC: Currency = Currency::from_iso_curr(ISOCurrency::CUC);
#[doc = "Cuban Peso"]
pub const CUP: Currency = Currency::from_iso_curr(ISOCurrency::CUP);
#[doc = "Cabo Verde Escudo"]
pub const CVE: Currency = Currency::from_iso_curr(ISOCurrency::CVE);
#[doc = "Czech Koruna"]
pub const CZK: Currency = Currency::from_iso_curr(ISOCurrency::CZK);
#[doc = "Djibouti Franc"]
pub const DJF: Currency = Currency::from_iso_curr(ISOCurrency::DJF);
#[doc = "Danish Krone"]
pub const DKK: Currency = Currency::from_iso_curr(ISOCurrency::DKK);
#[doc = "Dominican Peso"]
pub const DOP: Currency = Currency::from_iso_curr(ISOCurrency::DOP);
#[doc = "Algerian Dinar"]
pub const DZD: Currency = Currency::from_iso_curr(ISOCurrency::DZD);
#[doc = "Egyptian Pound"]
pub const EGP: Currency = Currency::from_iso_curr(ISOCurrency::EGP);
#[doc = "Nakfa"]
pub const ERN: Currency = Currency::from_iso_curr(ISOCurrency::ERN);
#[doc = "Ethiopian Birr"]
pub const ETB: Currency = Currency::from_iso_curr(ISOCurrency::ETB);
#[doc = "Euro"]
pub const EUR: Currency = Currency::from_iso_curr(ISOCurrency::EUR);
#[doc = "Fiji Dollar"]
pub const FJD: Currency = Currency::from_iso_curr(ISOCurrency::FJD);
#[doc = "Falkland Islands Pound"]
pub const FKP: Currency = Currency::from_iso_curr(ISOCurrency::FKP);
#[doc = "Pound Sterling"]
pub const GBP: Currency = Currency::from_iso_curr(ISOCurrency::GBP);
#[doc = "Lari"]
pub const GEL: Currency = Currency::from_iso_curr(ISOCurrency::GEL);
#[doc = "Ghana Cedi"]
pub const GHS: Currency = Currency::from_iso_curr(ISOCurrency::GHS);
#[doc = "Gibraltar Pound"]
pub const GIP: Currency = Currency::from_iso_curr(ISOCurrency::GIP);
#[doc = "Dalasi"]
pub const GMD: Currency = Currency::from_iso_curr(ISOCurrency::GMD);
#[doc = "Guinean Franc"]
pub const GNF: Currency = Currency::from_iso_curr(ISOCurrency::GNF);
#[doc = "Quetzal"]
pub const GTQ: Currency = Currency::from_iso_curr(ISOCurrency::GTQ);
#[doc = "Guyana Dollar"]
pub const GYD: Currency = Currency::from_iso_curr(ISOCurrency::GYD);
#[doc = "Hong Kong Dollar"]
pub const HKD: Currency = Currency::from_iso_curr(ISOCurrency::HKD);
#[doc = "Lempira"]
pub const HNL: Currency = Currency::from_iso_curr(ISOCurrency::HNL);
#[doc = "Kuna"]
pub const HRK: Currency = Currency::from_iso_curr(ISOCurrency::HRK);
#[doc = "Gourde"]
pub const HTG: Currency = Currency::from_iso_curr(ISOCurrency::HTG);
#[doc = "Forint"]
pub const HUF: Currency = Currency::from_iso_curr(ISOCurrency::HUF);
#[doc = "Rupiah"]
pub const IDR: Currency = Currency::from_iso_curr(ISOCurrency::IDR);
#[doc = "New Israeli Sheqel"]
pub const ILS: Currency = Currency::from_iso_curr(ISOCurrency::ILS);
#[doc = "Indian Rupee"]
pub const INR: Currency = Currency::from_iso_curr(ISOCurrency::INR);
#[doc = "Iraqi Dinar"]
pub const IQD: Currency = Currency::from_iso_curr(ISOCurrency::IQD);
#[doc = "Iranian Rial"]
pub const IRR: Currency = Currency::from_iso_curr(ISOCurrency::IRR);
#[doc = "Iceland Krona"]
pub const ISK: Currency = Currency::from_iso_curr(ISOCurrency::ISK);
#[doc = "Jamaican Dollar"]
pub const JMD: Currency = Currency::from_iso_curr(ISOCurrency::JMD);
#[doc = "Jordanian Dinar"]
pub const JOD: Currency = Currency::from_iso_curr(ISOCurrency::JOD);
#[doc = "Yen"]
pub const JPY: Currency = Currency::from_iso_curr(ISOCurrency::JPY);
#[doc = "Kenyan Shilling"]
pub const KES: Currency = Currency::from_iso_curr(ISOCurrency::KES);
#[doc = "Som"]
pub const KGS: Currency = Currency::from_iso_curr(ISOCurrency::KGS);
#[doc = "Riel"]
pub const KHR: Currency = Currency::from_iso_curr(ISOCurrency::KHR);
#[doc = "Comorian Franc "]
pub const KMF: Currency = Currency::from_iso_curr(ISOCurrency::KMF);
#[doc = "North Korean Won"]
pub const KPW: Currency = Currency::from_iso_curr(ISOCurrency::KPW);
#[doc = "Won"]
pub const KRW: Currency = Currency::from_iso_curr(ISOCurrency::KRW);
#[doc = "Kuwaiti Dinar"]
pub const KWD: Currency = Currency::from_iso_curr(ISOCurrency::KWD);
#[doc = "Cayman Islands Dollar"]
pub const KYD: Currency = Currency::from_iso_curr(ISOCurrency::KYD);
#[doc = "Tenge"]
pub const KZT: Currency = Currency::from_iso_curr(ISOCurrency::KZT);
#[doc = "Lao Kip"]
pub const LAK: Currency = Currency::from_iso_curr(ISOCurrency::LAK);
#[doc = "Lebanese Pound"]
pub const LBP: Currency = Currency::from_iso_curr(ISOCurrency::LBP);
#[doc = "Sri Lanka Rupee"]
pub const LKR: Currency = Currency::from_iso_curr(ISOCurrency::LKR);
#[doc = "Liberian Dollar"]
pub const LRD: Currency = Currency::from_iso_curr(ISOCurrency::LRD);
#[doc = "Loti"]
pub const LSL: Currency = Currency::from_iso_curr(ISOCurrency::LSL);
#[doc = "Libyan Dinar"]
pub const LYD: Currency = Currency::from_iso_curr(ISOCurrency::LYD);
#[doc = "Moroccan Dirham"]
pub const MAD: Currency = Currency::from_iso_curr(ISOCurrency::MAD);
#[doc = "Moldovan Leu"]
pub const MDL: Currency = Currency::from_iso_curr(ISOCurrency::MDL);
#[doc = "Malagasy Ariary"]
pub const MGA: Currency = Currency::from_iso_curr(ISOCurrency::MGA);
#[doc = "Denar"]
pub const MKD: Currency = Currency::from_iso_curr(ISOCurrency::MKD);
#[doc = "Kyat"]
pub const MMK: Currency = Currency::from_iso_curr(ISOCurrency::MMK);
#[doc = "Tugrik"]
pub const MNT: Currency = Currency::from_iso_curr(ISOCurrency::MNT);
#[doc = "Pataca"]
pub const MOP: Currency = Currency::from_iso_curr(ISOCurrency::MOP);
#[doc = "Ouguiya"]
pub const MRU: Currency = Currency::from_iso_curr(ISOCurrency::MRU);
#[doc = "Mauritius Rupee"]
pub const MUR: Currency = Currency::from_iso_curr(ISOCurrency::MUR);
#[doc = "Rufiyaa"]
pub const MVR: Currency = Currency::from_iso_curr(ISOCurrency::MVR);
#[doc = "Malawi Kwacha"]
pub const MWK: Currency = Currency::from_iso_curr(ISOCurrency::MWK);
#[doc = "Mexican Peso"]
pub const MXN: Currency = Currency::from_iso_curr(ISOCurrency::MXN);
#[doc = "Mexican Unidad de Inversion (UDI)"]
pub const MXV: Currency = Currency::from_iso_curr(ISOCurrency::MXV);
#[doc = "Malaysian Ringgit"]
pub const MYR: Currency = Currency::from_iso_curr(ISOCurrency::MYR);
#[doc = "Mozambique Metical"]
pub const MZN: Currency = Currency::from_iso_curr(ISOCurrency::MZN);
#[doc = "Namibia Dollar"]
pub const NAD: Currency = Currency::from_iso_curr(ISOCurrency::NAD);
#[doc = "Naira"]
pub const NGN: Currency = Currency::from_iso_curr(ISOCurrency::NGN);
#[doc = "Cordoba Oro"]
pub const NIO: Currency = Currency::from_iso_curr(ISOCurrency::NIO);
#[doc = "Norwegian Krone"]
pub const NOK: Currency = Currency::from_iso_curr(ISOCurrency::NOK);
#[doc = "Nepalese Rupee"]
pub const NPR: Currency = Currency::from_iso_curr(ISOCurrency::NPR);
#[doc = "New Zealand Dollar"]
pub const NZD: Currency = Currency::from_iso_curr(ISOCurrency::NZD);
#[doc = "Rial Omani"]
pub const OMR: Currency = Currency::from_iso_curr(ISOCurrency::OMR);
#[doc = "Balboa"]
pub const PAB: Currency = Currency::from_iso_curr(ISOCurrency::PAB);
#[doc = "Sol"]
pub const PEN: Currency = Currency::from_iso_curr(ISOCurrency::PEN);
#[doc = "Kina"]
pub const PGK: Currency = Currency::from_iso_curr(ISOCurrency::PGK);
#[doc = "Philippine Peso"]
pub const PHP: Currency = Currency::from_iso_curr(ISOCurrency::PHP);
#[doc = "Pakistan Rupee"]
pub const PKR: Currency = Currency::from_iso_curr(ISOCurrency::PKR);
#[doc = "Zloty"]
pub const PLN: Currency = Currency::from_iso_curr(ISOCurrency::PLN);
#[doc = "Guarani"]
pub const PYG: Currency = Currency::from_iso_curr(ISOCurrency::PYG);
#[doc = "Qatari Rial"]
pub const QAR: Currency = Currency::from_iso_curr(ISOCurrency::QAR);
#[doc = "Romanian Leu"]
pub const RON: Currency = Currency::from_iso_curr(ISOCurrency::RON);
#[doc = "Serbian Dinar"]
pub const RSD: Currency = Currency::from_iso_curr(ISOCurrency::RSD);
#[doc = "Russian Ruble"]
pub const RUB: Currency = Currency::from_iso_curr(ISOCurrency::RUB);
#[doc = "Rwanda Franc"]
pub const RWF: Currency = Currency::from_iso_curr(ISOCurrency::RWF);
#[doc = "Saudi Riyal"]
pub const SAR: Currency = Currency::from_iso_curr(ISOCurrency::SAR);
#[doc = "Solomon Islands Dollar"]
pub const SBD: Currency = Currency::from_iso_curr(ISOCurrency::SBD);
#[doc = "Seychelles Rupee"]
pub const SCR: Currency = Currency::from_iso_curr(ISOCurrency::SCR);
#[doc = "Sudanese Pound"]
pub const SDG: Currency = Currency::from_iso_curr(ISOCurrency::SDG);
#[doc = "Swedish Krona"]
pub const SEK: Currency = Currency::from_iso_curr(ISOCurrency::SEK);
#[doc = "Singapore Dollar"]
pub const SGD: Currency = Currency::from_iso_curr(ISOCurrency::SGD);
#[doc = "Saint Helena Pound"]
pub const SHP: Currency = Currency::from_iso_curr(ISOCurrency::SHP);
#[doc = "Leone"]
pub const SLE: Currency = Currency::from_iso_curr(ISOCurrency::SLE);
#[doc = "Leone"]
pub const SLL: Currency = Currency::from_iso_curr(ISOCurrency::SLL);
#[doc = "Somali Shilling"]
pub const SOS: Currency = Currency::from_iso_curr(ISOCurrency::SOS);
#[doc = "Surinam Dollar"]
pub const SRD: Currency = Currency::from_iso_curr(ISOCurrency::SRD);
#[doc = "South Sudanese Pound"]
pub const SSP: Currency = Currency::from_iso_curr(ISOCurrency::SSP);
#[doc = "Dobra"]
pub const STN: Currency = Currency::from_iso_curr(ISOCurrency::STN);
#[doc = "El Salvador Colon"]
pub const SVC: Currency = Currency::from_iso_curr(ISOCurrency::SVC);
#[doc = "Syrian Pound"]
pub const SYP: Currency = Currency::from_iso_curr(ISOCurrency::SYP);
#[doc = "Lilangeni"]
pub const SZL: Currency = Currency::from_iso_curr(ISOCurrency::SZL);
#[doc = "Baht"]
pub const THB: Currency = Currency::from_iso_curr(ISOCurrency::THB);
#[doc = "Somoni"]
pub const TJS: Currency = Currency::from_iso_curr(ISOCurrency::TJS);
#[doc = "Turkmenistan New Manat"]
pub const TMT: Currency = Currency::from_iso_curr(ISOCurrency::TMT);
#[doc = "Tunisian Dinar"]
pub const TND: Currency = Currency::from_iso_curr(ISOCurrency::TND);
#[doc = "Pa’anga"]
pub const TOP: Currency = Currency::from_iso_curr(ISOCurrency::TOP);
#[doc = "Turkish Lira"]
pub const TRY: Currency = Currency::from_iso_curr(ISOCurrency::TRY);
#[doc = "Trinidad and Tobago Dollar"]
pub const TTD: Currency = Currency::from_iso_curr(ISOCurrency::TTD);
#[doc = "New Taiwan Dollar"]
pub const TWD: Currency = Currency::from_iso_curr(ISOCurrency::TWD);
#[doc = "Tanzanian Shilling"]
pub const TZS: Currency = Currency::from_iso_curr(ISOCurrency::TZS);
#[doc = "Hryvnia"]
pub const UAH: Currency = Currency::from_iso_curr(ISOCurrency::UAH);
#[doc = "Uganda Shilling"]
pub const UGX: Currency = Currency::from_iso_curr(ISOCurrency::UGX);
#[doc = "US Dollar"]
pub const USD: Currency = Currency::from_iso_curr(ISOCurrency::USD);
#[doc = "US Dollar (Next day)"]
pub const USN: Currency = Currency::from_iso_curr(ISOCurrency::USN);
#[doc = "Uruguay Peso en Unidades Indexadas (UI)"]
pub const UYI: Currency = Currency::from_iso_curr(ISOCurrency::UYI);
#[doc = "Peso Uruguayo"]
pub const UYU: Currency = Currency::from_iso_curr(ISOCurrency::UYU);
#[doc = "Unidad Previsional"]
pub const UYW: Currency = Currency::from_iso_curr(ISOCurrency::UYW);
#[doc = "Uzbekistan Sum"]
pub const UZS: Currency = Currency::from_iso_curr(ISOCurrency::UZS);
#[doc = "Bolívar Soberano"]
pub const VED: Currency = Currency::from_iso_curr(ISOCurrency::VED);
#[doc = "Bolívar Soberano"]
pub const VES: Currency = Currency::from_iso_curr(ISOCurrency::VES);
#[doc = "Dong"]
pub const VND: Currency = Currency::from_iso_curr(ISOCurrency::VND);
#[doc = "Vatu"]
pub const VUV: Currency = Currency::from_iso_curr(ISOCurrency::VUV);
#[doc = "Tala"]
pub const WST: Currency = Currency::from_iso_curr(ISOCurrency::WST);
#[doc = "CFA Franc BEAC"]
pub const XAF: Currency = Currency::from_iso_curr(ISOCurrency::XAF);
#[doc = "East Caribbean Dollar"]
pub const XCD: Currency = Currency::from_iso_curr(ISOCurrency::XCD);
#[doc = "CFA Franc BCEAO"]
pub const XOF: Currency = Currency::from_iso_curr(ISOCurrency::XOF);
#[doc = "CFP Franc"]
pub const XPF: Currency = Currency::from_iso_curr(ISOCurrency::XPF);
#[doc = "Yemeni Rial"]
pub const YER: Currency = Currency::from_iso_curr(ISOCurrency::YER);
#[doc = "Rand"]
pub const ZAR: Currency = Currency::from_iso_curr(ISOCurrency::ZAR);
#[doc = "Zambian Kwacha"]
pub const ZMW: Currency = Currency::from_iso_curr(ISOCurrency::ZMW);
#[doc = "Zimbabwe Dollar"]
pub const ZWL: Currency = Currency::from_iso_curr(ISOCurrency::ZWL);

/// Represents a money amount, i.e. the combination of a numerical value and a
/// money unit, aka. currency.
///
/// Instances of `Money` can be created by:
/// * calling fn `Money::new`, providing a numerical amount and a `Currency`,
///   or
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
#[cfg_attr(
    feature = "serde",
    derive(::serde::Deserialize, ::serde::Serialize)
)]
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
    /// Returns the numerical order of the amounts of the compared instances,
    /// if their `Currency` units are identical, otherwise `None`.
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
    /// `rhs` and with an amount equal to `self * rhs.amount()`, rounded to
    /// the number of fractional digits defined by `rhs.unit()`.
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
    /// `self` and with an amount equal to `self.amount() * rhs`, rounded to
    /// the number of fractional digits defined by `self.unit()`.
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
    /// `self` and with an amount equal to `self.amount() / rhs`, rounded to
    /// the number of fractional digits defined by `self.unit()`.
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
        assert_eq!(m.unit, EUR);
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
