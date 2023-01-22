use async_mutex::Mutex;
use serde::de::DeserializeOwned;
use url::Url;
use crate::data::*;

pub static BASE_URL: &str = "https://eapi.stalcraft.net/";
pub static DEMO_URL: &str = "https://dapi.stalcraft.net";
pub static DEMO_APP_TOKEN: &str = "eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJhdWQiOiIxIiwibmJmIjoxNjczNzk3ODM4LCJleHAiOjQ4MjczOTc4MzgsImlhdCI6MTY3Mzc5NzgzOCwianRpIjoiYXhwbzAzenJwZWxkMHY5dDgzdzc1N2x6ajl1MmdyeHVodXVlb2xsZ3M2dml1YjVva3NwZTJ3eGFrdjJ1eWZxaDU5ZDE2ZTNlN2FqdW16Z3gifQ.ZNSsvwAX72xT5BzLqqYABuH2FGbOlfiXMK5aYO1H5llG51ZjcPvOYBDRR4HUoPZVLFY8jyFUsEXNM7SYz8qL9ePmLjJl6pib8FEtqVPmf9ldXvKkbaaaSp4KkJzsIEMY_Z5PejB2Vr-q-cL13KPgnLGUaSW-2X_sHPN7VZJNMjRgjw4mPiRZTe4CEpQq0BEcPrG6OLtU5qlZ6mLDJBjN2xtK0DI6xgmYriw_5qW1mj1nqF_ewtUiQ1KTVhDgXnaNUdkGsggAGqyicTei0td6DTKtnl3noD5VkipWn_CwSqb2Mhm16I9BPfX_d5ARzWrnrwPRUf6PA_7LipNU6KkkW0mhZfmwEPTm_sXPus0mHPENoVZArdFT3L5sOYBcpqwvVIEtxRUTdcsKp-y-gSzao5muoyPVoCc2LEeHEWx0cIi9spsZ46SPRQpN4baVFp7y5rp5pjRsBKHQYUJ0lTmh1_vyfzOzbtNN2v6W_5w9JTLrN1U6fhmifvKHppFSEqD6DameL1TC59kpIdufRkEU9HE4O-ErEf1GuJFRx-Dew6XDvb_ExhvEqcw31yNvKzpVqLYJfLazqn6tUbVuAiPwpy6rP9tYO2taT1vj5TGn_vxwDu9zoLWe796tFMPS-kmbCglxB5C9L4EbpfWNbWxYjUkTvjT2Ml9OnrB0UbYo1jI";
pub static DEMO_USER_TOKEN: &str = "eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJhdWQiOiIxIiwic3ViIjoiMSIsIm5iZiI6MTY3Mzc5NzgzOCwiZXhwIjo0ODI3Mzk3ODM4LCJpYXQiOjE2NzM3OTc4MzgsImp0aSI6IjJlamRwOG54a3A1djRnZWdhbWVyeWlkMW5ic24zZDhpZ2oyejgzem1vMDYzNjNoaXFkNWhwOTY1MHZwdWh4OXEybXBmd2hnbnUxNHR5cmp2In0.Ocw4CzkkuenkAOjkAR1RuFgLqix7VJ-8vWVS3KAJ1T3SgIWJG145xqG2qms99knu5azn_oaoeyMOXhyG_fuMQFGOju317GiS6pAXAFGOKvxcUCfdpFcEHO6TWGM8191-tlfV-0rAqCi62gprKyr-SrUG3nUJhv6XKegja_vYVujRVx0ouAaDvDKawiOssG5If_hXGhdhnmb3_7onnIc4hFsm4i9QVkWXe8GO6OsS999ZIX0ClNhTk2kKKTl2dDVIiKha_HB1aghm_LOYoRgb3i3B_DH4UO312rHYR5I4qO43c8x-TW7NwovItDSzhiCmcxZuUUeAUF3yFr5ovaR4fMj1LEy3y3V2piQDKPwmBOpI9S6OzWUIBJYcRYlT2HIrWCRc0YvM7AOGoxcH2Gf4ncqcF_M8fw7IMKf3pdnuxf1EbdEpzOapBD1Pw065em-U8PN4LVzw9lhIHx_Yj69qaFEx7Bhw3BCwsrx-o9hgg7T1TOV6kF11YfR99lIuj9z96XBLg5ipt-M_j7nHRoHWhM0Rc6uLIKPg0In0xYkybSfWG6v3Hs6kwgB7wkqpXpoVQltJvlqjtlf9Pp4zmkqlWQHx9as4xsgoTAQyCgaC0kisICNC58_g3QrJAfoFXW68x-OHlRKCAPqoR9V-0cVs-B83szaFmsEGegAttFLlDhE";

