#![deny(
    unstable_features,
    unused_must_use,
    unused_mut,
    unused_imports,
    unused_import_braces,
    unsafe_code
)]

pub mod spot;
pub mod model;
#[macro_use]
pub(crate) mod util;
pub use error::Error;
mod error;

use std::sync::Arc;

use reqwest::Client;
use model::ApiKey;

#[derive(Clone)]
pub struct GateIO {
    pub(crate) client: Arc<Client>,

    pub key: Option<ApiKey>
}

impl GateIO {
    pub fn new(key: Option<ApiKey>) -> Self {
        Self {
            client: Arc::new(Client::new()),
            key
        }
    }

    pub fn with_client(client: Arc<Client>, key: Option<ApiKey>) -> Self {
        Self {
            client,
            key
        }
    }
}