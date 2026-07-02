mod config;

use crate::config::config;
use yeepay_rs::YeepayClient;
use yeepay_rs::data::order_divide::OrderDivideCompleteReq;
use yeepay_rs::data::order_query::OrderQueryReq;

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

#[tokio::test]
async fn test_order_divide_complete() {
    let order_id = "1906384502419177472";
    let client = YeepayClient::new(config());
    let request = OrderDivideCompleteReq::new(&client.config.merchant_no, order_id);
    let data = client.order_divide_complete(&request).await;
    println!("{:?}", data);
}
