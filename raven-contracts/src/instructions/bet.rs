use std::collections::BTreeMap;

use anchor_lang::{prelude::*, solana_program::{program::invoke_signed, system_instruction}};
use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, PriceUpdateV2};
use crate::error::MarketError;
use crate::constants::*;
use crate::state::*;


// --------------------------
//    4.7 UserBet
// --------------------------

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct UserBetArgs {
    market_id: String,
    round_index: u32,
    direction: u8,
    amount: u64,
}

pub fn user_bet_impl<'a>(ctx: Context<UserBet>, args: UserBetArgs) -> Result<()> {
    require!(args.amount > 0, MarketError::InvalidArgument);

    let state = &mut ctx.accounts.state;
    let user = &ctx.accounts.user;
    let escrow = &ctx.accounts.escrow;

    let mk_opt = state.markets.iter().find(|m| m.market_id == args.market_id);
    require!(mk_opt.is_some(), MarketError::InvalidMarket);
    let mk = mk_opt.unwrap();

    require!(!mk.paused, MarketError::MarketPaused);
    require!(args.amount >= mk.min_betting_price, MarketError::InvalidArgument);

    let round = &mut ctx.accounts.round;

    let now = Clock::get()?.unix_timestamp as u32;
    require!(now <= round.start_time + mk.betting_period as u32, MarketError::InvalidTime);


    invoke_signed(
        &system_instruction::transfer(user.key, &state.escrow_pubkey, args.amount),
        &[
            user.to_account_info(),
            escrow.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
        &[],
    )?;

    let dir = match args.direction {
        1 => Direction::Up,
        2 => Direction::Down,
        _ => return err!(MarketError::InvalidArgument),
    };

    round.bets.push(Bet {
        user: user.key(),
        amount: args.amount,
        result: 0,
        direction: dir,
        refunded: false,
    });
    match dir {
        Direction::Up => round.total_up = round.total_up.saturating_add(args.amount),
        Direction::Down => round.total_down = round.total_down.saturating_add(args.amount),
    }

    msg!(
        "UserBet => market={}, user={}, amount={}, dir={:?}",
        mk.market_id,
        user.key(),
        args.amount,
        dir
    );
    Ok(())
}

#[derive(Accounts)]
#[instruction(args: UserBetArgs)]
pub struct UserBet<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [STATE_SEED],
        bump
    )]
    pub state: Account<'info, State>,

    #[account(
        mut, 
        seeds = [ROUND_SEED, args.round_index.to_le_bytes().as_ref()],
        bump
    )]
    pub round: Account<'info, Round>,

    /// CHECK: escrow_vault
    #[account(
        mut, 
        seeds = [ESCROW_SEED],
        bump
    )]
    pub escrow: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}


// --------------------------
//    4.8 AutoSettleAll
// --------------------------

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct ProcessRoundArgs {
    market_id: String,
    round_index: u32,
    maximum_age: u32,
    feed_id: String,
    // time: u32,
    // price: u64,
}

pub fn process_round_impl(ctx: Context<ProcessRound>, args: ProcessRoundArgs) -> Result<()> {
    let system_program = &ctx.accounts.system_program;
    let creater = &ctx.accounts.creater;
    let admin = &ctx.accounts.admin;
    let escrow = &ctx.accounts.escrow;

    require!(*admin.key == ctx.accounts.state.admin_pubkey, MarketError::InvalidArgument);

    let clock = Clock::get()?;
    let now = clock.unix_timestamp as u32;
    // let now = args.time;

    let state = &mut ctx.accounts.state;
    let escrow_bump = state.escrow_bump;
    let c_percent = state.creator_fee_percent;

    let mk_opt = state.markets.iter_mut().find(|m| m.market_id == args.market_id);
    require!(mk_opt.is_some(), MarketError::InvalidMarket);
    let mk = mk_opt.unwrap();

    require!(*creater.key == mk.creator_pubkey, MarketError::InvalidArgument);

    let round = &mut ctx.accounts.round;
    let price_update = &ctx.accounts.price_update;
    let pyth_price = price_update.get_price_no_older_than(
        &clock, args.maximum_age as u64, &get_feed_id_from_hex(&args.feed_id)?,
    )?;
    require!(pyth_price.price > 0, MarketError::InvalidPythPrice);
    let price = pyth_price.price as u64;
    // let price = args.price;

    if round.start_price == 0 && round.start_time == 0 {
        round.start_price = price;
        round.start_time = now;
        round.end_time = now + (mk.betting_period + mk.settling_period) as u32;
    }

    if round.end_price == 0 && now >= round.end_time {
        round.end_price = price;
    }

    if round.end_price > 0 && !round.settled {
        let sp = round.start_price;
        let ep = round.end_price;
        let total_up = round.total_up;
        let total_down = round.total_down;

        let fee_rate = mk.fee_rate;
        let winner_dir = if sp < ep {
            Some(Direction::Up)
        } else if sp > ep {
            Some(Direction::Down)
        } else {
            None
        };

        if let Some(wdir) = winner_dir {
            // loser_pool
            let loser_pool = if wdir == Direction::Up {
                total_down
            } else {
                total_up
            };
            if loser_pool == 0 {
                for bet in round.bets.iter_mut() {
                    bet.result = bet.amount;
                }
            } else {
                let total_fee = loser_pool.saturating_mul(fee_rate as u64) / 100;
                let distributable = loser_pool.saturating_sub(total_fee);

                // creator_fee_percent
                let fee_creator = total_fee.saturating_mul(c_percent as u64) / 100;
                let fee_admin = total_fee.saturating_sub(fee_creator);

                if fee_creator > 0 {
                    transfer_sol(&escrow, &creater, fee_creator, system_program, escrow_bump)?;
                }
                if fee_admin > 0 {
                    transfer_sol(&escrow, &admin, fee_admin, system_program, escrow_bump)?;
                }

                let winner_pool = if wdir == Direction::Up {
                    total_up
                } else {
                    total_down
                };
                if winner_pool == 0 {
                    msg!("No winners => leftover remains in vault");
                } else {
                    for bet in round.bets.iter_mut() {
                        if bet.direction == wdir {
                            let ratio = bet.amount.saturating_mul(1000u64) / winner_pool;
                            let share = distributable * ratio / 1000u64;
                            bet.result = bet.amount.saturating_add(share);
                        }
                    }
                }
            }
        } else {
            for bet in round.bets.iter_mut() {
                bet.result = bet.amount;
            }
        }

        mk.round_index = mk.round_index + 1;
        round.settled = true;
    }

    Ok(())
}


