use crate::data::enums::InvalidOrValid;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// # 钱包账户信息查询
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletQueryReq {
    /// 商户用户ID
    /// - 与会员号二选一必填
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_user_id: Option<String>,
    /// 易宝会员号
    /// - 与商户用户ID二选一必填
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_no: Option<String>,
}

impl WalletQueryReq {
    pub fn from_merchant_user_id(merchant_user_id: &str) -> Self {
        Self {
            external_user_id: Some(merchant_user_id.to_string()),
            member_no: None,
        }
    }
    pub fn from_member_no(member_no: &str) -> Self {
        Self {
            external_user_id: None,
            member_no: Some(member_no.to_string()),
        }
    }
}

/// 钱包查询结果
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletQueryResp {
    /// 会员编号
    pub member_no: Option<String>,
    /// 商户用户ID
    pub external_user_id: Option<String>,
    /// 钱包会员状态
    /// - 未激活(UNACTIVED)
    /// - 正常(AVAILABLY)
    /// - 冻结(FREEZE)
    /// - 已注销(CANCEL)
    pub wallet_status: Option<String>,
    /// 钱包账户状态
    ///
    /// 钱包账户状态，如果不是正常，则不允许发起钱包交易行为
    ///
    /// - 账户正常(ACCOUNT_AVAILABLE)
    /// - 账户冻结(ACCOUNT_FROZEN)
    /// - 账户注销(ACCOUNT_CANCELLED)
    pub wallet_account_status: Option<String>,
    /// 钱包账户等级
    /// - 一类(ONE_CATEGORY)
    /// - 二类(TWO_CATEGORY)
    /// - 三类(THREE_CATEGORY)
    pub wallet_category: Option<String>,
    /// 续费状态
    /// - 有效(VALID)
    /// - 失效(INVALID)
    pub renew_status: Option<String>,
    #[serde(default, flatten)]
    pub extra: Option<HashMap<String, Value>>,
}

/// 钱包开通/打开
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletIndexReq {
    /// 商户请求流水号
    pub request_no: String,
    /// 商户用户ID
    pub merchant_user_no: String,
    /// 页面重定向地址（钱包首页返回商户页面地址）
    pub return_url: String,
    /// 服务器回调地址（开户完成后通知地址）
    pub notify_url: String,
    /// 姓名
    pub name: String,
    /// 证件类型 (certificateNo有值时，该字段必填) 可选项如下: IDENTITY_CARD:身份证
    pub certificate_type: String,
    /// 证件号码（证件号码和手机号二选一必填）
    pub certificate_no: String,
}

/// 开通钱包/打开钱包结果
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletIndexResp {
    /// 钱包地址
    pub url: Option<String>,
    #[serde(default, flatten)]
    pub extra: Option<HashMap<String, Value>>,
}

/// # 注销会员钱包
///
/// https://open.yeepay.com/docs/products/ptssfk/api/post__rest__v1.0__m-wallet__wallet__cancel
///
/// 钱包注销需知：
///
/// - 1.注销前，用户需完成解绑全部银行卡
/// - 2.注销前，用户需保证钱包余额已清零并无在途提现资金
/// - 3.钱包注销后，将无法使用本钱包，包括绑卡、充值、提现、支付、收款等功能
/// - 4.钱包注销后，将无法再次找回曽添加或绑定的任何内容或信息，包括不限于交易记录
/// - 5.钱包注销后，该平台的此账号将无法再次开通钱包，请谨慎操作
/// - 6.如需接收注销通知，需提供通知地址
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletCancelReq {
    /// 商户用户ID
    pub merchant_user_no: String,
}

/// 注销会员钱包结果
///
/// https://open.yeepay.com/docs/products/ptssfk/api/post__rest__v1.0__m-wallet__wallet__cancel
///
/// - 无响应结果
///
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletCancelResp {
    #[serde(default, flatten)]
    pub extra: Option<HashMap<String, Value>>,
}

/// 免密协议查询
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FreePaymentAgreementQueryReq {
    /// 商户用户编号
    pub merchant_user_no: String,
}

/// 免密协议查询结果
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FreePaymentAgreementQueryResp {
    /// 协议状态：INVALID OR VALID
    pub agreement_status: Option<InvalidOrValid>,
    #[serde(default, flatten)]
    pub extra: Option<HashMap<String, Value>>,
}

