use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use anchor_spl::token;
use std::convert::TryFrom;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[derive(Debug)]
#[account]
pub struct Escrow {
    offered_pubkey: Pubkey,
    requested_pubkey: Pubkey,
    offered_token_mint_pubkey: Pubkey,
    requested_token_mint_pubkey: Pubkey,
    offered_token_account_pubkey: Pubkey,
    requested_token_account_pubkey: Pubkey,
}

pub fn init_escrow_handler(
    mut ctx: Context<InitEscrow>,
    mut requested_pubkey: Pubkey,
) -> Result<()> {
    let mut offerer_signer = &mut ctx.accounts.offerer_signer;
    let mut offered_token_mint = &mut ctx.accounts.offered_token_mint;
    let mut requested_token_mint = &mut ctx.accounts.requested_token_mint;
    let mut offered_holder_token_account = &mut ctx.accounts.offered_holder_token_account;
    let mut requested_holder_token_account = &mut ctx.accounts.requested_holder_token_account;
    let mut escrow = &mut ctx.accounts.escrow;
    let mut new_offered_token_account = &mut ctx.accounts.new_offered_token_account;
    let mut new_requested_token_account = &mut ctx.accounts.new_requested_token_account;
    let mut escrow = escrow;

    escrow.offered_pubkey = offerer_signer.key();

    escrow.requested_pubkey = requested_pubkey;

    let mut new_offered_token_account = new_offered_token_account;

    escrow.offered_token_mint_pubkey = offered_token_mint.key();

    escrow.requested_token_mint_pubkey = requested_token_mint.key();

    escrow.offered_token_account_pubkey = new_offered_token_account.key();

    escrow.requested_token_account_pubkey = new_requested_token_account.key();

    require!(
        offerer_signer.key() == offered_holder_token_account.owner,
        ProgramError::E000
    );

    require!(
        requested_pubkey == requested_holder_token_account.owner,
        ProgramError::E001
    );

    require!(
        offered_holder_token_account.amount == (1 as u64),
        ProgramError::E002
    );

    require!(
        requested_holder_token_account.amount == (1 as u64),
        ProgramError::E003
    );

    Ok(())
}

pub fn fund_offered_escrow_handler(mut ctx: Context<FundOfferedEscrow>) -> Result<()> {
    let mut offerer_signer = &mut ctx.accounts.offerer_signer;
    let mut escrow = &mut ctx.accounts.escrow;
    let mut offered_holder_token_account = &mut ctx.accounts.offered_holder_token_account;
    let mut new_offered_token_account = &mut ctx.accounts.new_offered_token_account;

    require!(
        escrow.offered_pubkey == offerer_signer.key(),
        ProgramError::E004
    );

    require!(
        escrow.offered_token_account_pubkey == new_offered_token_account.key(),
        ProgramError::E005
    );

    require!(
        new_offered_token_account.owner == escrow.key(),
        ProgramError::E006
    );

    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: offered_holder_token_account.to_account_info(),
                authority: offerer_signer.to_account_info(),
                to: new_offered_token_account.to_account_info(),
            },
        ),
        1 as u64,
    )?;

    Ok(())
}

pub fn defund_offered_escrow_handler(
    mut ctx: Context<DefundOfferedEscrow>,
    mut escrow_bump: u8,
) -> Result<()> {
    let mut offered_signer = &mut ctx.accounts.offered_signer;
    let mut escrow = &mut ctx.accounts.escrow;
    let mut offered_holder_token_account = &mut ctx.accounts.offered_holder_token_account;
    let mut requested_holder_token_account = &mut ctx.accounts.requested_holder_token_account;
    let mut new_offered_token_account = &mut ctx.accounts.new_offered_token_account;

    require!(
        offered_signer.key() == escrow.offered_pubkey,
        ProgramError::E004
    );

    require!(
        escrow.offered_token_account_pubkey == new_offered_token_account.key(),
        ProgramError::E005
    );

    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: new_offered_token_account.to_account_info(),
                authority: escrow.to_account_info(),
                to: offered_holder_token_account.to_account_info(),
            },
            &[&[
                "escrow".as_bytes().as_ref(),
                offered_holder_token_account.key().as_ref(),
                requested_holder_token_account.key().as_ref(),
                escrow_bump.to_le_bytes().as_ref(),
            ]],
        ),
        1 as u64,
    )?;

    Ok(())
}

