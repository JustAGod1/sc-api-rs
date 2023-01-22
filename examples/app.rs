use sc_api::{DEMO_APP_TOKEN, DEMO_URL, ScAppClient};

/**
* Example of using stalcraft api with app token
*/
#[tokio::main]
async fn main() {
    let client = ScAppClient::new_custom(DEMO_URL, DEMO_APP_TOKEN);
    println!("{:?}", client.get_clans_list("RU", None, None).await);
}