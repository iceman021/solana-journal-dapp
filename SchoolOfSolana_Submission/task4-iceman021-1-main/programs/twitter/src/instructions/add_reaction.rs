//-------------------------------------------------------------------------------
///
/// TASK: Implement the add reaction functionality for the Twitter program
///
/// Requirements:
/// - Initialize a new reaction account with proper PDA seeds
/// - Increment the appropriate counter (likes or dislikes) on the tweet
/// - Set reaction fields: type, author, parent tweet, and bump
/// - Handle both Like and Dislike reaction types
///
///-------------------------------------------------------------------------------

use anchor_lang::prelude::*;

use crate::errors::TwitterError;
use crate::states::*;

pub fn add_reaction(ctx: Context<AddReactionContext>, reaction: ReactionType) -> Result<()> {
    // Get mutable references to the accounts
    let tweet = &mut ctx.accounts.tweet;
    let tweet_reaction = &mut ctx.accounts.tweet_reaction;

    // 1. Increment the appropriate counter (likes or dislikes) on the tweet
    // We use checked_add to prevent overflow and return a custom error.
    match reaction {
        ReactionType::Like => {
            tweet.likes = tweet
                .likes
                .checked_add(1)
                .ok_or(TwitterError::MaxLikesReached)?;
        }
        ReactionType::Dislike => {
            tweet.dislikes = tweet
                .dislikes
                .checked_add(1)
                .ok_or(TwitterError::MaxDislikesReached)?;
        }
    }

    // 2. Set reaction fields: type, author, parent tweet, and bump
    tweet_reaction.reaction_author = *ctx.accounts.reaction_author.key;
    tweet_reaction.parent_tweet = tweet.key();
    tweet_reaction.reaction = reaction;
    tweet_reaction.bump = ctx.bumps.tweet_reaction;

    Ok(())
}

#[derive(Accounts)]
pub struct AddReactionContext<'info> {
    /// The user (signer) adding the reaction.
    /// Payer for the new `tweet_reaction` account.
    #[account(mut)]
    pub reaction_author: Signer<'info>,

    /// The new reaction account (PDA).
    /// `init` creates the account.
    /// `payer` is the `reaction_author`.
    /// `space` is 8-byte discriminator + struct size.
    /// `seeds` ensure one reaction per author per tweet:
    ///   - "TWEET_REACTION_SEED"
    ///   - The parent `tweet`'s public key
    ///   - The `reaction_author`'s public key
    /// `bump` stores the bump seed.
    #[account(
        init,
        payer = reaction_author,
        space = 8 + Reaction::INIT_SPACE,
        seeds = [
            TWEET_REACTION_SEED.as_bytes(),
            tweet.key().as_ref(),
            reaction_author.key().as_ref()
        ],
        bump
    )]
    pub tweet_reaction: Account<'info, Reaction>,

    /// The tweet being reacted to.
    /// Needs to be `mut` because its `likes` or `dislikes`
    /// counter will be incremented.
    #[account(mut)]
    pub tweet: Account<'info, Tweet>,

    /// The System Program is required by Anchor for `init`.
    pub system_program: Program<'info, System>,
}
