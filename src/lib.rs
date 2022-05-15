// ---------------------------------------------------------------------------
// Copyright:   (c) 2021 ff. Michael Amrhein (michael@adrhinum.de)
// License:     This program is part of a larger application. For license
//              details please read the file LICENSE.TXT provided together
//              with the application.
// ---------------------------------------------------------------------------
// $Source$
// $Revision$

//! Definition of basic quantity `Money`.

#![doc = include_str ! ("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(dead_code)]

extern crate core;
extern crate quantities;

use quantities::prelude::*;

#[derive(Copy, Clone, Debug)]
pub struct Money {
    amount: AmountT,
    unit: Currency,
}

impl Quantity for Money {
    type UnitType = Currency;
    #[inline(always)]
    fn new(amount: AmountT, unit: Self::UnitType) -> Self {
        Self { amount, unit }
    }
    #[inline(always)]
    fn amount(&self) -> AmountT {
        self.amount
    }
    #[inline(always)]
    fn unit(&self) -> Self::UnitType {
        self.unit
    }
}

#[doc = "Unit of quantity `Money`."]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Currency {
    #[doc = "European Euro"]
    Euro,
    #[doc = "US Dollar"]
    UsDollar,
}

impl Currency {
    const VARIANTS: [Currency; 2usize] = [Currency::Euro, Currency::UsDollar];
}

impl Unit for Currency {
    type QuantityType = Money;

    fn iter<'a>() -> core::slice::Iter<'a, Self> {
        Self::VARIANTS.iter()
    }

    fn name(&self) -> &'static str {
        match self {
            Currency::Euro => "Euro",
            Currency::UsDollar => "US Dollar",
        }
    }

    fn symbol(&self) -> &'static str {
        match self {
            Currency::Euro => "EUR",
            Currency::UsDollar => "USD",
        }
    }

    fn si_prefix(&self) -> Option<SIPrefix> {
        None
    }
}

impl Eq for Money {}

impl PartialEq<Self> for Money {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        <Self as Quantity>::eq(self, other)
    }
}

impl PartialOrd for Money {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        <Self as Quantity>::partial_cmp(self, other)
    }
}

impl Add<Self> for Money {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        <Self as Quantity>::add(self, rhs)
    }
}

impl Sub<Self> for Money {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        <Self as Quantity>::sub(self, rhs)
    }
}

impl Div<Self> for Money {
    type Output = AmountT;
    #[inline(always)]
    fn div(self, rhs: Self) -> Self::Output {
        <Self as Quantity>::div(self, rhs)
    }
}

#[doc = "European Euro"]
pub const EUR: Currency = Currency::Euro;
#[doc = "US Dollar"]
pub const USD: Currency = Currency::UsDollar;

impl Mul<Currency> for AmountT {
    type Output = Money;

    #[inline(always)]
    fn mul(self, rhs: Currency) -> Self::Output {
        Money::new(self, rhs)
    }
}

impl Mul<AmountT> for Currency {
    type Output = Money;

    #[inline(always)]
    fn mul(self, rhs: AmountT) -> Self::Output {
        Money::new(rhs, self)
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as Quantity>::fmt(self, f)
    }
}

impl Mul<Money> for AmountT {
    type Output = Money;

    #[inline(always)]
    fn mul(self, rhs: Money) -> Self::Output {
        Self::Output::new(self * rhs.amount(), rhs.unit())
    }
}

impl Mul<AmountT> for Money {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: AmountT) -> Self::Output {
        Self::Output::new(self.amount() * rhs, self.unit())
    }
}

impl Div<AmountT> for Money {
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: AmountT) -> Self::Output {
        Self::Output::new(self.amount() / rhs, self.unit())
    }
}

impl<TQ: Quantity> Mul<Rate<TQ, Money>> for Money {
    type Output = TQ;

    fn mul(self, rhs: Rate<TQ, Money>) -> Self::Output {
        let amnt: AmountT =
            (self / rhs.per_unit().as_qty()) / rhs.per_unit_multiple();
        Self::Output::new(amnt * rhs.term_amount(), rhs.term_unit())
    }
}

impl<PQ: Quantity> Div<Rate<Money, PQ>> for Money {
    type Output = PQ;

    fn div(self, rhs: Rate<Money, PQ>) -> Self::Output {
        let amnt: AmountT =
            (self / rhs.term_unit().as_qty()) / rhs.term_amount();
        Self::Output::new(amnt * rhs.per_unit_multiple(), rhs.per_unit())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_money() {
        let amnt: AmountT = Amnt!(29.35);
        let m = amnt * EUR;
        assert_eq!(m.amount, amnt);
        assert_eq!(m.unit, Currency::Euro);
        #[cfg(feature = "std")]
        assert_eq!(m.to_string(), "29.35 EUR");
    }
}
