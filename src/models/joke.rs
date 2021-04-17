use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Joke {
	pub id: String,
	pub joke: String,
	pub status: u16,
}
