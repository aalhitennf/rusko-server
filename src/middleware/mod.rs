mod auth;
mod jwt;

pub use auth::auth;
pub use jwt::jwt;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Claims {
    pub exp: i64,
    pub dev: String,
}
