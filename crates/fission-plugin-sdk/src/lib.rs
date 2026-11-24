//! Synchronous native SDK over a caller-provided JSON-lines transport.
pub use fission_plugin_protocol::{Error, Function, Hello};
use fission_plugin_protocol::{Outcome, Request, Response, VERSION};
use serde_json::{json, Value};
use std::io::{BufRead, Write};

pub struct Client<R, W> {
    reader: R,
    writer: W,
    next_id: u64,
}

impl<R: BufRead, W: Write> Client<R, W> {
    pub fn new(reader: R, writer: W) -> Self {
        Self { reader, writer, next_id: 0 }
    }

    pub fn call(&mut self, method: &str, params: Value) -> Result<Value, Error> {
        let id = self.next_id;
        self.next_id = id.checked_add(1).ok_or_else(|| Error::new("transport", "id exhausted"))?;
        let req = Request { version: VERSION, id, method: method.into(), params };
        serde_json::to_writer(&mut self.writer, &req).map_err(transport)?;
        self.writer.write_all(b"\n").map_err(transport)?;
        self.writer.flush().map_err(transport)?;
        let mut line = String::new();
        self.reader.read_line(&mut line).map_err(transport)?;
        let resp: Response = serde_json::from_str(&line).map_err(transport)?;
        if resp.version != VERSION || resp.id != id {
            return Err(Error::new("transport", "response version or id mismatch"));
        }
        match resp.outcome {
            Outcome::Ok { result } => Ok(result),
            Outcome::Err { error } => Err(error),
        }
    }

    pub fn hello(&mut self) -> Result<Hello, Error> {
        serde_json::from_value(self.call("hello", json!({}))?).map_err(transport)
    }

    pub fn functions(&mut self) -> Result<Vec<Function>, Error> {
        serde_json::from_value(self.call("functions.list", json!({}))?).map_err(transport)
    }
}

fn transport(e: impl std::fmt::Display) -> Error { Error::new("transport", e.to_string()) }

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    #[test]
    fn refuses_mismatched_id() {
        let response = Cursor::new(b"{\"version\":0,\"id\":9,\"result\":[]}\n");
        let mut c = Client::new(response, Vec::new());
        assert_eq!(c.functions().unwrap_err().code, "transport");
    }
}
