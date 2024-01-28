// settings should have
use std::io;
use std::net::Ipv4Addr;
use config::{Config, File};

use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct ServerSettings {
	host: Ipv4Addr,
	port: i32,
}

#[derive(Deserialize, Debug)]
pub struct Settings {
	pub server: ServerSettings,
}


impl Settings {
	pub fn new() -> Result<Settings, io::Error> {

		let config = Config::builder()
			.add_source(File::with_name("./settings"))
			.build().expect("expected file");

		let settings = config.try_deserialize::<Settings>().unwrap();
		Ok(settings)
	}

	pub fn url(self: &Self) -> String {
		format!("{}:{}", self.server.host, self.server.port)
	}
}


