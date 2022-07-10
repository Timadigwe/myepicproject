use anchor_lang::prelude::*;

declare_id!("Ei7K8nUgLGVC8BrXMTZqKTLJjfC4RDxHJaJkJaeonJGZ");

#[program]
pub mod myepicproject {
    use super::*;
    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> Result  <()> {
        let base_account = &mut ctx.accounts.base_account;

        base_account.total_gifs = 0;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account:Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>, 
}

#[account]
pub  struct BaseAccount {
    pub total_gifs: u64, 
}