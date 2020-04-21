use serde::de::{Deserialize, Deserializer, Error};

pub(crate) fn deserialize_option_string<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<String>, D::Error> {
    let value: String = Deserialize::deserialize(deserializer)?;
    Ok(match value.as_ref() {
        "none" => None,
        _ => Some(value),
    })
}

pub(crate) fn deserialize_option_date_time<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<chrono::NaiveDateTime>, D::Error> {
    use std::str::FromStr;
    let value: String = Deserialize::deserialize(deserializer)?;
    Ok(match value.as_ref() {
        "none" => None,
        _ => Some(chrono::NaiveDateTime::from_str(&value).map_err(D::Error::custom)?),
    })
}

pub(crate) fn deserialize_option_time<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<chrono::NaiveTime>, D::Error> {
    use std::str::FromStr;
    let mut value: String = Deserialize::deserialize(deserializer)?;
    Ok(match value.remove(0) {
        'T' => Some(chrono::NaiveTime::from_str(&value).map_err(D::Error::custom)?),
        _ => None,
    })
}
