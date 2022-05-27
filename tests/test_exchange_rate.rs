// ---------------------------------------------------------------------------
// Copyright:   (c) 2022 ff. Michael Amrhein (michael@adrhinum.de)
// License:     This program is part of a larger application. For license
//              details please read the file LICENSE.TXT provided together
//              with the application.
// ---------------------------------------------------------------------------
// $Source$
// $Revision$

#[cfg(test)]
mod test_money_exchange {
    use moneta::{Dec, Decimal, ExchangeRate, EUR, USD};
    use quantities::Quantity;

    #[test]
    fn test_convert_by_mul() {
        let usd = Dec!(17.95) * USD;
        let rate = ExchangeRate::new(USD, 1, EUR, Dec!(0.98078));
        let eur = usd * rate;
        assert_eq!(eur.unit(), EUR);
        assert_eq!(eur.amount(), Dec!(17.61));
        assert_eq!(usd * rate, rate * usd);
    }

    #[test]
    #[should_panic]
    fn test_money_mul_rate_fails() {
        let usd = Dec!(17.95) * USD;
        let rate = ExchangeRate::new(EUR, 1, USD, Dec!(0.98078));
        let _eur = usd * rate;
    }

    #[test]
    #[should_panic]
    fn test_rate_mul_money_fails() {
        let usd = Dec!(17.95) * USD;
        let rate = ExchangeRate::new(EUR, 1, USD, Dec!(0.98078));
        let _eur = rate * usd;
    }
}
