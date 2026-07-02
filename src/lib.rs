mod api;
pub mod data;

pub use yeepay_core::{YeepayClientTrait, YeepayConfig};

#[derive(Debug)]
pub struct YeepayClient {
    pub config: YeepayConfig,
}

impl YeepayClient {
    pub fn new(config: YeepayConfig) -> Self {
        Self { config }
    }
}

impl YeepayClientTrait for YeepayClient {
    fn config(&self) -> &YeepayConfig {
        &self.config
    }
}