pub fn fund_requested_escrow_handler(mut ctx: Context<FundRequestedEscrow>) -> Result<()> {
    let mut requested_signer = &mut ctx.accounts.requested_signer;
    let mut escrow = &mut ctx.accounts.escrow;
    let mut requested_holder_token_account = &mut ctx.accounts.requested_holder_token_account;
    let mut new_requested_token_account = &mut ctx.accounts.new_requested_token_account;

    require!(
        escrow.requested_pubkey == requested_signer.key(),
        ProgramError::E007
    );

    require!(
        escrow.requested_token_account_pubkey == new_requested_token_account.key(),
        ProgramError::E005
    );

    require!(
        new_requested_token_account.owner == escrow.key(),
        ProgramError::E008
    );

    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: requested_holder_token_account.to_account_info(),
                authority: requested_signer.to_account_info(),
                to: new_requested_token_account.to_account_info(),
            },
        ),
        1 as u64,
    )?;

    Ok(())
}

pub fn defund_requested_escrow_handler(
    mut ctx: Context<DefundRequestedEscrow>,
    mut escrow_bump: u8,
) -> Result<()> {
    let mut requested_signer = &mut ctx.accounts.requested_signer;
    let mut escrow = &mut ctx.accounts.escrow;
    let mut offered_holder_token_account = &mut ctx.accounts.offered_holder_token_account;
    let mut requested_holder_token_account = &mut ctx.accounts.requested_holder_token_account;
    let mut new_requested_token_account = &mut ctx.accounts.new_requested_token_account;

    "\n    - Tests to write\n    1. The requsted signer is the same as the requested pubkey on the escrow contract\n    2. The given requested token account pubkey matched the requested token escrow account pubkey \n    3. The given offered_holder_token_account authority matches the escrow's offered pubkey\n    4. The given requested_holder_token_account authiority matches the escrow's requested pubkey\n    " ;

    require!(
        escrow.requested_pubkey == requested_signer.key(),
        ProgramError::E007
    );

    require!(
        escrow.requested_token_account_pubkey == requested_holder_token_account.key(),
        ProgramError::E005
    );

    require!(
        escrow.offered_token_account_pubkey == offered_holder_token_account.key(),
        ProgramError::E005
    );

    require!(
        requested_holder_token_account.owner == escrow.requested_pubkey,
        ProgramError::E009
    );

    require!(
        offered_holder_token_account.owner == escrow.offered_pubkey,
        ProgramError::E010
    );

    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: new_requested_token_account.to_account_info(),
                authority: escrow.to_account_info(),
                to: requested_holder_token_account.to_account_info(),
            },
            &[&[
                "escrow".as_bytes().as_ref(),
                offered_holder_token_account.key().as_ref(),
                requested_holder_token_account.key().as_ref(),
                escrow_bump.to_le_bytes().as_ref(),
            ]],
        ),
        1 as u64,
    )?;

    Ok(())
}

