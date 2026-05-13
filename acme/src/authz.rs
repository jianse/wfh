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

#[derive(Debug, Deserialize, Clone)]
pub struct Challenge {
    #[serde(rename = "type")]
    pub(crate) typ: ChallengeType,
    pub url: String,
    pub status: ChallengeStatus,
    validated: Option<String>,
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
    pub fn get_challenge(&self, typ: ChallengeType) -> Option<&Challenge> {
        self.challenges.iter().find(|c| c.typ == typ)
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
    pub fn is_pending(&self) -> bool {
        self.status.is_pending()
    }

    pub fn is_valid(&self) -> bool {
        self.status.is_valid()
    }

    pub fn is_processing(&self) -> bool {
        self.status.is_processing()
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.status == ChallengeStatus::Valid {
            Ok(())
        } else {
            Err(format!("Challenge status is: {:?}", self.status))
        }
    }
}
