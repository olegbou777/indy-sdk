use super::session::{Id as SID, Time};


use domain::anoncreds::proof_request::ProofRequest as PRQ;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProofRequest
{
    sid : SID,
    time_window: Time,
    proof_request: PRQ,
    supports_rte: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    contract: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    signature: Option<String>
}