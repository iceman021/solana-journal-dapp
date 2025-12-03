//-------------------------------------------------------------------------------
///
/// TASK: Implement the deposit functionality for the on-chain vault
///
/// Requirements:
/// - Verify that the user has enough balance to deposit (Handled by the CPI)
/// - Verify that the vault is not locked
/// - Transfer lamports from user to vault using CPI (Cross-Program Invocation)
/// - Emit a deposit event after successful transfer
///
///-------------------------------------------------------------------------------

use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::system_instruction::transfer;
use crate::state::Vault;
use crate::errors::VaultError;
use crate::events::DepositEvent;

#[derive(Accounts)]
pub struct Deposit<'info> {
    /// The user (signer) who is depositing SOL.
    /// Needs to be mutable because their SOL balance will decrease.
    #[account(mut)]
    pub user: Signer<'info>,

    /// The vault PDA account we are depositing into.
    /// Needs to be mutable to receive the SOL.
    #[account(mut)]
    pub vault: Account<'info, Vault>,

    /// The System Program is required for the CPI transfer.
    pub system_program: Program<'info, System>,
}

pub fn _deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    // 1. Verify that the vault is not locked
    // We access the deserialized vault account data via ctx.accounts.vault
    if ctx.accounts.vault.locked {
        return err!(VaultError::VaultLocked);
    }

    // 2. Transfer lamports from user to vault using CPI
    // We use the low-level invoke call as suggested by the file's imports
    let transfer_instruction = transfer(
        ctx.accounts.user.to_account_info().key,
        ctx.accounts.vault.to_account_info().key,
        amount,
    );

    invoke(
        &transfer_instruction,
        &[
            ctx.accounts.user.to_account_info(),
            ctx.accounts.vault.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;

    // 3. Emit a deposit event after successful transfer
    emit!(DepositEvent {
        user: *ctx.accounts.user.key,
        vault: ctx.accounts.vault.key(),
        amount,
    });

    Ok(())
}
