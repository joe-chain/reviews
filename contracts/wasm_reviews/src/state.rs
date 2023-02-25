use std::collections::BTreeMap;

use cosmwasm_schema::cw_serde;
use cw_storage_plus::Item;

#[cw_serde]
pub struct State {
    pub max_saved: u8,
    pub cooldown_blocks: u64,
    pub reviews: Vec<String>,
}

#[cw_serde]
pub struct Cooldowns {
    // addr: block they can review again
    pub cooldown: BTreeMap<String, u64>,
}

pub const STATE: Item<State> = Item::new("state");

pub const COOLDOWN: Item<Cooldowns> = Item::new("cooldown");
