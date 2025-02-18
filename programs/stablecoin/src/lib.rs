use anchor_lang::prelude::*;

declare_id!("CqpgSVK8AQemB5BqmcmWG8BrgkKt2o837naTFURnPbfX");

mod state;
use state::*;
mod constants;
use constants::*;
mod instructions;
use instructions::*;
mod error;



#[program]
pub mod stablecoin {
    use super::*;

    pub fn init_config(ctx: Context<InitConfig>) -> Result<()> {
        process_init_config(ctx)
    }

    pub fn update_config(ctx: Context<UpdateConfig>, min_health_factor: u64) -> Result<()> {
        process_update_config(ctx, min_health_factor)
    }

    pub fn deposit_collateral_and_mint_tokens(ctx: Context<DepositCollateralAndMintTokens>, collateral_amount: u64, amount_to_mint: u64) -> Result<()> {
        process_deposit_collateral_and_mint_tokens(ctx, collateral_amount, amount_to_mint)
    }

    pub fn redeem_collateral_and_burn_tokens(ctx: Context<RedeemCollateralAndBurnTokens>, amount_collateral: u64, amount_to_burn: u64) -> Result<()> {
        process_redeem_collateral_and_burn_tokens(ctx, amount_collateral, amount_to_burn)
    }

    pub fn liquidate(ctx: Context<Liquidate>, amount_to_burn: u64) -> Result<()> {
        process_liquidate(ctx, amount_to_burn)
    }
    

}

