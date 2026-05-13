use jsonwebtoken::Algorithm;
use jsonwebtoken::EncodingKey;
use jsonwebtoken::Header;
use jsonwebtoken::jwk::Jwk;
use jsonwebtoken::jws::Jws;
use serde::Serialize;

use crate::LocalAccount;
use crate::error::AcmeResult;

pub fn create_jws<T: Serialize>(
    key: &EncodingKey,
    payload: Option<&T>,
    url: &str,
    nonce: &str,
    kid: Option<&str>,
    jwk: Option<&Jwk>,
) -> AcmeResult<Jws<T>> {
    let mut header = Header::new(Algorithm::ES256);

    if let Some(kid) = kid {
        header.kid = Some(kid.to_string());
    }

    if let Some(jwk) = jwk {
        header.jwk = Some(jwk.clone());
    }

    header.url = Some(url.to_string());

    let mut extras = std::collections::HashMap::new();
    extras.insert("nonce".to_string(), nonce.to_owned());
    header.extras = extras;

    let jws = jsonwebtoken::jws::encode(&header, payload, key)?;
    Ok(jws)
}

pub fn create_kid_jws<T: Serialize>(
    acc: &LocalAccount,
    payload: Option<&T>,
    url: &str,
    nonce: &str,
) -> AcmeResult<Jws<T>> {
    let key = EncodingKey::from_ec_pem(acc.key.as_bytes())?;
    create_jws(&key, payload, url, nonce, Some(&acc.kid), None)
}

pub fn create_jwk_jws<T: Serialize>(
    key: &EncodingKey,
    payload: Option<&T>,
    url: &str,
    nonce: &str,
) -> AcmeResult<Jws<T>> {
    let jwk = Jwk::from_encoding_key(key, Algorithm::ES256)?;
    create_jws(key, payload, url, nonce, None, Some(&jwk))
}

pub fn compute_dns01_challenge(token: &str, thumbprint: &str) -> String {
    use base64::Engine;
    use base64::prelude::BASE64_URL_SAFE_NO_PAD;
    use sha2::Digest;
    use sha2::Sha256;

    let data = format!("{}.{}", token, thumbprint);
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    let digest = hasher.finalize();
    BASE64_URL_SAFE_NO_PAD.encode(digest)
}

pub fn compute_http01_challenge(token: &str, thumbprint: &str) -> String {
    format!("{}.{}", token, thumbprint)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_challenge_computation() {
        let token = "test-token";
        let thumbprint = "test-thumbprint";
        let result = compute_http01_challenge(token, thumbprint);
        assert_eq!(result, "test-token.test-thumbprint");
    }
}
