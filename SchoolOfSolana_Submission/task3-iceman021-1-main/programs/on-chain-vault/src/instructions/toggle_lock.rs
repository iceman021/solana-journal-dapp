//-------------------------------------------------------------------------------
///
/// TASK: Implement the toggle lock functionality for the on-chain vault
///
/// Requirements:
/// - Toggle the locked state of the vault (locked becomes unlocked, unlocked becomes locked)
/// - Only the vault authority should be able to toggle the lock
/// - Emit a toggle lock event after successful state change
///
///-------------------------------------------------------------------------------

use anchor_lang::prelude::*;
use crate::state::Vault;
use crate::events::ToggleLockEvent;

#[derive(Accounts)]
pub struct ToggleLock<'info> {
    /// The vault authority (signer) who is toggling the lock.
    pub vault_authority: Signer<'info>,

    /// The vault PDA account we are toggling.
    /// Needs to be mutable to change its `locked` state.
    /// We use the `has_one` constraint to ensure the signer
    /// is the `vault_authority` stored on the vault account.
    #[account(
        mut,
        has_one = vault_authority
    )]
    pub vault: Account<'info, Vault>,
}

pub fn _toggle_lock(ctx: Context<ToggleLock>) -> Result<()> {
    // 1. Toggle the locked state of the vault
    // We get a mutable reference to the vault's data
    let vault = &mut ctx.accounts.vault;
    vault.locked = !vault.locked;

    // 2. Emit a toggle lock event
    emit!(ToggleLockEvent {
        vault: vault.key(),
        locked: vault.locked,
    });

    Ok(())
}
