use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use yeepay_core::YeepayBodyTrait;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderQueryReq {
    parent_merchant_no: String,
    merchant_no: String,
    order_id: String,
}

impl OrderQueryReq {
    pub fn new(merchant_no: &str, order_id: &str) -> OrderQueryReq {
        OrderQueryReq {
            parent_merchant_no: merchant_no.to_string(),
            merchant_no: merchant_no.to_string(),
            order_id: order_id.to_string(),
        }
    }
}

impl YeepayBodyTrait for OrderQueryReq {
    fn to_map(&self) -> BTreeMap<String, String> {
        let mut params: BTreeMap<String, String> = BTreeMap::new();
        params.insert(
            "parentMerchantNo".to_string(),
            self.parent_merchant_no.clone(),
        );
        params.insert("merchantNo".to_string(), self.merchant_no.clone());
        params.insert("orderId".to_string(), self.order_id.clone());
        params
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderQueryResp {
    #[serde(default)]
    pub cs_success_date: Option<String>,
    #[serde(default)]
    pub order_id: Option<String>,
    #[serde(default)]
    pub bank_order_id: Option<String>,
    #[serde(default)]
    pub yop_merchant_no: Option<String>,
    #[serde(default)]
    pub channel: Option<String>,
    #[serde(default)]
    pub pay_way: Option<String>,
    #[serde(default)]
    pub yp_settle_amount: Option<Decimal>,
    #[serde(default)]
    pub fee_contribute_info: Option<Vec<FeeContributeItem>>,
    #[serde(default)]
    pub pay_amount: Option<Decimal>,
    #[serde(default)]
    pub un_split_amount: Option<Decimal>,
    #[serde(default)]
    pub real_pay_amount: Option<Decimal>,
    #[serde(default)]
    pub basics_product_first: Option<String>,
    #[serde(default)]
    pub fee_merchant_no: Option<String>,
    #[serde(default)]
    pub merchant_fee: Option<Decimal>,
    #[serde(default)]
    pub fee_type: Option<String>,
    #[serde(default)]
    pub basics_product_second: Option<String>,
    #[serde(default)]
    pub out_clear_channel: Option<String>,
    #[serde(default)]
    pub fund_process_type: Option<String>,
    #[serde(default)]
    pub total_refund_amount: Option<Decimal>,
    #[serde(default)]
    pub channel_trx_id: Option<String>,
    #[serde(default)]
    pub channel_merchant_info: Option<ChannelMerchantInfo>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub sum_discount_promotion_amount: Option<Decimal>,
    #[serde(default)]
    pub settle_amount: Option<Decimal>,
    #[serde(default)]
    pub unique_order_no: Option<String>,
    #[serde(default)]
    pub total_divide_amount: Option<Decimal>,
    #[serde(default)]
    pub order_amount: Option<Decimal>,
    #[serde(default)]
    pub original_divide_amount: Option<Decimal>,
    #[serde(default)]
    pub trade_type: Option<String>,
    #[serde(default)]
    pub channel_order_id: Option<String>,
    #[serde(default)]
    pub pay_success_date: Option<String>,
    #[serde(default)]
    pub clear_status: Option<String>,
    #[serde(default)]
    pub biz_system_no: Option<String>,
    #[serde(default)]
    pub token: Option<String>,
    #[serde(default)]
    pub payer_info: Option<PayerInfo>,
    #[serde(default)]
    pub channel_settle_amount: Option<Decimal>,
    #[serde(default)]
    pub business_type: Option<String>,
    #[serde(default)]
    pub trade_channel: Option<String>,
    /// reserved field
    #[serde(default, flatten)]
    pub extra: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeContributeItem {
    pub amount: Option<Decimal>,
    pub r#type: Option<String>,
    pub merchant_no: Option<String>,
    /// 预留字段
    #[serde(default, flatten)]
    pub extra: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelMerchantInfo {
    pub channel_merchant_no: Option<String>,
    /// 预留字段
    #[serde(default, flatten)]
    pub extra: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PayerInfo {
    pub card_type: Option<String>,
    pub buyer_logon_id: Option<String>,
    pub user_id: Option<String>,
    pub bank_id: Option<String>,
    pub channel_trx_id: Option<String>,
    /// 预留字段
    #[serde(default, flatten)]
    pub extra: Option<HashMap<String, serde_json::Value>>,
}
