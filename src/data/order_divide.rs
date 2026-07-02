use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use yeepay_core::YeepayBodyTrait;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderDivideCompleteReq {
    parent_merchant_no: String,
    merchant_no: String,
    order_id: String,
    divide_request_id: String,
}

impl OrderDivideCompleteReq {
    pub fn new(merchant_no: &str, order_id: &str) -> OrderDivideCompleteReq {
        OrderDivideCompleteReq {
            parent_merchant_no: merchant_no.to_string(),
            merchant_no: merchant_no.to_string(),
            order_id: order_id.to_string(),
            divide_request_id: format!("{}_D", order_id),
        }
    }
}

impl YeepayBodyTrait for OrderDivideCompleteReq {
    fn to_map(&self) -> BTreeMap<String, String> {
        let mut params: BTreeMap<String, String> = BTreeMap::new();
        params.insert(
            "parentMerchantNo".to_string(),
            self.parent_merchant_no.clone(),
        );
        params.insert("merchantNo".to_string(), self.merchant_no.clone());
        params.insert("orderId".to_string(), self.order_id.clone());
        params.insert(
            "divideRequestId".to_string(),
            self.divide_request_id.clone(),
        );
        params
    }
}

#[derive(Debug, Deserialize)]
pub struct OrderDivideCompleteResp {
    #[serde(default, rename = "orderId")]
    pub order_id: String,
    #[serde(default, rename = "divideRequestId")]
    pub divide_request_id: String,
    #[serde(default, rename = "divideStatus")]
    pub divide_status: Option<String>,
    #[serde(default, rename = "divideSuccessDate")]
    pub divide_success_date: Option<String>,
    #[serde(default)]
    pub amount: Option<Decimal>,
    #[serde(default, rename = "bizSystemNo")]
    pub biz_system_no: Option<String>,
    #[serde(default, rename = "uniqueOrderNo")]
    pub unique_order_no: Option<String>,
    #[serde(default, rename = "createDate")]
    pub create_date: Option<String>,
    /// reserved field
    #[serde(default, flatten)]
    pub extra: Option<HashMap<String, serde_json::Value>>,
}
