use std::marker::PhantomData;
use cosmwasm_std::{from_json, to_json_binary, Addr, CosmosMsg, Empty, Response, WasmMsg};
use cw721::Cw721ReceiveMsg;

use cw721_base::helpers::Cw721Contract;
use sha3::{Digest, Keccak256};
use cw2981_royalties::ExecuteMsg as Cw2981ExecuteMsg;

use crate::{msg::{ExecuteLighthouseMsg, InnerMsg}, ContractError};


pub fn hash(address: &str) -> Vec<u8> {
    let mut hasher = Keccak256::new();
    hasher.update(address.as_bytes());
    hasher.finalize().to_vec()
}

pub fn receive_nft(receive_msg: Cw721ReceiveMsg) -> Result<Response, ContractError> {
    let token_id = receive_msg.token_id.as_str();
    let recipient = receive_msg.sender.as_str();

    // READ this: ReceiveNft{sender, token_id, msg} - This is designed to handle SendNft messages. The address of the contract is stored in env.sender so it cannot be faked. The contract should ensure the sender matches the token contract it expects to handle, and not allow arbitrary addresses.
    let lighthouse_addr = Addr::unchecked("sei12gjnfdh2kz06qg6e4y997jfgpat6xpv9dw58gtzn6g75ysy8yt5snzf4ac");
    let old_collection_addr = Addr::unchecked("sei1v97l92j2q8q0ywrg9hd553fxjx3r8kww5dxltcnfs7872sjm78fsu6vhcr");

    let inner_msg: InnerMsg = from_json(&receive_msg.msg)?;
    let new_collection_addr = inner_msg.new_collection_addr;
    let merkle_proof = inner_msg.merkle_proof;

    Cw721Contract::<Empty, Empty>(
        old_collection_addr.clone(),
        PhantomData,
        PhantomData
    ).call(Cw2981ExecuteMsg::Burn { token_id: token_id.to_string() }).unwrap();

    let mint_msg = ExecuteLighthouseMsg::MintNative {
        collection: new_collection_addr.to_string(),
        group: "public".to_string(),
        recipient: Some(Addr::unchecked(recipient)),
        merkle_proof,
        hashed_address: Some(hash(&recipient)),
    };

    let msg_binary = to_json_binary(&mint_msg)?;

    let _cosmos_msg: CosmosMsg<ExecuteLighthouseMsg> = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: lighthouse_addr.clone().to_string(),
        msg: msg_binary,
        funds: vec![],
    });



    let response = Response::new();
    // Return the response
    Ok(
        response
            .add_attribute("action", "burn_to_mint")
            .add_attribute("old_collection", old_collection_addr.to_string())
            .add_attribute("new_collection", new_collection_addr.to_string())
            .add_attribute("recipient", recipient.to_string())
            .add_attribute("token_id", token_id.to_string())
    )
}

