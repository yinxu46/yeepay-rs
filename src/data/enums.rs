use serde::{Deserialize, Serialize};

/// # INVALID OR VALID
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum InvalidOrValid {
    /// 无效
    INVALID,
    /// 有效
    VALID,
}

/// # ORDER DIVIDE RULE
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderDivideRule {
    /// 按金额
    AMOUNT,
    /// 按比例
    PROPORTION,
}
