# gateio-rs

Unofficial Rust library for [Gate.io API v4](https://www.gate.io/docs/apiv4/en/index.html#gate-api-v4).

This crate is *very* unfinished. It contains only the functionalities I needed.
Feel free to contribute.

## Usage

Add this to your Cargo.toml

```toml
[dependencies]
gateio = { git = "https://github.com/chipshort/gateio-rs.git" }
```

To run the unit tests, provide your `apiKey` and `secretKey` inside `api_key.json`.
The unit tests do not place any orders.