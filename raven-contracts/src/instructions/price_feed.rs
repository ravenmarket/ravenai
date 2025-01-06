use anchor_lang::prelude::*;

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct AddPriceFeedArgs {
    symbol: String,
    pyth_account: Pubkey,
    min_bet_period: u64,
    max_bet_period: u64,
    min_settle_period: u64,
    max_settle_period: u64,
}

pub fn add_price_feed_impl(ctx: Context<AddPriceFeed>, args: AddPriceFeedArgs) -> Result<()> {
    let state = &mut ctx.accounts.state;

    require!(
        max_bet_period >= min_bet_period && max_settle_period >= min_settle_period,
        MarketError::InvalidArgument
    );

    for pf in &state.allowed_pricefeeds {
        if pf.symbol == symbol {
            return err!(MarketError::AccountAlreadyInitialized);
        }
    }

    let config = PriceFeedConfig {
        symbol: symbol.clone(),
        pyth_account,
        min_betting_period: min_bet_period,
        max_betting_period: max_bet_period,
        min_settling_period: min_settle_period,
        max_settling_period: max_settle_period,
    };
    state.allowed_pricefeeds.push(config);

    msg!("AddPriceFeed => symbol={}", symbol);
    Ok(())
}

// 5.2 AddPriceFeed
#[derive(Accounts)]
pub struct AddPriceFeed<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [STATE_SEED],
        bump,
        constraint = state.admin_pubkey == admin.key() @ MarketError::IllegalOwner
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
