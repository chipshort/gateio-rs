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
pub use http_client;
mod error;

use std::sync::Arc;

use http_client::HttpClient;
use model::ApiKey;

#[derive(Clone)]
pub struct GateIO<C: HttpClient> {
    client: Arc<C>,

    pub key: Option<ApiKey>
}

impl<C: HttpClient> GateIO<C> {
    pub fn new(client: Arc<C>, key: Option<ApiKey>) -> Self {
        Self {
            client,
            key
        }
    }
}