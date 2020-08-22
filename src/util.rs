use chrono::{NaiveDateTime, NaiveTime};
use serde::de::{Deserialize, Deserializer, Error};

pub(crate) fn deserialize_option_string<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<String>, D::Error> {
    let value: Option<String> = Deserialize::deserialize(deserializer)?;
    Ok(match value.as_deref() {
        Some("none") | None => None,
        Some(_) => value,
    })
}

pub(crate) fn deserialize_option_date_time<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<NaiveDateTime>, D::Error> {
    let value: Option<String> = Deserialize::deserialize(deserializer)?;
    Ok(match value.as_deref() {
        Some("none") | None => None,
        Some(v) => {
            Some(NaiveDateTime::parse_from_str(v, "%Y-%m-%dT%H:%M:%S").map_err(D::Error::custom)?)
        }
    })
}

pub(crate) fn deserialize_option_time<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<NaiveTime>, D::Error> {
    let value: Option<String> = Deserialize::deserialize(deserializer)?;
    Ok(match value.as_deref() {
        Some("none") | None => None,
        Some(v) => Some(NaiveTime::parse_from_str(v, "T%H:%M:%S").map_err(D::Error::custom)?),
    })
}

macro_rules! custom_serialize {
    ($serializer:expr, $struct_name:expr; $($k:ident => ($($v:tt)*),)*) => {
        let mut len = 0;
        $(
            let $k = custom_serialize!(@VALUE $($v)*);
            if $k.is_some() {
                len += 1;
            }
        )*
        let mut state = $serializer.serialize_struct($struct_name, len)?;
        $(
            if let Some(v) = $k {
                state.serialize_field(stringify!($k), &v)?;
            }
        )*
        state.end()
    };
    (@VALUE $v:expr) => {
        $v
    };
    (@VALUE $v:expr, to_override) => {
        $v.and_then(|adjuster| match adjuster {
            Adjuster::Override(v) => Some(v),
            _ => None,
        })
    };
    (@VALUE $v:expr, to_increment, $t:ty) => {
        $v.and_then(|adjuster| match adjuster {
            Adjuster::Increment(v) => Some(v as $t),
            Adjuster::Decrement(v) => Some(-(v as $t)),
            _ => None,
        })
    };
    (@VALUE $v:expr, to_increment_tuple, $t:ty) => {
        $v.and_then(|adjuster| match adjuster {
            Adjuster::Increment(v) => Some((v.0 as $t, v.1 as $t)),
            Adjuster::Decrement(v) => Some((-(v.0 as $t), -(v.1 as $t))),
            _ => None,
        })
    };
}

#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
    use serde_json::json;

    #[test]
    fn deserialize_option_string() {
        let json = json!("none");
        let value = super::deserialize_option_string(json).unwrap();
        assert_eq!(value, None);

        let json = json!(null);
        let value = super::deserialize_option_string(json).unwrap();
        assert_eq!(value, None);

        let json = json!("test");
        let value = super::deserialize_option_string(json).unwrap();
        assert_eq!(value, Some("test".to_owned()));
    }

    #[test]
    fn deserialize_option_date_time() {
        let json = json!("none");
        let value = super::deserialize_option_date_time(json).unwrap();
        assert_eq!(value, None);

        let json = json!(null);
        let value = super::deserialize_option_date_time(json).unwrap();
        assert_eq!(value, None);

        let json = json!("2020-01-01T01:30:00");
        let value = super::deserialize_option_date_time(json).unwrap();
        let date = NaiveDate::from_ymd(2020, 1, 1);
        let time = NaiveTime::from_hms(1, 30, 0);
        assert_eq!(value, Some(NaiveDateTime::new(date, time)));
    }

    #[test]
    fn deserialize_option_time() {
        let json = json!("none");
        let value = super::deserialize_option_time(json).unwrap();
        assert_eq!(value, None);

        let json = json!(null);
        let value = super::deserialize_option_time(json).unwrap();
        assert_eq!(value, None);

        let json = json!("T02:00:20");
        let value = super::deserialize_option_time(json).unwrap();
        assert_eq!(value, Some(NaiveTime::from_hms(2, 0, 20)));
    }
}
