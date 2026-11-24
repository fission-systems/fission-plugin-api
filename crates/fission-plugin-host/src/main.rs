//! Read-only reference host. Production Fission integration is a later milestone.
use fission_plugin_protocol::{address, parse_address, Error, Function, Outcome, Request, Response, MAX_READ, VERSION};
use serde::Deserialize;
use serde_json::{json, Value};
use std::{env, fs, io::{self, BufRead, Write}, process::Command};

#[derive(Deserialize)]
struct Fixture {
    functions: Vec<Function>,
    base: String,
    bytes: Vec<u8>,
}

enum Backend {
    Fixture(Fixture),
    Fission { cli: String, binary: String },
}

impl Backend {
    fn functions(&self) -> Result<Vec<Function>, Error> {
        match self {
            Self::Fixture(f) => Ok(f.functions.clone()),
            Self::Fission { cli, binary } => {
                let output = Command::new(cli).args(["--contract", "v1", "list", binary, "--json"])
                    .output().map_err(|e| Error::new("backend", e.to_string()))?;
                let envelope: Value = serde_json::from_slice(&output.stdout)
                    .map_err(|e| Error::new("backend", e.to_string()))?;
                if !output.status.success() || envelope["ok"] != true {
                    return Err(Error::new("backend", format!("Fission list failed: {}", envelope["error"])));
                }
                #[derive(Deserialize)]
                struct Row { addr: u64, name: String, size: u64 }
                #[derive(Deserialize)]
                struct Data { functions: Vec<Row>, analyzing: bool }
                let data: Data = serde_json::from_value(envelope["data"].clone())
                    .map_err(|e| Error::new("backend", e.to_string()))?;
                if data.analyzing {
                    return Err(Error::new("backend", "function discovery incomplete"));
                }
                Ok(data.functions.into_iter().map(|f| Function {
                    address: address(f.addr), name: f.name, size: f.size
                }).collect())
            }
        }
    }

    fn call(&self, req: &Request) -> Result<Value, Error> {
        if req.version != VERSION {
            return Err(Error::new("version_mismatch", "only protocol v0 is supported"));
        }
        match req.method.as_str() {
            "hello" => {
                let (name, capabilities) = match self {
                    Self::Fixture(_) => ("fixture", vec!["functions.list", "memory.read"]),
                    Self::Fission { .. } => ("fission-cli", vec!["functions.list"]),
                };
                Ok(json!({"backend": name, "capabilities": capabilities}))
            }
            "functions.list" => Ok(json!(self.functions()?)),
            "memory.read" => {
                let Self::Fixture(f) = self else {
                    return Err(Error::new("unsupported", "memory.read is not implemented by this backend"));
                };
                let invalid = || Error::new("invalid_params", "expected address and size <= MAX_READ");
                let addr = parse_address(req.params["address"].as_str().ok_or_else(invalid)?)?;
                let base = parse_address(&f.base)?;
                let size = req.params["size"].as_u64().filter(|n| *n <= MAX_READ as u64)
                    .ok_or_else(invalid)? as usize;
                let unmapped = || Error::new("unmapped", "range is not wholly mapped");
                let offset = addr.checked_sub(base).and_then(|n| usize::try_from(n).ok()).ok_or_else(unmapped)?;
                let end = offset.checked_add(size).ok_or_else(unmapped)?;
                let bytes = f.bytes.get(offset..end).ok_or_else(unmapped)?;
                Ok(json!(bytes))
            }
            _ => Err(Error::new("unsupported", format!("unknown method {}", req.method))),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().skip(1).collect();
    let backend = match args.as_slice() {
        [flag, path] if flag == "--fixture" => Backend::Fixture(serde_json::from_str(&fs::read_to_string(path)?)?),
        [flag, binary] if flag == "--binary" => Backend::Fission {
            cli: env::var("FISSION_CLI").unwrap_or_else(|_| "fission_cli".into()),
            binary: binary.clone(),
        },
        _ => return Err("usage: fission-plugin-host (--fixture FILE | --binary FILE)".into()),
    };
    let mut out = io::stdout().lock();
    for line in io::stdin().lock().lines() {
        let line = line?;
        let req: Request = serde_json::from_str(&line)?;
        let outcome = match backend.call(&req) {
            Ok(result) => Outcome::Ok { result },
            Err(error) => Outcome::Err { error },
        };
        serde_json::to_writer(&mut out, &Response { version: VERSION, id: req.id, outcome })?;
        out.write_all(b"\n")?;
        out.flush()?;
    }
    Ok(())
}