pub fn crank_swap_handler(mut ctx: Context<CrankSwap>, mut escrow_bump: u8) -> Result<()> {
    let mut escrow = &mut ctx.accounts.escrow;
    let mut offered_holder_token_account = &mut ctx.accounts.offered_holder_token_account;
    let mut requested_holder_token_account = &mut ctx.accounts.requested_holder_token_account;
    let mut new_offered_token_account = &mut ctx.accounts.new_offered_token_account;
    let mut new_requested_token_account = &mut ctx.accounts.new_requested_token_account;
    let mut final_offered_token_account = &mut ctx.accounts.final_offered_token_account;
    let mut final_requested_token_account = &mut ctx.accounts.final_requested_token_account;

    require!(
        offered_holder_token_account.owner == final_requested_token_account.owner,
        ProgramError::E011
    );

    require!(
        requested_holder_token_account.owner == final_offered_token_account.owner,
        ProgramError::E011
    );

    require!(
        final_offered_token_account.owner == escrow.requested_pubkey,
        ProgramError::E012
    );

    require!(
        final_requested_token_account.owner == escrow.offered_pubkey,
        ProgramError::E013
    );

    require!(
        new_offered_token_account.amount == (1 as u64),
        ProgramError::E014
    );

    require!(
        new_requested_token_account.amount == (1 as u64),
        ProgramError::E015
    );

    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: new_requested_token_account.to_account_info(),
                authority: escrow.to_account_info(),
                to: final_requested_token_account.to_account_info(),
            },
            &[&[
                "escrow".as_bytes().as_ref(),
                offered_holder_token_account.key().as_ref(),
                requested_holder_token_account.key().as_ref(),
                escrow_bump.to_le_bytes().as_ref(),
            ]],
        ),
        1 as u64,
    )?;

    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: new_offered_token_account.to_account_info(),
                authority: escrow.to_account_info(),
                to: final_offered_token_account.to_account_info(),
            },
            &[&[
                "escrow".as_bytes().as_ref(),
                offered_holder_token_account.key().as_ref(),
                requested_holder_token_account.key().as_ref(),
                escrow_bump.to_le_bytes().as_ref(),
            ]],
        ),
        1 as u64,
    )?;

    Ok(())
}

#[derive(Accounts)]
pub struct InitEscrow<'info> {
    #[account(mut)]
    pub offerer_signer: Signer<'info>,
    #[account(mut)]
    pub offered_token_mint: Box<Account<'info, token::Mint>>,
    #[account(mut)]
    pub requested_token_mint: Box<Account<'info, token::Mint>>,
    #[account(mut)]
    pub offered_holder_token_account: Box<Account<'info, token::TokenAccount>>,
    #[account(mut)]
    pub requested_holder_token_account: Box<Account<'info, token::TokenAccount>>,
    #[account(
        init,
        payer = offerer_signer,
        seeds = [
            "escrow".as_bytes().as_ref(),
            offered_holder_token_account.key().as_ref(),
            requested_holder_token_account.key().as_ref()
        ],
        bump,
        space = 8 + std::mem::size_of::<Escrow>()
    )]
    pub escrow: Box<Account<'info, Escrow>>,
    #[account(
        init,
        payer = offerer_signer,
        seeds = [
            "escrow-offered-token-account".as_bytes().as_ref(),
            offered_holder_token_account.key().as_ref()
        ],
        bump,
        token::mint = offered_token_mint,
        token::authority = escrow
    )]
    pub new_offered_token_account: Box<Account<'info, token::TokenAccount>>,
    #[account(
        init,
        payer = offerer_signer,
        seeds = [
            "escrow-requested-token-account".as_bytes().as_ref(),
            requested_holder_token_account.key().as_ref()
        ],
        bump,
        token::mint = requested_token_mint,
        token::authority = escrow
    )]
    pub new_requested_token_account: Box<Account<'info, token::TokenAccount>>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct FundOfferedEscrow<'info> {
    #[account(mut)]
    pub offerer_signer: Signer<'info>,
    #[account(mut)]
    pub escrow: Box<Account<'info, Escrow>>,
    #[account(mut)]
    pub offered_holder_token_account: Box<Account<'info, token::TokenAccount>>,
    #[account(mut)]
    pub new_offered_token_account: Box<Account<'info, token::TokenAccount>>,
    pub token_program: Program<'info, token::Token>,
}

#[derive(Accounts)]
pub struct DefundOfferedEscrow<'info> {
    #[account(mut)]
    pub offered_signer: Signer<'info>,
    #[account(mut)]
    pub escrow: Box<Account<'info, Escrow>>,
    #[account(mut)]
    pub offered_holder_token_account: Box<Account<'info, token::TokenAccount>>,
    #[account(mut)]
    pub requested_holder_token_account: Box<Account<'info, token::TokenAccount>>,
    #[account(mut)]
    pub new_offered_token_account: Box<Account<'info, token::TokenAccount>>,
    pub token_program: Program<'info, token::Token>,
}

