use serde::Deserialize;

use crate::order::Identifier;

#[derive(Debug, Deserialize)]
pub struct Authz {
    identifier: Identifier,
    status: String,
    expires: Option<String>,
    pub(crate) challenges: Vec<Challenge>,
    wildcard: Option<bool>,
}

// #[derive(Debug, Deserialize, Clone)]
// pub struct Challenge {
//     #[serde(rename = "type")]
//     pub(crate) typ: ChallengeType,
//     pub url: String,
//     pub status: ChallengeStatus,
//     validated: Option<String>,
//     pub token: String,
// }

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum Challenge {
    #[serde(rename = "http-01")]
    Http01(Http01Challenge),
    #[serde(rename = "dns-01")]
    Dns01(Dns01Challenge),
    #[serde(rename = "tls-alpn-01")]
    TlsAlpn01(TlsAlpn01Challenge),
    #[serde(untagged)]
    Other(serde_json::Value),
}

#[derive(Debug, Deserialize, Clone)]
pub struct Dns01Challenge {
    pub url: String,
    pub status: ChallengeStatus,
    pub token: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Http01Challenge {
    pub url: String,
    pub status: ChallengeStatus,
    pub token: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TlsAlpn01Challenge {
    pub url: String,
    pub status: ChallengeStatus,
    pub token: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ChallengeStatus {
    Pending,
    Processing,
    Valid,
    Invalid,
    Expired,
    Deactivated,
}

impl ChallengeStatus {
    pub fn is_pending(&self) -> bool {
        matches!(self, ChallengeStatus::Pending)
    }

    pub fn is_valid(&self) -> bool {
        matches!(self, ChallengeStatus::Valid)
    }

    pub fn is_processing(&self) -> bool {
        matches!(self, ChallengeStatus::Processing)
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub enum ChallengeType {
    #[serde(rename = "http-01")]
    Http01,
    #[serde(rename = "dns-01")]
    Dns01,
    #[serde(rename = "tls-alpn-01")]
    TlsAlpn01,
    #[serde(untagged)]
    Other(String),
}

impl ChallengeType {
    pub fn as_str(&self) -> &str {
        match self {
            ChallengeType::Http01 => "http-01",
            ChallengeType::Dns01 => "dns-01",
            ChallengeType::TlsAlpn01 => "tls-alpn-01",
            ChallengeType::Other(s) => s,
        }
    }
}

impl std::fmt::Display for ChallengeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Authz {
    pub fn get_http01_challenge(&self) -> Option<&Http01Challenge> {
        self.challenges.iter().find_map(|c| match c {
            Challenge::Http01(ch) => Some(ch),
            _ => None,
        })
    }

    pub fn is_pending(&self) -> bool {
        self.status == "pending"
    }

    pub fn is_valid(&self) -> bool {
        self.status == "valid"
    }

    pub fn is_expired(&self) -> bool {
        self.expires
            .as_ref()
            .and_then(|e| chrono::DateTime::parse_from_rfc3339(e).ok())
            .map(|dt| dt < chrono::Utc::now())
            .unwrap_or(false)
    }
}

impl Challenge {
    pub fn url(&self) -> &str {
        match self {
            Challenge::Http01(ch) => &ch.url,
            Challenge::Dns01(ch) => &ch.url,
            Challenge::TlsAlpn01(ch) => &ch.url,
            Challenge::Other(ch) => ch.get("url").and_then(|v| v.as_str()).unwrap_or(""),
        }
    }

    pub fn is_pending(&self) -> bool {
        match self {
            Challenge::Http01(http01_challenge) => http01_challenge.status.is_pending(),
            Challenge::Dns01(dns01_challenge) => dns01_challenge.status.is_pending(),
            Challenge::TlsAlpn01(tls_alpn01_challenge) => tls_alpn01_challenge.status.is_pending(),
            Challenge::Other(value) => {
                value.get("status").and_then(|v| v.as_str()) == Some("pending")
            }
        }
    }
}
