use anchor_lang::prelude::*;

declare_id!("7bP565Kfda36jnWLjeHj5eJc8pMxzLDNWLs8Vvr9Too6");

#[program]
pub mod anchor_rust_test {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.start_time = u64::try_from(Clock::get()?.unix_timestamp)?;
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        msg!("Previous counter: {}", counter.count);
        let now = u64::try_from(Clock::get()?.unix_timestamp)?;
        require!(now > counter.start_time+(60*60),MyError::CounterNotReady);
        counter.count += 1;
        msg!("Counter incremented! Current count: {}", counter.count);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {

     #[account(
        init,
        payer = payer,
        seeds=[b"counter"],
        bump,
        space = 8 + 8 + 8
    )]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,

}
#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(
        mut,
        seeds=[b"counter"],
        bump
    )]
    pub counter: Account<'info, Counter>,
        #[account(mut)]
    pub payer: Signer<'info>,
}
 
#[account]
pub struct Counter {
    pub count: u64, // 8 bytes
    pub start_time: u64, // 8 bytes
}

#[error_code]
pub enum MyError {
    #[msg("The counter cannot be incremented yet.")]
    CounterNotReady,
}
