use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use anchor_spl::associated_token;
use anchor_spl::token;
use std::convert::TryFrom;

declare_id!("DyTz2YqoanAHMhDqUM9UwfzakXHoGdueLogNnbLTkceL");

#[derive(Debug)]
#[account]
pub struct Counter {
    owner: Pubkey,
    count: i64,
}

pub fn init_counter_handler(mut ctx: Context<InitCounter>) -> Result<()> {
    let mut owner = &mut ctx.accounts.owner;
    let mut counter = &mut ctx.accounts.counter;
    let mut counter = counter;

    counter.owner = owner.key();

    Ok(())
}

pub fn add_handler(mut ctx: Context<Add>) -> Result<()> {
    let mut owner = &mut ctx.accounts.owner;
    let mut counter = &mut ctx.accounts.counter;

    require!(owner.key() == counter.owner, ProgramError::E000);

    counter.count += 1;

    Ok(())
}

#[derive(Accounts)]
pub struct InitCounter<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        seeds = ["Counter".as_bytes().as_ref(), owner.key().as_ref()],
        bump,
        space = 8 + std::mem::size_of::<Counter>()
    )]
    pub counter: Box<Account<'info, Counter>>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Add<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub counter: Box<Account<'info, Counter>>,
}

#[program]
pub mod counter {
    use super::*;

    pub fn init_counter(ctx: Context<InitCounter>) -> Result<()> {
        init_counter_handler(ctx)
    }

    pub fn add(ctx: Context<Add>) -> Result<()> {
        add_handler(ctx)
    }
}

#[error_code]
pub enum ProgramError {
    #[msg("This is not your counter!")]
    E000,
}
