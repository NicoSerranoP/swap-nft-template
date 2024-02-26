use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw721::Cw721ReceiveMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[cw_serde]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    ReceiveNft(Cw721ReceiveMsg)
}


#[cw_serde]
pub struct InnerMsg {
    pub new_collection_addr: Addr
}


#[cw_serde]
pub enum ExecuteLighthouseMsg {
    MintNative {
        collection: String,
        group: String,
        recipient: Option<Addr>,
        merkle_proof: Option<Vec<Vec<u8>>>,
        hashed_address: Option<Vec<u8>>,
    }
}
