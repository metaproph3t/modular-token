//! A sample modular token program that targets the Sealevel runtime.
//! 
//! The instructions are a modified subset of [SPL token](https://github.com/solana-labs/solana-program-library/blob/master/token/program/src/instruction.rs),
//! instructions, including the following:
//! 
//! * InitializeMint
//! * InitializeTokenAccount
//! * Transfer
//! * MintTo

use anchor_lang::prelude::*;

declare_id!("HmbTLCmaGvZhKnn1Zfa1JVnp7vkMV4DYVxPLWBVoN65L");

#[derive(Accounts)]
pub struct InitializeMint {}

#[derive(Accounts)]
pub struct InitializeTokenAccount {}

#[derive(Accounts)]
pub struct MintTo {}

#[derive(Accounts)]
pub struct Transfer {}

#[program]
pub mod modular_token {
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
