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
use anchor_lang::solana_program;
use anchor_lang::solana_program::instruction::{Instruction, AccountMeta};

declare_id!("HmbTLCmaGvZhKnn1Zfa1JVnp7vkMV4DYVxPLWBVoN65L");

#[derive(Accounts)]
pub struct InitializeMint<'info> {
    /// CHECK: just calling, not reading
    #[account(executable)]
    pub token_backend: UncheckedAccount<'info>
}


// #[derive(Accounts)]
// pub struct MintTo {}

// #[derive(Accounts)]
// pub struct Transfer {}

#[derive(Accounts)]
#[instruction(mint: u64, authority: Pubkey)]
pub struct InitializeTokenAccount<'info> {
    /// CHECK: not needed
    #[account(executable)]
    pub backend_program: UncheckedAccount<'info>,
    #[account(
        init, 
        payer = payer, 
        space = 8 + 32 + 32 + 8,
        seeds = [b"token_account_front".as_ref(), &mint.to_le_bytes(), authority.key().as_ref()],
        bump
    )]
    pub token_account_front: Account<'info, TokenAccountFront>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[program]
pub mod modular_token {
    use super::*;

    pub fn initialize_token_account(
        ctx: Context<InitializeTokenAccount>,
        mint: u64,
        authority: Pubkey,
    ) -> Result<()> {
        let token_account_front = &mut ctx.accounts.token_account_front;

        token_account_front.authority = authority;
        token_account_front.backend_program = ctx.accounts.backend_program.key();
        token_account_front.mint = mint;

        // let accounts = vec![
        //     AccountMeta {
        //         pubkey: ctx.accounts.
        //     }
        // ];

        // let ix = Instruction { 
        //     program_id: ctx.accounts.token_backend.key(),
        //     accounts: (), 
        //     data: (),
        // };

        // let mut ix: Instruction = (*ctx.accounts.transaction).deref().into();
        // ix.accounts = ix
        //     .accounts
        //     .iter()
        //     .map(|acc| {
        //         let mut acc = acc.clone();
        //         if &acc.pubkey == ctx.accounts.multisig_signer.key {
        //             acc.is_signer = true;
        //         }
        //         acc
        //     })
        //     .collect();
        // let multisig_key = ctx.accounts.multisig.key();
        // let seeds = &[multisig_key.as_ref(), &[ctx.accounts.multisig.nonce]];
        // let signer = &[&seeds[..]];
        // let accounts = ctx.remaining_accounts;
        // solana_program::program::invoke_signed(&ix, accounts, signer)?;

        // solana_program::program::invoke_signed(instruction, account_infos, signers_seeds)

        Ok(())
    }

    pub fn initialize_mint(
        ctx: Context<InitializeMint>,
        data: Vec<u8>,
    ) -> Result<()> {
        let mut accounts = Vec::new();

        let account_infos = ctx.remaining_accounts;

        for account_info in account_infos {
            if account_info.key() != ctx.accounts.token_backend.key() {
                accounts.push(AccountMeta { pubkey: account_info.key(), is_signer: account_info.is_signer, is_writable: account_info.is_writable });
            }
        }

        let instruction = Instruction {
            program_id: ctx.accounts.token_backend.key(),
            accounts,
            data,
        };

        // solana_program::program::invoke(&instruction, account_infos);

        Ok(())
    }

    // pub fn mint_to(ctx: Context<MintTo>) -> Result<()> {
    //     Ok(())
    // }

    // pub fn transfer(ctx: Context<Transfer>) -> Result<()> {
    //     Ok(())
    // }
}

#[account]
pub struct TokenAccountFront {
    /// Account that owns this token account.
    pub authority: Pubkey,
    /// The backend where this token account is registered.
    pub backend_program: Pubkey,
    /// ID that identifies this token.
    pub mint: u64,
}
