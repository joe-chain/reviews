use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("You are on cooldown, try again in a {blocks_until:?} blocks")]
    OnCooldown { blocks_until: u64 },

    #[error("Invalid Review length {length:?}")]
    InvalidReviewLength { length: u64 },
}
