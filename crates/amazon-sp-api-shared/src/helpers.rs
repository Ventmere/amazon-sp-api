use serde::{Deserializer, Deserialize};

pub fn deserialize_opt_i32_with_parse<'de, D>(deserializer: D) -> Result<Option<i32>, D::Error>
where D: Deserializer<'de> {
    let buf = String::deserialize(deserializer)?;

    buf.parse().map(Some).map_err(serde::de::Error::custom)
}

pub fn deserialize_opt_bool_from_string<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where D: Deserializer<'de> {
    let buf = String::deserialize(deserializer)?;

    if buf == "" {
        return Ok(None);
    }

    Ok(Some(buf == "true"))
}