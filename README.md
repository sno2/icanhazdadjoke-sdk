# icanhazdadjoke-sdk

This is a crate for interacting with the [icanhazdadjoke API](https://icanhazdadjoke.com/).

Here are some examples for how you can use it:

## Examples

```rust
use icanhazdadjoke_sdk::{DadJokeSDK, PaginationOpts};
let dad_joke_sdk = DadJokeSDK::new("NAME_YOUR_APP".into()); // you can basically just put in here what your app is doing
let random_joke = dad_joke_sdk.get_random_joke().await; // gets a random joke
let paginated_jokes = dad_joke_sdk.query_jokes("pizza", PaginationOpts::default()).await; // you can specify more within the pagination options if you would like
```

### Contributing

First, install [cargo readme](https://crates.io/crates/cargo-readme). After that, just run the following command every time you edit the [README.md](./README.md) file:

```sh
cargo readme > README.md
```

Also, make sure you run `cargo test` to make sure all the code is still working.

### License

[See attached license](./LICENSE)

License: MIT
