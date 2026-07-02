use yeepay_core::YeepayConfig;

pub fn config() -> YeepayConfig {
    dotenvy::dotenv().ok();

    let app_key = std::env::var("HW_APPKEY").unwrap_or_default();
    let merchant_no = std::env::var("HW_MERCHANT_NO").unwrap_or_default();
    let public_key = std::env::var("HW_PUBLIC_KEY").unwrap_or_default();
    let private_key = std::env::var("HW_PRIVATE_KEY").unwrap_or_default();
    YeepayConfig::new(app_key, merchant_no, public_key, private_key).with_debug()
}