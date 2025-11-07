use anchor_lang::prelude::*;            

declare_id!("FEmrsw174xJ5ySpfLsXYBL3bbmSbovd6dByp3239zpMT");

#[program]
pub mod tweet_program {
    use super::*;

    pub fn create_tweet(ctx: Context<CreateTweet>, text: String) -> Result<()> {
        let tweet = &mut ctx.accounts.tweet;

        require!(text.len() <= 280, TweetError::TweetTooLong);

        tweet.author = ctx.accounts.user.key();
        tweet.text = text;
        tweet.timestamp = Clock::get()?.unix_timestamp;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateTweet<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + Tweet::MAX_SIZE,
        seeds = [b"tweet", user.key().as_ref()],
        bump,
    )]
    pub tweet: Account<'info, Tweet>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,

}

#[account]
pub struct Tweet {
    pub author: Pubkey,
    pub timestamp: i64,
    pub text: String,
}

impl Tweet {
    pub const MAX_TEXT: usize = 280;
    pub const MAX_SIZE: usize = 32 + 8 + 4 + Self::MAX_TEXT;
}

#[error_code]
pub enum TweetError {
    TweetTooLong,
}