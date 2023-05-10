use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[derive(Accounts)]
pub struct InitializeMint {}

#[derive(Accounts)]
pub struct InitializeTokenAccount {}

#[derive(Accounts)]
pub struct MintTo {}

#[derive(Accounts)]
pub struct Transfer {}

#[program]
pub mod basic_token_backend {
    use super::*;

    pub fn initialize_mint(ctx: Context<InitializeMint>) -> Result<()> {
        Ok(())
    }

    pub fn initialize_token_account(ctx: Context<InitializeTokenAccount>) -> Result<()> {
        Ok(())
    }

    pub fn mint_to(ctx: Context<MintTo>) -> Result<()> {
        Ok(())
    }

    pub fn transfer(ctx: Context<InitializeMint>) -> Result<()> {
        Ok(())
    }
}
