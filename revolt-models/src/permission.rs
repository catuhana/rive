use serde::{Deserialize, Serialize};

/// Representation of a single permission override
/// as it appears on models and in the database
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OverrideField {
    /// Allow bit flags
    a: u64,
    /// Disallow bit flags
    d: u64,
}

/// Representation of a single permission override
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Override {
    /// Allow bit flags
    allow: u64,
    /// Disallow bit flags
    deny: u64,
}
