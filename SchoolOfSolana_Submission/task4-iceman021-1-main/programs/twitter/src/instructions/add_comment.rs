//-------------------------------------------------------------------------------
///
/// TASK: Implement the add comment functionality for the Twitter program
///
/// Requirements:
/// - Validate that comment content doesn't exceed maximum length
/// - Initialize a new comment account with proper PDA seeds
/// - Set comment fields: content, author, parent tweet, and bump
/// - Use content hash in PDA seeds for unique comment identification
///
///-------------------------------------------------------------------------------

use anchor_lang::prelude::*;
// Import the hashing function
use anchor_lang::solana_program::hash::hash;

use crate::errors::TwitterError;
use crate::states::*;

pub fn add_comment(ctx: Context<AddCommentContext>, comment_content: String) -> Result<()> {
    // 1. Validate that comment content doesn't exceed maximum length
    if comment_content.len() > COMMENT_LENGTH {
        return err!(TwitterError::CommentTooLong);
    }

    // 2. Set comment fields
    let comment = &mut ctx.accounts.comment;
    comment.comment_author = *ctx.accounts.comment_author.key;
    comment.parent_tweet = ctx.accounts.tweet.key();
    comment.content = comment_content; // Store the full content
    comment.bump = ctx.bumps.comment;

    Ok(())
}

#[derive(Accounts)]
#[instruction(comment_content: String)] // Makes `comment_content` available for seeds
pub struct AddCommentContext<'info> {
    /// The user (signer) adding the comment.
    /// Payer for the new `comment` account.
    #[account(mut)]
    pub comment_author: Signer<'info>,

    /// The new comment account (PDA).
    /// `init` creates the account.
    /// `payer` is the `comment_author`.
    /// `space` is 8-byte discriminator + struct size.
    /// `seeds` use the content hash for uniqueness.
    #[account(
        init,
        payer = comment_author,
        space = 8 + Comment::INIT_SPACE,
        seeds = [
            COMMENT_SEED.as_bytes(),
            tweet.key().as_ref(),
            // We hash the content and use its bytes as the seed
            hash(comment_content.as_bytes()).to_bytes().as_ref()
        ],
        bump
    )]
    pub comment: Account<'info, Comment>,

    /// The tweet being commented on.
    /// We only need to read its key, so no `mut` is needed.
    pub tweet: Account<'info, Tweet>,

    /// The System Program is required by Anchor for `init`.
    pub system_program: Program<'info, System>,
}
