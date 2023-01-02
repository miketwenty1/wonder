use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Validate, Deserialize)]
pub struct BlockData {
    #[validate(length(equal = 64))]
    pub hash: String,
    #[validate(range(min = 0, max = 1_000_000))]
    pub height: i32,
}

#[derive(Debug, Serialize, Validate, Deserialize)]
pub struct InvoiceData {
    pub invoice: String,
}
