use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PaginatedData<T: Serialize> {
	pub current_page: u32,
	pub limit: u8,
	pub next_page: u32,
	pub previous_page: u32,
	pub results: Vec<T>,
	pub search_term: String,
	pub status: u16,
	pub total_jokes: u32,
	pub total_pages: u32,
}