/// 开通免密协议
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FreePaymentAgreementRequestReq {
    /// 商户用户编号
    pub merchant_user_no: String,
    /// requestNo: 请求流水号
    pub request_no: String,
    /// returnUrl: 页面返回地址
    pub return_url: String,
    /// notify_url: 服务器回调地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_url: Option<String>,
}

/// 开通免密协议结果
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FreePaymentAgreementRequestResp {
    /// url: 跳转地址
    pub url: Option<String>,
    #[serde(default, flatten)]
    pub extra: Option<HashMap<String, Value>>,
}

/// B2C 转帐
///
/// https://open.yeepay.com/docs/products/ptssfk/api/post__rest__v1.0__m-wallet__transfer__b2c__market
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferB2CMarketReq {
    /// 交易主体商编
    /// - 易宝支付分配的的商户唯一标识，转出方商编
    pub from_merchant_no: String,
    /// 转账接收方平台商编
    /// - 转账接收方用户所在的平台商编
    pub to_merchant_no: String,
    /// 转账接收商户用户ID
    /// - 转账接收商户用户ID
    pub to_merchant_user_no: String,
    /// 商户请求号
    /// - 商户系统内部生成的订单号，需要保持在商户下唯一
    pub request_no: String,
    /// 转账订单金额
    /// - 示例值：0.01（精确到两位小数）
    pub order_amount: Decimal,
    /// 手续费承担方
    /// - 不传默认处理为转出方承担（当手续费为商户承担且计费方式为实收时，此参数生效）
    /// - 转出方承担手续费(OUTSIDE)、转账接收方承担手续费(INSIDE)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_charge_side: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
}

impl TransferB2CMarketReq {
    pub fn new(merchant_no: &str, user_id: &str, request_no: &str, amount: Decimal) -> Self {
        Self {
            from_merchant_no: merchant_no.to_string(),
            to_merchant_no: merchant_no.to_string(),
            to_merchant_user_no: user_id.to_string(),
            request_no: request_no.to_string(),
            order_amount: amount,
            fee_charge_side: None,
            notify_url: None,
            remark: None,
        }
    }
}

/// B2C 转帐结果
///
/// https://open.yeepay.com/docs/products/ptssfk/api/post__rest__v1.0__m-wallet__transfer__b2c__market
///
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferB2CMarketResp {
    /// 返回码
    pub return_code: Option<String>,
    /// 返回消息
    pub return_msg: Option<String>,
    /// 请求状态
    /// - 请求已接收(易宝正在处理中，请勿重复下单)   (REQUEST_RECEIVE)、
    /// - 失败(该笔订单转账失败,可重新发起转账)(FAIL)
    pub status: Option<String>,
    /// 商户请求流水号
    pub request_no: Option<String>,
    /// 易宝统一订单号
    pub order_no: Option<String>,
    /// 转账订单金额
    pub order_amount: Option<Decimal>,
    #[serde(default, flatten)]
    pub extra: Option<HashMap<String, Value>>,
}

/// B2C转账查询
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferB2CQueryReq {
    /// 交易主体商编
    /// - 易宝支付分配的的商户唯一标识，转出方商编
    pub from_merchant_no: String,
    /// 商户请求号
    /// - 商户系统内部生成的订单号，与易宝订单号不能同时为空
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_no: Option<String>,
    /// 易宝订单号
    /// - 转账发起完成后，易宝返回的唯一订单号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_no: Option<String>,
}

/// B2C转账查询结果
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferB2CQueryResp {
    /// 商户请求流水号
    pub request_no: Option<String>,
    /// 易宝订单号
    pub business_no: Option<String>,
    /// 转账订单金额
    pub order_amount: Option<Decimal>,
    /// 转账订单状态
    ///
    /// 转账订单的4个状态
    ///
    /// - 已受理-处理中(UNCREDITED)
    /// - 待确认(NOT_VERIFIED)
    /// - 转账成功(CREDITED)
    /// - 转账失败(COMMIT_FAILURE)
    ///
    pub order_status: Option<String>,
    /// 交易主体商编
    /// - 易宝支付分配的的商户唯一标识，转出方商编
    pub from_merchant_no: Option<String>,
    /// 转帐接收方平台商编
    /// - 转帐接收方用户所在平台商编
    pub to_merchant_no: Option<String>,
    /// 转入方易宝用户编号
    /// - 转入方易宝用户编号
    pub to_member_no: Option<String>,

    #[serde(default, flatten)]
    pub extra: Option<HashMap<String, Value>>,
}
