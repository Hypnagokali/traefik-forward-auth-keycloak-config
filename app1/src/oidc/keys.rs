use std::sync::Arc;

use jsonwebtoken::DecodingKey;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RsaKeySetResponse {
    keys: Vec<RsaKeyInner>,
}

impl RsaKeySetResponse {
    pub fn into_keys(self) -> Vec<RsaKey> {
        self.keys.into_iter().map(|inner| RsaKey::new(inner)).collect()
    }
}

pub struct RsaKey {
    inner: Arc<RsaKeyInner>,
}

impl Clone for RsaKey {
    fn clone(&self) -> Self {
        RsaKey { inner: Arc::clone(&self.inner) }
    }
}

impl RsaKey {
    fn new(inner: RsaKeyInner) -> Self {
        RsaKey { inner: Arc::new(inner) }
    }
}

#[derive(Deserialize)]
struct RsaKeyInner {
    kid: String,
    n: String,
    e: String,
}

impl RsaKey {
    pub fn kid(&self) -> &str {
        &self.inner.kid
    }
    pub fn to_decoding_key(&self) -> Result<DecodingKey, jsonwebtoken::errors::Error> {
        DecodingKey::from_rsa_components(&self.inner.n, &self.inner.e)
    }
}