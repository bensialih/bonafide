use chrono::{Utc, DateTime};
use chrono::serde::ts_seconds_option;
use std::net::Ipv4Addr;
use serde::{Serialize, Deserialize};



#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
	ip: Ipv4Addr,
	#[serde(with = "ts_seconds_option")]
	datetime: Option<DateTime<Utc>>,
}

impl Default for Entry {
	fn default() -> Entry {
		Entry {
			ip: Ipv4Addr::new(127, 0, 0, 1),
			datetime: Some(Utc::now()),
		}
	}
}