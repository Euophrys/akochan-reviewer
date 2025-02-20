use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

use num_enum::TryFromPrimitive;
use once_cell::sync::Lazy;
use serde::ser::SerializeSeq;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_repr::Deserialize_repr as DeserializeRepr;
use thiserror::Error;

/// Describes a pai in tenhou.net/6 format.
///
/// By default, it deserializes from u8 (as in tenhou.net/6), but serializes to
/// String (as in mjai).
#[derive(Debug, Clone, Copy, PartialEq, Hash, DeserializeRepr, TryFromPrimitive)]
#[repr(u8)]
pub enum Pai {
    Unknown = 0,

    Man1 = 11,
    Man2 = 12,
    Man3 = 13,
    Man4 = 14,
    Man5 = 15,
    Man6 = 16,
    Man7 = 17,
    Man8 = 18,
    Man9 = 19,

    Pin1 = 21,
    Pin2 = 22,
    Pin3 = 23,
    Pin4 = 24,
    Pin5 = 25,
    Pin6 = 26,
    Pin7 = 27,
    Pin8 = 28,
    Pin9 = 29,

    Sou1 = 31,
    Sou2 = 32,
    Sou3 = 33,
    Sou4 = 34,
    Sou5 = 35,
    Sou6 = 36,
    Sou7 = 37,
    Sou8 = 38,
    Sou9 = 39,

    East = 41,
    South = 42,
    West = 43,
    North = 44,
    Haku = 45,
    Hatsu = 46,
    Chun = 47,

    AkaMan5 = 51,
    AkaPin5 = 52,
    AkaSou5 = 53,
}

impl Eq for Pai {}

const MJAI_PAI_STRINGS: &[&str] = &[
    "?", "?", "?", "?", "?", "?", "?", "?", "?", "?", // 0~9
    "?", "1m", "2m", "3m", "4m", "5m", "6m", "7m", "8m", "9m", // 10~19
    "?", "1p", "2p", "3p", "4p", "5p", "6p", "7p", "8p", "9p", // 20~29
    "?", "1s", "2s", "3s", "4s", "5s", "6s", "7s", "8s", "9s", // 30~39
    "?", "E", "S", "W", "N", "P", "F", "C", "?", "?", // 40~49
    "?", "5mr", "5pr", "5sr", // 50~53
];

static MJAI_PAI_STRINGS_MAP: Lazy<HashMap<String, Pai>> = Lazy::new(|| {
    let mut m = HashMap::new();

    for (i, &v) in MJAI_PAI_STRINGS.iter().enumerate() {
        if let Ok(pai) = Pai::try_from(i as u8) {
            m.insert(v.to_owned(), pai);
        }
    }
    assert_eq!(m.len(), 1 + 9 * 3 + 7 + 3);

    m
});

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("invalid pai string {0:?}")]
    InvalidPaiString(String),
}

impl FromStr for Pai {
    type Err = ParseError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(&pai) = MJAI_PAI_STRINGS_MAP.get(s) {
            Ok(pai)
        } else {
            Err(ParseError::InvalidPaiString(s.to_owned()))
        }
    }
}

impl fmt::Display for Pai {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            MJAI_PAI_STRINGS[self.as_usize() % MJAI_PAI_STRINGS.len()]
        )
    }
}

impl Serialize for Pai {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(self)
    }
}

impl Default for Pai {
    #[inline]
    fn default() -> Self {
        Self::Unknown
    }
}

impl Pai {
    #[inline]
    pub const fn as_u8(self) -> u8 {
        self as u8
    }
    #[inline]
    pub const fn as_usize(self) -> usize {
        self as usize
    }

    #[allow(clippy::trivially_copy_pass_by_ref)]
    #[inline]
    pub fn serialize_literal<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u8(self.as_u8())
    }

    pub fn serialize_slice_literal<S, P>(pais: P, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        P: AsRef<[Self]>,
    {
        let pais_ref = pais.as_ref();
        let mut seq = serializer.serialize_seq(Some(pais_ref.len()))?;
        for e in pais_ref {
            seq.serialize_element(&e.as_u8())?;
        }
        seq.end()
    }

    pub fn deserialize_mjai_str<'de, D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;

        let s = String::deserialize(deserializer)?;
        let pai = s.parse().map_err(Error::custom)?;
        Ok(pai)
    }
}
