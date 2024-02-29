use std::marker::PhantomData;
use cosmwasm_std::{from_json, Addr, Empty, MessageInfo, Response};
use cw721::Cw721ReceiveMsg;
use cw721_base::msg::ExecuteMsg as Cw721ExecuteMsg;
use cw721_base::helpers::Cw721Contract;

use crate::{msg::InnerMsg, ContractError};

pub fn receive_nft(info: MessageInfo, receive_msg: Cw721ReceiveMsg) -> Result<Response, ContractError> {
    let token_id = receive_msg.token_id.as_str();
    let recipient = receive_msg.sender.as_str();
    let inner_msg: InnerMsg = from_json(&receive_msg.msg)?;
    let new_collection_addr = inner_msg.new_collection_addr;

    let old_collection_env_addr = info.sender.as_str();
    let old_collection_addr = Addr::unchecked("sei1exm3fjundhdzf6wng3xcny4nhjlawwmztxd286f35zcvx4mav4jqcnese6");
    if old_collection_addr != old_collection_env_addr {
        return Err(ContractError::Unauthorized {});
    }

    let burn_msg: cw721_base::ExecuteMsg<Empty, Empty> = Cw721ExecuteMsg::Burn {
        token_id: token_id.to_string()
    };
    let burn_callback = Cw721Contract::<Empty, Empty>(
        old_collection_addr.clone(),
        PhantomData,
        PhantomData
    ).call(burn_msg)?;

    let mint_msg: cw721_base::ExecuteMsg<Empty, Empty> = Cw721ExecuteMsg::TransferNft {
        recipient: recipient.to_string(),
        token_id: token_id.to_string() as String
    };
    let mint_callback = Cw721Contract::<Empty, Empty>(
        new_collection_addr.clone(),
        PhantomData,
        PhantomData
    ).call(mint_msg)?;

    let response = Response::new();
    // Return the response
    Ok(
        response
            .add_message(burn_callback)
            .add_message(mint_callback)
            .add_attribute("action", "burn_to_mint")
            .add_attribute("old_collection", old_collection_addr.to_string())
            .add_attribute("old_collection_env", old_collection_env_addr.to_string())
            .add_attribute("new_collection", new_collection_addr.to_string())
            .add_attribute("recipient", recipient.to_string())
            .add_attribute("token_id", token_id.to_string())
    )
}

