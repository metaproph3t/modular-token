use anchor_lang::prelude::*;
use token::TokenHandler;

declare_id!("EDR4Ycrv7rsw3B4X9e5wLmxS7g7ULbFyoY9AeCJ5ZBSn");

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

#[derive(Accounts)]
pub struct MintTo<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
    // checks that it is owned by the token program
    #[account(signer)]
    pub signer: Account<'info, TokenHandler>, 
}

#[derive(Accounts)]
pub struct Transfer<'info> {
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
    #[account(signer)]
    pub signer: Account<'info, TokenHandler>,
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

    pub fn mint_to(
        ctx: Context<MintTo>,
        amount: u64,
    ) -> Result<()> {
        let mint = &mut ctx.accounts.mint;
        let to = &mut ctx.accounts.to;

        to.balance += amount;
        mint.supply += amount;

        Ok(())
    }

    pub fn transfer(
        ctx: Context<Transfer>,
        amount: u64,
    ) -> Result<()> {
        let from = &mut ctx.accounts.from;
        let to = &mut ctx.accounts.to;

        assert!(from.balance >= amount);
        assert!(from.mint == to.mint);

        from.balance -= amount;
        to.balance += amount;

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
    pub mint: u64,
    pub balance: u64,
}
