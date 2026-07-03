use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 创建订单
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderCreateReq {
    /// 商户收款请求号
    pub order_id: String,
    /// 订单金额
    pub order_amount: Decimal,
    /// 商品名称
    pub goods_name: String,
    /// 过期时间
    /// - 格式为yyyy-MM-dd HH:mm:ss，为空时默认在请求下单120分钟后失效，最长支持30天
    pub expired_time: String,
    /// 回调地址
    pub notify_url: String,
    /// 跳转地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    /// 分账标识：不传，默认不分账 DELAY_SETTLE：分账 合单收款场景中，请在子单域信息subOrderDetail里提供
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fund_process_type: Option<String>,
    /// 限制支付方式： WALLET_PAY：钱包支付, None 免密支付
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_pay_type: Option<String>,
    /// extra
    #[serde(default, flatten, skip_serializing_if = "Option::is_none")]
    pub extra: Option<HashMap<String, serde_json::Value>>,
}

/// 创建订单响应
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderCreateResp {
    /// 易宝收款订单号
    pub unique_order_no: String,
    /// 收银台链接
    pub cashier_url: String,
    /// extra
    #[serde(default, flatten)]
    pub extra: Option<HashMap<String, serde_json::Value>>,
}

impl OrderCreateReq {
    /// 创建订单
    pub fn new(
        order_id: &str,
        order_amount: Decimal,
        goods_name: &str,
        expired_time: &str,
        notify_url: &str,
    ) -> Self {
        Self {
            order_id: order_id.to_string(),
            order_amount,
            goods_name: goods_name.to_string(),
            expired_time: expired_time.to_string(),
            notify_url: notify_url.to_string(),
            redirect_url: None,
            fund_process_type: None,
            limit_pay_type: None,
            extra: None,
        }
    }

    /// 业务：平台售卖（无分账）
    pub fn with_biz_sell(&mut self, user_id: &str, redirect_url: &str) -> &mut Self {
        self.append_extra(
            "payerInfo",
            OrderCreatePayerInfo {
                user_id: user_id.to_string(),
            },
        )
        .with_no_settle()
        .with_wallet_pay()
        .with_redirect_url(redirect_url)
    }

    /// 业务：市场交易（延迟分账）
    pub fn with_biz_mark(
        &mut self,
        user_id: &str,
        redirect_url: &str,
        business_info: OrderBusinessInfo,
    ) -> &mut Self {
        self.append_extra("businessInfo", business_info)
            .append_extra(
                "payerInfo",
                OrderCreatePayerInfo {
                    user_id: user_id.to_string(),
                },
            )
            .with_delay_settle()
            .with_wallet_pay()
            .with_redirect_url(redirect_url)
    }

    /// 业务：市场交易 - 免密（延迟分账）
    /// - member_no: 易宝会员编号
    pub fn with_biz_mark_free(&mut self, member_no: &str) -> &mut Self {
        self.append_extra("payerIp", "0.0.0.0")
            .append_extra("payAgreement", "MICRO_FREE_PWD")
            .append_extra("memberNo", member_no)
            .with_delay_settle()
            .with_free_wallet_pay()
            .with_no_redirect_url()
    }

    /// 延迟分账
    pub fn with_delay_settle(&mut self) -> &mut Self {
        self.fund_process_type = Some("DELAY_SETTLE".to_string());
        self
    }

    /// 不分账
    pub fn with_no_settle(&mut self) -> &mut Self {
        self.fund_process_type = None;
        self
    }

    /// 钱包支付
    pub fn with_wallet_pay(&mut self) -> &mut Self {
        self.limit_pay_type = Some("WALLET_PAY".to_string());
        self
    }

    /// 免密钱包支付
    pub fn with_free_wallet_pay(&mut self) -> &mut Self {
        self.limit_pay_type = None;
        self
    }

    /// 跳转地址
    pub fn with_redirect_url(&mut self, redirect_url: &str) -> &mut Self {
        self.redirect_url = Some(redirect_url.to_string());
        self
    }

    /// 无跳转
    pub fn with_no_redirect_url(&mut self) -> &mut Self {
        self.redirect_url = None;
        self
    }

    /// 添加扩展参数
    pub fn append_extra<T>(&mut self, key: &str, value: T) -> &mut Self
    where
        T: Serialize,
    {
        let value = serde_json::to_value(value).expect("failed to serialize value to JSON");

        self.extra
            .get_or_insert_with(HashMap::new)
            .insert(key.to_string(), value);
        self
    }
}

/// 业务信息
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderBusinessInfo {
    /// 藏品系列
    pub collection_series: String,
    /// 藏品名称
    pub collection_name: String,
    /// 藏品编号
    pub collection_id: String,
    /// 市场类型
    pub market_type: String,
    /// 用户注册证件号码
    pub user_register_id_no: String,
    /// 用户注册手机号码
    pub user_register_mobile: String,
    /// 注册ID
    pub regist_id: String,
    /// 注册IP
    pub regist_ip: String,
    /// 格式：Y-m-d H:i:s
    pub regist_time: String,
}

/// 付款人信息
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderCreatePayerInfo {
    /// 易宝会员编号
    #[serde(rename = "userID")]
    pub user_id: String,
}

/// 关闭订单
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderCloseReq {
    /// 商户收款请求号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// 易宝收款订单号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_order_no: Option<String>,
}

/// 关闭订单响应
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderCloseResp {
    /// 商户收款请求号
    pub order_id: String,
    /// extra
    #[serde(default, flatten)]
    pub extra: Option<HashMap<String, serde_json::Value>>,
}
