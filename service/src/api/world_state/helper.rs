use std::fmt::Display;
use std::marker::PhantomData;
use std::str::FromStr;

use chrono::DateTime;
use chrono::Duration;
use chrono::Utc;
use serde::Deserialize;
use serde::de::Error as DeError;
use serde::de::Visitor;
use serde_with::DeserializeAs;
use serde_with::TimestampMilliSeconds;
use serde_with::serde_as;
// use serde_with::rust::double_option;

/// Deserialize a boolean from any type
/// string `false` `0` `no` ``(empty string) `null` `off` are false
/// number `0` are false
/// booleans are as is
/// `undefined` are false
pub struct BoolFromAny;

impl<'de> DeserializeAs<'de, bool> for BoolFromAny {
    fn deserialize_as<D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct StrVisitor;
        static FALSIFIABLE: [&str; 4] = ["false", "0", "no", ""];
        impl Visitor<'_> for StrVisitor {
            type Value = bool;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string or number")
            }

            // fn visit_

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(!FALSIFIABLE.contains(&value))
            }
            fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(value)
            }
            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(false)
            }
            fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(v != 0)
            }

            fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
            where
                E: DeError,
            {
                Ok(v != 0)
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: DeError,
            {
                Ok(v != 0)
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: DeError,
            {
                Ok(v != 0)
            }

            fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
            where
                E: DeError,
            {
                Ok(v != 0)
            }

            fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
            where
                E: DeError,
            {
                Ok(v != 0)
            }
        }

        deserializer.deserialize_any(StrVisitor)
    }
}

pub struct DurationFromStr;

impl<'de> DeserializeAs<'de, Duration> for DurationFromStr {
    fn deserialize_as<D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct StrVisitor;

        impl<'de> Visitor<'de> for StrVisitor {
            type Value = Duration;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string")
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let sign = if value.starts_with('-') { -1 } else { 1 };
                let secs = value
                    .split(' ')
                    .map(|s| {
                        let unit = s.chars().last().ok_or(E::custom("Invalid duration"))?;
                        let num = s
                            .strip_suffix(unit)
                            .ok_or(E::custom("Invalid duration"))?
                            .parse::<i64>()
                            .map_err(E::custom)?;
                        let uv = match unit {
                            's' => 1,
                            'm' => 60,
                            'h' => 60 * 60,
                            'd' => 60 * 60 * 24,
                            _ => return Err(E::custom("Invalid duration")),
                        };
                        Ok(num.abs() * uv)
                    })
                    .collect::<Result<Vec<_>, _>>()?
                    .iter()
                    .fold(0, std::ops::Add::add);
                Duration::new(secs * sign, 0).ok_or(E::custom("msg"))
            }
        }
        deserializer.deserialize_str(StrVisitor)
    }
}

pub fn bool_false() -> bool {
    false
}

#[derive(Deserialize, Debug)]
pub struct DeDate {
    #[serde(rename = "$date")]
    inner: DeInnerDate,
}

#[serde_as]
#[derive(Deserialize, Debug)]
struct DeInnerDate {
    #[serde_as(as = "TimestampMilliSeconds<String>")]
    #[serde(rename = "$numberLong")]
    value: DateTime<Utc>,
}

impl From<DeDate> for DateTime<Utc> {
    fn from(value: DeDate) -> Self {
        value.inner.value
    }
}

impl From<DateTime<Utc>> for DeDate {
    fn from(date: DateTime<Utc>) -> Self {
        DeDate {
            inner: DeInnerDate { value: date },
        }
    }
}

/// A serde_with plugin to parse number from string
/// like the [serde_with::DisplayFromStr]
/// but it take an empty string as zero
///
/// currently support i32, i64, f64
pub struct NumberFromStringLoose;

trait Num {
    fn zero() -> Self;
}

impl Num for i32 {
    fn zero() -> Self {
        0
    }
}
impl Num for i64 {
    fn zero() -> Self {
        0
    }
}
impl Num for f64 {
    fn zero() -> Self {
        0.0
    }
}

impl<'de, T> DeserializeAs<'de, T> for NumberFromStringLoose
where
    T: Num + FromStr<Err: Display>,
{
    fn deserialize_as<D>(deserializer: D) -> Result<T, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct StrVisitor<T>(PhantomData<T>);

        impl<'de, T> Visitor<'de> for StrVisitor<T>
        where
            T: Num + FromStr<Err: Display>,
        {
            type Value = T;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string or number")
            }

            fn visit_str<E>(self, value: &str) -> Result<T, E>
            where
                E: serde::de::Error,
            {
                if value.is_empty() {
                    Ok(T::zero())
                } else {
                    value.parse::<T>().map_err(|e| E::custom(e))
                }
            }
        }
        deserializer.deserialize_str(StrVisitor(PhantomData))
    }
}
