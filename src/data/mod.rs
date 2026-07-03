use serde::{Deserialize, Serialize};

/// Enums
pub mod enums;

/// Order Create
pub mod order_create;

/// Order Divide
pub mod order_divide;

/// Order Query
pub mod order_query;

/// Order Refund
pub mod order_refund;

/// Wallet
pub mod wallet;

/// Global Merchant Wrap
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GlobalMerchantWrap<Q> {
    /// parent merchant no
    pub parent_merchant_no: String,
    /// merchant no
    pub merchant_no: String,
    /// data
    #[serde(flatten)]
    pub data: Q,
}

impl<Q> GlobalMerchantWrap<Q> {
    /// new
    pub fn new(merchant_no: &str, data: Q) -> Self {
        Self {
            parent_merchant_no: merchant_no.to_string(),
            merchant_no: merchant_no.to_string(),
            data,
        }
    }
}
