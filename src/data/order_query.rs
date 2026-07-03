use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 订单查询
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderQueryReq {
    /// 订单ID
    pub order_id: String,
}

/// 订单查询返回
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderQueryResp {
    /// 清算完成时间
    pub cs_success_date: Option<String>,
    /// 订单ID
    pub order_id: Option<String>,
    /// 银行订单ID
    pub bank_order_id: Option<String>,
    /// 商户ID
    pub yop_merchant_no: Option<String>,
    /// 渠道
    pub channel: Option<String>,
    /// 支付方式
    pub pay_way: Option<String>,
    /// 渠道结算金额
    pub yp_settle_amount: Option<Decimal>,
    /// 手续费贡献信息
    pub fee_contribute_info: Option<Vec<FeeContributeItem>>,
    /// 渠道支付金额
    pub pay_amount: Option<Decimal>,
    /// 未分账金额
    pub un_split_amount: Option<Decimal>,
    /// 实际支付金额
    pub real_pay_amount: Option<Decimal>,
    /// 商品名称
    pub basics_product_first: Option<String>,
    /// 手续费商户号
    pub fee_merchant_no: Option<String>,
    /// 手续费
    pub merchant_fee: Option<Decimal>,
    /// 手续费类型
    pub fee_type: Option<String>,
    ///
    pub basics_product_second: Option<String>,
    /// 渠道
    pub out_clear_channel: Option<String>,
    /// 资金处理类型
    pub fund_process_type: Option<String>,
    /// 退款金额
    pub total_refund_amount: Option<Decimal>,
    /// 渠道订单号
    pub channel_trx_id: Option<String>,
    /// 渠道商户信息
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
    /// 结算金额
    pub settle_amount: Option<Decimal>,
    /// 订单号
    pub unique_order_no: Option<String>,
    /// 渠道分账金额之和
    pub total_divide_amount: Option<Decimal>,
    /// 订单金额
    pub order_amount: Option<Decimal>,
    /// 渠道原始分账金额之和
    pub original_divide_amount: Option<Decimal>,
    /// 交易类型
    pub trade_type: Option<String>,
    /// 渠道单号
    pub channel_order_id: Option<String>,
    /// 支付成功时间
    pub pay_success_date: Option<String>,
    /// 清算状态
    pub clear_status: Option<String>,
    /// 业务系统单号
    pub biz_system_no: Option<String>,
    /// token
    pub token: Option<String>,
    /// 支付方信息
    pub payer_info: Option<PayerInfo>,
    /// 渠道结算金额
    pub channel_settle_amount: Option<Decimal>,
    /// 业务类型
    pub business_type: Option<String>,
    /// 交易渠道
    pub trade_channel: Option<String>,
    /// extra
    #[serde(default, flatten)]
    pub extra: Option<HashMap<String, serde_json::Value>>,
}

/// 手续费贡献信息
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeContributeItem {
    /// 贡献金额
    pub amount: Option<Decimal>,
    /// 贡献方
    pub r#type: Option<String>,
    /// 贡献方商户号
    pub merchant_no: Option<String>,
    /// 预留字段
    #[serde(default, flatten)]
    pub extra: Option<HashMap<String, serde_json::Value>>,
}

/// 渠道商户信息
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelMerchantInfo {
    /// 渠道商户号
    pub channel_merchant_no: Option<String>,
    /// 预留字段
    #[serde(default, flatten)]
    pub extra: Option<HashMap<String, serde_json::Value>>,
}

/// 支付方信息
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PayerInfo {
    /// 卡类型
    pub card_type: Option<String>,
    /// 买家支付账号
    pub buyer_logon_id: Option<String>,
    /// 买家用户号
    pub user_id: Option<String>,
    /// 银行ID
    pub bank_id: Option<String>,
    /// 渠道交易ID
    pub channel_trx_id: Option<String>,
    /// 预留字段
    #[serde(default, flatten)]
    pub extra: Option<HashMap<String, serde_json::Value>>,
}
