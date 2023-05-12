use anchor_lang::prelude::*;
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    program::{get_return_data, invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    system_program,
};

declare_id!("HmbTLCmaGvZhKnn1Zfa1JVnp7vkMV4DYVxPLWBVoN65L");

#[derive(Accounts)]
pub struct RegisterBackend<'info> {
    #[account(init, payer = payer, space = 8 + 32 + 4 + 4)]
    pub backend: Account<'info, Backend>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    /// CHECK: not reading this, just need to be executable
    #[account(executable)]
    pub backend_program: UncheckedAccount<'info>,
}

#[derive(Accounts)]
#[instruction(nonce: u64)]
pub struct InitializeMint<'info> {
    #[account(has_one = backend_program)]
    pub backend: Account<'info, Backend>,
    /// CHECK: not reading this, just need to be executable
    #[account(executable)]
    pub backend_program: UncheckedAccount<'info>,
    /// CHECK: not reading this, just passing along to backend
    #[account(mut)]
    pub mint: UncheckedAccount<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[program]
pub mod token_frontend {
    use super::*;

    pub fn register_backend(
        ctx: Context<RegisterBackend>,
        token_account_needed_space: u32,
        mint_account_needed_space: u32,
    ) -> Result<()> {
        let backend = &mut ctx.accounts.backend;

        backend.backend_program = ctx.accounts.backend_program.key();
        backend.token_account_needed_space = token_account_needed_space;
        backend.mint_account_needed_space = mint_account_needed_space;

        Ok(())
    }

    pub fn initialize_mint(
        ctx: Context<InitializeMint>,
        nonce: u64,
    ) -> Result<()> {
        let mint_account_bytes = ctx.accounts.backend.mint_account_needed_space;

        let (mint_address, bump_seed) = Pubkey::find_program_address(
            &[
                "mint".as_bytes()
            ],
            ctx.program_id,
        );
        if ctx.accounts.mint.key() != mint_address {
            msg!("INVALID SEEDS");
        }

        if *ctx.accounts.mint.owner != system_program::id() {
            msg!("INVALID OWNER");
        }

        let nonce = nonce.to_le_bytes();
        let mint_signer_seeds: &[&[_]] = &[
            "mint".as_ref(),
            &[bump_seed]
        ];

        let rent = Rent::get()?;

        invoke_signed(
            &system_instruction::create_account(
                ctx.accounts.payer.key,
                ctx.accounts.mint.key,
                rent.minimum_balance(mint_account_bytes.try_into().unwrap()).max(1),
                mint_account_bytes as u64,
                ctx.accounts.backend_program.key,
            ),
            &[
                ctx.accounts.payer.to_account_info(),
                ctx.accounts.mint.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[mint_signer_seeds],
        )?;

        Ok(())
    }
} 

#[account]
pub struct Backend {
    pub backend_program: Pubkey,
    pub token_account_needed_space: u32,
    pub mint_account_needed_space: u32,
}