struct ScClient {
    client: reqwest::Client,
    base_url: String,
    token: String,
}

impl ScClient {
    fn new<T: Into<String>>(base_url: T, token: String) -> ScClient {
        ScClient {
            client: reqwest::Client::new(),
            base_url: base_url.into(),
            token,
        }
    }

    async fn request<'a, Path: Into<&'a str>, Output: DeserializeOwned>(
        &self,
        path: Path,
        parameters: &mut [(String, String)],
    ) -> Result<Output, Box<dyn std::error::Error>> {
        let mut url = Url::parse(&self.base_url).unwrap();
        url.set_path(path.into());
        for (key, value) in parameters {
            url.query_pairs_mut().append_pair(key, value);
        }

        let response = self.client.get(url)
            .header("Authorization", format!("Bearer {}", self.token))
            .send()
            .await?;
        if !response.status().is_success() {
            return Err(format!("Request failed: {}", response.status()).into());
        }
        let body = response.text().await?;
        let output: Output = serde_json::from_str(&body)?;
        Ok(output)
    }
}

pub async fn get_regions_list(url: &str) -> Result<Vec<Region>, Box<dyn std::error::Error>> {
    let mut url = Url::parse(url).unwrap();
    url.set_path("regions");

    let response = reqwest::get(url)
        .await?;
    let body = response.text().await?;
    let output = serde_json::from_str(&body)?;
    Ok(output)
}

/**
 * Client for requests that do not require user authentication.
 */
pub struct ScAppClient {
    inner: Mutex<ScClient>,
}

impl ScAppClient {
    pub fn new_custom<S: Into<String>>(base_url: S, user_token: impl Into<String>) -> Self {
        Self {
            inner: Mutex::new(ScClient::new(base_url, user_token.into()))
        }
    }
    pub fn new(user_token: String) -> Self {
        Self {
            inner: Mutex::new(ScClient::new(BASE_URL, user_token))
        }
    }
}

impl ScAppClient {
    async fn make_request<'a, Output: DeserializeOwned>(
        &self,
        path: &'a str,
        parameters: &mut [(String, String)],
    ) -> Result<Output, Box<dyn std::error::Error>> {
        let client = self.inner.lock().await;
        client.request(path, parameters).await
    }
}

macro_rules! params {
    () => {
        Vec::<(String, String)>::new()
    };
    ($($name: ident : $value: expr),+) => {
        {
            let mut params = Vec::new();
            $(
                if let Some(v) = $value {
                    params.push((stringify!($name).to_string(), v.to_string()));
                }
            )+
            params
        }
    };
}
macro_rules! request {
    ($name: ident, $result: path, $pattern: expr, [$($part: ident : $t: path),*], [$($param: ident : $tt: path), *]) => {
        pub async fn $name(&self, $($part: impl Into<$t>,)* $($param: Option<$tt>,)*) -> Result<$result, Box<dyn std::error::Error>> {
            let mut params = params!($($param: $param),*);
            let url = format!($pattern, $($part.into().to_string()),*);
            self.make_request(&url, &mut params).await
        }
    };
}


impl ScAppClient {
    request!(get_auction_price_history, crate::data::AuctionPriceHistory,
        "{}/auction/{}/history", [region: RegionId, item_id: ItemId], [offset: u32, limit: u32]
    );

