use anchor_spl::{
    associated_token::AssociatedToken, token::{ transfer_checked, TransferChecked}, token_2022::CloseAccount, token_interface::{Mint,TokenAccount,TokenInterface,close_account}
};
use anchor_lang::prelude::*;
use crate::state::{ Escrow};
#[derive(Accounts)]
pub struct Take<'info> {
    #[account(mut)]
     pub taker: Signer<'info>,

     #[account()]
     pub maker:SystemAccount<'info>,
     pub mint_a:InterfaceAccount<'info, Mint>,
    pub mint_b:InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = taker
    )]
    pub taker_mint_b_ata:InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer=taker,
        associated_token::mint = mint_a,associated_token::authority = taker)]
    pub taker_mint_a_ata:InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer=taker,
        associated_token::mint = mint_b,
        associated_token::authority = maker
    )]
    pub maker_mint_b_ata:InterfaceAccount<'info, TokenAccount>,

    #[account(mut,
        close=taker,
        has_one=mint_a,
        has_one=mint_b,
        has_one=maker,
        seeds = [b"escrow", escrow.maker.to_bytes().as_ref(), escrow.seed.to_le_bytes().as_ref()],
         bump=escrow.bump,
    )]
    pub escrow:Account<'info, Escrow>,
    #[account(
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
)]
    pub vault:InterfaceAccount<'info, TokenAccount>,
    pub system_program:Program<'info, System>,

    pub token_program:Interface<'info, TokenInterface>,
    pub associated_token_program:Program<'info, AssociatedToken>,
}

impl<'info>Take<'info>{
    pub fn withdraw(&mut self,amount:u64)->Result<()>{

        let cpi_program=self.token_program.to_account_info();
        let cpi_accounts=TransferChecked{
            from:self.taker_mint_b_ata.to_account_info(),
            mint:self.mint_b.to_account_info(),
            to:self.maker_mint_b_ata.to_account_info(),
            authority:self.taker.to_account_info()
        };
        let cpi_context=CpiContext::new(cpi_program.clone(),cpi_accounts);
        transfer_checked(cpi_context, self.escrow.receive_amount, self.mint_b.decimals)?;
        //Transfer token A from vault to taker
        let cpi_accounts=TransferChecked{
            from:self.vault.to_account_info(),
            mint:self.mint_a.to_account_info(),
            to:self.taker_mint_a_ata.to_account_info(),
            authority:self.escrow.to_account_info()
        };
        let seed_binding=self.escrow.seed.to_le_bytes();
        let maker_binding=self.escrow.maker.to_bytes();
        let bump=self.escrow.bump;
        let seed: [&[u8]; 4 ]=[
            b"escrow",
            &seed_binding,
            &maker_binding,
            &[bump]
            ];

            let signer_seeds: &[&[&[u8]]]=&[&seed];

        let cpi_context=CpiContext::new_with_signer(cpi_program,cpi_accounts,&signer_seeds);
        transfer_checked(cpi_context, self.vault.amount, self.mint_a.decimals)?;
        Ok(())
    }

    pub fn close(&mut self)->Result<()>{
        let cpi_program=self.token_program.to_account_info();

        let cpi_accounts=CloseAccount{
            account:self.vault.to_account_info(),
            destination:self.taker.to_account_info(),
            authority:self.escrow.to_account_info()
        };
        let seed_binding=self.escrow.seed.to_le_bytes();
        let maker_binding=self.escrow.maker.to_bytes();
        let bump=self.escrow.bump;
        let seed: [&[u8]; 4 ]=[
            b"escrow",
            &seed_binding,
            &maker_binding,
            &[bump]
            ];

            let signer_seeds: &[&[&[u8]]]=&[&seed];

        let cpi_context=CpiContext::new_with_signer(cpi_program,cpi_accounts,&signer_seeds);
        close_account(cpi_context)?;
        Ok(())
    }
}