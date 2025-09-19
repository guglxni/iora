use anchor_lang::prelude::*;

declare_id!("GVetpCppi9v1BoZYCHwzL18b6a35i3HbgFUifQLbt5Jz");

#[program]
pub mod iora_oracle {
    use super::*;

    /// Initialize the oracle account
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let oracle_data = &mut ctx.accounts.oracle_data;
        oracle_data.authority = *ctx.accounts.authority.key;
        oracle_data.bump = ctx.bumps.oracle_data;
        oracle_data.symbol = "INIT".to_string();
        oracle_data.price = 0.0;
        oracle_data.insight = "Oracle initialized".to_string();
        oracle_data.confidence = 0.0;
        oracle_data.recommendation = "HOLD".to_string();
        oracle_data.timestamp = Clock::get()?.unix_timestamp;

        Ok(())
    }

    /// Update oracle data with price and analysis
    pub fn update_data(
        ctx: Context<UpdateData>,
        symbol: String,
        price: f64,
        insight: String,
        confidence: f32,
        recommendation: String,
        timestamp: i64,
    ) -> Result<()> {
        let oracle_data = &mut ctx.accounts.oracle_data;

        // Validate input data
        require!(symbol.len() <= 20, OracleError::SymbolTooLong);
        require!(insight.len() <= 500, OracleError::InsightTooLong);
        require!(recommendation.len() <= 10, OracleError::RecommendationTooLong);
        require!(confidence >= 0.0 && confidence <= 1.0, OracleError::InvalidConfidence);
        require!(price > 0.0, OracleError::InvalidPrice);

        // Update the oracle data
        oracle_data.symbol = symbol;
        oracle_data.price = price;
        oracle_data.insight = insight;
        oracle_data.confidence = confidence;
        oracle_data.recommendation = recommendation;
        oracle_data.timestamp = timestamp;
        oracle_data.authority = *ctx.accounts.authority.key;
        oracle_data.bump = ctx.bumps.oracle_data;

        // Emit event for indexing
        emit!(DataUpdatedEvent {
            symbol: oracle_data.symbol.clone(),
            price: oracle_data.price,
            confidence: oracle_data.confidence,
            recommendation: oracle_data.recommendation.clone(),
            timestamp: oracle_data.timestamp,
        });

        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateData<'info> {
    #[account(
        mut,
        seeds = [b"oracle-data"],
        bump,
    )]
    pub oracle_data: Account<'info, OracleData>,

    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = OracleData::LEN,
        seeds = [b"oracle-data"],
        bump,
    )]
    pub oracle_data: Account<'info, OracleData>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct OracleData {
    pub symbol: String,      // Cryptocurrency symbol (max 20 chars)
    pub price: f64,          // Current price
    pub insight: String,     // Analysis insight (max 500 chars)
    pub confidence: f32,     // Confidence score (0.0-1.0)
    pub recommendation: String, // BUY/SELL/HOLD (max 10 chars)
    pub timestamp: i64,      // Unix timestamp
    pub authority: Pubkey,   // Authority that can update
    pub bump: u8,           // PDA bump
}

impl OracleData {
    pub const LEN: usize = 8 + // discriminator
        (4 + 20) + // symbol (String)
        8 + // price (f64)
        (4 + 500) + // insight (String)
        4 + // confidence (f32)
        (4 + 10) + // recommendation (String)
        8 + // timestamp (i64)
        32 + // authority (Pubkey)
        1; // bump (u8)
}

#[event]
pub struct DataUpdatedEvent {
    pub symbol: String,
    pub price: f64,
    pub confidence: f32,
    pub recommendation: String,
    pub timestamp: i64,
}

#[error_code]
pub enum OracleError {
    #[msg("Symbol is too long (max 20 characters)")]
    SymbolTooLong,
    #[msg("Insight is too long (max 500 characters)")]
    InsightTooLong,
    #[msg("Recommendation is too long (max 10 characters)")]
    RecommendationTooLong,
    #[msg("Confidence must be between 0.0 and 1.0")]
    InvalidConfidence,
    #[msg("Price must be positive")]
    InvalidPrice,
}
