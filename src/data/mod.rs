use serde::{Deserialize, Serialize};

pub mod order_create;
pub mod order_divide;
pub mod order_query;
pub mod order_refund;
pub mod wallet;
pub mod enums;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GlobalMerchantWrap<Q> {
    pub parent_merchant_no: String,
    pub merchant_no: String,
    #[serde(flatten)]
    pub data: Q,
}

impl<Q> GlobalMerchantWrap<Q> {
    pub fn new(merchant_no: &str, data: Q) -> Self {
        Self {
            parent_merchant_no: merchant_no.to_string(),
            merchant_no: merchant_no.to_string(),
            data,
        }
    }
}
