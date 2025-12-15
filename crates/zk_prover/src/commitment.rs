//! Helpers for computing policy commitments.

use sha2::{Digest, Sha256};

use crate::circuit::PredicateKind;

/// Compute a 32 byte commitment from the predicate metadata and a blinding.
pub fn policy_commitment(kind: PredicateKind, threshold: u64, blinding: &[u8; 32]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(b"fixr-policy-v1");
    hasher.update(&[kind.as_u8()]);
    hasher.update(&threshold.to_le_bytes());
    hasher.update(blinding);
    hasher.finalize().into()
}

/// Mix a proof hash into a transcript that downstream consumers can verify.
pub fn mix_transcript(prefix: &[u8], proof_hash: &[u8; 32]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(prefix);
    hasher.update(proof_hash);
    hasher.finalize().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn commitment_is_deterministic() {
        let a = policy_commitment(PredicateKind::MinBalance, 100, &[1u8; 32]);
        let b = policy_commitment(PredicateKind::MinBalance, 100, &[1u8; 32]);
        assert_eq!(a, b);
    }

    #[test]
    fn mix_preimage() {
        let digest = mix_transcript(b"ctx", &[3u8; 32]);
        assert_ne!(digest, [0u8; 32]);
    }
}
// rev-01018
