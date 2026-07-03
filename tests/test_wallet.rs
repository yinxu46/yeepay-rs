mod common;

use crate::common::util::*;
use yeepay_rs::YeepayClient;
use yeepay_rs::data::GlobalMerchantWrap;
use yeepay_rs::data::enums::InvalidOrValid;
use yeepay_rs::data::wallet::{
    FreePaymentAgreementQueryReq, FreePaymentAgreementRequestReq, WalletIndexReq, WalletQueryReq,
};

#[tokio::test]
async fn test_wallet_member_query() {
    let client = YeepayClient::new(config());
    let query = GlobalMerchantWrap::new(
        &client.config.merchant_no,
        WalletQueryReq {
            external_user_id: Some("16463".to_string()),
            member_no: None,
        },
    );
    let result = client.wallet_member_query(&query).await.unwrap();
    let data = result.into_data();
    println!("{:?}", data);
}

#[tokio::test]
async fn test_wallet() {
    let client = YeepayClient::new(config());
    let query = GlobalMerchantWrap::new(
        &client.config.merchant_no,
        WalletIndexReq {
            request_no: uuid::Uuid::new_v4().to_string(),
            merchant_user_no: "1234".to_string(),
            return_url: "https://api.cn/return".to_string(),
            notify_url: "https://api.cn/notify".to_string(),
            name: "XX".to_string(),
            certificate_type: "IDENTITY_CARD".to_string(),
            certificate_no: "xxxxxxxxxxxxxx".to_string(),
        },
    );
    let result = client.wallet(&query).await.unwrap();
    let data = result.into_data();
    println!("{:?}", data);
}

#[tokio::test]
async fn test_free_payment_agreement_request() {
    let client = YeepayClient::new(config());
    let query = GlobalMerchantWrap::new(
        &client.config.merchant_no,
        FreePaymentAgreementRequestReq {
            merchant_user_no: "1234".to_string(),
            request_no: uuid::Uuid::new_v4().to_string(),
            return_url: "https://api.cn".to_string(),
            notify_url: None,
        },
    );
    let result = client.free_payment_agreement_request(&query).await.unwrap();
    let data = result.into_data();
    println!("{:?}", data);
}

#[tokio::test]
async fn test_free_payment_agreement_query() {
    let client = YeepayClient::new(config());
    let query = GlobalMerchantWrap::new(
        &client.config.merchant_no,
        FreePaymentAgreementQueryReq {
            merchant_user_no: "1234".to_string(),
        },
    );
    let result = client.free_payment_agreement_query(&query).await.unwrap();
    let agreement_status = result.data.unwrap().agreement_status.unwrap();
    assert_eq!(agreement_status, InvalidOrValid::INVALID);
}
