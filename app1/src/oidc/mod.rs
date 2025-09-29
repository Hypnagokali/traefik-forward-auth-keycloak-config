use serde::Deserialize;

pub mod keys;
pub mod jwks;

#[derive(Debug, Deserialize)]
pub struct Claims {
    pub name: String,
    pub roles: Vec<String>,
}
