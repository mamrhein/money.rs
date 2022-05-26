// ---------------------------------------------------------------------------
// Copyright:   (c) 2022 ff. Michael Amrhein (michael@adrhinum.de)
// License:     This program is part of a larger application. For license
//              details please read the file LICENSE.TXT provided together
//              with the application.
// ---------------------------------------------------------------------------
// $Source$
// $Revision$

use core::cmp::min;

use fpdec::{Decimal, DivRounded};
use fpdec_core::ten_pow;

use crate::{AmountT, Currency};

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
#[derive(Copy, Clone)]
pub struct ExchangeRate {
    unit_currency: Currency,
    unit_multiple: u32,
    term_currency: Currency,
    term_amount: AmountT,
}

impl ExchangeRate {
    /// Returns a new instance of `ExchangeRate`.
    ///
    /// Panics:
    /// * `unit_currency` is equal to `term_currency`.
    /// * `unit_multiple == 0` or `unit_multiple > 1000000000`
    /// * `term_amount` <= 0
    pub fn new(
        unit_currency: Currency,
        unit_multiple: u32,
        term_currency: Currency,
        term_amount: AmountT,
    ) -> Self {
        if unit_currency == term_currency {
            panic!("The currencies given must not be identical.")
        }
        if !term_amount.is_positive() {
            panic!("Term amount must be > 0.")
        }
        // adjust unit_multiple and term_amount so that unit_multiple is a power
        // to 10 and term_amount.magnitude >= -1
        let magn = Decimal::from(unit_multiple).magnitude()
            - min(0, term_amount.magnitude() + 1);
        if magn < 0 || magn > 9 {
            panic!("Adjusted unit multiple must be >= 1 and <= 1000000000.")
        }
        let mult = ten_pow(magn as u8) as u32;
        Self {
            unit_currency,
            unit_multiple: mult,
            term_currency,
            term_amount: (term_amount * mult).div_rounded(unit_multiple, 6),
        }
    }

    #[inline(always)]
    pub fn unit_currency(&self) -> Currency {
        self.unit_currency
    }

    #[inline(always)]
    pub fn unit_multiple(&self) -> u32 {
        self.unit_multiple
    }

    #[inline(always)]
    pub fn term_currency(&self) -> Currency {
        self.term_currency
    }

    #[inline(always)]
    pub fn term_amount(&self) -> AmountT {
        self.term_amount
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
        assert_eq!(rate.unit_multiple, 1);
        assert_eq!(rate.term_amount, Amnt!(0.96614));
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
