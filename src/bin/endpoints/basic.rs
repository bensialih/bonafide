use axum::{
	http::StatusCode,
	Json, 
};

use crate::models::{Entry};


pub async fn root() -> &'static str {
	"here"
}

pub async fn basic() -> (StatusCode, Json<Entry>) {
	let entry = Entry::default();
	(StatusCode::OK, Json(entry))
}

pub async fn list_ips() -> (StatusCode, Json<Vec<Entry>>) {
	let mut entries = Vec::new();
	let entry = Entry::default();
	entries.push(entry);

	(StatusCode::OK, Json(entries))
}
