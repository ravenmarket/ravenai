use anchor_lang::prelude::*;

#[account]
#[derive(PartialEq, Debug)]
pub struct State {
    pub admin_pubkey: Pubkey,
    pub escrow_pubkey: Pubkey,
    pub escrow_bump: u8,
    pub creator_fee_percent: u8,

    pub allowed_pricefeeds: Vec<PriceFeedConfig>,
    pub markets: Vec<Market>,
}

#[account]
#[derive(PartialEq, Debug)]
pub struct PriceFeedConfig {
    pub symbol: String,
    pub pyth_feed_id: String,
    pub create_market_lamports: u64,
    pub min_betting_lamports: u64,
    pub min_betting_period: u16,
    pub max_betting_period: u16,
    pub min_settling_period: u16,
    pub max_settling_period: u16,
}

#[account]
#[derive(PartialEq, Debug)]
pub struct Market {
    pub market_id: String,
    pub pyth_feed_id: String,
    pub creation_time: u32,
    pub paused: bool,
    pub fee_rate: u8,       
    pub min_betting_price: u64,
    pub betting_period: u16, 
    pub settling_period: u16,
    pub creator_pubkey: Pubkey,

    pub round_index: u32,
}

#[account]
#[derive(InitSpace, Default, PartialEq, Debug)]
pub struct Round {
    #[max_len(100)]
    pub market_id: String,
    pub start_time: u32,
    pub end_time: u32,

    pub start_price: u64,
    pub end_price: u64,

    pub total_up: u64,
    pub total_down: u64,
    #[max_len(200)]
    pub bets: Vec<Bet>,

    pub settled: bool,
}

#[derive(InitSpace, AnchorSerialize, AnchorDeserialize, PartialEq, Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
}

#[derive(InitSpace, AnchorSerialize, AnchorDeserialize, PartialEq, Debug, Clone, Copy)]
pub struct Bet {
    pub user: Pubkey,
    pub amount: u64,
    pub result: u64,
    pub direction: Direction,

    pub refunded: bool,
}
