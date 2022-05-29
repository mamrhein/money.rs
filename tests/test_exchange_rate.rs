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

    #[test]
    fn test_convert_by_div() {
        let usd = Dec!(17.95) * USD;
        let rate = ExchangeRate::new(EUR, 1, USD, Dec!(1.0807));
        let eur = usd / rate;
        assert_eq!(eur.unit(), EUR);
        assert_eq!(eur.amount(), Dec!(16.61));
    }

    #[test]
    #[should_panic]
    fn test_money_div_rate_fails() {
        let usd = Dec!(17.95) * USD;
        let rate = ExchangeRate::new(USD, 1, EUR, Dec!(0.98078));
        let _eur = usd / rate;
    }
}

#[cfg(test)]
mod test_triangulation {
    use moneta::{Dec, Decimal, ExchangeRate, EUR, HKD, USD};

    #[test]
    fn test_mul_rates() {
        let usd_2_eur = ExchangeRate::new(USD, 1, EUR, Dec!(0.98078));
        let eur_2_hkd = ExchangeRate::new(EUR, 1, HKD, Dec!(8.225));
        let usd_2_hkd = usd_2_eur * eur_2_hkd;
        assert_eq!(usd_2_hkd.unit_currency(), USD);
        assert_eq!(usd_2_hkd.unit_multiple(), 1);
        assert_eq!(usd_2_hkd.term_currency(), HKD);
        assert_eq!(usd_2_hkd.term_amount(), Dec!(8.066916));
        let eur_2_usd = usd_2_eur.inverted();
        let hkd_2_eur = eur_2_hkd.inverted();
        let usd_2_hkd = eur_2_usd * hkd_2_eur;
        assert_eq!(usd_2_hkd.unit_currency(), HKD);
        assert_eq!(usd_2_hkd.unit_multiple(), 1);
        assert_eq!(usd_2_hkd.term_currency(), USD);
        assert_eq!(usd_2_hkd.term_amount(), Dec!(0.123964));
    }

    #[test]
    #[should_panic]
    fn test_mul_rates_fails() {
        let usd_2_eur = ExchangeRate::new(USD, 1, EUR, Dec!(0.98078));
        let usd_2_hkd = ExchangeRate::new(USD, 1, HKD, Dec!(8.0047));
        let _r = usd_2_eur * usd_2_hkd;
    }
}
