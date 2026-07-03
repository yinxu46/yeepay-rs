mod common;

use crate::common::util::*;
use rust_decimal::Decimal;
use yeepay_rs::YeepayClient;
use yeepay_rs::data::GlobalMerchantWrap;
use yeepay_rs::data::order_create::{OrderBusinessInfo, OrderCreateReq};
use yeepay_rs::data::order_divide::OrderDivideCompleteReq;
use yeepay_rs::data::order_query::OrderQueryReq;

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

#[tokio::test]
async fn test_order_query() {
    let order_id = "123456789";
    let client = YeepayClient::new(config());
    let req = GlobalMerchantWrap::new(
        &client.config.merchant_no,
        OrderQueryReq {
            order_id: order_id.to_string(),
        },
    );
    let result = client.order_query(&req).await.unwrap();
    let data = result.into_data().unwrap();
    println!("{:?}", data);
}

#[tokio::test]
async fn test_order_divide_complete() {
    let order_id = "123456789";
    let client = YeepayClient::new(config());
    let req = GlobalMerchantWrap::new(
        &client.config.merchant_no,
        OrderDivideCompleteReq {
            order_id: order_id.to_string(),
            divide_request_id: format!("{}_D", order_id),
        },
    );
    let data = client.order_divide_complete(&req).await;
    println!("{:?}", data);
}
