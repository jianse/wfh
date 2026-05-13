use serde::Deserialize;
use serde::Serialize;

/// Order status as defined in ACME protocol (RFC 8555)
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum OrderStatus {
    Pending,
    Ready,
    Processing,
    Valid,
    Invalid,
}

impl std::fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderStatus::Pending => write!(f, "pending"),
            OrderStatus::Ready => write!(f, "ready"),
            OrderStatus::Processing => write!(f, "processing"),
            OrderStatus::Valid => write!(f, "valid"),
            OrderStatus::Invalid => write!(f, "invalid"),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct OrderRes {
    pub status: OrderStatus,
    pub expires: Option<String>,
    pub identifiers: Vec<Identifier>,
    #[serde(rename = "notBefore", skip_serializing_if = "Option::is_none")]
    pub not_before: Option<String>,

    #[serde(rename = "notAfter", skip_serializing_if = "Option::is_none")]
    pub not_after: Option<String>,

    pub error: Option<()>,

    pub authorizations: Vec<String>,

    pub finalize: String,

    pub certificate: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct NewOrderPayload {
    pub(crate) identifiers: Vec<Identifier>,

    #[serde(rename = "notBefore", skip_serializing_if = "Option::is_none")]
    pub(crate) not_before: Option<String>,

    #[serde(rename = "notAfter", skip_serializing_if = "Option::is_none")]
    pub(crate) not_after: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Identifier {
    #[serde(rename = "type")]
    pub(crate) typ: String,
    pub(crate) value: String,
}

#[derive(Debug)]
pub struct NewOrderRes {
    pub order: OrderRes,
    // pub nonce: String,
    pub location: String,
}

#[derive(Debug, Serialize)]
pub struct FinalizeOrderPayload {
    pub(crate) csr: String,
}
