use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::Sysvar as SolanaSysvar;

pub mod error;
pub mod instructions;

use instructions::init_state::*;
use instructions::price_feed::*;
use instructions::market::*;
use instructions::bet::*;

declare_id!("Cph9VnsRozrHidnJykp1eT1ZAQZbZ3zKvsJxj89zjttu");

#[program]
pub mod anchor_prediction_market {
    use instructions::market::{create_market_impl, pause_market_impl, resume_market_impl};

    use super::*;

    // --------------------------
    //    4.1 InitState
    // --------------------------
    pub fn init_state(ctx: Context<InitState>, args: InitStateArgs) -> Result<()> {
        init_state_impl(ctx, args)
    }

    // --------------------------
    //    4.2 AddPriceFeed
    // --------------------------
    pub fn add_price_feed(ctx: Context<AddPriceFeed>, args: AddPriceFeedArgs) -> Result<()> {
        add_price_feed_impl(ctx, args)
    }

    // --------------------------
    //    4.3 RemovePriceFeed
    // --------------------------
    pub fn remove_price_feed(ctx: Context<RemovePriceFeed>, symbol: String) -> Result<()> {
        remove_price_feed_impl(ctx, symbol)
    }

    // --------------------------
    //    4.4 CreateMarket
    // --------------------------
    pub fn create_market(ctx: Context<CreateMarket>, args: CreateMarketArgs) -> Result<()> {
        create_market_impl(ctx, args)
    }

    // --------------------------
    //    4.5 PauseMarket
    // --------------------------
    pub fn pause_market(ctx: Context<PauseMarket>, market_id: String) -> Result<()> {
        pause_market_impl(ctx, market_id)
    }

    // --------------------------
    //    4.6 ResumeMarket
    // --------------------------
    pub fn resume_market(ctx: Context<ResumeMarket>, market_id: String) -> Result<()> {
        resume_market_impl(ctx, market_id)
    }

    // --------------------------
    //    4.7 UserBet
    // --------------------------
    pub fn user_bet(
        ctx: Context<UserBet>,
        direction: u8,
        amount: u64,
        market_id: String,
    ) -> Result<()> {
        user_bet_impl(ctx, direction, amount, market_id)
    }

    // --------------------------
    //    4.8 AutoSettleAll
    // --------------------------
    pub fn auto_settle_all(
        ctx: Context<AutoSettleAll>,
        max_confidence: u64,
    ) -> Result<()> {
        auto_settle_all_impl(ctx, max_confidence)
    }

    // --------------------------
    //    4.9 UpdateSettleIncentive
    // --------------------------
    pub fn update_settle_incentive(
        ctx: Context<UpdateSettleIncentive>,
        new_start_incentive_percent: u64,
        new_end_incentive_percent: u64,
    ) -> Result<()> {
        update_settle_incentive_impl(ctx, new_start_incentive_percent, new_end_incentive_percent)
    }


    pub fn query_all_markets(_ctx: Context<QueryAllMarkets>) -> Result<()> {
        msg!("QueryAllMarkets => simply read state.markets on client side");
        Ok(())
    }

    pub fn query_market_by_id(_ctx: Context<QueryMarketById>, market_id: String) -> Result<()> {
        msg!("QueryMarketById => market_id={}, read from state in client", market_id);
        Ok(())
    }

    pub fn query_all_pricefeeds(_ctx: Context<QueryAllPriceFeeds>) -> Result<()> {
        msg!("QueryAllPriceFeeds => read from state.allowed_pricefeeds in client");
        Ok(())
    }

    pub fn query_settle_incentive(_ctx: Context<QuerySettleIncentive>) -> Result<()> {
        msg!("QuerySettleIncentive => read from state in client");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct QueryAllMarkets<'info> {
    #[account(
        seeds = [STATE_SEED],
        bump
    )]
    pub state: Account<'info, State>,
}

#[derive(Accounts)]
pub struct QueryMarketById<'info> {
    #[account(
        seeds = [STATE_SEED],
        bump
    )]
    pub state: Account<'info, State>,
}

#[derive(Accounts)]
pub struct QueryAllPriceFeeds<'info> {
    #[account(
        seeds = [STATE_SEED],
        bump
    )]
    pub state: Account<'info, State>,
}

#[derive(Accounts)]
pub struct QuerySettleIncentive<'info> {
    #[account(
        seeds = [STATE_SEED],
        bump
    )]
    pub state: Account<'info, State>,
}
