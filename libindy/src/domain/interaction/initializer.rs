use super::session::{Id as SID,Time};


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeVerification
{
    sid : SID,
    time_window: Time,
    verifier_did: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    signature: Option<String>

}

