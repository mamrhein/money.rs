// ---------------------------------------------------------------------------
// Copyright:   (c) 2024 ff. Michael Amrhein (michael@adrhinum.de)
// License:     This program is part of a larger application. For license
//              details please read the file LICENSE.TXT provided together
//              with the application.
// ---------------------------------------------------------------------------
// $Source$
// $Revision$

mod setup {
    use moneta::{Currency, Unit};

    pub(crate) fn setup() {
        let _btc = Currency::new("BTC", "Bitcoin", 7);
        let _etc = Currency::new("ETC", "Ethereum Classic", 4);
        let _xqd = Currency::new("XQD", "XQuad", 5);
        let btc = Currency::from_symbol("BTC").unwrap();
        let etc = Currency::from_symbol("ETC").unwrap();
        let xqd = Currency::from_symbol("XQD").unwrap();
        let usd = Currency::from_symbol("USD").unwrap();
        assert_eq!(
            Currency::iter().collect::<Vec<_>>(),
            [btc, etc, xqd, usd]
        );
    }
}

#[cfg(test)]
mod test_custom_currency_creation {
    use moneta::{Currency, Dec, Decimal, Unit};

    use super::setup::setup;

    #[test]
    fn test_new_currencies() {
        setup();
        let btc = Currency::from_symbol("BTC").unwrap();
        assert_eq!(btc.symbol(), "BTC");
        assert_eq!(btc.name(), "Bitcoin");
        assert_eq!(btc.minor_unit(), 7);
        let etc = Currency::from_symbol("ETC").unwrap();
        assert_eq!(etc.symbol(), "ETC");
        assert_eq!(etc.name(), "Ethereum Classic");
        assert_eq!(etc.minor_unit(), 4);
        let xqd = Currency::from_symbol("XQD").unwrap();
        assert_eq!(xqd.symbol(), "XQD");
        assert_eq!(xqd.name(), "XQuad");
        assert_eq!(xqd.minor_unit(), 5);
    }

    #[test]
    fn test_custom_curr_to_string() {
        setup();
        let xqd = Currency::from_symbol("XQD").unwrap();
        assert_eq!(xqd.to_string(), "XQD");
    }

    #[test]
    fn test_custom_curr_fmt() {
        setup();
        let xqd = Currency::from_symbol("XQD").unwrap();
        assert_eq!(format!("{:^5}", xqd), " XQD ");
        assert_eq!(format!("{:>7.2}", xqd), "     XQ");
    }

    #[test]
    fn test_custom_money_to_string() {
        setup();
        let xqd = Currency::from_symbol("XQD").unwrap();
        let val = Dec!(-137.95) * xqd;
        assert_eq!(val.to_string(), "-137.95000 XQD");
    }
}

#[cfg(test)]
mod test_money_exchange {
    use fpdec_core::Round;
    use moneta::{Currency, Dec, Decimal, ExchangeRate, Quantity, Unit, USD};

    use super::setup::setup;

    #[test]
    fn test_custom_exchange_rate() {
        setup();
        let xqd = Currency::from_symbol("XQD").unwrap();
        let usd_to_xqd = ExchangeRate::new(USD, 1000, xqd, Dec!(17.0028));
        assert_eq!(usd_to_xqd.term_currency(), xqd);
        assert_eq!(usd_to_xqd.quotation(), (USD, xqd, Dec!(0.0170028)));
        let usd_amnt = Dec!(17.95) * USD;
        let xqd_amnt = usd_amnt * usd_to_xqd;
        assert_eq!(xqd_amnt.unit(), xqd);
        assert_eq!(
            xqd_amnt.amount(),
            (Dec!(17.95) * Dec!(0.0170028)).round(xqd.minor_unit() as i8)
        );
    }
}
