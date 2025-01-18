use anchor_lang::prelude::*;

declare_id!("YourProgramID"); // Gantilah dengan ID program Anda

#[program]
pub mod token_contract {
    use super::*;

    // Inisialisasi akun token untuk pengguna
    pub fn initialize(ctx: Context<Initialize>, initial_supply: u64) -> Result<()> {
        let token_account = &mut ctx.accounts.token_account;
        token_account.supply = initial_supply;
        token_account.balance = initial_supply;
        Ok(())
    }

    // Transfer token antar akun
    pub fn transfer(ctx: Context<Transfer>, amount: u64) -> Result<()> {
        let sender = &mut ctx.accounts.sender;
        let receiver = &mut ctx.accounts.receiver;

        if sender.balance < amount {
            return Err(ErrorCode::InsufficientBalance.into());
        }

        sender.balance -= amount;
        receiver.balance += amount;

        Ok(())
    }

    // Mint token baru ke akun
    pub fn mint(ctx: Context<Mint>, amount: u64) -> Result<()> {
        let token_account = &mut ctx.accounts.token_account;

        token_account.supply += amount;
        token_account.balance += amount;

        Ok(())
    }

    // Burn token dari akun
    pub fn burn(ctx: Context<Burn>, amount: u64) -> Result<()> {
        let token_account = &mut ctx.accounts.token_account;

        if token_account.balance < amount {
            return Err(ErrorCode::InsufficientBalance.into());
        }

        token_account.balance -= amount;
        token_account.supply -= amount;

        Ok(())
    }

    // Query saldo token
    pub fn query_balance(ctx: Context<QueryBalance>) -> Result<u64> {
        Ok(ctx.accounts.token_account.balance)
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8 + 8)] // Alokasi ruang untuk supply dan saldo token
    pub token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Transfer<'info> {
    #[account(mut)]
    pub sender: Account<'info, TokenAccount>,
    #[account(mut)]
    pub receiver: Account<'info, TokenAccount>,
}

#[derive(Accounts)]
pub struct Mint<'info> {
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
}

#[derive(Accounts)]
pub struct Burn<'info> {
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
}

#[derive(Accounts)]
pub struct QueryBalance<'info> {
    pub token_account: Account<'info, TokenAccount>,
}

#[account]
pub struct TokenAccount {
    pub balance: u64,  // Saldo token yang dimiliki oleh akun
    pub supply: u64,   // Total pasokan token
}

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient balance")]
    InsufficientBalance,
}

