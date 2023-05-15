//! A modular token program that targets the SVM.
//! 
//! This program gives many of the security benefits of a singleton
//! token program (e.g., SPL token as it stands) while allowing for
//! the permissionless deployment of custom token logic.
//! 
//! Anyone may deploy a token handler. These handlers implement the 
//! logic of a token program, and can have custom functionality such
//! as freezes, fees, and rebasing. Then, they may call `register_handler`
//! here, specifying how many bytes need to be allocated for token
//! and mint accounts.
//! 
//! At that point, a user can safely interact with that handler by
//! calling this program's functions. This program will perform some
//! extra security steps before passing along a modified version of
//! the user's call to the handler.
//! 
//! The chief purpose of this program is ensuring that a signed user
//! account never gets passed to a token handler. If that were to occur,
//! that token handler would be able to take arbitrary actions on
//! behalf of the user, which would allow them to steal all of the
//! user's assets. Signed user accounts are normally required for two
//! purposes: (1) paying for account allocation and (2) authorizing
//! an action, such as a transfer. To solve the first, this program
//! pre-allocates all token and mint accounts before giving ownership
//! to the respective handler. To solve the second, this program
//! authorizes users and indicates to handlers that an action (e.g.,
//! a transfer) is authorized by passing in a signed `TokenHandler`
//! account. The result of these two actions is that users and other
//! programs can be certain that their signed accounts aren't being
//! passed to untrusted code.

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
pub struct RegisterHandler<'info> {
    #[account(init, payer = payer, space = 8 + 32 + 4 + 4 + 1, seeds = [b"handler", handler_program.key().as_ref()], bump)]
    pub handler: Account<'info, TokenHandler>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    /// CHECK: not reading this, just need to be executable
    #[account(executable)]
    pub handler_program: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct InitializeMint<'info> {
    #[account(has_one = handler_program)]
    pub handler: Account<'info, TokenHandler>,
    /// CHECK: not reading this, just need to be executable
    #[account(executable)]
    pub handler_program: UncheckedAccount<'info>,
    /// CHECK: not reading this, just passing along to backend
    #[account(mut)]
    pub mint: UncheckedAccount<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeTokenAccount<'info> {
    #[account(has_one = handler_program)]
    pub handler: Account<'info, TokenHandler>,
    /// CHECK: not reading this, just need to be executable
    #[account(executable)]
    pub handler_program: UncheckedAccount<'info>,
    /// CHECK: not reading this, just passing along to backend
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MintTo<'info> {
    #[account(has_one = handler_program)]
    pub handler: Account<'info, TokenHandler>,
    /// CHECK: not reading it
    #[account(executable)]
    pub handler_program: UncheckedAccount<'info>,
    /// CHECK: passing along to backend_program
    #[account(mut)]
    pub to: UncheckedAccount<'info>,
    pub mint_authority: Signer<'info>,
    /// CHECK: we run checks in code
    #[account(mut)]
    pub mint: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct Transfer<'info> {
    #[account(has_one = handler_program)]
    pub handler: Account<'info, TokenHandler>,
    /// CHECK: not reading it
    #[account(executable)]
    pub handler_program: UncheckedAccount<'info>,
    /// CHECK: passing along to backend_program
    #[account(mut)]
    pub from: UncheckedAccount<'info>,
    /// CHECK: passing along to backend_program
    #[account(mut)]
    pub to: UncheckedAccount<'info>,
    pub authority: Signer<'info>,
}

#[program]
pub mod token {
    use solana_program::instruction::Instruction;

    use super::*;

    pub fn register_handler(
        ctx: Context<RegisterHandler>,
        token_account_bytes: u32,
        mint_account_bytes: u32,
    ) -> Result<()> {
        let handler = &mut ctx.accounts.handler;

        handler.handler_program = ctx.accounts.handler_program.key();
        handler.token_account_bytes = token_account_bytes;
        handler.mint_account_bytes = mint_account_bytes;
        handler.pda_bump = *ctx.bumps.get("handler").unwrap();

        Ok(())
    }