#[derive(Accounts)]
#[instruction(args: ProcessRoundArgs)]
pub struct ProcessRound<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [STATE_SEED],
        bump
    )]
    pub state: Account<'info, State>,

    pub price_update: Account<'info, PriceUpdateV2>,

    #[account(
        init_if_needed, 
        payer = signer,
        space = ANCHOR_DISCRIMINATOR + Round::INIT_SPACE, 
        seeds = [ROUND_SEED, args.round_index.to_le_bytes().as_ref()],
        bump
    )]
    pub round: Account<'info, Round>,

    /// CHECK: market creater
    #[account(mut)]
    pub creater: AccountInfo<'info>,

    /// CHECK: market administrator
    #[account(mut)]
    pub admin: AccountInfo<'info>,

    /// CHECK: escrow_vault
    #[account(
        mut, 
        seeds = [ESCROW_SEED],
        bump
    )]
    pub escrow: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

// --------------------------
//    refund round
// --------------------------
pub fn refund_round_impl<'info>(
    ctx: Context<'_, '_, '_, 'info, RefundRound<'info>>,
    _market_id: String,
    _round_index: u32,
) -> Result<()> {
    let state = &ctx.accounts.state;
    let round = &mut ctx.accounts.round;

    let mut pubkey_ai_map = BTreeMap::new();
    for ai in ctx.remaining_accounts.iter() {
        pubkey_ai_map.insert(ai.key, ai.to_account_info());
    }

    for bet in round.bets.iter_mut() {
        if let Some(ai) = pubkey_ai_map.get(&bet.user) {
            if !bet.refunded && bet.result > 0 {
                transfer_sol(&ctx.accounts.escrow, &ai, bet.result, &ctx.accounts.system_program, state.escrow_bump)?;
                bet.refunded = true;
            }
        }
    }
    Ok(())
}

#[derive(Accounts)]
#[instruction(market_id: String, round_index: u32)]
pub struct RefundRound<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [STATE_SEED],
        bump
    )]
    pub state: Account<'info, State>,

    #[account(
        mut, 
        seeds = [ROUND_SEED, round_index.to_le_bytes().as_ref()],
        bump
    )]
    pub round: Account<'info, Round>,

    /// CHECK: escrow_vault
    #[account(
        mut, 
        seeds = [ESCROW_SEED],
        bump
    )]
    pub escrow: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

// --------------------------
//    close round
// --------------------------
pub fn close_round_impl(
    _ctx: Context<CloseRound>,
) -> Result<()> {
    Ok(())
}


#[derive(Accounts)]
#[instruction(round_index: u32)]
pub struct CloseRound<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [STATE_SEED],
        bump
    )]
    pub state: Account<'info, State>,

    #[account(
        mut, 
        seeds = [ROUND_SEED, round_index.to_le_bytes().as_ref()],
        close = signer,
        bump
    )]
    pub round: Account<'info, Round>,

    pub system_program: Program<'info, System>,

}

fn transfer_sol<'info>(
    escrow: &AccountInfo<'info>,
    to: &AccountInfo<'info>,
    amount: u64,
    system_program: &Program<'info, System>,
    bump: u8,
) -> Result<()> {
    let seeds = &[ESCROW_SEED.as_ref(), &[bump]];
    let ix = system_instruction::transfer(&escrow.key, &to.key, amount);
    invoke_signed(
        &ix,
        &[
            escrow.to_account_info(),
            to.to_account_info(),
            system_program.to_account_info(),
        ],
        &[seeds],
    )?;
    Ok(())
}
