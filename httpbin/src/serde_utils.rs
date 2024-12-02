// fn as_optional_number<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
// where
//   T: FromStr + serde::Deserialize<'de>,
//   T::Err: std::fmt::Display,
//   D: Deserializer<'de>,
// {
//   let value: Option<Value> = Deserialize::deserialize(deserializer)?;
//   match value {
//     Some(Value::String(s)) => s
//       .trim()
//       .parse::<T>()
//       .map(Some)
//       .map_err(serde::de::Error::custom),
//     None => Ok(None),
//     _ => Err(serde::de::Error::custom("excepted string as number")),
//   }
// }

// fn as_number<'de, T, D>(deserializer: D) -> Result<T, D::Error>
// where
//   T: FromStr + serde::Deserialize<'de>,
//   T::Err: std::fmt::Display,
//   D: Deserializer<'de>,
// {
//   let value = Deserialize::deserialize(deserializer)?;
//   match value {
//     Value::String(s) => s.trim().parse::<T>().map_err(serde::de::Error::custom),
//     _ => Err(serde::de::Error::custom("excepted string as number")),
//   }
// }

// fn as_i32<'de, D>(deserializer: D) -> Result<i32, D::Error>
// where
//   D: Deserializer<'de>,
// {
//   let value = Deserialize::deserialize(deserializer)?;
//   match value {
//     Value::String(s) => s.trim().parse::<i32>().map_err(serde::de::Error::custom),
//     _ => Err(serde::de::Error::custom("Excepted string")),
//   }
// }

// @see https://users.rust-lang.org/t/why-are-there-2-types-for-deserializing-in-serde/35735/16
// @see https://github.com/rnag/serde-this-or-that/blob/main/src/de_impl.rs

use serde::{de, Deserializer};

pub fn as_i64<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
  D: Deserializer<'de>,
{
  deserializer.deserialize_any(I64Visitor)
}

struct I64Visitor;

impl<'de> de::Visitor<'de> for I64Visitor {
  type Value = i64;

  fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
    formatter.write_str("i64 for visitor")
  }

  fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
  where
    E: de::Error,
  {
    if let Ok(n) = v.parse::<i64>() {
      Ok(n)
    } else {
      Err(E::invalid_type(de::Unexpected::Str(v), &self))
    }
  }

  fn visit_unit<E>(self) -> Result<Self::Value, E>
  where
    E: de::Error,
  {
    Ok(0)
  }
}
