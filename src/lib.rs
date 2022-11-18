#![forbid(unsafe_code)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]

use core::borrow::Borrow;

mod enums;
pub use enums::{CCA2, CCA3};

mod types;
pub use types::*;

mod consts;
pub use consts::*;

#[cfg(all(feature = "async-graphql", feature = "alloc"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "async-graphql", feature = "alloc"))))]
mod async_graphql;

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
mod serde;

// #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
// pub enum ParseCountryError {
//     UnknownCountry,
//     UnknownAlpha2,
//     UnknownAlpha3,
//     UnknownCountryCode,
//     Unkonwn3166_2,
// }

// impl core::fmt::Display for ParseCountryError {
//     fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
//         match self {
//             ParseCountryError::UnknownCountry => write!(f, "unknown country name"),
//             ParseCountryError::UnknownAlpha2 => write!(f, "unknown country alpha2 code"),
//             ParseCountryError::UnknownAlpha3 => write!(f, "unknown country alpha3 code"),
//             ParseCountryError::UnknownCountryCode => write!(f, "unknown country code"),
//             ParseCountryError::Unkonwn3166_2 => write!(f, "unknown ISO 3166-2"),
//         }
//     }
// }

// #[cfg(feature = "std")]
// impl std::error::Error for ParseCountryError {}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct StaticMap<K: 'static, V: 'static> {
    map: &'static [(K, V)],
}

impl<K: 'static, V: 'static> Clone for StaticMap<K, V> {
    fn clone(&self) -> Self {
        Self { map: self.map }
    }
}

impl<K: 'static, V: 'static> Copy for StaticMap<K, V> {}

impl<K: 'static + Eq, V: 'static> core::ops::Index<K> for StaticMap<K, V> {
    type Output = V;

    fn index(&self, index: K) -> &Self::Output {
        self.map
            .iter()
            .find(|(k, _)| k.eq(&index))
            .map(|(_, v)| v)
            .expect("key not found")
    }
}

impl<K: 'static + Eq, V: 'static> core::ops::Index<&K> for StaticMap<K, V> {
    type Output = V;

    fn index(&self, index: &K) -> &Self::Output {
        self.map
            .iter()
            .find(|(k, _)| k.eq(index))
            .map(|(_, v)| v)
            .expect("key not found")
    }
}

impl<K: 'static, V: 'static> StaticMap<K, V> {
    /// Creates a new static map
    #[inline]
    pub const fn new(map: &'static [(K, V)]) -> Self {
        Self { map }
    }

    /// Creates a new iterator for the map
    #[inline]
    pub fn iter(&self) -> core::slice::Iter<'_, (K, V)> {
        self.map.iter()
    }

    /// Returns the length of the static map
    #[inline]
    pub const fn len(&self) -> usize {
        self.map.len()
    }

    /// Returns true if the map has a length of 0.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

impl<K: 'static + Eq, V: 'static> StaticMap<K, V> {
    pub fn get<Q>(&self, k: &Q) -> Option<&'static V>
    where
        K: Borrow<Q>,
        Q: ?Sized + Eq,
    {
        self.map
            .iter()
            .find(|(key, _)| key.borrow().eq(k))
            .map(|(_, v)| v)
    }

    pub fn contains<Q>(&self, k: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: ?Sized + Eq,
    {
        self.map.iter().any(|(key, _)| key.borrow().eq(k))
    }
}

impl<K: 'static, V: 'static> core::iter::IntoIterator for StaticMap<K, V> {
    type Item = &'static (K, V);

    type IntoIter = core::slice::Iter<'static, (K, V)>;

    fn into_iter(self) -> Self::IntoIter {
        self.map.iter()
    }
}
