use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub(crate) struct RaidresResponse {
    #[serde(rename = "raidId")]
    pub(crate) raid_id: i32,
    pub(crate) reservations: Vec<ReservationData>,
    #[serde(rename = "disabledRaidItemIds")]
    pub(crate) disabled_raid_item_ids: Vec<i32>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct ReservationData {
    #[serde(rename = "raidItemId")]
    pub(crate) raid_item_id: Option<i32>,
    pub(crate) character: Character,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Character {
    pub(crate) name: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct RaidResponse {
    pub(crate) name: String,
    #[serde(rename = "raidItems")]
    pub(crate) raid_items: Vec<RaidItem>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct RaidItem {
    pub(crate) id: i32,
    #[serde(rename = "turtleDbItemId")]
    pub(crate) turtle_db_item_id: i32,
    pub(crate) quality: i32,
}

#[derive(Serialize)]
pub(crate) struct Output {
    pub(crate) metadata: Metadata,
    pub(crate) softreserves: Vec<SoftReserve>,
    pub(crate) hardreserves: Vec<Item>,
}

#[derive(Serialize)]
pub(crate) struct Metadata {
    pub(crate) id: String,
    pub(crate) instance: i32,
    pub(crate) instances: Vec<String>,
}

#[derive(Serialize)]
pub(crate) struct SoftReserve {
    pub(crate) name: String,
    pub(crate) items: Vec<Item>,
}

#[derive(Serialize)]
pub(crate) struct Item {
    pub(crate) id: i32,
    pub(crate) quality: i32,
}
