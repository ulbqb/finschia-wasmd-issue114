#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    Binary, Deps, DepsMut, DistributionMsg, Env, MessageInfo, Reply, Response, StakingMsg,
    StdError, StdResult, SubMsg, SubMsgResult,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:delegator";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Handling contract instantiation
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let delegate_msg = StakingMsg::Delegate {
        validator: msg.validator,
        amount: info.funds[0].clone(),
    };

    Ok(Response::new()
        .add_message(delegate_msg)
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

/// Handling contract migration
/// To make a contract migratable, you need
/// - this entry_point implemented
/// - only contract admin can migrate, so admin has to be set at contract initiation time
/// Handling contract execution
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    match msg {
        // Find matched incoming message variant and execute them with your custom logic.
        //
        // With `Response` type, it is possible to dispatch message to invoke external logic.
        // See: https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md#dispatching-messages
    }
}

/// Handling contract execution
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::DebugSetWithdrawAddress {} => execute_debug_set_withdraw_address(),
        ExecuteMsg::DebugWithdrawDelegatorReward {} => {
            execute_debug_withdraw_delegator_reward(deps, env)
        }
    }
}

fn execute_debug_set_withdraw_address() -> Result<Response, ContractError> {
    let msg = DistributionMsg::SetWithdrawAddress {
        address: "link1cm0pgvsscsjveltaqucxh267vu2a3t60w7as0ydqgm".to_string(),
    };

    Ok(Response::new()
        .add_submessage(SubMsg::reply_always(msg, 0))
        .add_attribute("method", "execute")
        .add_attribute("action", "debug_set_withdraw_address"))
}

fn execute_debug_withdraw_delegator_reward(
    deps: DepsMut,
    env: Env,
) -> Result<Response, ContractError> {
    let delegations = deps
        .querier
        .query_all_delegations(env.contract.clone().address)?;

    let msg = DistributionMsg::WithdrawDelegatorReward {
        validator: delegations[0].clone().validator,
    };

    Ok(Response::new()
        .add_submessage(SubMsg::reply_always(msg, 1))
        .add_attribute("method", "execute")
        .add_attribute("action", "debug_withdraw_delegator_reward"))
}

/// Handling contract query
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        // Find matched incoming message variant and query them your custom logic
        // and then construct your query response with the type usually defined
        // `msg.rs` alongside with the query message itself.
        //
        // use `cosmwasm_std::to_binary` to serialize query response to json binary.
    }
}

/// Handling submessage reply.
/// For more info on submessage and reply, see https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md#submessages
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(_deps: DepsMut, _env: Env, msg: Reply) -> StdResult<Response> {
    match (msg.id, msg.result) {
        (0, SubMsgResult::Ok(response)) => Ok(Response::new()
            .add_attribute("action", "debug_set_withdraw_address")
            .add_attribute("reply_event_num", response.events.len().to_string())),
        (1, SubMsgResult::Ok(response)) => Ok(Response::new()
            .add_attribute("action", "deubg_withdraw_delegator_reward")
            .add_attribute("reply_event_num", response.events.len().to_string())),
        _ => Err(StdError::generic_err("invalid reply id or result")),
    }
}
