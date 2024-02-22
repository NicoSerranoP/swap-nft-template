use std::marker::PhantomData;

use cosmwasm_std::{from_json, Addr, Empty, Response};
use cw721::Cw721ReceiveMsg;
use cw721_base::helpers::Cw721Contract;
use cw2981_royalties::{ ExecuteMsg as Cw2981ExecuteMsg, Metadata as Cw2981Metadata };

use crate::{helpers::create_token_uri, msg::InnerMsg, ContractError};


pub fn receive_nft(receive_msg: Cw721ReceiveMsg) -> Result<Response, ContractError> {
    let token_id = receive_msg.token_id.as_str();
    let recipient = receive_msg.sender.as_str();
    let inner_msg: InnerMsg = from_json(&receive_msg.msg)?;
    // READ this: ReceiveNft{sender, token_id, msg} - This is designed to handle SendNft messages. The address of the contract is stored in env.sender so it cannot be faked. The contract should ensure the sender matches the token contract it expects to handle, and not allow arbitrary addresses.
    let old_collection_addr = Addr::unchecked("sei1v97l92j2q8q0ywrg9hd553fxjx3r8kww5dxltcnfs7872sjm78fsu6vhcr");
    let new_collection_addr = inner_msg.new_collection_addr;

    Cw721Contract::<Empty, Empty>(
        old_collection_addr.clone(),
        PhantomData,
        PhantomData
    ).call(Cw2981ExecuteMsg::Burn { token_id: token_id.to_string() }).unwrap();

    // Init royalty extension
    let extension = Some(Cw2981Metadata {
        royalty_payment_address: Some("sei1qstajuq9svlx8t905z5rgktt9f93atazs6243d".to_string()), // change the royalties address
        royalty_percentage: Some(6), // change the royalty percentage
        ..Cw2981Metadata::default()
    });

    // Prepare the mint message
    let mint_msg = Cw2981ExecuteMsg::Mint {
        token_id: token_id.to_string(),
        owner: recipient.to_string(),
        token_uri: Some(
            create_token_uri(
                "https://arweave.net/xl8OmVJNJH-fcFAQolUozUXGmtk18ItdGkl1we-rP8Y", // change the arweave link
                token_id,
                &false
            )
        ),
        extension,
    };

    // Send the mint message
    let callback = Cw721Contract::<Empty, Empty>(
        new_collection_addr.clone(),
        PhantomData,
        PhantomData
    ).call(mint_msg).unwrap();


    let response = Response::new();
    // Return the response
    Ok(
        response
            .add_message(callback)
            .add_attribute("action", "burn_to_mint")
            .add_attribute("old_collection", old_collection_addr.to_string())
            .add_attribute("new_collection", new_collection_addr.to_string())
            .add_attribute("recipient", recipient.to_string())
            .add_attribute("token_id", token_id.to_string())
    )
}

