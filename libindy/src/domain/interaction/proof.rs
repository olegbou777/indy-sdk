use super::session::{Id as SID,Time};
use super::proof_request::ProofRequest;

#[derive(Serialize, Deserialize)]
pub struct EncryptedProof {
    v : String
}

#[derive(Serialize, Deserialize)]
pub struct EncryptedRecipientKey {
    v : String
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProverSignature {
    sid: SID,
    time_window: Time,
    verifier_did: String,
    notary_did: String,
    proof_request: ProofRequest,
    recipient_key_hash: String,
    proof_hash: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    signature: Option<String>
}


#[derive(Serialize, Deserialize)]
pub struct Proof {
    ps: ProverSignature,
    ek: EncryptedRecipientKey,
    ep: EncryptedProof
}

