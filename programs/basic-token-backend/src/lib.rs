use anchor_lang::prelude::*;
// use modular_token::TokenAccountFront;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[derive(Accounts)]
pub struct InitializeMint<'info> {
    #[account(init, payer = payer, space = 8 + 32 + 8 + 1)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(mint: u64)]
pub struct InitializeTokenAccount<'info> {
    pub token_account_back: Account<'info, TokenAccountBack>,
    // pub token_account_front: Account<'info, TokenAccountFront>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[program]
pub mod basic_token_backend {
    use super::*;

    pub fn initialize_token_account(_ctx: Context<InitializeTokenAccount>) -> Result<()> {
        Ok(())
    }

    pub fn initialize_mint(
        ctx: Context<InitializeMint>,
        mint_authority: Pubkey,
        decimals: u8,
        // _data: Vec<u8>,
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

#[account]
pub struct TokenAccountBack {
    pub balance: u64,
}
