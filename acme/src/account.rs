use std::collections::HashMap;

use jsonwebtoken::Algorithm;
use jsonwebtoken::EncodingKey;
use jsonwebtoken::Header;
use jsonwebtoken::jwk::Jwk;
use reqwest::Client;
use reqwest::header::CONTENT_TYPE;
use reqwest::header::LOCATION;
use serde::Deserialize;
use serde::Serialize;
use tracing::debug;

use crate::error::AcmeResult;

#[derive(Debug, Serialize, Default)]
pub struct NewAccountPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Vec<String>>,

    #[serde(
        rename = "termsOfServiceAgreed",
        skip_serializing_if = "Option::is_none"
    )]
    pub terms_of_service_agreed: Option<bool>,

    #[serde(rename = "onlyReturnExisting", skip_serializing_if = "Option::is_none")]
    pub only_return_existing: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct Account {
    pub status: String,
    pub contact: Option<Vec<String>>,
    #[serde(
        rename = "termsOfServiceAgreed",
        skip_serializing_if = "Option::is_none"
    )]
    pub terms_of_service_agreed: Option<bool>,
    pub orders: String,
}

#[derive(Debug)]
pub struct NewAccountRes {
    pub account: Account,
    pub nonce: String,
    pub kid: String,
}

pub async fn new_account(
    client: &Client,
    url: &str,
    nonce: &str,
    payload: &NewAccountPayload,
    key: &EncodingKey,
) -> AcmeResult<NewAccountRes> {
    let mut header = Header::new(Algorithm::ES256);
    let mut extras = HashMap::new();
    extras.insert("nonce".to_string(), nonce.to_owned());
    header.extras = extras;
    header.url = Some(url.to_owned());
    let jwk = Jwk::from_encoding_key(key, Algorithm::ES256)?;
    header.jwk = Some(jwk);

    let e = crate::jws::create_jwk_jws(key, Some(&payload), url, nonce)?;

    let b = serde_json::to_string(&e)?;
    let req = client
        .post(url)
        .body(b)
        .header(CONTENT_TYPE, "application/jose+json")
        .build()?;
    debug!("{:#?}", &req);
    let resp = client.execute(req).await?;
    debug!("{:#?}", &resp);
    debug!("{:#?}", &resp.headers());

    let nonce = resp
        .headers()
        .get("Replay-Nonce")
        .ok_or_else(|| crate::error::AcmeError::MissingHeader("Replay-Nonce".to_string()))?
        .to_str()?
        .to_owned();

    let kid = resp
        .headers()
        .get(LOCATION)
        .ok_or_else(|| crate::error::AcmeError::MissingHeader("Location".to_string()))?
        .to_str()?
        .to_owned();

    let text = resp.text().await?;
    debug!("{:#?}", &text);
    let account: Account = serde_json::from_str(&text)?;

    Ok(NewAccountRes {
        account,
        nonce,
        kid,
    })
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::fs::create_dir_all;

    use jsonwebtoken::EncodingKey;
    use p256::elliptic_curve::rand_core::OsRng;
    use p256::pkcs8::EncodePrivateKey;
    use p256::pkcs8::LineEnding;
    use tracing::debug;

    use crate::account::NewAccountPayload;
    use crate::error::AcmeResult;

    #[tokio::test]
    async fn it_works() -> AcmeResult<()> {
        let buf = fs::read("certs/key.pem")?;
        let key = EncodingKey::from_ec_pem(&buf)?;

        let p = NewAccountPayload {
            contact: Some(vec!["mailto:<EMAIL>".to_string()]),
            terms_of_service_agreed: Some(true),
            ..Default::default()
        };

        let nonce = "test-nonce";
        let url = "https://example.com/new-account";
        let jws = crate::jws::create_jwk_jws(&key, Some(&p), url, nonce)?;
        debug!("{:#?}", &jws);

        Ok(())
    }

    #[test]
    fn jwk_gen_test() -> AcmeResult<()> {
        let mut rng = OsRng;
        let key = p256::SecretKey::random(&mut rng);
        let pk = key.public_key();

        create_dir_all("certs")?;
        fs::write("certs/key.pem", key.to_pkcs8_pem(LineEnding::default())?)?;
        fs::write("certs/pub.pem", pk.to_string())?;
        Ok(())
    }
}
