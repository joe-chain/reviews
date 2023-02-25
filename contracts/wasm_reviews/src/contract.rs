use std::collections::BTreeMap;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, Reviews};
use crate::state::{Cooldowns, State, COOLDOWN, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw20-burn";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let state = State {
        max_saved: msg.max_saved.unwrap_or(50),
        cooldown_blocks: msg.cooldown_blocks.unwrap_or(25),
        reviews: Vec::new(),
    };

    STATE.save(deps.storage, &state)?;

    let cooldowns = Cooldowns {
        cooldown: BTreeMap::new(),
    };

    COOLDOWN.save(deps.storage, &cooldowns)?;

    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Review { text } => {
            let mut state: State = STATE.load(deps.storage)?;
            let mut cooldown = COOLDOWN.load(deps.storage)?;

            // check if the user has a cooldown
            if let Some(block) = cooldown.cooldown.get(&_info.sender.to_string()) {
                // if the block is > than the current block, return an error
                if block > &env.block.height {
                    return Err(ContractError::OnCooldown {
                        blocks_until: block - env.block.height,
                    });
                }
            }

            if text.is_empty() || text.len() > 255 {
                return Err(ContractError::InvalidReviewLength {
                    length: text.len() as u64,
                });
            }

            // remove oldest review
            if state.reviews.len() >= state.max_saved as usize {
                // reviews is a Vec<String>, so remove the last element in the array
                state.reviews.remove(state.reviews.len() - 1);
            }

            // add the new review to the top of the array (so we do not have to reverse it on query)
            state.reviews.insert(0, text);
            STATE.save(deps.storage, &state)?;

            // add a cooldown
            let cooldown_block = env.block.height + state.cooldown_blocks;

            // sets a cooldown for 25 blocks from now. Once this is up, the user can review again even if their key is still in the cooldown map
            cooldown
                .cooldown
                .insert(_info.sender.to_string(), cooldown_block);
            COOLDOWN.save(deps.storage, &cooldown)?;

            Ok(Response::new().add_attribute("method", "review"))
        }
    }
}
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetReviews {} => {
            let reviews = STATE.load(deps.storage)?;

            to_binary(&Reviews { reviews })
        }
    }
}
