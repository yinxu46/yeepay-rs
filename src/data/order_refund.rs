use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// 订单退款
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderRefundReq {
    /// 退款请求号
    pub refund_request_id: String,
    /// 商户订单编号: 商户收款请求号、易宝收款订单号 二选一必填
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// 易宝订单编号: 商户收款请求号、易宝收款订单号 二选一必填
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_order_no: Option<String>,
    /// 退款金额
    pub refund_amount: Decimal,
    /// 退款原因
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 对账备注
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
}

/// 订单退款响应
#[derive(Debug, Deserialize)]
pub struct OrderRefundResp {
    /// 商户收款订单号
    pub order_id: Option<String>,
    /// 商户退款请求号
    pub refund_request_id: Option<String>,
    /// 易宝退款订单号
    pub unique_refund_no: Option<String>,
    /// 退款订单状态
    ///
    /// - PROCESSING：退款处理中
    /// - SUCCESS：退款成功
    /// - FAILED：退款失败
    /// - CANCEL:退款关闭,商户通知易宝结束该笔退款后返回该状态
    /// - SUSPEND:退款中断,如需继续退款,请调用上送卡信息退款进行打款退款;如想结束退款,请调用结束退款来关闭退款订单
    ///
    /// 说明:调用申请极速退款、上送卡信息退款、结束退款前,请联系易宝提前开通相应的退款服务。
    ///
    /// 示例值：SUCCESS
    pub status: Option<String>,
    /// 退款申请金额
    pub refund_amount: Option<Decimal>,
    /// 退款受理时间
    pub refund_request_date: Option<String>,
    /// 退还商户手续费
    pub refund_merchant_fee: Option<Decimal>,
    /// 退款资金来源信息
    ///
    /// - debitAmount：扣账金额
    /// - accountType：扣账资金来源
    ///     - FUND_ACCOUNT：资金账户余额
    ///     - DIVIDE_ACCOUNT：分账账户余额
    ///     - SETTLE_ACCOUNT：待结算账户余额
    ///     - HANDLE_ACCOUNT：后收手续费账户余额
    ///
    /// 示例值：[{"accountType":"FUND_ACCOUNT","debitAmount":6.00}]
    pub refund_account_detail: Option<Value>,
    /// 扣账时间
    pub refund_cs_finish_date: Option<String>,
    /// 信用分单号
    pub credit_order_id: Option<String>,
    /// extra
    #[serde(default, flatten)]
    pub extra: Option<HashMap<String, Value>>,
}

/// 订单退款查询
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderRefundQueryReq {
    /// 商户订单编号
    pub order_id: String,
    /// 退款请求号
    pub refund_request_id: String,
}

/// 订单退款查询响应
#[derive(Debug, Deserialize)]
pub struct OrderRefundQueryResp {
    /// 商户收款请求号
    pub order_id: Option<String>,
    /// 商户退款请求号
    pub refund_request_id: Option<String>,
    /// 易宝收款订单号
    pub unique_order_no: Option<String>,
    /// 易宝退款订单号
    pub unique_refund_no: Option<String>,
    /// 退款申请金额
    pub refund_amount: Option<Decimal>,
    /// 退回商户手续费金额
    pub return_merchant_fee: Option<Decimal>,
    /// 退款状态
    /// - PROCESSING：退款处理中
    /// - SUCCESS：退款成功
    /// - FAILED：退款失败
    /// - CANCEL:退款关闭,商户通知易宝结束该笔退款后返回该状态
    /// - SUSPEND:退款中断,如需继续退款,请调用上送卡信息退款进行打款退款;如想结束退款,请调用结束退款来关闭退款订单
    /// 说明:调用申请极速退款、上送卡信息退款、结束退款前,请联系易宝提前开通相应的退款服务。
    ///
    /// 示例值：SUCCESS
    pub status: Option<String>,
    /// 退款原因的简要描述
    pub description: Option<String>,
    /// 退款受理时间
    pub refund_request_date: Option<String>,
    /// 退款成功日期
    pub refund_success_date: Option<String>,
    /// 退款失败原因
    pub fail_reason: Option<String>,
    /// 实际退款金额
    /// - 用户付手续费场景下,实际退款金额即退款金额和退费金额之和
    pub real_refund_amount: Option<Decimal>,
    /// 用户实退金额
    pub cash_refund_fee: Option<Decimal>,
    /// 支付方式:
    /// - ACCOUNTBOOKPAY:记账簿支付
    /// - ALIPAY:支付宝
    /// - WECHAT:微信
    /// - UNIONPAY:云闪付
    /// - BANKACCOUNTPAY:银行账户支付
    /// - BANKTRANSFERPAY:银行转账支付
    /// - DCEP:数字人民币支付
    /// - E_BANK:网银支付
    /// - ENTERPRISEACCOUNTPAY:企业账户支付
    /// - INSTALLMENT:信用卡分期支付
    /// - MEMBERACCOUNTPAY:个人账户支付
    /// - NCPAY:快捷支付
    /// - OVERSEASCARDPAY:境外卡支付
    /// - POS:刷卡支付
    /// - YZS:一站式还款
    pub payment_method: Option<String>,
    /// extra
    #[serde(default, flatten)]
    pub extra: Option<HashMap<String, Value>>,
}
