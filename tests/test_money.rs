// ---------------------------------------------------------------------------
// Copyright:   (c) 2022 ff. Michael Amrhein (michael@adrhinum.de)
// License:     This program is part of a larger application. For license
//              details please read the file LICENSE.TXT provided together
//              with the application.
// ---------------------------------------------------------------------------
// $Source$
// $Revision$

#[cfg(test)]
mod test_money_formatting {
    use moneta::{Dec, Decimal, Quantity, USD, UYW, VUV};

    #[test]
    fn test_unit_to_string() {
        assert_eq!(USD.to_string(), "USD");
    }

    #[test]
    fn test_unit_fmt() {
        assert_eq!(format!("{:^7}", UYW), "  UYW  ");
        assert_eq!(format!("{:>7.2}", VUV), "     VU");
    }

    #[test]
    fn test_money_to_string() {
        let val = Dec!(-18.95) * USD;
        assert_eq!(val.to_string(), "-18.95 USD");
        let val = Dec!(-18.95) * UYW;
        assert_eq!(val.to_string(), "-18.9500 UYW");
    }

    #[test]
    fn test_money_fmt() {
        let val = Dec!(2091.5) * USD;
        assert_eq!(format!("{:_>15}", val), "____2091.50 USD");
        assert_eq!(format!("{:<+15.0}", val), "+2092 USD      ");
        assert_eq!(format!("{:+15.4}", val), " +2091.5000 USD");
        let val = Dec!(295) * VUV;
        assert_eq!(format!("{:_>10}", val), "___295 VUV");
        assert_eq!(format!("{:_>10.1}", val), "_295.0 VUV");
        let val = Dec!(834.287) * UYW;
        assert_eq!(format!("{:+}", val), "+834.2870 UYW");
        assert_eq!(
            format!("{} {:+}", val.unit(), val.amount()),
            "UYW +834.287"
        );
        assert_eq!(
            format!("{} {:+12.4}", val.unit(), val.amount()),
            "UYW    +834.2870"
        );
    }
}

#[cfg(test)]
mod test_money_zero_sign {
    use moneta::{Dec, Decimal, Money, Quantity, EUR, USD, UYW, VUV};
    use quantities::prelude::*;

    // A generic quantity, used as the reference oracle: the crate-level
    // `Quantity` Display (in the `quantities` crate) is the behaviour
    // `Money`'s own Display is meant to mirror.
    #[quantity]
    #[ref_unit(Kilogram, "kg", KILO, "ref")]
    #[unit(Gram, "g", NONE, 0.001, "0.001 kg")]
    struct Mass {}

    // A zero amount has no sign, so its formatted form must never start
    // with '-', and the '+' flag must yield '+', not '-'.
    #[test]
    fn test_zero_has_no_negative_sign() {
        let zero = Money::new(Dec!(0.00), EUR);
        assert_eq!(zero.to_string(), "0.00 EUR");
        assert_eq!(format!("{}", zero), "0.00 EUR");
        assert_eq!(format!("{:+}", zero), "+0.00 EUR");
        assert_eq!(format!("{: }", zero), "0.00 EUR");
        // Minor unit other than 2 (UYW: 4, VUV: 0).
        assert_eq!((Dec!(0) * UYW).to_string(), "0.0000 UYW");
        assert_eq!(format!("{:+}", Dec!(0) * VUV), "+0 VUV");
    }

    // Zero reached by arithmetic must format identically to a literal zero;
    // the amount type has no negative zero to leak a stray sign.
    #[test]
    fn test_arithmetic_zero_has_no_sign() {
        let sub = Dec!(5.00) * EUR - Dec!(5.00) * EUR;
        assert_eq!(sub.to_string(), "0.00 EUR");
        let neg_times_zero = Dec!(-1) * (Dec!(0.00) * EUR);
        assert_eq!(neg_times_zero.to_string(), "0.00 EUR");
    }

    // Width, fill, precision and the '+' flag all compose correctly on zero.
    #[test]
    fn test_zero_with_format_spec() {
        let zero = Money::new(Dec!(0.00), EUR);
        assert_eq!(format!("{:*>+12.2}", zero), "***+0.00 EUR");
        assert_eq!(format!("{:>10.0}", zero), "     0 EUR");
        assert_eq!(format!("{:<+15.4}", zero), "+0.0000 EUR    ");
    }

    // Non-zero values keep their sign (regression guard for the fix).
    #[test]
    fn test_nonzero_sign_unchanged() {
        assert_eq!((Dec!(12.50) * EUR).to_string(), "12.50 EUR");
        assert_eq!((Dec!(-12.50) * EUR).to_string(), "-12.50 EUR");
        assert_eq!(format!("{:+}", Dec!(12.50) * EUR), "+12.50 EUR");
        // A truly negative amount that rounds to zero only at the requested
        // display precision keeps its sign (taken from the real value).
        let small_neg = Money::new(Dec!(-0.01), EUR);
        assert_eq!(format!("{:.1}", small_neg), "-0.0 EUR");
        assert_eq!(format!("{:.0}", small_neg), "-0 EUR");
    }

