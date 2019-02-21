use serde_json;
use futures::Future;
use indy::ledger;

use settings;
use utils::libindy::pool::get_pool_handle;
use utils::libindy::wallet::get_wallet_handle;
use utils::libindy::error_codes::map_rust_indy_sdk_error;
use error::prelude::*;

pub fn multisign_request(did: &str, request: &str) -> VcxResult<String> {
    ledger::multi_sign_request(get_wallet_handle(), did, request)
        .wait()
        .map_err(map_rust_indy_sdk_error)
}

pub fn libindy_sign_request(did: &str, request: &str) -> VcxResult<String> {
    ledger::sign_request(get_wallet_handle(), did, request)
        .wait()
        .map_err(map_rust_indy_sdk_error)
}

pub fn libindy_sign_and_submit_request(issuer_did: &str, request_json: &str) -> VcxResult<String> {
    if settings::test_indy_mode_enabled() { return Ok(r#"{"rc":"success"}"#.to_string()); }

    let pool_handle = get_pool_handle()?;
    let wallet_handle = get_wallet_handle();

    ledger::sign_and_submit_request(pool_handle, wallet_handle, issuer_did, request_json)
        .wait()
        .map_err(map_rust_indy_sdk_error)
}

pub fn libindy_submit_request(request_json: &str) -> VcxResult<String> {
    let pool_handle = get_pool_handle()?;

    //TODO there was timeout here (before future-based Rust wrapper)
    ledger::submit_request(pool_handle, request_json)
        .wait()
        .map_err(map_rust_indy_sdk_error)
}

pub fn libindy_build_get_txn_request(submitter_did: &str, sequence_num: i32) -> VcxResult<String> {
    ledger::build_get_txn_request(Some(submitter_did), None, sequence_num)
        .wait()
        .map_err(map_rust_indy_sdk_error)
}

pub fn libindy_build_schema_request(submitter_did: &str, data: &str) -> VcxResult<String> {
    ledger::build_schema_request(submitter_did, data)
        .wait()
        .map_err(map_rust_indy_sdk_error)
}

pub fn libindy_build_get_schema_request(submitter_did: &str, schema_id: &str) -> VcxResult<String> {
    ledger::build_get_schema_request(Some(submitter_did), schema_id)
        .wait()
        .map_err(map_rust_indy_sdk_error)
}

pub fn libindy_parse_get_schema_response(get_schema_response: &str) -> VcxResult<(String, String)> {
    ledger::parse_get_schema_response(get_schema_response)
        .wait()
        .map_err(map_rust_indy_sdk_error)
}

pub fn libindy_parse_get_cred_def_response(get_cred_def_response: &str) -> VcxResult<(String, String)> {
    ledger::parse_get_cred_def_response(get_cred_def_response)
        .wait()
        .map_err(map_rust_indy_sdk_error)
}

pub fn libindy_build_get_credential_def_txn(cred_def_id: &str) -> VcxResult<String> {
    let submitter_did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID)?;
    ledger::build_get_cred_def_request(Some(&submitter_did), cred_def_id)
        .wait()
        .map_err(map_rust_indy_sdk_error)
}

pub fn libindy_build_create_credential_def_txn(submitter_did: &str,
                                               credential_def_json: &str) -> VcxResult<String> {
    ledger::build_cred_def_request(submitter_did, credential_def_json)
        .wait()
        .map_err(map_rust_indy_sdk_error)
}

pub fn parse_response(response: &str) -> VcxResult<Response> {
    serde_json::from_str::<Response>(response)
        .to_vcx(VcxErrorKind::InvalidJson, "Cannot deserialize transaction response")
}

#[serde(tag = "op")]
#[derive(Deserialize, Debug)]
pub enum Response {
    #[serde(rename = "REQNACK")]
    ReqNACK(Reject),
    #[serde(rename = "REJECT")]
    Reject(Reject),
    #[serde(rename = "REPLY")]
    Reply(Reply),
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Reject {
    pub reason: String
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Reply {
    ReplyV0(ReplyV0),
    ReplyV1(ReplyV1)
}

#[derive(Debug, Deserialize)]
pub struct ReplyV0 {
    pub result: serde_json::Value
}

#[derive(Debug, Deserialize)]
pub struct ReplyV1 {
    pub data: ReplyDataV1
}

#[derive(Debug, Deserialize)]
pub struct ReplyDataV1 {
    pub  result: serde_json::Value
}
