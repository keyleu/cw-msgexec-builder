use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Empty, Coin};
use cosmwasm_schema::QueryResponses;

#[cw_serde]
pub enum ExecMsg {
    Delegate { valaddress: Addr, granter: Addr, amount: Coin },
}

#[cw_serde]
pub enum InstantiateMsg {
    Instantiate{},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Empty)]
    Query {},
}