#[derive(Accounts)]
pub struct FundRequestedEscrow<'info> {
    #[account(mut)]
    pub requested_signer: Signer<'info>,
    #[account(mut)]
    pub escrow: Box<Account<'info, Escrow>>,
    #[account(mut)]
    pub requested_holder_token_account: Box<Account<'info, token::TokenAccount>>,
    #[account(mut)]
    pub new_requested_token_account: Box<Account<'info, token::TokenAccount>>,
    pub token_program: Program<'info, token::Token>,
}

#[derive(Accounts)]
pub struct DefundRequestedEscrow<'info> {
    #[account(mut)]
    pub requested_signer: Signer<'info>,
    #[account(mut)]
    pub escrow: Box<Account<'info, Escrow>>,
    #[account(mut)]
    pub offered_holder_token_account: Box<Account<'info, token::TokenAccount>>,
    #[account(mut)]
    pub requested_holder_token_account: Box<Account<'info, token::TokenAccount>>,
    #[account(mut)]
    pub new_requested_token_account: Box<Account<'info, token::TokenAccount>>,
    pub token_program: Program<'info, token::Token>,
}

#[derive(Accounts)]
pub struct CrankSwap<'info> {
    #[account(mut)]
    pub escrow: Box<Account<'info, Escrow>>,
    #[account(mut)]
    pub offered_holder_token_account: Box<Account<'info, token::TokenAccount>>,
    #[account(mut)]
    pub requested_holder_token_account: Box<Account<'info, token::TokenAccount>>,
    #[account(mut)]
    pub new_offered_token_account: Box<Account<'info, token::TokenAccount>>,
    #[account(mut)]
    pub new_requested_token_account: Box<Account<'info, token::TokenAccount>>,
    #[account(mut)]
    pub final_offered_token_account: Box<Account<'info, token::TokenAccount>>,
    #[account(mut)]
    pub final_requested_token_account: Box<Account<'info, token::TokenAccount>>,
    pub token_program: Program<'info, token::Token>,
}

#[program]
pub mod seahorseswap {
    use super::*;

    pub fn init_escrow(ctx: Context<InitEscrow>, requested_pubkey: Pubkey) -> Result<()> {
        init_escrow_handler(ctx, requested_pubkey)
    }

    pub fn fund_offered_escrow(ctx: Context<FundOfferedEscrow>) -> Result<()> {
        fund_offered_escrow_handler(ctx)
    }

    pub fn defund_offered_escrow(ctx: Context<DefundOfferedEscrow>, escrow_bump: u8) -> Result<()> {
        defund_offered_escrow_handler(ctx, escrow_bump)
    }

    pub fn fund_requested_escrow(ctx: Context<FundRequestedEscrow>) -> Result<()> {
        fund_requested_escrow_handler(ctx)
    }

    pub fn defund_requested_escrow(
        ctx: Context<DefundRequestedEscrow>,
        escrow_bump: u8,
    ) -> Result<()> {
        defund_requested_escrow_handler(ctx, escrow_bump)
    }

    pub fn crank_swap(ctx: Context<CrankSwap>, escrow_bump: u8) -> Result<()> {
        crank_swap_handler(ctx, escrow_bump)
    }
}

#[error_code]
pub enum ProgramError {
    #[msg("mismatch in token auth + signers")]
    E000,
    #[msg("mismatch in token auth + requested pubkey")]
    E001,
    #[msg("the supply must equal 1 for the offered token")]
    E002,
    #[msg("the supply must equal 1 for the requested token")]
    E003,
    #[msg("This swap escrow was not iniated by you.")]
    E004,
    #[msg("The escrow account does not match the given account.")]
    E005,
    #[msg("the given new_offered_token_account is now owned by the escrow")]
    E006,
    #[msg("This swap escrow was not requested to you.")]
    E007,
    #[msg("The given new_requested_token_account is not owned by the escrow")]
    E008,
    #[msg("The escrow requested pubkey does not match the authority for the given token account")]
    E009,
    #[msg("The escrow offered pubkey does not match the authority for the given token account")]
    E010,
    #[msg("there is a mismatch in where the token should go")]
    E011,
    #[msg("the destination token account is now owned by the requested authority")]
    E012,
    #[msg("the destination token account is now owned by the offering authority")]
    E013,
    #[msg("the escrow account does not have the offered token")]
    E014,
    #[msg("the escrow account does not have the requested token")]
    E015,
}
