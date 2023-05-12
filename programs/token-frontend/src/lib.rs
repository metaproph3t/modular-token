use anchor_lang::prelude::*;
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    program::{get_return_data, invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction, system_program,
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
    use solana_program::instruction::Instruction;

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

    pub fn initialize_mint(ctx: Context<InitializeMint>, nonce: u64, mint_data: Vec<u8>) -> Result<()> {
        let mint_account_bytes = ctx.accounts.backend.mint_account_needed_space;

        let nonce = nonce.to_le_bytes();

        let (mint_address, bump_seed) =
            Pubkey::find_program_address(&["mint".as_bytes(), &nonce], ctx.program_id);
        if ctx.accounts.mint.key() != mint_address {
            msg!("INVALID SEEDS");
            msg!("{:?}: {:?}", mint_address, bump_seed);
            msg!("{:?}", nonce);
        }

        if *ctx.accounts.mint.owner != system_program::id() {
            msg!("INVALID OWNER");
        }

        let mint_signer_seeds: &[&[_]] = &["mint".as_bytes(), &nonce, &[bump_seed]];

        let rent = Rent::get()?;

        // create a PDA that will be owned by the backend program, like what
        // the associated token program does
        invoke_signed(
            &system_instruction::create_account(
                ctx.accounts.payer.key,
                ctx.accounts.mint.key,
                rent.minimum_balance(mint_account_bytes.try_into().unwrap())
                    .max(1),
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

        // we're packing the instruction manually here, but in production
        // you would likely want to do something like this: https://github.com/solana-labs/solana-program-library/blob/68dbd449642b856ded8a674218ff9415b7e3091c/token/program/src/instruction.rs#L799
        let mut initialize_mint_ix_data: Vec<u8> =
            vec![0xd1, 0x2a, 0xc3, 0x04, 0x81, 0x55, 0xd1, 0x2c];
        initialize_mint_ix_data.append(&mut mint_data.clone());

        let initialize_mint_accounts = vec![
            AccountMeta::new(mint_address, false)
        ];

        let mint_ix = Instruction {
            program_id: ctx.accounts.backend_program.key(),
            accounts: initialize_mint_accounts,
            data: initialize_mint_ix_data,
        };

        invoke(
            &mint_ix, 
            &[
                ctx.accounts.backend_program.to_account_info(),
                ctx.accounts.mint.to_account_info(),
            ]
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
