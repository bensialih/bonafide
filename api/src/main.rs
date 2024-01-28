// use axum::{
// 	routing::{get},
// 	Router,
// };

mod models;
mod endpoints;

use endpoints::{root, basic, list_ips};
use models::{Settings, Entry};
use tokio::net::{TcpListener, UdpSocket};
use std::io;
use bincode::{deserialize, deserialize_from};

#[tokio::main]
async fn main() -> io::Result<()> {
	let settings = Settings::new().unwrap();

	println!("starting server deployment on http://{}", settings.url());
	let listener = UdpSocket::bind(settings.url()).await.unwrap();

	let mut buf = [0; 1024];
	loop {
		listener.writable().await?;

		let (len, add) = listener.recv_from(&mut buf).await?;
		let item: Entry = serde_json::from_slice(&buf[..len]).unwrap();
		println!("Entry is {:?}", item);
	}
}
