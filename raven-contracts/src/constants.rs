use anchor_lang::prelude::*;

#[constant]
pub const STATE_SEED: &[u8] = b"state_v1";
#[constant]
pub const ESCROW_SEED: &[u8] = b"escrow_vault";
#[constant]
pub const ROUND_SEED: &[u8] = b"Round";

pub const ANCHOR_DISCRIMINATOR: usize = 8;
