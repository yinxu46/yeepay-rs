mod common;
mod config;
mod csv;
mod xlsx;

use crate::config::config;
use crate::xlsx::{ExcelReaderTrait, read_xlsx};
use calamine::{Data, DataType};
use common::string_to_decimal;
use rust_decimal::Decimal;
use rust_decimal::prelude::Zero;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;
use yeepay_rs::YeepayClient;
use yeepay_rs::data::order_divide::OrderDivideCompleteReq;

#[tokio::test]
async fn batch_divide() {
    let orders = read_xlsx::<Order>("data/20250401-0420.xlsx").expect("Read Xlsx Fail");
    // let orders = read_csv::<Order>("data/20250401-0420.csv").expect("Read Csv Fail");

    println!("Total Order Size: {}", orders.len());

    let un_divide_orders: Vec<_> = orders.iter().filter(|o| o.has_un_split()).collect();
    let size = un_divide_orders.len();

    println!("UnDivide Order Size: {}", size);

    let un_divide_amount: Decimal = un_divide_orders.iter().map(|o| o.last_unsplit_amount).sum();
    println!("UnDivide Order Amount: {}", un_divide_amount);

    let client = YeepayClient::new(config());
    let api = Arc::new(client);

    // ✅ 控制并发数（根据 Yop 接口 QPS 调整）
    let semaphore = Arc::new(Semaphore::new(80));
    let mut join_set = JoinSet::new();

    // ✅ 跳过部分订单（用于断线重开）
    let start_index = 0;
    let mut start = false;

    for (index, order) in un_divide_orders.iter().enumerate() {
        println!("✅ Run: {} / {}", index, size);
        if index >= start_index {
            start = true;
        }
        if !start {
            continue;
        }

        let api = Arc::clone(&api);
        let permit = Arc::clone(&semaphore).acquire_owned().await.unwrap();

        let order_id = order.order_id.clone();

        join_set.spawn(async move {
            // ✅ 可选：防止请求过快
            // tokio::time::sleep(Duration::from_millis(50)).await;

            let request = OrderDivideCompleteReq::new(&api.config.merchant_no, &order_id);
            let result = api.order_divide_complete(&request).await;

            drop(permit); // ✅ 释放并发许可

            match result {
                Ok(_) => {
                    println!("✅ Ok: {}", order_id)
                }
                Err(err) => eprintln!("❌ Error {}: {}", order_id, err),
            }
        });
    }

    // ✅ 等待所有任务完成
    while let Some(res) = join_set.join_next().await {
        if let Err(e) = res {
            eprintln!("任务异常: {:?}", e);
        }
    }

    println!("✅ 所有订单分账完成！")
}

#[derive(Debug, Deserialize, Serialize)]
struct Order {
    #[serde(rename = "商户订单号")]
    order_id: String,
    #[serde(rename = "剩余可分账金额", deserialize_with = "string_to_decimal")]
    last_unsplit_amount: Decimal,
}

impl Order {
    pub fn has_un_split(&self) -> bool {
        self.last_unsplit_amount > Decimal::zero()
    }
}

impl ExcelReaderTrait for Order {
    fn parse_row(row: &[Data]) -> Result<Option<Self>, Box<dyn Error>>
    where
        Self: Sized,
    {
        let order_id = row[1].as_string().unwrap_or_default().to_string();
        let last_unsplit_amount = row[10]
            .as_string()
            .unwrap_or_default()
            .parse::<Decimal>()
            .unwrap_or(Decimal::zero());
        Ok(Some(Order {
            order_id,
            last_unsplit_amount,
        }))
    }
}
