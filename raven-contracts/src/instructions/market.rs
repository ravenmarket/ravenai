use anchor_lang::{prelude::*, solana_program::{program::invoke_signed, system_instruction}};

use crate::{error::MarketError, Market, Round};

use super::{State, STATE_SEED};

use crate::instructions::bet::*;

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct CreateMarketArgs {
    fee_rate: u64,
    betting_period: u64,
    settling_period: u64,
    market_id: String,
}

pub fn create_market_impl(ctx: Context<CreateMarket>, args: CreateMarketArgs) -> Result<()> {
    let state = &mut ctx.accounts.state;
    let creator = &ctx.accounts.creator;

    let pf_opt = state
        .allowed_pricefeeds
        .iter()
        .find(|pf| pf.symbol == args.market_id);
    require!(pf_opt.is_some(), MarketError::InvalidArgument);
    let pf = pf_opt.unwrap();

    require!(
        (args.betting_period >= pf.min_betting_period)
            && (args.betting_period <= pf.max_betting_period)
            && (args.settling_period >= pf.min_settling_period)
            && (args.settling_period <= pf.max_settling_period),
        MarketError::InvalidArgument
    );

    if state.market_creation_fee_lamports > 0 {
        let ix = system_instruction::transfer(
            creator.key,
            ctx.accounts.admin_info.key,
            state.market_creation_fee_lamports,
        );
        invoke_signed(
            &ix,
            &[
                creator.to_account_info(),
                ctx.accounts.admin_info.clone(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[],
        )?;
    }

    require!(args.fee_rate <= 10000, MarketError::InvalidArgument);

    for m in &state.markets {
        if m.market_id == args.market_id {
            return err!(MarketError::AccountAlreadyInitialized);
        }
    }

    let now = Clock::get()?.unix_timestamp as u64;
    let mut new_mk = Market {
        market_id: args.market_id.clone(),
        creation_time: now,
        paused: false,
        fee_rate: args.fee_rate.clone(),
        betting_period: args.betting_period,
        settling_period: args.settling_period,
        creator_pubkey: creator.key(),
        pyth_price_account: pf.pyth_account,
        // current_round: null_mut(),
    };
    let (s, e) = compute_round_time(now, args.betting_period, args.settling_period, 1);
    let r1 = Round {
        round_index: 1,
        start_time: s,
        end_time: e,
        start_price: None,
        end_price: None,
        start_price_set: false,
        end_price_set: false,
        total_up: 0,
        total_down: 0,
        bets: vec![],
        settled: false,
    };
    state.markets.push(new_mk);

    msg!("CreateMarket => {}", args.market_id);
    Ok(())
}

#[derive(Accounts)]
pub struct CreateMarket<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    pub admin_info: AccountInfo<'info>,

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
    require!(mk_opt.is_some(), MarketError::InvalidArgument);
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
    pub signer: Signer<'info>,

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
    require!(mk_opt.is_some(), MarketError::InvalidArgument);
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
