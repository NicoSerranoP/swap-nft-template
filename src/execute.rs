use std::marker::PhantomData;

use cosmwasm_std::{
    Addr, Empty, Response
};
use cw721::Cw721ReceiveMsg;
use cw721_base::helpers::Cw721Contract;
use cw2981_royalties::{ ExecuteMsg as Cw2981ExecuteMsg, Metadata as Cw2981Metadata };

use crate::{helpers::create_token_uri, ContractError};


pub fn receive_nft(receive_msg: Cw721ReceiveMsg) -> Result<Response, ContractError> {
    let token_id = receive_msg.token_id.as_str();
    let recipient = receive_msg.sender.as_str();
    let collection_addr = Addr::unchecked("sei1a3qpnlf474jvqecgu0pl0uvewpkd2cp3vd5cw8q2tsprecmk0y8qsmnhlu");

    Cw721Contract::<Empty, Empty>(
        collection_addr.clone(),
        PhantomData,
        PhantomData
    ).call(Cw2981ExecuteMsg::Burn { token_id: token_id.to_string() }).unwrap();

    // Init royalty extension
    let extension = Some(Cw2981Metadata {
        royalty_payment_address: Some("sei1xvj6tqezcac83mdnql9ynx0fc6f2ljy5wz4ydu".to_string()), // change the royalties address
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
        collection_addr.clone(),
        PhantomData,
        PhantomData
    ).call(mint_msg).unwrap();


    let response = Response::new();
    // Return the response
    Ok(
        response
            .add_message(callback)
            .add_attribute("action", "burn_to_mint")
            .add_attribute("collection", collection_addr.to_string())
            .add_attribute("recipient", recipient.to_string())
            .add_attribute("token_id", token_id.to_string())
    )
}

