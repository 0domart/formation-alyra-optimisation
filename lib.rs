use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWxqSWzJ3JDjU7MZxx9Q1w3KP9J7");

#[program]
pub mod beginner_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, initial_value: u64, extra_value: u64) -> Result<()> {
        msg!("Initializing the account with initial_value: {}", initial_value);
        let my_account = &mut ctx.accounts.my_account;
        my_account.data = initial_value.to_string();
        my_account.sum = 0;
        my_account.extra = extra_value;
        Ok(())
    }

    pub fn update(ctx: Context<Update>, new_value: String, increment: u64) -> Result<()> {
        msg!("Updating the account with new_value: {} and increment: {}", new_value, increment);
        let my_account = &mut ctx.accounts.my_account;
        let mut sum = 0;
        for i in 0..10000 {
            sum += i;
        }
        my_account.data = new_value;
        my_account.sum = sum + increment;
        Ok(())
    }

    pub fn complex_operation(ctx: Context<ComplexOperation>) -> Result<()> {
        msg!("Performing a complex operation");
        let my_account = &mut ctx.accounts.my_account;
        let mut result = 0;
        for i in 0..my_account.data.len() {
            result += i as u64;
        }
        my_account.sum = result;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 96)]
    pub my_account: Account<'info, MyAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub my_account: Account<'info, MyAccount>,
}

#[derive(Accounts)]
pub struct ComplexOperation<'info> {
    #[account(mut)]
    pub my_account: Account<'info, MyAccount>,
}

#[account]
pub struct MyAccount {
    pub data: String,
    pub sum: u64,
    pub extra: u64,
    pub unused_variable: u128,
}
