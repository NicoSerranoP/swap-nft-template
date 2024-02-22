#[cfg(test)]
mod tests {
    use crate::{msg::ExecuteMsg, ContractError};
    use cosmwasm_schema::cw_serde;
    use cosmwasm_std::{from_json, to_json_binary, Addr, Attribute, Binary, DepsMut, Env, MessageInfo, Response, StdError};
    use cw721_base::entry::query;
    use cw_multi_test::{App, ContractWrapper, Executor};

    const ADMIN: &str = "sei1xvj6tqezcac83mdnql9ynx0fc6f2ljy5wz4ydu";

    #[cw_serde]
    pub enum InnerMsg {
        Succeed,
        Fail,
    }

    struct Contracts {
        nft_contract: Addr,
        receiver_contract: Addr,
    }

    #[cw_serde]
    pub struct InstantiateMsg {}

    #[test]
    fn test_cw721_base_receive_succeed() {
        use cw721_base::msg::*;

        let mut app = App::default();
        let admin = Addr::unchecked(ADMIN);

        let Contracts {
            nft_contract,
            receiver_contract,
        } = setup_contracts(&mut app, admin.clone());

        // send token to receiver contract
        let response = app
            .execute_contract(
                admin.clone(),
                nft_contract,
                &ExecuteMsg::<(), ()>::SendNft {
                    contract: receiver_contract.to_string(),
                    token_id: "test".to_string(),
                    msg: to_json_binary(&InnerMsg::Succeed).unwrap(),
                },
                &[],
            )
            .unwrap();
        let mut wasm_events = response.events.iter().filter(|e| e.ty == "wasm");

        let send_event = wasm_events.next().unwrap();
        assert_eq!(
            get_attribute(&send_event.attributes, "action"),
            Some("send_nft")
        );
        assert_eq!(
            get_attribute(&send_event.attributes, "token_id"),
            Some("test")
        );
        assert_eq!(
            get_attribute(&send_event.attributes, "recipient"),
            Some(receiver_contract.as_str())
        );

        let receive_event = wasm_events.next().unwrap();
        assert_eq!(
            get_attribute(&receive_event.attributes, "action"),
            Some("receive_nft")
        );
        assert_eq!(
            get_attribute(&receive_event.attributes, "token_id"),
            Some("test")
        );
        assert_eq!(
            get_attribute(&receive_event.attributes, "sender"),
            Some(admin.as_str()) // this is set to the sender of the original message
        );
    }

    #[test]
    fn test_cw721_base_receive_fail() {
        use cw721_base::msg::*;

        let mut app = App::default();
        let admin = Addr::unchecked(ADMIN);

        let Contracts {
            nft_contract,
            receiver_contract,
        } = setup_contracts(&mut app, admin.clone());

        // send fail message
        let result = app.execute_contract(
            admin.clone(),
            nft_contract.clone(),
            &ExecuteMsg::<(), ()>::SendNft {
                contract: receiver_contract.to_string(),
                token_id: "test".to_string(),
                msg: to_json_binary(&InnerMsg::Fail).unwrap(),
            },
            &[],
        );
        assert!(result.is_err());

        // send incorrect message
        let result = app.execute_contract(
            admin,
            nft_contract,
            &ExecuteMsg::<(), ()>::SendNft {
                contract: receiver_contract.to_string(),
                token_id: "test".to_string(),
                msg: Binary::from(br#"{"invalid": "fields"}"#),
            },
            &[],
        );
        assert!(result.is_err());
    }

    pub fn execute(
        _deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        match msg {
            ExecuteMsg::ReceiveNft(receive_msg) => {
                let inner: InnerMsg = from_json(&receive_msg.msg)?;
                match inner {
                    InnerMsg::Succeed => Ok(Response::new()
                        .add_attributes([
                            ("action", "receive_nft"),
                            ("token_id", receive_msg.token_id.as_str()),
                            ("sender", receive_msg.sender.as_str()),
                            ("msg", receive_msg.msg.to_base64().as_str()),
                        ])
                        .set_data(
                            [
                                receive_msg.token_id,
                                receive_msg.sender,
                                receive_msg.msg.to_base64(),
                            ]
                            .concat()
                            .as_bytes(),
                        )),
                    InnerMsg::Fail => Err(ContractError::Std(StdError::generic_err("Failed"))),
                }
            }
        }
    }

    pub fn instantiate(
        _deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        _msg: InstantiateMsg,
    ) -> Result<Response, ContractError> {
        Ok(Response::default())
    }

    /// Setup the cw721-receiver and cw721-base contracts and mint a test token
    fn setup_contracts(app: &mut App, admin: Addr) -> Contracts {
        use cw721_base::msg as base_msg;

        let code_id = app.store_code(Box::new(ContractWrapper::new(execute, instantiate, query)));
        let nft_code_id = app.store_code(Box::new(ContractWrapper::new(
            cw721_base::entry::execute,
            cw721_base::entry::instantiate,
            cw721_base::entry::query,
        )));

        // setup contracts
        let nft_contract = app
            .instantiate_contract(
                nft_code_id,
                admin.clone(),
                &base_msg::InstantiateMsg {
                    name: "nft".to_string(),
                    symbol: "NFT".to_string(),
                    minter: admin.to_string(),
                },
                &[],
                "nft".to_string(),
                None,
            )
            .unwrap();

        let receiver_contract = app
            .instantiate_contract(
                code_id,
                admin.clone(),
                &InstantiateMsg {},
                &[],
                "receiver".to_string(),
                None,
            )
            .unwrap();

        // mint token
        app.execute_contract(
            admin.clone(),
            nft_contract.clone(),
            &base_msg::ExecuteMsg::<(), ()>::Mint {
                token_id: "test".to_string(),
                owner: admin.to_string(),
                token_uri: Some("https://example.com".to_string()),
                extension: (),
            },
            &[],
        )
        .unwrap();

        Contracts {
            nft_contract,
            receiver_contract,
        }
    }

    fn get_attribute<'a>(attributes: &'a [Attribute], key: &str) -> Option<&'a str> {
        attributes
            .iter()
            .find(|a| a.key == key)
            .map(|a| a.value.as_str())
    }
}
