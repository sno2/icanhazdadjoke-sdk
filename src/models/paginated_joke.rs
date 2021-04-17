use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PaginatedJoke {
	pub id: String,
	pub joke: String,
}
