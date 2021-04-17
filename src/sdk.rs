use super::models::*;

use hyper::{Body, Client, Method, Request, Uri};
use hyper_tls::HttpsConnector;

type FutureResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;
type HttpsClient = Client<HttpsConnector<hyper::client::HttpConnector>>;

#[derive(Debug)]
/// The structure used for manipulating the ianhazdadjoke API.
///
/// Internally, this manages the Hyper [`Client`] (for requests) and the User-Agent.
///
/// ## Examples
///
/// ```
/// use icanhazdadjoke_sdk::DadJokeSDK;
/// let dad_joke_sdk = DadJokeSDK::default(); // uses default user agent
/// ```
pub struct DadJokeSDK {
	client: HttpsClient,
	user_agent: String,
}

/// Initializes a Hyper [`Client`] using the HTTPS connectors from `hyper_tls`.
fn create_client() -> HttpsClient {
	let https = HttpsConnector::new();
	Client::builder().build::<_, Body>(https)
}

impl DadJokeSDK {
	/// Initializes the dad joke sdk with the given User-Agent that is sent with requests.
	pub fn new(user_agent: String) -> Self {
		DadJokeSDK {
			client: create_client(),
			user_agent,
		}
	}

	/// Gets a single joke by its given [`String`] id from the API.
	///
	/// ## Examples
	///
	/// ```ignore
	/// use icanhazdadjoke_sdk::DadJokeSDK;
	/// let dad_joke_sdk = DadJokeSDK::default();
	/// dad_joke_sdk.get_joke("0189hNRf2g".into()).await;
	/// ```
	pub async fn get_joke(self, id: String) -> FutureResult<Joke> {
		let payload = format!("https://icanhazdadjoke.com/j/{}", id);
		let payload_uri: Uri = payload.parse()?;
		let req = Request::builder()
			.header("Accept", "application/json")
			.header("User-Agent", self.user_agent)
			.method(Method::GET)
			.uri(payload_uri)
			.body(Body::empty())?;
		let res = self.client.request(req).await?;
		let bytes = hyper::body::to_bytes(res.into_body()).await?;
		let joke: Joke = serde_json::from_slice(bytes.to_vec().as_slice())?;
		Ok(joke)
	}

	/// Gets a random joke from the API.
	///
	/// ## Examples
	///
	/// ```ignore
	/// use icanhazdadjoke_sdk::DadJokeSDK;
	/// let dad_joke_sdk = DadJokeSDK::default();
	/// dad_joke_sdk.get_random_joke().await;
	/// ```
	///
	pub async fn get_random_joke(self) -> FutureResult<Joke> {
		let payload_uri: Uri = "https://icanhazdadjoke.com".parse()?;
		let req = Request::builder()
			.header("Accept", "application/json")
			.header("User-Agent", self.user_agent)
			.method(Method::GET)
			.uri(payload_uri)
			.body(Body::empty())?;
		let res = self.client.request(req).await?;
		let bytes = hyper::body::to_bytes(res.into_body()).await?;
		let joke: Joke = serde_json::from_slice(bytes.to_vec().as_slice())?;
		Ok(joke)
	}

	/// Searches for jokes that match the given [`String`] term and customizes the result pages based upon the [`PaginationOpts`] pagination options.
	pub async fn query_jokes(
		self,
		term: String,
		pagination_opts: PaginationOpts,
	) -> FutureResult<PaginatedData<PaginatedJoke>> {
		let payload_uri: Uri = format!(
			"https://icanhazdadjoke.com/search?term={}&page={}&limit={}",
			term, pagination_opts.page, pagination_opts.limit
		)
		.parse()?;
		let req = Request::builder()
			.header("Accept", "application/json")
			.header("User-Agent", self.user_agent)
			.method(Method::GET)
			.uri(payload_uri)
			.body(Body::empty())?;
		let res = self.client.request(req).await?;
		let bytes = hyper::body::to_bytes(res.into_body()).await?;
		let data: PaginatedData<PaginatedJoke> = serde_json::from_slice(bytes.to_vec().as_slice())?;
		Ok(data)
	}
}

impl std::default::Default for DadJokeSDK {
	fn default() -> Self {
		Self::new(
			"Anonymous icanhazdadjoke Rust SDK User (https://github.com/sno2/icanhazdadjoke-sdk)"
				.into(),
		)
	}
}
