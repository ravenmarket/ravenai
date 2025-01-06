use anchor_lang::prelude::*;
use crate::error::MarketError;

pub const STATE_SEED: &[u8] = b"state_v1";
pub const ESCROW_SEED: &[u8] = b"escrow_vault";

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct InitStateArgs {
    admin_pubkey: Pubkey,
    creation_fee: u64,
    creator_fee_percent: u64,
    start_incentive_percent: u64,
    end_incentive_percent: u64,
}

pub fn init_state_impl(ctx: Context<InitState>, args: InitStateArgs) -> Result<()> {
    require!(args.creator_fee_percent <= 100, MarketError::InvalidArgument);
    require!(args.start_incentive_percent <= 100, MarketError::InvalidArgument);
    require!(args.end_incentive_percent <= 100, MarketError::InvalidArgument);

    let state = &mut ctx.accounts.state;
    state.admin_pubkey = args.admin_pubkey;
    state.escrow_vault = ctx.accounts.escrow.key();
    state.market_creation_fee_lamports = args.creation_fee;
    state.creator_fee_percent = args.creator_fee_percent;
    state.start_incentive_percent = args.start_incentive_percent;
    state.end_incentive_percent = args.end_incentive_percent;

    state.allowed_pricefeeds = vec![];
    state.markets = vec![];

    msg!(
        "InitState => admin={:?}, creationFee={}, creatorFee={}, startIncentive={}, endIncentive={}, escrow={:?}",
        args.admin_pubkey,
        args.creation_fee,
        args.creator_fee_percent,
        args.start_incentive_percent,
        args.end_incentive_percent,
        ctx.accounts.escrow.key()
    );
    Ok(())
}

// 5.1 InitState
#[derive(Accounts)]
#[instruction(admin_pubkey: Pubkey, creation_fee: u64, creator_fee_percent: u64, start_incentive_percent: u64, end_incentive_percent: u64)]
pub struct InitState<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        seeds = [STATE_SEED],
        bump,
        payer = payer,
        space = 50_000 
    )]
    pub state: Account<'info, State>,

    #[account(
        init_if_needed,
        seeds = [ESCROW_SEED],
        bump,
        payer = payer,
        space = 8 // 不存数据，仅用作 PDA
    )]
    /// CHECK: 只用来做 Escrow 的 PDA。这里我们不需要存储数据。
    pub escrow: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

/// 全局 State
#[account]
#[derive(Debug)]
pub struct State {
    pub admin_pubkey: Pubkey,
    pub escrow_vault: Pubkey,

    pub market_creation_fee_lamports: u64,
    pub creator_fee_percent: u64,

    pub start_incentive_percent: u64, 
    pub end_incentive_percent: u64,

    pub allowed_pricefeeds: Vec<PriceFeedConfig>,
    pub markets: Vec<Market>,
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Debug, Clone)]
pub struct PriceFeedConfig {
    pub symbol: String,
    pub pyth_account: Pubkey,
    pub min_betting_period: u64,
    pub max_betting_period: u64,
    pub min_settling_period: u64,
    pub max_settling_period: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Debug, Clone)]
pub struct Market {
    pub market_id: String,
    pub creation_time: u64,
    pub paused: bool,
    pub fee_rate: u64,       
    pub betting_period: u64, 
    pub settling_period: u64,
    pub creator_pubkey: Pubkey,

    // pyth feed
    pub pyth_price_account: Pubkey,

    pub current_round: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Debug, Clone)]
pub struct Round {
    pub round_index: u64,
    pub start_time: u64,
    pub end_time: u64,

    pub start_price: Option<i64>,
    pub end_price: Option<i64>,
    pub start_price_set: bool,
    pub end_price_set: bool,

    pub total_up: u64,
    pub total_down: u64,
    pub bets: Vec<Bet>,

    pub settled: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Debug, Clone)]
pub struct Bet {
    pub user: Pubkey,
    pub amount: u64,
    pub direction: Direction,
}
