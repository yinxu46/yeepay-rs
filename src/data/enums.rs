use serde::{Deserialize, Serialize};

/// # INVALID OR VALID
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum InvalidOrValid {
    INVALID,
    VALID,
}

/// # ORDER DIVIDE RULE
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderDivideRule {
    AMOUNT,
    PROPORTION,
}
