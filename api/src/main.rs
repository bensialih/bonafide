use axum::{
	routing::{get},
	Router,
};

mod models;
mod endpoints;
use endpoints::{root, basic, list_ips};

const PORT: u16 = 3000;

#[tokio::main]
async fn main() {
	println!("starting application on port {:?}", PORT);
	let app = Router::new()
		.route("/", get(root))
		.route("/entry/", get(basic))
		.route("/ips/", get(list_ips));

	let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", PORT)).await.unwrap();
	axum::serve(listener, app).await.unwrap();
}
