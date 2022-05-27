// ---------------------------------------------------------------------------
// Copyright:   (c) 2022 ff. Michael Amrhein (michael@adrhinum.de)
// License:     This program is part of a larger application. For license
//              details please read the file LICENSE.TXT provided together
//              with the application.
// ---------------------------------------------------------------------------
// $Source$
// $Revision$

use core::{
    cmp::min,
    ops::{Div, Mul},
};

use fpdec::{Decimal, DivRounded};
use fpdec_core::ten_pow;

use crate::{AmountT, Currency, Money, Quantity};

/// Basic representation of a conversion factor between two currencies.
///
/// An instance of `ExchangeRate` can be constructed by `fn new`, using the
/// following arguments:
/// * unit_currency: currency to be converted from, aka base currency
/// * unit_multiple: amount of base currency
/// * term_currency: currency to be converted to, aka price currency
/// * term_amount: equivalent amount of term currency
///
/// `unit_multiple` and `term_amount` will always be adjusted so that the
/// resulting unit multiple is a power to 10 and the resulting term amounts
/// magnitude is >= -1. The latter will always be rounded to 6 decimal
/// fractional digits.
///
/// Example:
///
/// ```rust
/// # use moneta::{ExchangeRate, EUR, USD, Dec, Decimal};
/// // 1 USD ≣ 0.9683 EUR =>
/// let usd_2_eur = ExchangeRate::new(USD, 1, EUR, Dec!(0.9683));
/// ```
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ExchangeRate {
    unit_currency: Currency,
    unit_multiple: u32,
    term_currency: Currency,
    term_amount: AmountT,
}

impl ExchangeRate {
    /// Returns a new instance of `ExchangeRate`.
    ///
    /// ### Panics
    /// The function panics in the following cases:
    /// * `unit_currency` is equal to `term_currency`.
    /// * `unit_multiple` == 0
    /// * `term_amount` <= 0
    /// * adjusted unit multiple > 1_000_000_000
    pub fn new(
        unit_currency: Currency,
        unit_multiple: u32,
        term_currency: Currency,
        term_amount: AmountT,
    ) -> Self {
        if unit_currency == term_currency {
            panic!("The currencies given must not be identical.")
        }
        if unit_multiple == 0 {
            panic!("Unit multiple must be >= 1.")
        }
        if !term_amount.is_positive() {
            panic!("Term amount must be > 0.")
        }
        // adjust unit_multiple and term_amount so that unit_multiple is a power
        // to 10 and term_amount.magnitude >= -1
        let magn = Decimal::from(unit_multiple).magnitude()
            - min(0, term_amount.magnitude() + 1);
        if magn < 0 || magn > 9 {
            panic!("Adjusted unit multiple must be <= 1_000_000_000.")
        }
        Self {
            unit_currency,
            unit_multiple: ten_pow(magn as u8) as u32,
            term_currency,
            term_amount: (term_amount * ten_pow(magn as u8))
                .div_rounded(unit_multiple, 6),
        }
    }

    /// Currency to be converted from, aka base currency
    #[inline(always)]
    pub fn unit_currency(&self) -> Currency {
        self.unit_currency
    }

    /// Amount of base currency
    #[inline(always)]
    pub fn unit_multiple(&self) -> u32 {
        self.unit_multiple
    }

    /// Currency to be converted to, aka price currency
    #[inline(always)]
    pub fn term_currency(&self) -> Currency {
        self.term_currency
    }

    /// Equivalent amount of term currency
    #[inline(always)]
    pub fn term_amount(&self) -> AmountT {
        self.term_amount
    }

    /// Relative value of term currency to unit currency
    #[inline(always)]
    pub fn rate(&self) -> AmountT {
        self.term_amount() / self.unit_multiple()
    }

    /// Inverted rate, i.e. relative value of unit currency to term currency
    #[inline(always)]
    pub fn inverse_rate(&self) -> AmountT {
        self.unit_multiple() / self.term_amount()
    }

    /// Returns a tuple of unit currency, term currency and rate.
    #[inline(always)]
    pub fn quotation(&self) -> (Currency, Currency, AmountT) {
        (self.unit_currency(), self.term_currency(), self.rate())
    }

    /// Returns a tuple of term currency, unit currency and inverse rate.
    #[inline(always)]
    pub fn inverse_quotation(&self) -> (Currency, Currency, AmountT) {
        (
            self.term_currency(),
            self.unit_currency(),
            self.inverse_rate(),
        )
    }

