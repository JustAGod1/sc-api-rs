use sc_api::BASE_URL;

/**
* As get_regions_list does not require authentication, it is not a method of any client.
*/
#[tokio::main]
async fn main() {
    println!("{:?}", sc_api::get_regions_list(BASE_URL).await.unwrap());
}