    pub fn initialize_mint(
        ctx: Context<InitializeMint>,
        nonce: u64,
        mint_data: Vec<u8>,
    ) -> Result<()> {
        let mint_account_bytes = ctx.accounts.handler.mint_account_bytes;

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

        // msg!("{:?}", ctx.accounts.mint.to_account_info());

        // create a PDA that will be owned by the backend program, like what
        // the associated token program does
        invoke_signed(
            &system_instruction::create_account(
                ctx.accounts.payer.key,
                ctx.accounts.mint.key,
                rent.minimum_balance(mint_account_bytes.try_into().unwrap())
                    .max(1),
                mint_account_bytes as u64,
                ctx.accounts.handler_program.key,
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

        let initialize_mint_accounts = vec![AccountMeta::new(mint_address, false)];

        let mint_ix = Instruction {
            program_id: ctx.accounts.handler_program.key(),
            accounts: initialize_mint_accounts,
            data: initialize_mint_ix_data,
        };

        invoke(
            &mint_ix,
            &[
                ctx.accounts.handler_program.to_account_info(),
                ctx.accounts.mint.to_account_info(),
            ],
        )?;

        Ok(())
    }

    // This function functions similarly to `initialize_mint`, albeit
    // with some minor adjustments.
    pub fn initialize_token_account(
        ctx: Context<InitializeTokenAccount>,
        authority: Pubkey,
        mint_nonce: u64,
        token_account_data: Vec<u8>,
    ) -> Result<()> {
        let token_account_bytes = ctx.accounts.handler.token_account_bytes;

        let (token_account_address, bump_seed) = Pubkey::find_program_address(
            &[
                "token".as_bytes(),
                &mint_nonce.to_le_bytes(),
                &authority.to_bytes(),
            ],
            ctx.program_id,
        );
        if ctx.accounts.token_account.key() != token_account_address {
            msg!("INVALID SEEDS");
            msg!("{:?}: {:?}", token_account_address, bump_seed);
        }

        if *ctx.accounts.token_account.owner != system_program::id() {
            msg!("INVALID OWNER");
        }

        let token_account_signer_seeds: &[&[_]] = &[
            "token".as_bytes(),
            &mint_nonce.to_le_bytes(),
            &authority.to_bytes(),
            &[bump_seed],
        ];

        let rent = Rent::get()?;

        // create a PDA that will be owned by the backend program, like what
        // the associated token program does
        invoke_signed(
            &system_instruction::create_account(
                ctx.accounts.payer.key,
                ctx.accounts.token_account.key,
                rent.minimum_balance(token_account_bytes.try_into().unwrap())
                    .max(1),
                token_account_bytes as u64,
                ctx.accounts.handler_program.key,
            ),
            &[
                ctx.accounts.payer.to_account_info(),
                ctx.accounts.token_account.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[token_account_signer_seeds],
        )?;

        // we're packing the instruction manually here, but in production
        // you would likely want to do something like this: https://github.com/solana-labs/solana-program-library/blob/68dbd449642b856ded8a674218ff9415b7e3091c/token/program/src/instruction.rs#L799
        let mut initialize_token_account_ix_data: Vec<u8> =
            vec![0x96, 0x55, 0x2c, 0x1c, 0x95, 0x0e, 0xd2, 0x1a];
        initialize_token_account_ix_data.extend_from_slice(&authority.to_bytes());
        initialize_token_account_ix_data.extend_from_slice(&mint_nonce.to_le_bytes());
        initialize_token_account_ix_data.append(&mut token_account_data.clone());

        let initialize_token_account_accounts =
            vec![AccountMeta::new(token_account_address, false)];

        let token_account_ix = Instruction {
            program_id: ctx.accounts.handler_program.key(),
            accounts: initialize_token_account_accounts,
            data: initialize_token_account_ix_data,
        };

        invoke(
            &token_account_ix,
            &[
                ctx.accounts.handler_program.to_account_info(),
                ctx.accounts.token_account.to_account_info(),
            ],
        )?;

        Ok(())
    }

    pub fn mint_to(
        ctx: Context<MintTo>,
        amount: u64
    ) -> Result<()> {
        let handler_program = ctx.accounts.handler_program.key();
        assert!(ctx.accounts.to.owner == &handler_program);
        assert!(ctx.accounts.mint.owner == &handler_program);

        let mint_nonce_bytes: [u8; 8] = ctx.accounts.to.data.borrow()[40..48].try_into().unwrap();

        let (mint_address, bump_seed) =
            Pubkey::find_program_address(&[b"mint", &mint_nonce_bytes], ctx.program_id);
        
        assert!(ctx.accounts.mint.key() == mint_address); 

        let mint_authority_bytes: [u8; 32] = ctx.accounts.mint.data.borrow()[8..40].try_into().unwrap();
        let mint_authority: Pubkey = mint_authority_bytes.into();

        assert!(ctx.accounts.mint_authority.key() == mint_authority);

        let handler_signer_seeds: &[&[_]] = &[
            b"handler",
            handler_program.as_ref(),
            &[ctx.accounts.handler.pda_bump],
        ];

        let mut mint_to_ix_data: Vec<u8> =
            vec![0xf1, 0x22, 0x30, 0xba, 0x25, 0xb3, 0x7b, 0xc0];
        mint_to_ix_data.extend_from_slice(&amount.to_le_bytes());

        let mint_to_accounts =
            vec![
                AccountMeta::new(ctx.accounts.mint.key(), false),
                AccountMeta::new(ctx.accounts.to.key(), false),
                AccountMeta::new_readonly(ctx.accounts.handler.key(), true),
            ];
        
        let mint_to_ix = Instruction {
            program_id: ctx.accounts.handler_program.key(),
            accounts: mint_to_accounts,
            data: mint_to_ix_data,
        };

        invoke_signed(
            &mint_to_ix,
            &[
                ctx.accounts.handler_program.to_account_info(),
                ctx.accounts.mint.to_account_info(),
                ctx.accounts.to.to_account_info(),
                ctx.accounts.handler.to_account_info(),
            ],
            &[handler_signer_seeds]
        )?;

        Ok(())
    }

    // pub fn transfer(
    //     ctx: Context<Transfer>,
    //     amount: u64,
    // ) -> Result<()> {
    //     // Perhaps these checks are unnecessary since a non-owning
    //     // program wouldn't be able to mutate the state anyway
    //     assert!(ctx.accounts.from.owner == ctx.accounts.handler_program.key);
    //     assert!(ctx.accounts.to.owner == ctx.accounts.handler_program.key);

    //     // Since we can't pass the signer to an untrusted program, we
    //     // need to check that from's authority has signed here and then
    //     // indicate this to the handler.
    //     // 
    //     // This is handled in the following way:
    //     // - token handlers must store the authority of a token account
    //     //   in bytes 8..39
    //     // - we make the check here, and if it looks good, we pass
    //     //   along a signed version of the `TokenHandler` account.
    //     // - handler can infer that the authority has signed from the
    //     //   presence of the signed `TokenHandler account.
    //     //
    //     // This is secure because the `TokenHandler` account shouldn't
    //     // have any resources to steal or grief.

    //     let from_authority_bytes: [u8; 32] = ctx.accounts.from.data.borrow()[8..40].try_into().unwrap();
    //     let from_authority: Pubkey = from_authority_bytes.into();

    //     assert!(from_authority == ctx.accounts.authority.key());

    //     let token_handler_signer_seeds: &[&[_]] = &[
    //         "token_handler".as_bytes(),
    //         &ctx.accounts.handler_program.key().to_bytes(),
    //         &[ctx.accounts.handler.pda_bump],
    //     ];

    //     let mut transfer_ix_data: Vec<u8> =
    //         vec![0x96, 0x55, 0x2c, 0x1c, 0x95, 0x0e, 0xd2, 0x1a];
    //     // initialize_token_account_ix_data.extend_from_slice(&authority.to_bytes());
    //     // initialize_token_account_ix_data.extend_from_slice(&mint_nonce.to_le_bytes());
    //     // initialize_token_account_ix_data.append(&mut token_account_data.clone());
    //     Ok(())
    // }
}

#[account]
pub struct TokenHandler {
    pub handler_program: Pubkey,
    pub token_account_bytes: u32,
    pub mint_account_bytes: u32,
    pub pda_bump: u8,
}
