use anchor_lang::{prelude::*, solana_program::{program::invoke_signed, system_instruction}};
use pyth_sdk::{Price, PriceFeed};
use pyth_sdk_solana::state::{PriceStatus, SolanaPriceAccount};
use crate::{error::MarketError, Bet, Direction, Round, State, ESCROW_SEED, STATE_SEED};

// --------------------------
//    4.7 UserBet
// --------------------------
pub fn user_bet_impl(
    ctx: Context<UserBet>,
    direction: u8,
    amount: u64,
    market_id: String,
) -> Result<()> {
    require!(amount > 0, MarketError::InvalidArgument);

    let st = &mut ctx.accounts.state;
    let user = &ctx.accounts.user;
    let escrow = &ctx.accounts.escrow;

    require_keys_eq!(escrow.key(), st.escrow_vault);

    let mk_opt = st.markets.iter_mut().find(|m| m.market_id == market_id);
    require!(mk_opt.is_some(), MarketError::InvalidArgument);
    let mk = mk_opt.unwrap();

    require!(!mk.paused, MarketError::MarketPaused);

    let r_opt = mk.rounds.iter_mut().find(|r| r.round_index == mk.current_round);
    require!(r_opt.is_some(), MarketError::InvalidArgument);

    let round = r_opt.unwrap();

    let now = Clock::get()?.unix_timestamp as u64;
    require!(now <= round.end_time, MarketError::InvalidArgument);

    let ix = system_instruction::transfer(user.key, escrow.key, amount);
    invoke_signed(
        &ix,
        &[
            user.to_account_info(),
            escrow.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
        &[],
    )?;

    let dir = match direction {
        1 => Direction::Up,
        2 => Direction::Down,
        _ => return err!(MarketError::InvalidArgument),
    };

    round.bets.push(Bet {
        user: user.key(),
        amount,
        direction: dir,
    });
    match dir {
        Direction::Up => round.total_up = round.total_up.saturating_add(amount),
        Direction::Down => round.total_down = round.total_down.saturating_add(amount),
    }

    msg!(
        "UserBet => market={}, user={}, amount={}, dir={:?}",
        mk.market_id,
        user.key(),
        amount,
        dir
    );
    Ok(())
}

#[derive(Accounts)]
pub struct UserBet<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [STATE_SEED],
        bump
    )]
    pub state: Account<'info, State>,

    /// CHECK: escrow_vault
    #[account(mut)]
    pub escrow: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

// --------------------------
//    4.8 AutoSettleAll
// --------------------------
pub fn auto_settle_all_impl(
    ctx: Context<AutoSettleAll>,
    max_confidence: u64,
) -> Result<()> {
    let state = &mut ctx.accounts.state;
    let signer = &ctx.accounts.signer;
    let escrow = &ctx.accounts.escrow;
    let system_program = &ctx.accounts.system_program;

    require_keys_eq!(escrow.key(), state.escrow_vault);

    let clock = Clock::get()?;
    let now = clock.unix_timestamp as u64;

    // Pyth test first
    let pyth_pid = Pubkey::new_unique();

    let seeds: &[&[u8]] = &[ESCROW_SEED];
    let (_pda, bump) = Pubkey::find_program_address(seeds, ctx.program_id);


    for mk in state.markets.iter_mut() {
        //todo: read from PDA
        let (roundAccount, bump) = Pubkey::find_program_address(
            &[b"round", mk.current_round.as_ref()],
            ctx.program_id,
        );
        let mut data_slice: &[u8] = &roundAccount.data.borrow();

        let round: Round = 
            Round::try_deserialize(
                &mut data_slice,
            )?;

        if !round.start_price_set && now >= round.start_time {
            let pyth_acc_opt = ctx.remaining_accounts.iter().find(|acc| acc.key() == mk.pyth_price_account);
            if let Some(oracle_acc) = pyth_acc_opt {
                let p_res = get_pyth_price(oracle_acc, &pyth_pid, max_confidence);
                if p_res.is_ok() {
                    let price = p_res.unwrap().price;
                    round.start_price = Some(price);
                    round.start_price_set = true;

                    let total_pool = (round.total_up as u128).saturating_add(round.total_down as u128);
                    let incentive_a = total_pool
                        .saturating_mul(state.start_incentive_percent as u128)
                        / 100;
                    if incentive_a > 0 {
                        let inc_amount = incentive_a as u64;
                        transfer_from_escrow(
                            &escrow.to_account_info(),
                            &signer.key(),
                            inc_amount,
                            system_program,
                            ctx.program_id,
                            bump,
                        )?;
                        msg!("Incentive => lock startPrice: {} lamports to {}", inc_amount, signer.key());
                    }
                }
            }
        }

        if !round.end_price_set && now >= round.end_time {
            let pyth_acc_opt = ctx.remaining_accounts.iter().find(|acc| acc.key() == mk.pyth_price_account);
            if let Some(oracle_acc) = pyth_acc_opt {
                let p_res = get_pyth_price(oracle_acc, &pyth_pid, max_confidence);
                if p_res.is_ok() {
                    let price = p_res.unwrap().price;
                    round.end_price = Some(price);
                    round.end_price_set = true;

                    let total_pool = (round.total_up as u128).saturating_add(round.total_down as u128);
                    state.end_incentive_percent = 1;
                    let incentive_b = total_pool
                        .saturating_mul(state.end_incentive_percent as u128)
                        / 100;
                    if incentive_b > 0 {
                        let inc_amount = incentive_b as u64;
                        transfer_from_escrow(
                            &escrow.to_account_info(),
                            &signer.key(),
                            inc_amount,
                            system_program,
                            ctx.program_id,
                            bump,
                        )?;
                        msg!("Incentive => lock endPrice: {} lamports to {}", inc_amount, signer.key());
                    }
                }
            }
        }

        round_settle(&ctx, state, escrow, system_program, bump, mk, round)?;
    }

    Ok(())
}

