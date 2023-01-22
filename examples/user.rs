use sc_api::{DEMO_URL, DEMO_USER_TOKEN};

/**
* Example of using stalcraft api with user token.
*/
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