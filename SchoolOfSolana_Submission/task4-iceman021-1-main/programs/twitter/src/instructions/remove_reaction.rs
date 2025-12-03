//-------------------------------------------------------------------------------
///
/// TASK: Implement the remove reaction functionality for the Twitter program
///
/// Requirements:
/// - Verify that the tweet reaction exists and belongs to the reaction author
/// - Decrement the appropriate counter (likes or dislikes) on the tweet
/// - Close the tweet reaction account and return rent to reaction author
///
///-------------------------------------------------------------------------------

use anchor_lang::prelude::*;

use crate::errors::TwitterError;
use crate::states::*;

pub fn remove_reaction(ctx: Context<RemoveReactionContext>) -> Result<()> {
    // Get mutable reference to the tweet
    let tweet = &mut ctx.accounts.tweet;

    // 1. Decrement the appropriate counter (likes or dislikes) on the tweet
    // We use checked_sub for underflow safety, though it should not be possible
    // if the program logic is correct.
    match ctx.accounts.tweet_reaction.reaction {
        ReactionType::Like => {
            tweet.likes = tweet
                .likes
                .checked_sub(1)
                .ok_or(TwitterError::MinLikesReached)?;
        }
        ReactionType::Dislike => {
            tweet.dislikes = tweet
                .dislikes
                .checked_sub(1)
                .ok_or(TwitterError::MinDislikesReached)?;
        }
    }

    // 2. Account closing and rent return is handled by the `#[account(close = ...)]`
    //    constraint on the `tweet_reaction` account in the struct below.

    Ok(())
}

#[derive(Accounts)]
pub struct RemoveReactionContext<'info> {
    /// The user (signer) who is removing their reaction.
    /// Needs to be `mut` to receive the rent from the closed account.
    #[account(mut)]
    pub reaction_author: Signer<'info>,

    /// The reaction account (PDA) being removed.
    /// `mut` because it's being closed.
    /// `seeds` and `bump` are used to verify this is the correct PDA.
    /// `has_one = reaction_author` verifies the signer owns this reaction.
    /// `has_one = tweet` verifies this reaction belongs to the provided tweet.
    /// `close = reaction_author` closes the account and sends the lamports
    /// (rent) to the `reaction_author`.
    #[account(
        mut,
        seeds = [
            TWEET_REACTION_SEED.as_bytes(),
            tweet.key().as_ref(),
            reaction_author.key().as_ref()
        ],
        bump = tweet_reaction.bump,
        has_one = reaction_author,
        has_one = tweet @ TwitterError::InvalidTweet, // Custom error on mismatch
        close = reaction_author
    )]
    pub tweet_reaction: Account<'info, Reaction>,

    /// The tweet being reacted to.
    /// Needs to be `mut` because its `likes` or `dislikes`
    /// counter will be decremented.
    #[account(mut)]
    pub tweet: Account<'info, Tweet>,
}
