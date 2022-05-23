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
    use moneta::{Dec, Decimal, USD, UYW, VUV};
    use quantities::Quantity;

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
