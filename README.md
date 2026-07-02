# Yeepay OpenAPI SDK for Rust

使用示例

```Rust
get!(
    YeepayClient,order_query,"/rest/v1.0/trade/order/query",OrderQueryReq,OrderQueryResp
);

post_form!(
    YeepayClient,order_divide_complete,"/rest/v1.0/divide/complete",OrderDivideCompleteReq,OrderDivideCompleteResp
);

#[tokio::test]
async fn test_order_query() {
    let order_id = "1906384502419177472";
    let client = YeepayClient::new(config());
    let query = OrderQueryReq::new(&client.config.merchant_no, order_id);
    let result = client.order_query(&query).await.unwrap();
    let data = result.into_data().unwrap();
    println!("{:?}", data);
    println!("{:?}", data.extra);
}

```