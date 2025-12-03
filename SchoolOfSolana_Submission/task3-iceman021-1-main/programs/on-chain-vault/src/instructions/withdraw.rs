//-------------------------------------------------------------------------------
///
/// TASK: Implement the withdraw functionality for the on-chain vault
///
/// Requirements:
/// - Verify that the vault is not locked
/// - Verify that the vault has enough balance to withdraw
/// - Transfer lamports from vault to vault authority (using a CPI)
/// - Emit a withdraw event after successful transfer
///
///-------------------------------------------------------------------------------

use anchor_lang::prelude::*;
use anchor_lang::system_program; // Import for the Anchor-style CPI
use crate::state::Vault;
use crate::errors::VaultError;
use crate::events::WithdrawEvent;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    /// The vault authority (signer) who is withdrawing.
    /// This account is also the recipient, so it must be mutable.
    #[account(mut)]
    pub vault_authority: Signer<'info>,

    /// The vault PDA we are withdrawing from.
    /// Needs to be mutable to send SOL.
    /// We use constraints to:
    /// 1. Verify the PDA seeds (assuming `b"vault"` + authority key)
    /// 2. Get the `bump`
    /// 3. Ensure the signer `vault_authority` matches the one stored on the `vault` account
    #[account(
        mut,
        seeds = [b"vault", vault_authority.key().as_ref()],
        bump,
        has_one = vault_authority
    )]
    pub vault: Account<'info, Vault>,

    /// The System Program, required for the CPI.
    pub system_program: Program<'info, System>,
}

pub fn _withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    // 1. Verify that the vault is not locked
    if ctx.accounts.vault.locked {
        return err!(VaultError::VaultLocked);
    }

    // 2. Verify that the vault has enough balance to withdraw
    // Although the CPI will fail, checking here gives a cleaner error.
    let vault_balance = ctx.accounts.vault.to_account_info().lamports();
    if vault_balance < amount {
        return err!(VaultError::InsufficientBalance);
    }

    // 3. Prepare the CPI accounts for the transfer
    let cpi_accounts = system_program::Transfer {
        from: ctx.accounts.vault.to_account_info(),
        to: ctx.accounts.vault_authority.to_account_info(),
    };
    let cpi_program = ctx.accounts.system_program.to_account_info();

    // 4. Prepare the signer seeds for the PDA
    // The program signs on behalf of the `vault` PDA.
    let authority_key = ctx.accounts.vault_authority.key();
    let seeds = &[
        b"vault",
        authority_key.as_ref(),
        &[ctx.bumps.vault] // Use the bump from the Accounts context
    ];
    let signer_seeds = &[&seeds[..]];

    // 5. Create the CpiContext with the signer seeds
    let cpi_context = CpiContext::new_with_signer(
        cpi_program,
        cpi_accounts,
        signer_seeds
    );

    // 6. Execute the CPI to transfer lamports
    system_program::transfer(cpi_context, amount)?;

    // 7. Emit a withdraw event after successful transfer
    emit!(WithdrawEvent {
        authority: *ctx.accounts.vault_authority.key,
        vault: ctx.accounts.vault.key(),
        amount,
    });

    Ok(())
}
