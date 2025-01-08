use anchor_lang::prelude::*;
use std::mem::size_of;
use anchor_spl::{
    token::{
        Mint, Token, TokenAccount, Transfer, transfer
    },
    associated_token::AssociatedToken
};

declare_id!("GCXWjriX8w99cwbkyEYVU67aeG2ESjZcshDizCqHMuwv");

#[program]
pub mod spl_transfer_poc {

    use super::*;

    pub fn init_transfer(ctx: Context<TransferSpl>) -> Result<()> { 
        msg!("The pda {}", ctx.accounts.transfer_pda.key());
        msg!("The token ata is {}", ctx.accounts.token_ata.key());
        Ok(())
    }

    pub fn withdraw(ctx: Context<TransferSpl>, amount: u64) -> Result<()> {
        let (pda, _bump) = Pubkey::find_program_address(
            &[b"transfer".as_ref()],
            ctx.program_id
        );

        msg!("The pda of the account after deriving is {}", pda);

        let bump_seed = ctx.bumps.transfer_pda;
        let signer_seeds : &[&[&[u8]]] = &[&[b"transfer".as_ref(), &[bump_seed]]];

        let cpi_context = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(), 
            Transfer {
                from: ctx.accounts.token_ata.to_account_info(),
                to: ctx.accounts.to_ata.to_account_info(),
                authority: ctx.accounts.transfer_pda.to_account_info(),
            }, 
            signer_seeds
        );

        transfer(cpi_context, amount)?;

        Ok(())
    }


}

#[account]
pub struct CustomPda {
    x: u64,
    y: u64
}

#[derive(Accounts)]
pub struct TransferSpl<'info> {
    #[account(
        init_if_needed,
        payer = signer,
        space = size_of::<CustomPda>() + 8,
        seeds = [b"transfer".as_ref()],
        bump
    )]
    pub transfer_pda: Account<'info, CustomPda>,

    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = transfer_pda
    )]
    pub token_ata: Account<'info, TokenAccount>,

    pub to_owner: SystemAccount<'info>,
    #[account(
        mut,
        token::mint = mint,
        token::authority = to_owner
    )]
    pub to_ata: Account<'info, TokenAccount>,

    pub mint: Account<'info, Mint>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>

}


