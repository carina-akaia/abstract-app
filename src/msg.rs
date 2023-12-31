use cosmwasm_schema::QueryResponses;

use crate::contract::DemoApp;

// This is used for type safety and re-exporting the contract endpoint structs.
abstract_app::app_msg_types!(DemoApp, AppExecuteMsg, AppQueryMsg);

/// DemoApp instantiate message
#[cosmwasm_schema::cw_serde]
pub struct AppInstantiateMsg {}

/// DemoApp execute messages
#[cosmwasm_schema::cw_serde]
#[cfg_attr(feature = "interface", derive(cw_orch::ExecuteFns))]
#[cfg_attr(feature = "interface", impl_into(ExecuteMsg))]
pub enum AppExecuteMsg {
    UpdateConfig {},
}

/// DemoApp query messages
#[cosmwasm_schema::cw_serde]
#[cfg_attr(feature = "interface", derive(cw_orch::QueryFns))]
#[cfg_attr(feature = "interface", impl_into(QueryMsg))]
#[derive(QueryResponses)]
pub enum AppQueryMsg {
    #[returns(ConfigResponse)]
    Config {},
}

#[cosmwasm_schema::cw_serde]
pub enum AppMigrateMsg {}

#[cosmwasm_schema::cw_serde]
pub struct ConfigResponse {}