    /// Returns the inversion of `self`.
    pub fn inverted(&self) -> ExchangeRate {
        ExchangeRate::new(
            self.term_currency,
            1,
            self.unit_currency,
            self.inverse_rate(),
        )
    }
}

impl Mul<Money> for ExchangeRate {
    type Output = Money;

    /// Returns `Money` equivalent of `rhs` in term currency.
    ///
    /// ### Panics
    /// The function panics in the following cases:
    /// * Currency of `rhs` is not equal to `self.unit_currency`.
    fn mul(self, rhs: Money) -> Self::Output {
        if self.unit_currency() != rhs.unit() {
            panic!(
                "Can't divide '{}' by '{}'",
                rhs.unit(),
                self.unit_currency(),
            );
        }
        Self::Output::new(rhs.amount() * self.rate(), self.term_currency)
    }
}

impl Mul<ExchangeRate> for Money {
    type Output = Money;

    /// Returns `Money` equivalent of `self` in term currency.
    ///
    /// ### Panics
    /// The function panics in the following cases:
    /// * Currency of `self` is not equal to `rhs.unit_currency`.
    fn mul(self, rhs: ExchangeRate) -> Self::Output {
        if self.unit() != rhs.unit_currency() {
            panic!(
                "Can't divide '{}' by '{}'",
                self.unit(),
                rhs.unit_currency(),
            );
        }
        Self::Output::new(self.amount() * rhs.rate(), rhs.term_currency)
    }
}

impl Div<ExchangeRate> for Money {
    type Output = Money;

    /// Returns `Money` equivalent of `rhs` in unit currency.
    ///
    /// ### Panics
    /// The function panics in the following cases:
    /// * Currency of `self` is not equal to `rhs.term_currency`.
    fn div(self, rhs: ExchangeRate) -> Self::Output {
        if self.unit() != rhs.term_currency() {
            panic!(
                "Can't divide '{}' by '{}'",
                self.unit(),
                rhs.term_currency(),
            );
        }
        Self::Output::new(self.amount() / rhs.rate(), rhs.unit_currency)
    }
}

impl Mul<ExchangeRate> for ExchangeRate {
    type Output = Self;

    /// Returns "triangulated" exchange rate.
    ///
    /// ### Panics
    /// The function panics in the following cases:
    /// * No unit currency of a multiplicant does equal the term currency of the
    ///   other multiplicant.
    fn mul(self, rhs: ExchangeRate) -> Self::Output {
        if self.unit_currency() == rhs.term_currency() {
            return ExchangeRate::new(
                rhs.unit_currency(),
                1,
                self.term_currency(),
                self.rate() * rhs.rate(),
            );
        } else if self.term_currency() == rhs.unit_currency() {
            return ExchangeRate::new(
                self.unit_currency(),
                1,
                rhs.term_currency(),
                self.rate() * rhs.rate(),
            );
        } else {
            panic!(
                "Can't multiply '{}/{}' and '{}/{}'.",
                self.term_currency(),
                self.unit_currency(),
                rhs.term_currency(),
                rhs.unit_currency(),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Amnt, Dec, Decimal, EUR, USD};

    #[test]
    fn test_exchange_rate() {
        let rate = ExchangeRate::new(USD, 5, EUR, Amnt!(4.8307));
        assert_eq!(rate.unit_currency(), Currency::USD);
        assert_eq!(rate.term_currency(), Currency::EUR);
        assert_eq!(rate.unit_multiple(), 1);
        assert_eq!(rate.term_amount(), Amnt!(0.96614));
        assert_eq!(rate.rate(), Amnt!(0.96614));
        assert_eq!(rate.inverse_rate(), Amnt!(1.035046680605295299));
        assert_eq!(rate.quotation(), (USD, EUR, Amnt!(0.96614)));
        assert_eq!(
            rate.inverse_quotation(),
            (EUR, USD, Amnt!(1.035046680605295299))
        );
        assert_eq!(
            rate.inverted(),
            ExchangeRate::new(EUR, 1, USD, Amnt!(1.035047))
        );
    }

    #[test]
    #[should_panic]
    fn test_exchange_rate_same_curr() {
        let _r = ExchangeRate::new(USD, 1, USD, Amnt!(1));
    }

    #[test]
    #[should_panic]
    fn test_exchange_rate_zero_multiple() {
        let _r = ExchangeRate::new(USD, 0, EUR, Amnt!(1));
    }

    #[test]
    #[should_panic]
    fn test_exchange_rate_zero_term_amnt() {
        let _r = ExchangeRate::new(USD, 1, EUR, Amnt!(0));
    }
}