fn round_settle(ctx: &Context<'_, '_, '_, '_, AutoSettleAll<'_>>, state: &mut Account<'_, State>, escrow: &AccountInfo<'_>, system_program: &Program<'_, System>, bump: u8, mk: &mut super::Market, mut round: Pubkey) -> Result<(), Error> {
    Ok(if round.end_price_set && !round.settled {
        let sp = round.start_price.unwrap_or(0);
        let ep = round.end_price.unwrap_or(0);
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
                for bet in &round.bets {
                    refund_bet(bet, &escrow.to_account_info(), system_program, ctx.program_id, bump)?;
                }
            } else {
                let total_fee = loser_pool.saturating_mul(fee_rate) / 10000;
                let distributable = loser_pool.saturating_sub(total_fee);

                // creator_fee_percent
                let c_percent = state.creator_fee_percent.min(100);
                let fee_creator = total_fee.saturating_mul(c_percent) / 100;
                let fee_admin = total_fee.saturating_sub(fee_creator);

                if fee_creator > 0 {
                    transfer_from_escrow(
                        &escrow.to_account_info(),
                        &mk.creator_pubkey,
                        fee_creator,
                        system_program,
                        ctx.program_id,
                        bump,
                    )?;
                }
                if fee_admin > 0 {
                    transfer_from_escrow(
                        &escrow.to_account_info(),
                        &state.admin_pubkey,
                        fee_admin,
                        system_program,
                        ctx.program_id,
                        bump,
                    )?;
                }

                let winner_pool = if wdir == Direction::Up {
                    total_up
                } else {
                    total_down
                };
                if winner_pool == 0 {
                    msg!("No winners => leftover remains in vault");
                } else {
                    for bet in &round.bets {
                        if bet.direction == wdir {
                            let ratio = (bet.amount as u128)
                                .saturating_mul(1_000_000_000u128)
                                / (winner_pool as u128);
                            let share = distributable as u128
                                * ratio
                                / 1_000_000_000u128;
                            let share_u64 = share as u64;
                            let total_payout = bet.amount.saturating_add(share_u64);

                            if total_payout > 0 {
                                transfer_from_escrow(
                                    &escrow.to_account_info(),
                                    &bet.user,
                                    total_payout,
                                    system_program,
                                    ctx.program_id,
                                    bump,
                                )?;
                            }
                        }
                    }
                }
            }
        } else {
            for bet in &round.bets {
                refund_bet(bet, &escrow.to_account_info(), system_program, ctx.program_id, bump)?;
            }
        }

        round.settled = true;

        let nxt_idx = mk.current_round + 1;
        let (ns, ne) = compute_round_time(
            mk.creation_time,
            mk.betting_period,
            mk.settling_period,
            nxt_idx,
        );
        let nr = Round {
            round_index: nxt_idx,
            start_time: ns,
            end_time: ne,
            start_price: None,
            end_price: None,
            start_price_set: false,
            end_price_set: false,
            total_up: 0,
            total_down: 0,
            bets: vec![],
            settled: false,
        };
        mk.rounds.push(nr);
        mk.current_round = nxt_idx;
    })
}

