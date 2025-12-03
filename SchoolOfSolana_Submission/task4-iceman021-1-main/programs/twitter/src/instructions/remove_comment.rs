//-------------------------------------------------------------------------------
///
/// TASK: Implement the remove comment functionality for the Twitter program
///
/// Requirements:
/// - Close the comment account and return rent to comment author
///
/// NOTE: No implementation logic is needed in the function body - this
/// functionality is achieved entirely through account constraints!
///
///-------------------------------------------------------------------------------

use anchor_lang::prelude::*;
use anchor_lang::solana_program::hash::hash;

use crate::states::*;

pub fn remove_comment(_ctx: Context<RemoveCommentContext>) -> Result<()> {
    // This function is intentionally empty.
    // The `#[account(close = ...)]` constraint handles all the logic
    // of closing the account and refunding the rent.
    Ok(())
}

#[derive(Accounts)]
pub struct RemoveCommentContext<'info> {
    /// The user (signer) who is removing their comment.
    /// Needs to be `mut` to receive the rent from the closed account.
    #[account(mut)]
    pub comment_author: Signer<'info>,

    /// The comment account (PDA) being removed.
    /// `mut` because it's being closed.
    /// `seeds` validates the PDA by reconstructing the address
    ///   using fields *from the comment account itself*.
    /// `bump` uses the stored bump for validation.
    /// `has_one` verifies the signer is the `comment_author`.
    /// `close` closes the account and refunds rent to `comment_author`.
    #[account(
        mut,
        seeds = [
            COMMENT_SEED.as_bytes(),
            comment.parent_tweet.as_ref(),
            hash(comment.content.as_bytes()).to_bytes().as_ref()
        ],
        bump = comment.bump,
        has_one = comment_author,
        close = comment_author
    )]
    pub comment: Account<'info, Comment>,
}
