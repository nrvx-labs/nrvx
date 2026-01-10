use anchor_lang::prelude::*;

use crate::state::{GenerateProofArgs, ProtocolRegistry};

#[derive(Accounts)]
#[instruction(args: GenerateProofArgs)]
pub struct GenerateProof<'info> {
    #[account(
        mut,
        seeds = [b"fixr-registry"],
        bump = registry.bump,
    )]
    pub registry: Account<'info, ProtocolRegistry>,
    pub requester: Signer<'info>,
}

pub fn handler(ctx: Context<GenerateProof>, args: GenerateProofArgs) -> Result<()> {
    let registry = &mut ctx.accounts.registry;
    registry.proof_counter = registry.proof_counter.saturating_add(1);

    msg!(
        "proof requested: policy_seed=0x{} nonce={}",
        hex_bytes(&args.policy_seed),
        args.nonce,
    );
    Ok(())
}

fn hex_bytes(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push_str(&format!("{:02x}", byte));
    }
    out
}
// rev-01050