    request!(get_auction_lots, crate::data::AuctionLots,
        "{}/auction/{}/lots", [region: RegionId, item_id: ItemId], [offset: u32, limit: u32, sort: Sort, order: Order]
    );
    request!(get_clan_information, crate::data::ClanInfo,
        "{}/clan/{}/info", [region: RegionId, clan_id: ClanId], []
    );
    request!(get_clans_list, ClansList,
        "{}/clans", [region: RegionId], [offset: u32, limit: u32]
    );
    request!(get_emission_information, EmissionInformation,
        "{}/emission", [region: RegionId], []
    );
}


/**
 * Client for requests that do require user authentication.
 */
pub struct ScUserClient {
    pub app_client: ScAppClient,
}

impl ScUserClient {
    pub fn new_custom<S: Into<String>>(base_url: S, user_token: impl Into<String>) -> ScUserClient {
        Self {
            app_client: ScAppClient::new_custom(base_url, user_token.into())
        }
    }
    pub fn new(user_token: String) -> Self {
        Self {
            app_client: ScAppClient::new(user_token)
        }
    }
}

impl ScUserClient {
    async fn make_request<'a, Output: DeserializeOwned>(
        &self,
        path: &'a str,
        parameters: &mut [(String, String)],
    ) -> Result<Output, Box<dyn std::error::Error>> {
        self.app_client.make_request(path, parameters).await
    }
}

impl ScUserClient {
    request!(get_characters_list, Vec<Character>,
        "{}/characters", [region: RegionId], []
    );

    request!(get_clan_members, Vec<ClanMember>,
        "{}/clan/{}/members", [region: RegionId, clan_id: ClanId], []
    );
    request!(get_list_of_friends, Vec<String>,
        "{}/friends/{}", [region: RegionId, character_id: CharacterId], []
    );
}


#[cfg(test)]
mod test {
    use super::*;

    fn demo_user() -> ScUserClient {
        ScUserClient::new_custom(DEMO_URL, DEMO_USER_TOKEN)
    }

    fn demo_app() -> ScAppClient {
        ScAppClient::new_custom(DEMO_URL, DEMO_APP_TOKEN)
    }

    #[tokio::test]
    async fn regions_list() {
        let regions = get_regions_list(DEMO_URL).await.unwrap();
        assert!(!regions.is_empty());
    }

    #[tokio::test]
    async fn auction_price_history() {
        let client = demo_app();
        let history = client.get_auction_price_history("RU", "3grl", None, None).await.unwrap();
        println!("{:?}", history);
    }

    #[tokio::test]
    async fn auction_lots() {
        let client = demo_app();
        let history = client.get_auction_lots("RU", "3grl", None, None, None, None).await.unwrap();
        println!("{:?}", history);
    }

    #[tokio::test]
    async fn clan_info() {
        let client = demo_app();
        let info = client.get_clan_information("RU", "647d6c53-b3d7-4d30-8d08-de874eb1d845").await.unwrap();
        println!("{:?}", info);
    }

    #[tokio::test]
    async fn characters_list() {
        let client = demo_user();
        let info = client.get_characters_list("RU").await.unwrap();
        println!("{:?}", info);
    }

    #[tokio::test]
    async fn clan_members() {
        let client = demo_user();
        let info = client.get_clan_members("RU", "647d6c53-b3d7-4d30-8d08-de874eb1d845").await.unwrap();
        println!("{:?}", info);
    }

    #[tokio::test]
    async fn clans_list() {
        let client = demo_app();
        let info = client.get_clans_list("RU", None, None).await.unwrap();
        println!("{:?}", info);
    }

    #[tokio::test]
    async fn clans_list_off_1() {
        let client = demo_app();
        let info = client.get_clans_list("RU", Some(1), None).await.unwrap();
        println!("{:?}", info);
    }

    #[tokio::test]
    async fn clans_list_lim_1() {
        let client = demo_app();
        let info = client.get_clans_list("RU", None, Some(1)).await.unwrap();
        assert_eq!(info.data.len(), 1);
        println!("{:?}", info);
    }

    #[tokio::test]
    async fn emission_info() {
        let client = demo_app();
        let info = client.get_emission_information("RU").await.unwrap();
        println!("{:?}", info);
    }

    #[tokio::test]
    async fn friends_list() {
        let client = demo_user();
        let info = client.get_list_of_friends("RU", "Test-1").await.unwrap();
        println!("{:?}", info);
    }
}