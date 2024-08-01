// ---------------------------------------------------------------------------
// Copyright:   (c) 2024 ff. Michael Amrhein (michael@adrhinum.de)
// License:     This program is part of a larger application. For license
//              details please read the file LICENSE.TXT provided together
//              with the application.
// ---------------------------------------------------------------------------
// $Source$
// $Revision$

use alloc::{
    borrow::ToOwned, collections::BTreeMap, format, string::String, vec::Vec,
};
use core::str::FromStr;
#[cfg(feature = "std")]
use std::sync::RwLock;

#[cfg(not(feature = "std"))]
use synctools::rwlock::RwLock;

use crate::{
    currency_info::{CurrencyInfo, CurrencyKey, CurrencyKeyError},
    ISOCurrency,
};

struct CurrencyDB {
    registered_currencies: Vec<Currency>,
    currency_info_map: BTreeMap<Currency, CurrencyInfo>,
    symbol_currency_map: BTreeMap<String, Currency>,
}

impl CurrencyDB {
    fn insert(&mut self, curr: Currency, info: &CurrencyInfo) {
        let symbol = info.symbol.clone();
        self.registered_currencies.push(curr);
        self.currency_info_map.insert(curr, info.to_owned());
        self.symbol_currency_map.insert(symbol, curr);
    }
}

pub(crate) struct CurrencyRegistry {
    inner: RwLock<CurrencyDB>,
}

impl CurrencyRegistry {
    pub(crate) const fn new() -> Self {
        Self {
            inner: RwLock::new(CurrencyDB {
                registered_currencies: Vec::new(),
                currency_info_map: BTreeMap::new(),
                symbol_currency_map: BTreeMap::new(),
            }),
        }
    }

    pub(crate) fn register_currency(
        &self,
        symbol: &str,
        name: &str,
        minor_units: u8,
    ) -> Result<Currency, CurrencyKeyError> {
        let key = CurrencyKey::from_str(symbol)?;
        let info = CurrencyInfo {
            key,
            symbol: symbol.into(),
            name: name.into(),
            minor_unit: minor_units,
        };
        let curr = Currency { key };
        #[cfg(feature = "std")]
        let mut db = self.inner.write().unwrap();
        #[cfg(not(feature = "std"))]
        let mut db = self.inner.write();
        if db.currency_info_map.contains_key(&curr) {
            let msg = format!(
                "Currency with key derived from '{symbol}' already \
                 registered."
            );
            return Err(CurrencyKeyError(msg));
        }
        db.insert(curr, &info);
        Ok(curr)
    }

    pub(crate) fn get_registered_currencies(&self) -> Vec<Currency> {
        #[cfg(feature = "std")]
        let db = self.inner.read().unwrap();
        #[cfg(not(feature = "std"))]
        let db = self.inner.read();
        db.registered_currencies.clone()
    }

    pub(crate) fn get_currency_info(&self, curr: &Currency) -> CurrencyInfo {
        #[cfg(feature = "std")]
        let db = self.inner.read().unwrap();
        #[cfg(not(feature = "std"))]
        let db = self.inner.read();
        match db.currency_info_map.get(curr) {
            Some(info) => info.clone(),
            None => {
                // Can happen only if `self` is a not yet registered ISO
                // currency, so register it!
                let iso_curr = ISOCurrency::from_key(curr.key).unwrap();
                let props = iso_curr.info();
                let info = CurrencyInfo::new(props.0, props.1, props.2);
                // Release the read lock and get a write lock.
                drop(db);
                #[cfg(feature = "std")]
                let mut db = self.inner.write().unwrap();
                #[cfg(not(feature = "std"))]
                let mut db = self.inner.write();
                // Maybe another thread registered it in the meantime, so
                // check again!
                if db.currency_info_map.get(curr).is_none() {
                    db.insert(*curr, &info);
                };
                info
            }
        }
    }

    pub(crate) fn get_currency_from_symbol(
        &self,
        sym: &str,
    ) -> Option<Currency> {
        #[cfg(feature = "std")]
        let db = self.inner.read().unwrap();
        #[cfg(not(feature = "std"))]
        let db = self.inner.read();
        match db.symbol_currency_map.get(sym) {
            Some(c) => Some(*c),
            None => {
                // Check whether `sym` is the symbol of a not yet registered
                // ISOCurrency; if so, register and return it!
                let iso_curr = ISOCurrency::from_symbol(sym)?;
                let props = iso_curr.info();
                let info = CurrencyInfo::new(props.0, props.1, props.2);
                let curr = Currency::from_iso_curr(*iso_curr);
                // Release the read lock and get a write lock.
                drop(db);
                #[cfg(feature = "std")]
                let mut db = self.inner.write().unwrap();
                #[cfg(not(feature = "std"))]
                let mut db = self.inner.write();
                // Maybe another thread registered it in the meantime, so
                // check again!
                if db.currency_info_map.get(&curr).is_none() {
                    db.insert(curr, &info);
                };
                Some(curr)
            }
        }
    }
}

/// Unit of quantity `Money`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[cfg_attr(
    feature = "serde",
    derive(::serde::Deserialize, ::serde::Serialize)
)]
pub struct Currency {
    key: CurrencyKey,
}

impl Currency {
    /// Returns the `Currency` instance corresponding to the given
    /// `ISOCurrency` instance without registering it!
    pub(crate) const fn from_iso_curr(iso_curr: ISOCurrency) -> Self {
        Self {
            key: iso_curr.key(),
        }
    }
}
