use std::sync::Mutex;

use serde::Deserialize;

use crate::oidc::keys::{RsaKeySetResponse, RsaKey};

#[derive(Debug, thiserror::Error)]
#[error("JwksStore error: {0}")]
pub struct JwksError(String);

pub struct JwksStore {
    uri: String,
    jwks: Mutex<Option<Vec<RsaKey>>>,
}

impl From<reqwest::Error> for JwksError {
    fn from(err: reqwest::Error) -> Self {
        JwksError(err.to_string())
    }
}

#[derive(Deserialize)]
struct OidcConfig {
    jwks_uri: String,
}

impl JwksStore {
    pub fn new(uri: &str) -> Self {
        JwksStore { uri: uri.to_string(), jwks: Mutex::new(None) }
    }

    pub fn has_jwks(&self) -> bool {
        match self.jwks.lock() {
            Ok(guard) => guard.is_some(),
            Err(_) => false, // proper error handling or logging needed here
        }
    }

    pub async fn fetch_jwks(&self) -> Result<(), JwksError> {
        let response = reqwest::get(&self.uri).await?;
        println!("Fetched OIDC config: {:?}", response);
        let config: OidcConfig = response.json().await?;
        let jwks_response = reqwest::get(&config.jwks_uri).await?;
        println!("Fetched JWKS: {:?}", jwks_response);
        let jwks_ser: RsaKeySetResponse = jwks_response.json().await?;
        let jwks = jwks_ser.into_keys();

        match self.jwks.lock() {
            Err(_) => return Err(JwksError("Mutex lock error".into())),
            Ok(mut guard) => {
                *guard = Some(jwks);
            }
        }
        Ok(())
    }

    pub fn get_key(&self, kid: &str) -> Option<RsaKey> {
        match self.jwks.lock() {
            Err(_) => return None, // proper error handling or logging needed here
            Ok(guard) => {
                if let Some(jwks) = &*guard {
                    jwks.iter().find(|key| key.kid() == kid).map(|key| key.clone())
                } else {
                    None
                }
            },
        }
    }
}
