use serde::{Serialize, Deserialize};

/// A wrapper that contains meta information that the client sends to the Auth Server in order to correctly make requests
#[derive(Serialize, Deserialize)]
pub struct ClientHttpRequest<T> {
    pub access_token: Option<String>,
    pub request: T,
}
