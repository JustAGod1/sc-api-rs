# Rust implementation of the Stalcraft API.

---

This library provides a Rust implementation of the Stalcraft API. You can
read more about the API at https://eapi.stalcraft.net/.

## With App Token

```rust
use sc_api::{DEMO_APP_TOKEN, DEMO_URL, ScAppClient};

#[tokio::main]
async fn main() {
    let client = ScAppClient::new_custom(DEMO_URL, DEMO_APP_TOKEN);
    println!("{:?}", client.get_clans_list("RU", None, None).await);
}

```

## With User Token

```rust
use sc_api::{DEMO_URL, DEMO_USER_TOKEN};

#[tokio::main]
async fn main() {
    let client = sc_api::ScUserClient::new_custom(DEMO_URL, DEMO_USER_TOKEN);
    // Get list of characters for user.
    let info = client.get_characters_list("RU").await.unwrap();
    println!("{:?}", info);

    // You can also use methods from ScAppClient.
    let info = client.app_client.get_clans_list("RU", None, None).await.unwrap();
    println!("{:?}", info);
}

```

## OAuth

This library does not provide any way to acquire tokens automatically. 