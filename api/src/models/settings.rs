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
pub struct DatabaseSettings {
	pub host: Ipv4Addr,
	pub user: String,
	pub password: Option<String>,
	pub dbname: String,
	pub port: u16,
}

#[derive(Deserialize, Debug)]
pub struct Settings {
	pub server: ServerSettings,
	pub database: DatabaseSettings,
}


impl Settings {
	pub fn new(filename: &'static str) -> Result<Settings, io::Error> {

		let config = Config::builder()
			.add_source(File::with_name(filename))
			.build()
			.expect(format!("expected file {} not found", filename).as_str());

		let settings = config.try_deserialize::<Settings>().unwrap();
		Ok(settings)
	}

	pub fn url(self: &Self) -> String {
		format!("{}:{}", self.server.host, self.server.port)
	}
}


