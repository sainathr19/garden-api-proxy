use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,Debug)]
pub enum ResponseStatus{
    Ok,
    Error
}

#[derive(Serialize,Deserialize,Debug)]
pub struct FetchNonceResponse{
    pub status : ResponseStatus,
    pub error : Option<String>,
    pub result : Option<String>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SiweMessage {
    pub message: String,
    pub signature: String,
    pub nonce: String,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct VerifySignatureResponse{
    pub status : ResponseStatus,
    pub error : Option<String>,
    pub result : Option<String>
}
