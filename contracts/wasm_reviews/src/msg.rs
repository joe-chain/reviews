use cosmwasm_schema::{cw_serde, QueryResponses};

use crate::state::State;

#[cw_serde]
pub struct InstantiateMsg {
    pub cooldown_blocks: Option<u64>,
    pub max_saved: Option<u8>,
}

#[cw_serde]
pub enum ExecuteMsg {
    // writes a review to the state, and adds a cooldown for when the next review from this account can take place.
    // At max 50 reviews are saved at 1 time. THen enw reviews purge old ones
    Review { text: String },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Reviews)]
    GetReviews {},
}

// We define a custom struct for each query response
#[cw_serde]
pub struct Reviews {
    pub reviews: State,
}
