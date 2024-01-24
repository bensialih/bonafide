use axum::{
	routing::{get},
	Router,
};

mod models;
mod endpoints;
use endpoints::{root, basic, list_ips};


#[tokio::main]
async fn main() {
	let app = Router::new()
		.route("/", get(root))
		.route("/entry/", get(basic))
		.route("/ips/", get(list_ips));

	let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
	axum::serve(listener, app).await.unwrap();
}


