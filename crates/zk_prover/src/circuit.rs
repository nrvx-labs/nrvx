//! Predicate definitions used by the prover.

use serde::{Deserialize, Serialize};

/// Supported predicate kinds. These mirror the kinds accepted by the on-chain
/// program and must stay in sync with the SDK enum.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[repr(u8)]
pub enum PredicateKind {
    MinBalance = 0,
    MaxBalance = 1,
    NftOwnership = 2,
    TradingVolume = 3,
    NotBlacklisted = 4,
    CustomHash = 5,
}

impl PredicateKind {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::MinBalance),
            1 => Some(Self::MaxBalance),
            2 => Some(Self::NftOwnership),
            3 => Some(Self::TradingVolume),
            4 => Some(Self::NotBlacklisted),
            5 => Some(Self::CustomHash),
            _ => None,
        }
    }

    pub fn as_u8(self) -> u8 {
        self as u8
    }
}

/// Describe a predicate in a form that can be logged or serialised for CLI
/// tools.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PredicateDescriptor {
    pub kind: PredicateKind,
    pub threshold: u64,
    pub label: String,
}

impl PredicateDescriptor {
    pub fn min_sol(threshold_sol: u64) -> Self {
        Self {
            kind: PredicateKind::MinBalance,
            threshold: threshold_sol.saturating_mul(1_000_000_000),
            label: format!("min_sol_{}", threshold_sol),
        }
    }

    pub fn max_sol(threshold_sol: u64) -> Self {
        Self {
            kind: PredicateKind::MaxBalance,
            threshold: threshold_sol.saturating_mul(1_000_000_000),
            label: format!("max_sol_{}", threshold_sol),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn predicate_kind_round_trip() {
        for raw in 0u8..=5u8 {
            let kind = PredicateKind::from_u8(raw).unwrap();
            assert_eq!(kind.as_u8(), raw);
        }
    }

    #[test]
    fn min_sol_label_is_stable() {
        let descriptor = PredicateDescriptor::min_sol(100);
        assert_eq!(descriptor.label, "min_sol_100");
        assert_eq!(descriptor.threshold, 100_000_000_000);
    }
}
// rev-01019
