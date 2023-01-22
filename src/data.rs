use serde::Deserialize;
use serde_json::{Map, Value};



#[derive(Deserialize, Debug)]
pub struct RegionId(pub String);

impl From<String> for RegionId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for RegionId {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl ToString for RegionId {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

#[derive(Deserialize, Debug)]
pub struct Region {
    pub id: RegionId,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct ItemId(pub String);

impl ToString for ItemId {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl From<String> for ItemId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for ItemId {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

#[derive(Deserialize, Debug)]
pub struct AuctionPriceHistory {
    pub total: i64,
    pub prices: Vec<PriceEntry>
}

#[derive(Deserialize, Debug)]
pub struct PriceEntry {
    pub amount: i32,
    pub price: i32,
    pub time: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Lot {
    pub item_id: ItemId,

    pub start_price: i64,

    pub current_price: Option<i64>,

    pub buyout_price: i64,

    pub start_time: String,

    pub end_time: String,

    pub additional: Map<String, Value>
}

#[derive(Deserialize, Debug)]
pub struct AuctionLots {
    pub total: i64,
    pub lots: Vec<Lot>
}

#[derive(Deserialize, Debug)]
pub struct CharacterId(pub String);

impl ToString for CharacterId {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl From<String> for CharacterId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for CharacterId {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CharacterMetaInfo {
    pub id: CharacterId,
    pub name: String,
    pub creation_time: String,
}

#[derive(Deserialize, Debug)]
pub struct ClanId(pub String);

impl ToString for ClanId {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl From<String> for ClanId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for ClanId {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClanInfo {
    pub id: ClanId,
    pub name: String,
    pub tag: String,
    pub level: i32,
    pub level_points: i32,
    pub registration_time: String,
    pub alliance: Option<String>,
    pub description: String,
    pub leader: String,
    pub member_count: i32
}

#[derive(Deserialize, Debug)]
pub enum Rank {
    RECRUIT,
    COMMONER,
    SOLDIER,
    SERGEANT,
    OFFICER,
    COLONEL,
    LEADER,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClanMember {
    pub name: String,
    pub rank: Rank,
    pub join_time: String,
}

#[derive(Deserialize, Debug)]
pub struct CharacterClanInfo {
    pub info: ClanInfo,
    pub member: ClanMember,
}

#[derive(Deserialize, Debug)]
pub struct Character {
    pub information: CharacterMetaInfo,
    pub clan: Option<CharacterClanInfo>,
}

#[derive(Deserialize, Debug)]
pub struct ClanMembers(pub Vec<ClanMember>);

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClansList {
    pub total_clans: i32,
    pub data: Vec<ClanInfo>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EmissionInformation {
    pub current_start: String,
    pub previous_start: String,
    pub previous_end: String,
}

#[derive(Deserialize, Debug)]
pub struct FriendsList(pub Vec<String>);

pub enum Sort {
    TimeCreated,
    TimeLeft,
    CurrentPrice,
    BuyoutPrice,
}

impl ToString for Sort {
    fn to_string(&self) -> String {
        match self {
            Sort::TimeCreated => "time_created",
            Sort::TimeLeft => "time_left",
            Sort::CurrentPrice => "current_price",
            Sort::BuyoutPrice => "buyout_price",
        }.to_string()
    }
}

pub enum Order {
    Asc,
    Desc,
}

impl ToString for Order {
    fn to_string(&self) -> String {
        match self {
            Order::Asc => "asc",
            Order::Desc => "desc",
        }.to_string()
    }
}
