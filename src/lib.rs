//! This is a crate for interacting with the [icanhazdadjoke API](https://icanhazdadjoke.com/).
//!
//! Here are some examples for how you can use it:
//!
//! # Examples
//!
//! ```ignore
//! use icanhazdadjoke_sdk::{DadJokeSDK, PaginationOpts};
//! let dad_joke_sdk = DadJokeSDK::new("NAME_YOUR_APP".into()); // you can basically just put in here what your app is doing
//! let random_joke = dad_joke_sdk.get_random_joke().await; // gets a random joke
//! let paginated_jokes = dad_joke_sdk.query_jokes("pizza", PaginationOpts::default()).await; // you can specify more within the pagination options if you would like
//! ```
//!
//! ## Documentation
//!
//! Check out the full documentation at [docs.rs](https://docs.rs/icanhazdadjoke-sdk).
//!
//! ## Contributing
//!
//! First, install [cargo readme](https://crates.io/crates/cargo-readme). After that, just run the following command every time you edit the [README.md](./README.md) file:
//!
//! ```sh
//! cargo readme > README.md
//! ```
//!
//! Also, make sure you run `cargo test` to make sure all the code is still working.
//!
//! ## License
//!
//! [See attached license](./LICENSE)

pub mod models;
mod sdk;

pub use models::PaginationOpts;
pub use sdk::DadJokeSDK;

#[cfg(test)]
mod tests {
	use super::*;
	type FutureResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

	const TESTING_USER_AGENT: &'static str =
		"icanhazdadjoke Rust SDK Testing (https://github.com/sno2/icanhazdadjoke-sdk)";

	#[tokio::test]
	async fn getting_single_joke() -> FutureResult<()> {
		let dad_joke_sdk = DadJokeSDK::new(TESTING_USER_AGENT.into());

		const JOKE_ID: &'_ str = "0189hNRf2g";
		const JOKE_CONTENT: &'_ str = "I'm tired of following my dreams. I'm just going to ask them where they are going and meet up with them later.";

		let joke = dad_joke_sdk.get_joke(JOKE_ID.into()).await?;

		assert_eq!(joke.status, 200);
		assert_eq!(joke.id, String::from(JOKE_ID));
		assert_eq!(joke.joke, String::from(JOKE_CONTENT));

		Ok(())
	}

	#[tokio::test]
	async fn getting_random_joke() -> FutureResult<()> {
		let dad_joke_sdk = DadJokeSDK::new(TESTING_USER_AGENT.into());

		let joke = dad_joke_sdk.get_random_joke().await?;

		assert_eq!(joke.status, 200);

		Ok(())
	}

	#[tokio::test]
	async fn getting_paginated_jokes() -> FutureResult<()> {
		let dad_joke_sdk = DadJokeSDK::new(TESTING_USER_AGENT.into());

		let pagination_opts = PaginationOpts::default();

		let paginated_jokes = dad_joke_sdk.query_jokes("".into(), pagination_opts).await?;

		assert!(!paginated_jokes.results.is_empty());
		assert!(paginated_jokes.results.len() <= pagination_opts.limit as usize);

		Ok(())
	}
}
