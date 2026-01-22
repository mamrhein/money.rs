// ---------------------------------------------------------------------------
// Copyright:   (c) 2024 ff. Michael Amrhein (michael@adrhinum.de)
// License:     This program is part of a larger application. For license
//              details please read the file LICENSE.TXT provided together
//              with the application.
// ---------------------------------------------------------------------------
// $Source$
// $Revision$

use alloc::{borrow::ToOwned, format, string::String};
use core::str::FromStr;

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[cfg_attr(
    feature = "serde",
    derive(::serde::Deserialize, ::serde::Serialize)
)]
pub struct CurrencyKey(u64);

#[derive(Debug, Eq, PartialEq)]
pub struct CurrencyKeyError(pub String);

impl CurrencyKey {
    pub(crate) const fn from_ascii(s: &str) -> Self {
        debug_assert!(s.is_ascii());
        debug_assert!(s.len() == 3);
        let b = s.as_bytes();
        let k = [b[0], b[1], b[2], 0_u8, 0_u8, 0_u8, 0_u8, 0_u8];
        Self(u64::from_be_bytes(k))
    }
}

impl FromStr for CurrencyKey {
    type Err = CurrencyKeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut k = [0_u8; 8];
        let t = s
            .trim()
            .chars()
            .filter(|c| c.is_ascii())
            .collect::<String>();
        match t.len() {
            0 => Err(CurrencyKeyError(format!(
                "Given code '{s}' is empty or contains only white-space or \
                 non-ascii chars."
            ))),
            mut l => {
                l = l.min(8_usize);
                k[..l].copy_from_slice(&AsRef::<[u8]>::as_ref(&t)[..l]);
                Ok(Self(u64::from_be_bytes(k)))
            }
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct CurrencyInfo {
    pub(crate) key: CurrencyKey,
    pub(crate) symbol: String,
    pub(crate) name: String,
    pub(crate) minor_unit: u8,
}

impl CurrencyInfo {
    #[inline]
    pub(crate) fn new(symbol: &str, name: &str, minor_unit: u8) -> Self {
        Self {
            key: CurrencyKey::from_ascii(symbol),
            symbol: symbol.to_owned(),
            name: name.to_owned(),
            minor_unit,
        }
    }
}
