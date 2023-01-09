use cosmwasm_std::{DepsMut, Response, StdResult};
use cw2::set_contract_version;

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

//We include our generated Rust proto files.
include!("protos/mod.rs");

pub fn instantiate(deps: DepsMut) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let resp = Response::new().add_attribute("action", "Instantiation");

    Ok(resp)
}

pub mod exec {

    use cosmwasm_std::{Addr, Env, Response, StdResult, CosmosMsg, Binary};
    use protobuf::{well_known_types::any::Any, Message, MessageField};

    use super::{coin::Coin, CosmosMsgDelegate::MsgDelegate, CosmosMsgExec::MsgExec};

    pub fn delegate(
        valaddress: Addr,
        env: Env,
        granter: Addr,
        amount: cosmwasm_std::Coin,    //Need to specify as we are not passing a protobuf Coin structure
    ) -> StdResult<Response> { 
        let mut coin = Coin::new();

        coin.denom = amount.denom.to_string();
        coin.amount = amount.amount.to_string();

        let mut delegation_msg = MsgDelegate::new();

        delegation_msg.delegator_address = granter.to_string();
        delegation_msg.validator_address = valaddress.to_string();
        delegation_msg.amount = MessageField::some(coin);

        
        let mut any = Any::new();
        any.type_url = "/cosmos.staking.v1beta1.MsgDelegate".to_string();
        any.value = delegation_msg.write_to_bytes().unwrap();
        let mut messages: Vec<Any> = Vec::new();
        messages.push(any);

        //If the granter didn't grant this contract permission to delegate in his place, this execution will fail.
        let mut exec_msg = MsgExec::new();
        exec_msg.grantee = env.contract.address.to_string();
        exec_msg.msgs = messages;   

        let stargate_exec_msg: CosmosMsg = CosmosMsg::Stargate {
            type_url: "/cosmos.authz.v1beta1.MsgExec".to_string(),
            value: Binary::from(exec_msg.write_to_bytes().unwrap()),
        };

        Ok(Response::new()
            .add_message(stargate_exec_msg)
            .add_attribute("action", "delegating")
            .add_attribute("granter", granter.to_string())
            .add_attribute("validator", valaddress.to_string())
            .add_attribute("amount", amount.amount.to_string()))

        //This stakes the funds sent in the delegate execution of the contract using the smart contract as delegator address
        //No protobuf files required for this operation. We can't use this for our MsgExec as we can't modify the granter (it always delegates from the contract) so
        //we need to send funds to the contract which we don't want to do.
            
        /*let staking_msg = StakingMsg::Delegate { validator: valaddress.to_string(), amount: info.funds[0].clone() };

        Ok(Response::new()
            .add_message(staking_msg)
            .add_attribute("action", "delegating")
            .add_attribute("granter", granter.to_string())
            .add_attribute("validator", valaddress.to_string())
            .add_attribute("amount", info.funds[0].amount.to_string()))*/
    }
}
