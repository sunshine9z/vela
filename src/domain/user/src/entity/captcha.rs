use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CaptchaImage {
    pub image: String,
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CaptchaCacheInfo {
    pub client_id: String,
    pub cache_text: String,
}
