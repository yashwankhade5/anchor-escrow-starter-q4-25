#![allow(unused_imports)]

use anchor_lang::{prelude::*, system_program::Transfer};

use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        close_account, transfer_checked, CloseAccount, Mint, TokenAccount, TokenInterface,
        TransferChecked,
    },
};

use crate::Escrow;

#[derive(Accounts)]
pub struct Take<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    pub maker: SystemAccount<'info>,
    #[account(
        mint::token_program = token_program,
        constraint=mint_a.key() == escrow.mint_a
    )]
    pub mint_a: InterfaceAccount<'info, Mint>,
    #[account(
        mint::token_program = token_program
    )]
    pub mint_b: InterfaceAccount<'info, Mint>,
    #[account(
        init_if_needed,
        payer=taker,
        associated_token::mint = mint_a,
        associated_token::authority = taker,
        associated_token::token_program = token_program
    )]
    pub taker_ata_a: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = taker,
        associated_token::token_program = token_program
    )]
    pub taker_ata_b: InterfaceAccount<'info,TokenAccount>,
     #[account(
        init_if_needed,
        payer=taker,
        associated_token::mint = mint_b,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    pub maker_ata_b: Box<InterfaceAccount<'info, TokenAccount>>,
       #[account(
        mut,
        close = taker,
        seeds = [b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump
    )]
    pub escrow: Account<'info, Escrow>,
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program
    )]
    pub escrow_vault: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Take<'info> {
    //  TODO: Implement Take Instruction
    //  Includes Deposit, Withdraw and Close Vault
    pub fn vault_transfer(&mut self)-> Result<()>{
        let transferacc= TransferChecked{
        from:self.escrow_vault.to_account_info(),
        mint:self.mint_a.to_account_info(),
        to:self.taker_ata_a.to_account_info(),
        authority:self.escrow.to_account_info()
        };
        let seeds = &self.escrow.seed.to_le_bytes();
        let signer_seeds&{&[&[u8]]} =&[&[b"escrow",self.escrow.maker.as_ref(),
        seeds.as_ref(),&[self.escrow.bump]]];
          let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            transferacc,
            signer_seeds,
        );
        transfer_checked(cpi_ctx, self.escrow_vault.amount, self.mint_a.decimals)?;
        Ok(())
    }
}
