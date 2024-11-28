use cosmwasm_std::{entry_point, Deps, DepsMut, Env, MessageInfo, StdResult, Response, to_json_binary, Binary, StdError};
use schemars::JsonSchema;
use bincode;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum ExecuteMsg {
    SetString { value: String },
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum QueryMsg {
    GetString {},
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub string: String,
}

#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    // No initial state
    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::SetString { value } => {
            let state = State { string: value };
            match bincode::serialize(&state) {
                Ok(serialized_state) => {
                    deps.storage.set(b"string", &serialized_state);
                    Ok(Response::new()
                        .add_attribute("action", "set_string")
                        .add_attribute("sender", info.sender))
                }
                Err(e) => Err(StdError::generic_err(format!("Serialization error: {}", e))),
            }
        }
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetString {} => {
            match deps.storage.get(b"string") {
                Some(state_bytes) => match bincode::deserialize::<State>(&state_bytes) { 
                    Ok(state) => to_json_binary(&state.string),  
                    Err(e) => Err(StdError::generic_err(format!("Deserialization error: {}", e))),
                },
                None => Err(StdError::not_found("State not found").into()),
            }
        }
    }
}
