use anchor_lang::{prelude::*, solana_program::system_instruction};

use crate::error::MarketError;
use crate::constants::*;
use crate::state::*;

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct CreateMarketArgs {
    fee_rate: u8,
    betting_period: u16,
    settling_period: u16,
    market_id: String,
    symbol: String,
}

pub fn create_market_impl(ctx: Context<CreateMarket>, args: CreateMarketArgs) -> Result<()> {
    require!(args.fee_rate <= 100, MarketError::InvalidArgument);

    let state = &mut ctx.accounts.state;
    let creator = &ctx.accounts.creator;

    require!(args.fee_rate <= 100, MarketError::InvalidArgument);

    let pf_opt = state.allowed_pricefeeds.iter()
        .find(|pf| pf.symbol == args.symbol);
    require!(pf_opt.is_some(), MarketError::InvalidPriceFeed);
    let pf = pf_opt.unwrap();

    require!(
        (args.betting_period >= pf.min_betting_period)
            && (args.betting_period <= pf.max_betting_period)
            && (args.settling_period >= pf.min_settling_period)
            && (args.settling_period <= pf.max_settling_period),
        MarketError::InvalidArgument
    );
    require!(*ctx.accounts.admin.key == state.admin_pubkey, MarketError::InvalidArgument);

    if pf.create_market_lamports > 0 {
        let ix = system_instruction::transfer(
            creator.key,
            ctx.accounts.admin.key,
            pf.create_market_lamports,
        );
        anchor_lang::solana_program::program::invoke_signed(
            &ix,
            &[
                creator.to_account_info(),
                ctx.accounts.admin.clone(),
                // ctx.accounts.system_program.to_account_info(),
            ], &[]
        )?;
    }

    for m in &state.markets {
        if m.market_id == args.market_id {
            return err!(MarketError::AccountAlreadyInitialized);
        }
    }

    let now = Clock::get()?.unix_timestamp as u32;
    let new_mk = Market {
        market_id: args.market_id.clone(),
        pyth_feed_id: pf.pyth_feed_id.clone(),
        creation_time: now,
        paused: false,
        fee_rate: args.fee_rate,
        min_betting_price: pf.min_betting_lamports,
        betting_period: args.betting_period,
        settling_period: args.settling_period,
        creator_pubkey: creator.key(),
        round_index: 0,
        // current_round: null_mut(),
    };
    state.markets.push(new_mk);

    msg!("CreateMarket => {}", args.market_id);
    Ok(())
}

#[derive(Accounts)]
pub struct CreateMarket<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    /// CHECK: 
    #[account(mut)]
    pub admin: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [STATE_SEED],
        bump
    )]
    pub state: Account<'info, State>,

    pub system_program: Program<'info, System>,
}

// --------------------------
//    4.5 PauseMarket
// --------------------------
pub fn pause_market_impl(ctx: Context<PauseMarket>, market_id: String) -> Result<()> {
    let signer = &ctx.accounts.signer;
    let is_admin_user = signer.key() == ctx.accounts.state.admin_pubkey && signer.is_signer;

    let mk_opt = ctx.accounts.state.markets.iter_mut().find(|m| m.market_id == market_id);
    require!(mk_opt.is_some(), MarketError::InvalidMarket);
    let mk = mk_opt.unwrap();

    let is_creator_user = signer.key() == mk.creator_pubkey && signer.is_signer;
    require!((is_admin_user || is_creator_user), MarketError::IllegalOwner);

    mk.paused = true;
    msg!("PauseMarket => {}", mk.market_id);
    Ok(())
}

#[derive(Accounts)]
pub struct PauseMarket<'info> {
    #[account(mut)]
    pub signer: Signer<'info>, // 可能是 creator 或 admin

    #[account(
        mut,
        seeds = [STATE_SEED],
        bump
    )]
    pub state: Account<'info, State>,
}

// --------------------------
//    4.6 ResumeMarket
// --------------------------
pub fn resume_market_impl(ctx: Context<ResumeMarket>, market_id: String) -> Result<()> {
    let state = &mut ctx.accounts.state;
    let admin = &ctx.accounts.admin;

    require!(admin.key() == state.admin_pubkey && admin.is_signer, MarketError::IllegalOwner);

    let mk_opt = state.markets.iter_mut().find(|m| m.market_id == market_id);
    require!(mk_opt.is_some(), MarketError::InvalidMarket);
    mk_opt.unwrap().paused = false;

    msg!("ResumeMarket => {}", market_id);
    Ok(())
}

#[derive(Accounts)]
pub struct ResumeMarket<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [STATE_SEED],
        bump,
        constraint = state.admin_pubkey == admin.key() @ MarketError::IllegalOwner
    )]
    pub state: Account<'info, State>,
}
