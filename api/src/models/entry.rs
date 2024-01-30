use chrono::{NaiveDateTime, NaiveDate, NaiveTime};
use std::net::Ipv4Addr;
use serde::{Serialize, Deserialize};
use serde::de;
use std::fmt;


#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
	pub ip: Ipv4Addr,
	pub datetime: Option<NaiveDateTime>,
}

struct DateTimeVisitor;

const DEFAULT_FORMAT: &str = "%Y-%m-%dT%H:%M:%S.%f";


impl <'de> de::Visitor<'de> for DateTimeVisitor {
	type Value = NaiveDateTime;

	fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		write!(formatter, "a string represents chrono::NaiveDateTime")
	}

	fn visit_str<E>(self, s: &str) -> Result<Self::Value, E> 
	where
		E: de::Error,
	{
		match NaiveDateTime::parse_from_str(s, DEFAULT_FORMAT) {
			Ok(t) =>  Ok(t.into()),
			Err(_) => Err(de::Error::invalid_value(de::Unexpected::Str(s), &self)),
		}
	}
}


impl Default for Entry {
	fn default() -> Entry {
		let dt = NaiveDate::from_ymd_opt(2015, 6, 3).unwrap();
		let tim = NaiveTime::from_hms_milli_opt(12, 34, 56, 789).unwrap();

		Entry {
			ip: Ipv4Addr::new(127, 0, 0, 1),
			datetime: Some(NaiveDateTime::new(dt, tim)),
		}
	}
}


#[cfg(test)]
mod tests {
	use crate::models::{Entry};
	use std::net::Ipv4Addr;
	use chrono::{NaiveDate};
	use serde_json;

	#[test]
	fn datetime_hardcoded() {
		let entry = Entry::default();
		let dt = entry.datetime.unwrap();
		assert_eq!(
			dt.date(),
			NaiveDate::from_ymd_opt(2015, 6, 3).unwrap()
		);
	}

	#[test]
	fn datetime_from_string() {
		let result: Entry =
        serde_json::from_str(r#"{"ip": "127.0.0.1", "datetime": "2019-08-15T17:41:18.106108"}"#)
            .unwrap();

        assert_eq!(result.ip, Ipv4Addr::new(127, 0, 0, 1));
        assert_eq!(
        	result.datetime.unwrap().date(),
        	NaiveDate::from_ymd_opt(2019, 8, 15).unwrap()
    	);
	}
}
