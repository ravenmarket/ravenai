use anchor_lang::prelude::*;

use crate::error::MarketError;
use crate::constants::*;
use crate::state::*;

pub fn init_state_impl(ctx: Context<InitState>, admin_pubkey: Pubkey, creator_fee_percent: u8) -> Result<()> {
    require!(creator_fee_percent <= 100, MarketError::InvalidArgument);

    let state = &mut ctx.accounts.state;
    state.admin_pubkey = admin_pubkey;
    state.creator_fee_percent = creator_fee_percent;

    state.allowed_pricefeeds = vec![];
    state.markets = vec![];

    let (escrow_pubkey, bump) = Pubkey::find_program_address(&[ESCROW_SEED], ctx.program_id);
    state.escrow_pubkey = escrow_pubkey;
    state.escrow_bump = bump;
    
    msg!(
        "InitState => admin={:?}, creationFee={}",
        admin_pubkey,
        creator_fee_percent,
    );
    Ok(())
}

// 5.1 InitState
#[derive(Accounts)]
pub struct InitState<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        seeds = [STATE_SEED],
        bump,
        payer = payer,
        space = 10_000 
    )]
    pub state: Account<'info, State>,

    pub system_program: Program<'info, System>,
}
