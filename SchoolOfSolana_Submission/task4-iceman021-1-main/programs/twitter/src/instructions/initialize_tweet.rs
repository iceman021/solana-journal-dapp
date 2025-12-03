//-------------------------------------------------------------------------------
///
/// TASK: Implement the initialize tweet functionality for the Twitter program
///
/// Requirements:
/// - Validate that topic and content don't exceed maximum lengths
/// - Initialize a new tweet account with proper PDA seeds
/// - Set tweet fields: topic, content, author, likes, dislikes, and bump
/// - Initialize counters (likes and dislikes) to zero
/// - Use topic in PDA seeds for tweet identification
///
///-------------------------------------------------------------------------------

use anchor_lang::prelude::*;

use crate::errors::TwitterError;
use crate::states::*;

pub fn initialize_tweet(
    ctx: Context<InitializeTweet>,
    topic: String,
    content: String,
) -> Result<()> {
    // 1. Validate that topic and content don't exceed maximum lengths
    // The topic length is also constrained by the PDA seed, but this provides
    // a better error message.
    if topic.len() > TOPIC_LENGTH {
        return err!(TwitterError::TopicTooLong);
    }
    if content.len() > CONTENT_LENGTH {
        return err!(TwitterError::ContentTooLong);
    }

    // 2. Set all the fields for the new tweet account
    let tweet = &mut ctx.accounts.tweet;
    tweet.tweet_author = *ctx.accounts.tweet_authority.key;
    tweet.topic = topic;
    tweet.content = content;
    tweet.likes = 0;
    tweet.dislikes = 0;
    tweet.bump = ctx.bumps.tweet; // Store the bump

    Ok(())
}

#[derive(Accounts)]
#[instruction(topic: String)] // Make `topic` available for PDA seed validation
pub struct InitializeTweet<'info> {
    /// The user (signer) initializing the tweet.
    /// This user will be the payer for the new account.
    #[account(mut)]
    pub tweet_authority: Signer<'info>,

    /// The new tweet account (PDA).
    /// `init` creates the account.
    /// `payer` is the `tweet_authority`.
    /// `space` is calculated: 8-byte discriminator + struct size.
    /// `seeds` define the PDA's address:
    ///   - "TWEET_SEED"
    ///   - The topic (as bytes)
    ///   - The authority's public key
    /// `bump` stores the bump seed.
    #[account(
        init,
        payer = tweet_authority,
        space = 8 + Tweet::INIT_SPACE,
        seeds = [
            TWEET_SEED.as_bytes(),
            topic.as_bytes(),
            tweet_authority.key().as_ref()
        ],
        bump
    )]
    pub tweet: Account<'info, Tweet>,

    /// The System Program is required by Anchor for `init`.
    pub system_program: Program<'info, System>,
}
