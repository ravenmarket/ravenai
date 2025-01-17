use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::Sysvar as SolanaSysvar;

pub mod error;
pub mod state;
pub mod constants;
pub mod instructions;

pub use crate::constants::*;
pub use crate::state::*;

pub use instructions::init_state::*;
pub use instructions::price_feed::*;
pub use instructions::market::*;
pub use instructions::bet::*;

declare_id!("21fdwXkLrfsh1H8tto8fYKmJ3sfJP9W258YTb7J9wbFT");

#[program]
pub mod raven_market {
    use instructions::market::{create_market_impl, pause_market_impl, resume_market_impl};

    use super::*;

    // --------------------------
    //    4.1 InitState
    // --------------------------
    pub fn init_state(ctx: Context<InitState>, admin_pubkey: Pubkey, creator_fee_percent: u8) -> Result<()> {
        init_state_impl(ctx, admin_pubkey, creator_fee_percent)
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
    pub fn user_bet(ctx: Context<UserBet>, args: UserBetArgs) -> Result<()> {
        user_bet_impl(ctx, args)
    }

    // --------------------------
    //    4.8 AutoSettleAll
    // --------------------------
    pub fn process_round(ctx: Context<ProcessRound>, args: ProcessRoundArgs) -> Result<()> {
        process_round_impl(ctx, args)
    }

    // --------------------------
    //    4.9 AutoSettleAll
    // --------------------------
    pub fn refund_round<'info>(
        ctx: Context<'_, '_, '_, 'info, RefundRound<'info>>,
        market_id: String,
        round_index: u32,
    ) -> Result<()> {
        refund_round_impl(ctx, market_id, round_index)
    }

    // --------------------------
    //    4.9 close_round
    // --------------------------
    pub fn close_round(
        ctx: Context<CloseRound>,
    ) -> Result<()> {
        close_round_impl(ctx)
    }
}
