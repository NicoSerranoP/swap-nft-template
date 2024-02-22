use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;

/// CwTemplateContract is a wrapper around Addr that provides a lot of helpers
/// for working with this.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CwTemplateContract(pub Addr);

pub fn create_token_uri(token_uri: &str, token_id: &str, iterated_uri: &bool) -> String {
    if !iterated_uri {
        format!("{}/{}", token_uri, token_id)
    } else {
        token_uri.to_string()
    }
}
