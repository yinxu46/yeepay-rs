# Yeepay OpenAPI SDK for Rust

基于 `yeepay-core` 封装的易宝（Yeepay）钱包 & 交易相关接口 Client，覆盖**钱包管理、转账、分账、订单、退款**等核心场景。

## 方法示例

```rust
use yeepay_rs::YeepayConfig;
use yeepay_rs::YeepayClient;

async fn main() {
    let config = YeepayConfig {};

    let client = YeepayClient::new(config);

    client.wallet_member_query(...);

    client.wallet(...);

    client.wallet_cancel(...);

    client.transfer_b2c_market(...);

    client.transfer_b2c_query(...);

    client.free_payment_agreement_request(...);

    client.free_payment_agreement_query(...);
}

```

---

## 功能概览

### 🪪 钱包相关（m-wallet）

| 接口          | CLIENT 方法                        | 说明               |
|-------------|----------------------------------|------------------|
| 钱包信息查询      | `wallet_member_query`            | 查询会员钱包账户信息       |
| 钱包注册 / 登录   | `wallet`                         | 会员钱包开户 / 登录入口    |
| 注销会员钱包      | `wallet_cancel`                  | 注销钱包（需先解绑卡、清零余额） |
| 营销红包转账（B2C） | `transfer_b2c_market`            | 商户向用户钱包转账        |
| 营销红包转账查询    | `transfer_b2c_query`             | 查询 B2C 转账结果      |
| 开通免密支付      | `free_payment_agreement_request` | 签约免密支付           |
| 免密支付协议查询    | `free_payment_agreement_query`   | 查询免密签约状态         |

---

### 🧾 订单相关（trade / cashier）

| 接口      | CLIENT 方法            | 说明                |
|---------|----------------------|-------------------|
| 收银台统一下单 | `order_create`       | 创建支付订单，获取收银台      |
| 查询订单    | `order_query`        | 查询订单状态与详情         |
| 关闭订单    | `order_close`        | 关闭订单              |
| 订单退款    | `order_refund`       | 发起退款（支持部分 / 多次退款） |
| 订单退款查询  | `order_refund_query` | 查询退款执行结果          |

---

### ➗ 分账相关（divide）

| 接口         | CLIENT 方法                 | 说明               |
|------------|---------------------------|------------------|
| 申请订单分账     | `order_divide_apply`      | 发起分账请求           |
| 订单分账查询     | `order_divide_query`      | 查询分账结果           |
| 订单分账资金退回   | `order_divide_back`       | 将已分账资金退回分账方      |
| 订单分账资金退回查询 | `order_divide_back_query` | 查询资金退回结果         |
| 订单分账完成     | `order_divide_complete`   | 标记分账结束，剩余金额归收款商户 |

---

## 文档来源

 **易宝开放平台**：

https://open.yeepay.com/

---
