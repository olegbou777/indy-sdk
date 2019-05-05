use super::session::{Id as SID, Time};
use super::proof::ProverSignature;
use super::proof::EncryptedRecipientKey;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifierSignature {
    sid: SID,
    time_window: Time,
    notary_did: String,
    proof_hash: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    signature: Option<String>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotarizeRequest {
    ps: ProverSignature,
    vs: VerifierSignature,
    ek: EncryptedRecipientKey
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotaryRecord  {
    sid: SID,
    timestamp: Time,
    prover_signature_hash: String,
    verifier_signature_hash: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    signature: Option<String>
}