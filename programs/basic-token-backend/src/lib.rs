use anchor_lang::prelude::*;
// use modular_token::TokenAccountFront;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[derive(Accounts)]
pub struct InitializeMint<'info> {
    #[account(zero)]
    pub mint: Account<'info, Mint>,
}

#[program]
pub mod basic_token_backend {
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
}

#[account]
pub struct Mint {
    pub mint_authority: Pubkey,
    pub supply: u64,
    pub decimals: u8,
}
