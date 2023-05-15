use anchor_lang::prelude::*;
// use modular_token::TokenAccountFront;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[derive(Accounts)]
pub struct InitializeMint<'info> {
    #[account(zero)]
    pub mint: Account<'info, Mint>,
}

#[derive(Accounts)]
pub struct InitializeTokenAccount<'info> {
    #[account(zero)]
    pub token_account: Account<'info, TokenAccount>,
}

#[program]
pub mod basic_token_handler {
    use super::*;

    pub fn initialize_mint(
        ctx: Context<InitializeMint>,
        mint_authority: Pubkey,
        decimals: u8,
    ) -> Result<()> {
        let mint = &mut ctx.accounts.mint;

        mint.mint_authority = mint_authority;
        mint.decimals = decimals;

        Ok(())
    }

    pub fn initialize_token_account(
        ctx: Context<InitializeTokenAccount>,
        authority: Pubkey,
        mint: u64,
    ) -> Result<()> {
        let token_account = &mut ctx.accounts.token_account;

        token_account.authority = authority;
        token_account.mint = mint;

        Ok(())
    }
}

#[account]
pub struct Mint {
    pub mint_authority: Pubkey,
    pub supply: u64,
    pub decimals: u8,
}

#[account]
pub struct TokenAccount {
    pub authority: Pubkey,
    pub balance: u64,
    pub mint: u64,
}
