// ---------------------------------------------------------------------------
// Copyright:   (c) 2023 ff. Michael Amrhein (michael@adrhinum.de)
// License:     This program is part of a larger application. For license
//              details please read the file LICENSE.TXT provided together
//              with the application.
// ---------------------------------------------------------------------------
// $Source$
// $Revision$

#[cfg(feature = "serde")]
#[cfg(test)]
mod serde_tests {
    use moneta::{Currency, Dec, Decimal, Money, USD};
    use serde_json;

    #[test]
    fn test_currency() {
        let unit = USD;
        let s = serde_json::to_value(unit).unwrap();
        assert_eq!(unit, serde_json::from_value::<Currency>(s).unwrap());
    }

    #[test]
    fn test_money() {
        let amnt = Dec!(17.4) * USD;
        let s = serde_json::to_value(amnt).unwrap();
        assert_eq!(amnt, serde_json::from_value::<Money>(s).unwrap());
    }
}
