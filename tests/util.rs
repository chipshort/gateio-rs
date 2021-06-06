use serde_json::from_str;

pub fn unauthenticated() -> gateio::GateIO {
    gateio::GateIO::new(None)
}

pub fn authenticated() -> gateio::GateIO {
    gateio::GateIO::new(Some(from_str(include_str!("../api_key.json")).unwrap()))
}