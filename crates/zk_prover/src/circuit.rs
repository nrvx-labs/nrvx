use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[repr(u8)]
pub enum PredicateKind {
    MinBalance = 0, MaxBalance = 1, NftOwnership = 2,
    TradingVolume = 3, NotBlacklisted = 4, CustomHash = 5,
}
