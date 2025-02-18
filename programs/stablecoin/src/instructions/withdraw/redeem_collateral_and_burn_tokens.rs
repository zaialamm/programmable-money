use anchor_lang::prelude::*;
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

use anchor_spl::token_interface::{Mint, Token2022, TokenAccount};

use crate::
{
    state::*, check_health_factor, burn_tokens, 
    withdraw_sol, constants::{SEED_CONFIG_ACCOUNT, SEED_COLLATERAL_ACCOUNT}
};


#[derive(Accounts)]
pub struct RedeemCollateralAndBurnTokens<'info> {
    #[account(mut)]
    pub depositor: Signer<'info>,

    pub price_update: Account<'info, PriceUpdateV2>,

    #[account(
        seeds = [SEED_CONFIG_ACCOUNT],
        bump = config_account.bump,
        has_one = mint_account,
    )]

    pub config_account: Account<'info, Config>,

    #[account(
        mut,
        seeds = [SEED_COLLATERAL_ACCOUNT, depositor.key().as_ref()],
        bump = collateral_account.bump,
        has_one = token_account,
        has_one = sol_account,
    )]

    pub collateral_account: Account<'info, Collateral>,

    #[account(mut)]
    pub sol_account: SystemAccount<'info>,

    #[account(mut)]
    pub mint_account: InterfaceAccount<'info, Mint>,

    #[account(mut)]
    pub token_account: InterfaceAccount<'info, TokenAccount>, 

    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>
}
 

pub fn process_redeem_collateral_and_burn_tokens(
    ctx: Context<RedeemCollateralAndBurnTokens>,
    amount_collateral: u64,
    amount_to_burn: u64
) -> Result<()> {

    let collateral_account = &mut ctx.accounts.collateral_account;

    collateral_account.lamport_balance = ctx.accounts.sol_account.lamports() - amount_collateral;

    collateral_account.amount_minted -= amount_to_burn;

    check_health_factor(
        &ctx.accounts.collateral_account,
        &ctx.accounts.config_account,
        &ctx.accounts.price_update
    )?;

    burn_tokens(
        &ctx.accounts.token_program,
        &ctx.accounts.mint_account,
        &ctx.accounts.token_account,
        &ctx.accounts.depositor,
        amount_to_burn
    )?;

    withdraw_sol(
        ctx.accounts.collateral_account.bump_sol_account,
        &ctx.accounts.depositor.key(),
        &ctx.accounts.system_program,
        &ctx.accounts.sol_account,
        &ctx.accounts.depositor.to_account_info(),
        amount_collateral
    )?;


    Ok(())

}