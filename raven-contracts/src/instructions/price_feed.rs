use anchor_lang::prelude::*;

use crate::error::MarketError;
use crate::constants::*;
use crate::state::*;

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct AddPriceFeedArgs {
    symbol: String,
    pyth_feed_id: String,
    min_bet_period: u16,
    max_bet_period: u16,
    min_settle_period: u16,
    max_settle_period: u16,
    create_market_lamports: u64,
    min_betting_lamports: u64,
}

pub fn add_price_feed_impl(ctx: Context<AddPriceFeed>, args: AddPriceFeedArgs) -> Result<()> {
    let state = &mut ctx.accounts.state;

    require!(
        args.max_bet_period >= args.min_bet_period && args.max_settle_period >= args.min_settle_period,
        MarketError::InvalidArgument
    );

    for pf in &state.allowed_pricefeeds {
        if pf.symbol == args.symbol {
            return err!(MarketError::AccountAlreadyInitialized);
        }
    }

    let config = PriceFeedConfig {
        symbol: args.symbol.clone(),
        pyth_feed_id: args.pyth_feed_id,
        min_betting_period: args.min_bet_period,
        max_betting_period: args.max_bet_period,
        min_settling_period: args.min_settle_period,
        max_settling_period: args.max_settle_period,
        create_market_lamports: args.create_market_lamports,
        min_betting_lamports: args.min_betting_lamports,
    };
    state.allowed_pricefeeds.push(config);

    msg!("AddPriceFeed => symbol={}", args.symbol);
    Ok(())
}

// 5.2 AddPriceFeed
#[derive(Accounts)]
pub struct AddPriceFeed<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [STATE_SEED],
        bump,
        // constraint = state.admin_pubkey == payer.key() @ MarketError::IllegalOwner
    )]
    pub state: Account<'info, State>,

    pub system_program: Program<'info, System>,
}

pub fn remove_price_feed_impl(ctx: Context<RemovePriceFeed>, symbol: String) -> Result<()> {
    let state = &mut ctx.accounts.state;
    let pos_opt = state
        .allowed_pricefeeds
        .iter()
        .position(|pf| pf.symbol == symbol);
    if let Some(idx) = pos_opt {
        state.allowed_pricefeeds.remove(idx);
        msg!("RemovePriceFeed => symbol={}", symbol);
        Ok(())
    } else {
        err!(MarketError::InvalidArgument)
    }
}

// 5.3 RemovePriceFeed
#[derive(Accounts)]
pub struct RemovePriceFeed<'info> {
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