    // Leading sign character produced by a formatted value, or None.
    fn sign_of(s: &str) -> Option<char> {
        match s.chars().next() {
            Some(c @ ('+' | '-')) => Some(c),
            _ => None,
        }
    }

    // `Money` Display must agree with the generic `Quantity` Display on the
    // sign it emits for every amount, with and without the '+' flag.
    #[test]
    fn test_sign_parity_with_generic_quantity() {
        let amounts = [Dec!(0), Dec!(12.50), Dec!(-12.50), Dec!(0.30)];
        for a in amounts {
            let money = a * USD;
            let qty = a * KILOGRAM;
            assert_eq!(
                sign_of(&format!("{}", money)),
                sign_of(&format!("{}", qty)),
                "plain sign mismatch for {a}"
            );
            assert_eq!(
                sign_of(&format!("{:+}", money)),
                sign_of(&format!("{:+}", qty)),
                "'+' sign mismatch for {a}"
            );
        }
    }
}

#[cfg(test)]
mod test_money_ops {
    use moneta::{Dec, Decimal, Quantity, USD, UYW};

    #[test]
    fn test_add_sub_same_currency() {
        let x = Dec!(27.4) * USD;
        let y = Dec!(35.89) * USD;
        let z = x + y;
        assert_eq!(z.amount(), Dec!(63.29));
        assert_eq!(z.unit(), USD);
        assert_eq!(x + y, y + x);
        let z = x - y;
        assert_eq!(z.amount(), Dec!(-8.49));
        assert_eq!(z.unit(), USD);
    }

    #[test]
    #[should_panic]
    fn test_add_diff_currency() {
        let x = Dec!(27.4) * USD;
        let y = Dec!(35.89) * UYW;
        let _z = x + y;
    }

    #[test]
    #[should_panic]
    fn test_sub_diff_currency() {
        let x = Dec!(27.4) * USD;
        let y = Dec!(35.89) * UYW;
        let _z = x - y;
    }

    #[test]
    fn test_mul_amnt_money() {
        let x = Dec!(7.5);
        let y = Dec!(23.85) * USD;
        let z = x * y;
        assert_eq!(z.amount(), Dec!(178.88));
        assert_eq!(z.unit(), USD);
        assert_eq!(x * y, y * x);
        let y = Dec!(23.85) * UYW;
        let z = x * y;
        assert_eq!(z.amount(), Dec!(178.875));
        assert_eq!(z.unit(), UYW);
    }

    #[test]
    fn test_div_money_amnt() {
        let x = Dec!(700.5) * USD;
        let y = Dec!(23.85);
        let z = x / y;
        assert_eq!(z.amount(), Dec!(29.37));
        assert_eq!(z.unit(), USD);
        let x = Dec!(700.5) * UYW;
        let z = x / y;
        assert_eq!(z.amount(), Dec!(29.3711));
        assert_eq!(z.unit(), UYW);
    }
}

#[cfg(test)]
mod test_money_rated_to_qty {
    use moneta::{Dec, Decimal, Money, Quantity, USD, UYW};
    use quantities::prelude::*;

    #[quantity]
    #[ref_unit(Kilogram, "kg", KILO, "Reference unit of quantity `Mass`")]
    #[unit(Gram, "g", NONE, 0.001, "0.001·kg")]
    struct Mass {}

    #[test]
    fn test_rate_qty_per_money() {
        let d = Dec!(7.5) * USD;
        let r = Rate::<Mass, Money>::new(Dec!(3.5), KILOGRAM, Dec!(10), USD);
        let m = d * r;
        assert_eq!(m, Dec!(2.625) * KILOGRAM);
        assert_eq!(d * r, r * d);
        assert_eq!(m / r, d);
        let m = Dec!(450) * GRAM;
        assert_eq!(m / r, Dec!(1.29) * USD);
    }

    #[test]
    #[should_panic]
    fn test_money_mul_rate_diff_currency() {
        let d = Dec!(7.5) * USD;
        let r = Rate::<Mass, Money>::new(Dec!(3), KILOGRAM, Dec!(10), UYW);
        let _m = d * r;
    }

    #[test]
    fn test_rate_money_per_qty() {
        let d = Dec!(7.5) * USD;
        let r = Rate::<Money, Mass>::new(Dec!(3), USD, Dec!(10), KILOGRAM);
        let m = d / r;
        assert_eq!(m, Dec!(25) * KILOGRAM);
        assert_eq!(m * r, d);
        let m = Dec!(450) * GRAM;
        assert_eq!(m * r, Dec!(0.14) * USD);
    }

    #[test]
    #[should_panic]
    fn test_money_div_rate_diff_currency() {
        let d = Dec!(7.5) * USD;
        let r = Rate::<Money, Mass>::new(Dec!(3), UYW, Dec!(10), KILOGRAM);
        let _mr = d / r;
    }
}
