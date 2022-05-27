### Currency-safe computations with money amounts.

`Money` is a special type of quantity. Its unit type is known as `Currency`.

Money differs from physical quantities mainly in two aspects:

* Money amounts are discrete. For each currency there is a smallest fraction
  that can not be split further.

* The relation between different currencies is not fixed, instead, it varies
  over time.

This package provides types to deal with these specifics. It is based an the
package "quantities" with feature "fpdec" (aliasing "AmountT" to "Decimal").

`Currency` is an enumeration of all currencies defined in ISO 4217. In
addition, for each currency there is a constant named after the 3-character
ISO 4217 code.

`Currency` implements `quantities::Unit`, so all operations on units can be 
applied to `Currency`. Especially, a `Currency` instance can be multiplied 
with an `AmountT` to create a `Money` instance.

Example:

```rust
# use moneta::{Amnt, Currency, Dec, Decimal, EUR, Money, Quantity, Unit};
let amnt = Amnt!(17.95);
let eur_amnt = amnt * EUR;
assert_eq!(eur_amnt.amount(), amnt);
assert_eq!(eur_amnt.unit(), Currency::EUR);
```

`Money` implements trait `quantities::Quantity`, so all operations on
quantities can also be applied to instances of `Money`. Because there is no
fixed relation between currencies, there is no implicit conversion between
money amounts of different currencies. Resulting values are always quantized
to the smallest fraction defined with the currency.

Example:

```rust
# use moneta::{Amnt, Currency, Dec, Decimal, EUR, Money, Quantity, Unit};
let qty = Amnt!(3.2);
let price = Money::new(Amnt!(13.58), EUR);
let total = qty * price;
assert_eq!(total.to_string(), "43.46 EUR");
```

A conversion factor between two currencies can be defined by using the
type `ExchangeRate`. It is given a unit currency (aka base currency), a unit
multiple, a term currency (aka price currency) and a term amount, i.e. the
amount in term currency equivalent to unit multiple in unit currency.

Multiplying an amount in some currency with an exchange rate with the same
currency as unit currency results in the equivalent amount in term currency.
Likewise, dividing an amount in some currency with an exchange rate with the
same currency as term currency results in the equivalent amount in unit
currency.

Examples:

```rust
# use moneta::{Dec, Decimal, ExchangeRate, EUR, HKD, USD};
let usd = Dec!(17.95) * USD;
let rate = ExchangeRate::new(USD, 1, EUR, Dec!(0.98078));
let eur = usd * rate;
assert_eq!(eur.to_string(), "17.61 EUR");
let rate = ExchangeRate::new(HKD, 10, EUR, Dec!(1.187253));
let hkd = eur / rate;
assert_eq!(hkd.to_string(), "148.33 HKD");
```
