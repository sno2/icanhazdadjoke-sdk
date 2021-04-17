#[derive(Debug, Clone, Copy)]
pub struct PaginationOpts {
	pub page: u32,
	pub limit: u8,
}

impl std::default::Default for PaginationOpts {
	fn default() -> Self {
		Self { page: 0, limit: 20 }
	}
}
