use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
use crate::instructions::*;
declare_id!("9NBFTMbDFGEKxXXn2mvuKHrnBie69KumnxX6DRXKw1mA");

#[program]
pub mod vault {
   
    use super::*;

    pub fn make(ctx: Context<Make>,seed:u64,receive_amount:u64,deposit_amount:u64) -> Result<()> {
        ctx.accounts.init_escrow(seed, receive_amount, ctx.bumps)?;
        ctx.accounts.deposit(deposit_amount)?;
        Ok(())
    }

    pub fn take(ctx: Context<Take>,amount:u64) -> Result<()> {
        ctx.accounts.withdraw(amount)?;
        ctx.accounts.close()?;
        Ok(())
    }
    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.withdraw()?;
        ctx.accounts.close()?;
        Ok(())
    }
}


