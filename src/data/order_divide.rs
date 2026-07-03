use crate::data::enums::OrderDivideRule;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderDivideCompleteReq {
    pub order_id: String,
    pub divide_request_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderDivideCompleteResp {
    pub order_id: Option<String>,
    pub divide_request_id: Option<String>,
    pub divide_status: Option<String>,
    pub divide_success_date: Option<String>,
    pub amount: Option<Decimal>,
    pub biz_system_no: Option<String>,
    pub unique_order_no: Option<String>,
    pub create_date: Option<String>,
    #[serde(default, flatten)]
    pub extra: Option<HashMap<String, Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderDivideApplyReq {
    /// 商户收款请求号
    pub order_id: String,
    /// 商户分账请求号
    pub divide_request_id: String,
    /// 易宝收款订单号: 收款交易对应在易宝的收款单号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_order_no: Option<String>,
    /// 分账详情
    pub divide_detail: Vec<OrderDivideDetail>,
    /// 分账总金额
    ///
    /// 基于此金额按商户传入比例计算接收方应分金额，
    ///
    /// 如：待分账金额 100.00, 分账总金额 50.00 (小于或等于待分账金额)
    /// - A分账比例80% = 40.00,
    /// - B分账比例20% = 10.00
    ///
    /// - 【金额格式】支持两位小数，不可为0
    /// - 【最大金额】需要小于等于订单剩余可分账金额。
    /// - 【必填条件】分账规则为指定比例分账时，必填
    #[serde(skip_serializing_if = "Option::is_none")]
    pub divide_amount: Option<Decimal>,
    /// 分账规则，非必填，不填默认按照指定金额分账
    /// - AMOUNT：指定金额分账
    /// - PROPORTION：指定比例分账
    #[serde(skip_serializing_if = "Option::is_none")]
    pub divide_rule: Option<OrderDivideRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderDivideDetail {
    // ledgerNoFrom：分账发起方编号，非必填。不填默认为收款商编，长度不能超过 32 字节。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ledger_no_from: Option<String>,
    // ledgerNo: 分账接收方编号，必填。分账属性为分账给商户时，为接收分账资金的易宝商户编号；分账属性为分账给个人会员时，为接收分账资金的易宝会员，长度不能超过 32 字节
    pub ledger_no: String,
    /// 分账金额: 分账金额，按金额分账时必填，两位小数，不可为0；按比例分账时不可填。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Decimal>,
    /// 分账比例: 分账比例，按比例分账时必填，最大支持两位小数，必须大于 0 小于 100，且所有分账明细比例之和必须等于 100，按金额分账时不可填。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proportion: Option<Decimal>,
    // divideDetailDesc: 分账说明，非必填，长度不能超过 128 字节。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub divide_detail_desc: Option<String>,
    // ledgerType: 分账属性，非必填。可选项如下：MERCHANT2MERCHANT（分账给商户），MERCHANT2MEMBER（分账给个人会员）。不填默认分账给商户
    pub ledger_type: OrderDivideLedgerType,
    #[serde(default, flatten, skip_serializing_if = "Option::is_none")]
    pub extra: Option<HashMap<String, Value>>,
}

impl OrderDivideDetail {
    /// # New OrderDivideDetail
    /// - ledger_type: 只允许 MERCHANT2MERCHANT（分账给商户），MERCHANT2MEMBER（分账给个人会员）
    pub fn new(ledger_type: OrderDivideLedgerType, ledger_no: &str) -> Self {
        Self {
            ledger_no_from: None,
            ledger_no: ledger_no.to_string(),
            amount: None,
            proportion: None,
            divide_detail_desc: None,
            ledger_type,
            extra: None,
        }
    }

    /// 按金额分账
    pub fn with_amount(&mut self, amount: Decimal) -> &mut Self {
        self.amount = Some(amount);
        self.proportion = None;
        self
    }

    /// 按比例分账
    pub fn with_proportion(&mut self, proportion: Decimal) -> &mut Self {
        self.proportion = Some(proportion);
        self.amount = None;
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderDivideLedgerType {
    /// 商户分账给商户
    MERCHANT2MERCHANT,
    /// 商户分账会员
    MERCHANT2MEMBER,
    /// 商户分账至入账方
    MERCHANT2RECEIVER,
}

#[derive(Debug, Deserialize)]
pub struct OrderDivideApplyResp {
    #[serde(default, flatten)]
    pub extra: Option<HashMap<String, Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderDivideQueryReq {
    /// 商户收款请求号: 原支付订单的商户订单号
    pub order_id: String,
    /// 商户分账请求号
    pub divide_request_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderDivideQueryResp {
    pub order_id: String,
    pub divide_request_id: String,
    pub unique_order_no: Option<String>,
    pub status: Option<String>,
    pub divide_detail: Option<Vec<OrderDivideDetail>>,
    pub create_date: Option<String>,
    pub divide_success_date: Option<String>,

    #[serde(default, flatten)]
    pub extra: Option<HashMap<String, Value>>,
}

/// 分账退回
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderDivideBackReq {
    /// 商户分账资金归还请求号
    pub divide_back_request_id: String,
    /// 商户分账请求号: 原分账的商户请求号
    pub divide_request_id: String,
    /// 商户收款请求号
    pub order_id: String,
    /// 分账资金归还明细
    pub divide_back_detail: Vec<OrderDivideBackDetail>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderDivideBackDetail {
    // amount: 退回金额
    pub amount: Decimal,
    // divideBackReason: 退回原因
    pub divide_back_reason: String,
    // divideDetailNo: 易宝分账明细单号,分账完成后易宝返回
    pub divide_detail_no: String,
}

#[derive(Debug, Deserialize)]
pub struct OrderDivideBackResp {
    /// 业务方标识
    pub biz_system_no: Option<String>,
    /// 商户收款请求号
    pub order_id: Option<String>,
    /// 易宝收款订单号
    pub unique_order_no: Option<String>,
    /// 商户分账请求号
    pub divide_request_id: Option<String>,
    /// 商户分账资金归还请求号
    pub divide_back_request_id: Option<String>,
    /// 易宝分账资金归还订单号
    pub unique_divide_back_no: Option<String>,
    /// 分账资金归还明细
    pub divide_back_detail: Option<Value>,
    /// 分账资金归还状态
    /// - PROCESSING 处理中
    /// - SUCCESS 归还成功
    /// - FAIL 归还失败
    pub status: Option<String>,
    /// 分账申请时间
    pub create_date: Option<String>,
    /// 分账退回失败原因
    pub fail_reason: Option<String>,
    /// 分账退回资金来源详情
    ///
    /// JSON字符传
    /// - debitAmount：扣账金额
    /// - accountType：扣账账户类型
    /// - returnFee：退手续费金额
    /// - realDivideBackAmount：分账退回金额
    /// - ledgerNo：分账接收方
    ///
    ///
    /// 扣账账户类型枚举值：
    /// - FUND_ACCOUNT：资金账户
    /// - SETTLE_ACCOUNT：待结算账户
    ///
    /// 示例值：[{"ledgerNo":"123456678","debitAmount":"10.01","accountType":"SETTLE_ACCOUNT"}]
    pub divide_back_account_detail: Option<Value>,
    /// 申请分账资金归还成功时间
    pub divide_back_success_date: Option<String>,
    #[serde(default, flatten)]
    pub extra: Option<HashMap<String, Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderDivideBackQueryReq {
    /// 商户收款请求号
    pub order_id: String,
    /// 商户分账资金归还请求号
    pub divide_back_request_id: String,
}

#[derive(Debug, Deserialize)]
pub struct OrderDivideBackQueryResp {
    /// 业务方标识
    pub biz_system_no: Option<String>,
    /// 商户收款请求号
    pub order_id: Option<String>,
    /// 易宝收款订单号
    pub unique_order_no: Option<String>,
    /// 商户分账请求号
    pub divide_request_id: Option<String>,
    /// 商户分账资金归还请求号
    pub divide_back_request_id: Option<String>,
    /// 易宝分账资金归还订单号
    pub unique_divide_back_no: Option<String>,
    /// 分账资金归还详情
    pub divide_back_detail: Option<Value>,
    /// 分账资金归还状态
    ///
    /// - PROCESSING 处理中
    /// - SUCCESS 归还成功
    /// - FAIL 归还失败
    pub status: Option<String>,
    /// 分账申请时间
    pub create_date: Option<String>,
    /// 分账退回失败原因
    /// - 示例值：商户1234567账户余额不足
    pub error_message: Option<String>,
    /// 分账退回资金来源详情
    ///
    /// JSON字符传
    /// - debitAmount：扣账金额
    /// - accountType：扣账账户类型
    /// - ledgerNo：分账接收方
    /// - returnFee：退手续费金额
    /// - realDivideBackAmount：分账退回金额
    ///
    ///
    /// 扣账资金来源枚举值：
    /// - FUND_ACCOUNT：资金账户
    /// - SETTLE_ACCOUNT：待结算账户
    ///
    /// 示例值：[{"ledgerNo":"123456678","debitAmount":"10.01","accountType":"SETTLE_ACCOUNT"}]
    pub divide_back_account_detail: Option<Value>,
    pub divide_back_success_date: Option<String>,
    #[serde(default, flatten)]
    pub extra: Option<HashMap<String, Value>>,
}
