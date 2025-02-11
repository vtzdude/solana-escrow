use anchor_spl::{
    associated_token::AssociatedToken, token::{transfer_checked, TransferChecked}, token_interface::{Mint,TokenAccount,TokenInterface}
};
use anchor_lang::prelude::*;
use crate::state::Escrow;
#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Make<'info> {
    #[account(mut)]
     pub maker: Signer<'info>,
     pub mint_a:InterfaceAccount<'info, Mint>,
    pub mint_b:InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint_a,associated_token::authority = maker)]
    pub maker_mint_a_ata:InterfaceAccount<'info, TokenAccount>,

    #[account(init,
        payer=maker,
        space=Escrow::INIT_SPACE +8,
        seeds = [b"escrow", maker.key.as_ref(), seed.to_le_bytes().as_ref()],
         bump,
    )]
    pub escrow:Account<'info, Escrow>,
    #[account(
        init,
        payer=maker,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
)]
    pub vault:InterfaceAccount<'info, TokenAccount>,
    pub system_program:Program<'info, System>,

    pub token_program:Interface<'info, TokenInterface>,
    pub associated_token_program:Program<'info, AssociatedToken>,
}

impl<'info>Make<'info>{
    pub fn init_escrow(&mut self,seed:u64,receive_amount:u64,bumps:MakeBumps)->Result<()>{
        self.escrow.set_inner(Escrow{
            receive_amount,
            maker:self.maker.key(),
            seed,
            mint_a:self.mint_a.key(),
            mint_b:self.mint_b.key(),
            bump:bumps.escrow,

        });
        Ok(())
    }

    pub fn deposit(&mut self,amount:u64)->Result<()>{

        let cpi_program=self.token_program.to_account_info();
        let cpi_account=TransferChecked{
            from:self.maker.to_account_info(),
            mint:self.mint_a.to_account_info(),
            to:self.vault.to_account_info(),
            authority:self.maker.to_account_info()
        };
        let cpi_context=CpiContext::new(cpi_program,cpi_account);
        transfer_checked(cpi_context, amount, self.mint_a.decimals)?;
        Ok(())
    }
}