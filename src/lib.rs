use cosmwasm_std::{Deps, DepsMut, Env, MessageInfo, Response, StdResult, Binary};

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use msg::{InstantiateMsg, QueryMsg};

mod contract;
pub mod msg;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    contract::instantiate(deps)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary>{
    Ok(Binary::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: msg::ExecMsg,
) -> StdResult<Response> {
    use contract::exec;
    use msg::ExecMsg::*;

    match msg {
        Delegate { valaddress, granter, amount} => exec::delegate(valaddress, env, granter, amount),
    }
}
