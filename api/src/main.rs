mod models;
mod endpoints;

// use endpoints::{root, basic, list_ips};
use models::{Settings, Entry, create_pool};
use tokio::net::{ UdpSocket };
use std::io;

use deadpool_postgres::{Pool, Object, Manager, Client};
use tokio_postgres;
use chrono::{TimeZone, Utc};

async fn insert_entry(entry: &Entry, client: &Client) -> Result<u64, tokio_postgres::Error> {
	client.execute(
		"INSERT INTO logs (ip, datetime) VALUES ($1, $2)",
		&[
			&entry.ip.to_string(),
			&Utc.from_local_datetime(&entry.datetime.unwrap()).unwrap()
		]
	).await
}

#[tokio::main]
async fn main() -> io::Result<()> {
	let settings = Settings::new("./settings").unwrap();

	println!("starting server deployment on http://{}", settings.url());
	let listener = UdpSocket::bind(settings.url()).await.unwrap();

	let mut buf = [0; 1024];

	let db_pool = create_pool(settings.database); 
	
	
	loop {
		listener.writable().await?;
		let (len, add) = listener.recv_from(&mut buf).await?;
		let item: Entry = serde_json::from_slice(&buf[..len]).unwrap();

		let client: Client = db_pool.get().await.unwrap();

		insert_entry(&item, &client).await.unwrap();
	}
}
