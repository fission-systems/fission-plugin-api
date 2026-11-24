//! Versioned wire types. No dependency on Fission internals.
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const VERSION: u32 = 0;
pub const MAX_READ: usize = 1 << 20;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    pub version: u32,
    pub id: u64,
    pub method: String,
    #[serde(default)]
    pub params: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Error {
    pub code: String,
    pub message: String,
}

impl Error {
    pub fn new(code: &str, message: impl Into<String>) -> Self {
        Self { code: code.into(), message: message.into() }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub version: u32,
    pub id: u64,
    #[serde(flatten)]
    pub outcome: Outcome,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Outcome {
    Ok { result: Value },
    Err { error: Error },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Function {
    pub address: String,
    pub name: String,
    pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hello {
    pub capabilities: Vec<String>,
    pub backend: String,
}

pub fn address(n: u64) -> String { format!("0x{n:x}") }

pub fn parse_address(s: &str) -> Result<u64, Error> {
    let n = s.strip_prefix("0x").and_then(|s| u64::from_str_radix(s, 16).ok())
        .ok_or_else(|| Error::new("invalid_params", "expected hexadecimal u64 address"))?;
    if address(n) != s {
        return Err(Error::new("invalid_params", "address must be canonical lowercase hex"));
    }
    Ok(n)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn address_roundtrip() {
        for n in [0, 0x1000, u64::MAX] {
            assert_eq!(parse_address(&address(n)).unwrap(), n);
        }
        for s in ["123", "0x01", "0xA", "0x10000000000000000"] {
            assert!(parse_address(s).is_err());
        }
    }
}
