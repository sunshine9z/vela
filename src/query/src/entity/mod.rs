use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PageReq {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

impl Default for PageReq {
    fn default() -> Self {
        Self {
            page: Some(1),
            page_size: Some(10),
        }
    }
}
