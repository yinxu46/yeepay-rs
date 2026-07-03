# Yeepay OpenAPI SDK for Rust

[易宝支付(Yeepay)](https://open.yeepay.com/) Rust SDK，封装易宝 API 签名、请求与响应解析。


当前已完成 **钱包、转账、分账、订单、退款** 等核心场景。



## API列表

| 接口          | 方法                        | 说明               |
|-------------|----------------------------------|------------------|
| 钱包信息查询      | `wallet_member_query`            | 查询会员钱包账户信息       |
| 钱包注册 / 登录   | `wallet`                         | 会员钱包开户 / 登录入口    |
| 注销会员钱包      | `wallet_cancel`                  | 注销钱包（需先解绑卡、清零余额） |
| 营销红包转账（B2C） | `transfer_b2c_market`            | 商户向用户钱包转账        |
| 营销红包转账查询    | `transfer_b2c_query`             | 查询 B2C 转账结果      |
| 开通免密支付      | `free_payment_agreement_request` | 签约免密支付           |
| 免密支付协议查询    | `free_payment_agreement_query`   | 查询免密签约状态         |
| 收银台统一下单 | `order_create`       | 创建支付订单，获取收银台      |
| 查询订单    | `order_query`        | 查询订单状态与详情         |
| 关闭订单    | `order_close`        | 关闭订单              |
| 订单退款    | `order_refund`       | 发起退款（支持部分 / 多次退款） |
| 订单退款查询  | `order_refund_query` | 查询退款执行结果          |
| 申请订单分账     | `order_divide_apply`      | 发起分账请求           |
| 订单分账查询     | `order_divide_query`      | 查询分账结果           |
| 订单分账资金退回   | `order_divide_back`       | 将已分账资金退回分账方      |
| 订单分账资金退回查询 | `order_divide_back_query` | 查询资金退回结果         |
| 订单分账完成     | `order_divide_complete`   | 标记分账结束，剩余金额归收款商户 |

---

## 使用示例

```rust
use yeepay_rs::{YeepayClient, YeepayConfig};
use yeepay_rs::data::GlobalMerchantWrap;
use yeepay_rs::data::order_create::*;
use rust_decimal::Decimal;

fn config() -> YeepayConfig {}

#[tokio::test]
async fn test_order_create() {
    let order_id = "123456789012";
    let client = YeepayClient::new(config());
    let mut order = OrderCreateReq::new(
        order_id,
        Decimal::new(1, 0),
        "Goods Name",
        "2026-07-30 12:00:00",
        "https://api.cn/notify",
    );

    let order_type = 1;

    if order_type == 1 {
        // 售卖订单（无分账）
        order.with_biz_sell("123456789", "https://h5.cn/redirect");
    } else if order_type == 2 {
        // 市场订单（延迟分账）
        order.with_biz_mark(
            "123456",
            "https://h5.cn/redirect",
            OrderBusinessInfo {
                collection_series: "collection_series".to_string(),
                collection_name: "collection_name".to_string(),
                collection_id: "collection_id".to_string(),
                market_type: "BUY".to_string(),
                user_register_id_no: "user_register_id_no".to_string(),
                user_register_mobile: "user_register_mobile".to_string(),
                regist_id: "123456".to_string(),
                regist_ip: "0.0.0.0".to_string(),
                regist_time: "2025-12-01 12:00:00".to_string(),
            },
        );
    } else if order_type == 3 {
        // 免密接口（延迟分账）
        order.with_biz_mark_free("12345678907");
    } else {
        panic!("order_type is invalid");
    }

    println!("{:?}", serde_json::to_value(order.clone()).unwrap());

    let req = GlobalMerchantWrap::new(&client.config.merchant_no, order.to_owned());
    let result = client.order_create(&req).await.unwrap();
    match result.is_success_with_code("00000") {
        true => {
            if let Some(data) = result.data {
                println!("{:?}", data);
            } else {
                println!("data is None");
            }
        }
        false => {
            panic!(
                "code: {} message: {}",
                result.code.unwrap_or_default(),
                result.message.unwrap_or_default()
            );
        }
    }
}
```

---
