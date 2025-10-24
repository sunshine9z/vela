use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Claims {
    pub username: String,
    pub id: i64,
    pub role: i64,
    pub exp: i64,
    pub token_id: i64,
}
