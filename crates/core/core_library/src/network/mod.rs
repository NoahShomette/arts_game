use serde::{Deserialize, Serialize};

/// A wrapper that contains meta information that clients/game_server/Auth_server can sends to any of the Servers that run http servers
/// in order to correctly make and process requests
///
/// Specific requests require this wrapper to desereialize the request properly. Basically all authentication requred requests
#[derive(Serialize, Deserialize)]
pub struct HttpRequestMeta<T> {
    pub request: T,
}