#[derive(Accounts)]
pub struct AutoSettleAll<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [STATE_SEED],
        bump
    )]
    pub state: Account<'info, State>,

    /// CHECK: escrow_vault
    #[account(mut)]
    pub escrow: AccountInfo<'info>,

    pub system_program: Program<'info, System>,

}

// --------------------------
//    4.9 UpdateSettleIncentive
// --------------------------
pub fn update_settle_incentive_impl(
    ctx: Context<UpdateSettleIncentive>,
    new_start_incentive_percent: u64,
    new_end_incentive_percent: u64,
) -> Result<()> {
    let state = &mut ctx.accounts.state;
    require!(new_start_incentive_percent <= 100, MarketError::InvalidArgument);
    require!(new_end_incentive_percent <= 100, MarketError::InvalidArgument);

    state.start_incentive_percent = new_start_incentive_percent;
    state.end_incentive_percent = new_end_incentive_percent;

    msg!(
        "UpdateSettleIncentive => start={}, end={}",
        new_start_incentive_percent,
        new_end_incentive_percent
    );
    Ok(())
}

#[derive(Accounts)]
pub struct UpdateSettleIncentive<'info> {
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

pub fn compute_round_time(
    creation_time: u64,
    betting_period: u64,
    settling_period: u64,
    i: u64,
) -> (u64, u64) {
    let start = creation_time + betting_period + (i - 1) * (betting_period + settling_period);
    let end = start + settling_period;
    (start, end)
}

fn transfer_from_escrow<'info>(
    escrow_info: &AccountInfo<'info>,
    to: &Pubkey,
    amount: u64,
    system_program: &Program<'info, System>,
    program_id: &Pubkey,
    bump: u8,
) -> Result<()> {
    if amount == 0 {
        return Ok(());
    }
    let seeds = &[b"escrow_vault".as_ref(), &[bump]];
    let ix = system_instruction::transfer(escrow_info.key, to, amount);
    invoke_signed(
        &ix,
        &[
            escrow_info.clone(),
            system_program.to_account_info().clone(),
        ],
        &[seeds],
    )?;
    Ok(())
}

fn refund_bet<'info>(
    bet: &Bet,
    escrow_info: &AccountInfo<'info>,
    system_program: &Program<'info, System>,
    program_id: &Pubkey,
    bump: u8,
) -> Result<()> {
    if bet.amount == 0 {
        return Ok(());
    }
    let seeds = &[b"escrow_vault".as_ref(), &[bump]];
    let ix = system_instruction::transfer(escrow_info.key, &bet.user, bet.amount);
    invoke_signed(
        &ix,
        &[
            escrow_info.clone(),
            system_program.to_account_info().clone(),
        ],
        &[seeds],
    )?;
    Ok(())
}

#[derive(Debug)]
pub struct PythPriceResult {
    pub price: i64,
    pub exponent: i32,
    pub confidence: u64,
    pub status: PriceStatus,
    pub valid_slot: u64,
}

fn get_pyth_price(
    oracle_account: &AccountInfo,
    pyth_pid: &Pubkey,
    max_conf: u64,
) -> Result<PythPriceResult> {
    require_keys_eq!(*oracle_account.owner, *pyth_pid, MarketError::IllegalOwner);

    const STALENESS_THRESHOLD: u64 = 60;

    let clock = Clock::get()?;
    let price_feed: PriceFeed = SolanaPriceAccount::account_info_to_feed(oracle_account)
        .map_err(|_| error!(MarketError::InvalidAccountData))?;

    let current_price: Price = price_feed
        .get_price_no_older_than(clock.unix_timestamp, STALENESS_THRESHOLD)
        .ok_or_else(|| error!(MarketError::PriceStale))?;

    let raw_price = current_price.price;
    let expo = current_price.expo;
    let conf = current_price.conf;

    require!(conf <= max_conf, MarketError::ConfidenceTooHigh);

    Ok(PythPriceResult {
        price: raw_price,
        exponent: expo,
        confidence: conf,
        status: PriceStatus::Trading,
        valid_slot: 0,
    })
}
