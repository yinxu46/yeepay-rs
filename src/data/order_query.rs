use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderQueryReq {
    pub order_id: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderQueryResp {
    pub cs_success_date: Option<String>,
    pub order_id: Option<String>,
    pub bank_order_id: Option<String>,
    pub yop_merchant_no: Option<String>,
    pub channel: Option<String>,
    pub pay_way: Option<String>,
    pub yp_settle_amount: Option<Decimal>,
    pub fee_contribute_info: Option<Vec<FeeContributeItem>>,
    pub pay_amount: Option<Decimal>,
    pub un_split_amount: Option<Decimal>,
    pub real_pay_amount: Option<Decimal>,
    pub basics_product_first: Option<String>,
    pub fee_merchant_no: Option<String>,
    pub merchant_fee: Option<Decimal>,
    pub fee_type: Option<String>,
    pub basics_product_second: Option<String>,
    pub out_clear_channel: Option<String>,
    pub fund_process_type: Option<String>,
    pub total_refund_amount: Option<Decimal>,
    pub channel_trx_id: Option<String>,
    pub channel_merchant_info: Option<ChannelMerchantInfo>,
    /// 订单状态
    ///
    /// - PROCESSING：订单待支付
    /// - SUCCESS：订单支付成功
    /// - TIME_OUT：订单已过期
    /// - FAIL:订单支付失败
    /// - CLOSE:订单关闭
    pub status: Option<String>,
    /// 渠道免充值卡券金额之和
    pub sum_discount_promotion_amount: Option<Decimal>,
    pub settle_amount: Option<Decimal>,
    pub unique_order_no: Option<String>,
    pub total_divide_amount: Option<Decimal>,
    pub order_amount: Option<Decimal>,
    pub original_divide_amount: Option<Decimal>,
    pub trade_type: Option<String>,
    pub channel_order_id: Option<String>,
    pub pay_success_date: Option<String>,
    pub clear_status: Option<String>,
    pub biz_system_no: Option<String>,
    pub token: Option<String>,
    pub payer_info: Option<PayerInfo>,
    pub channel_settle_amount: Option<Decimal>,
    pub business_type: Option<String>,
    pub trade_channel: Option<String>,
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

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
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
