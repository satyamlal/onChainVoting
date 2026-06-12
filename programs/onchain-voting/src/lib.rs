use anchor_lang::prelude::*;

declare_id!("GUTMvs4uirowQBJstmRsiHkvKL2UdS6F3FTTeq29tqoC");

#[program]
pub mod onChainVoting {
    use super::*;

    pub fn init_poll(ctx: Context<InitPoll>) -> Result<()> {
        initialize::handler(ctx)
    }
}

#[account]
#[derive(InitSpace)]
pub struct PollAccount {
    #[max_length(32)]
    pub poll_name: String,

    #[max_length(280)]
    pub poll_description: String,
    pub poll_start: u64,
    pub poll_end: u64,
    pub poll_option_index: u64
}

#[account]
#[derive(InitSpace)]
pub struct CandidateAccount {
    #[max_length(32)]
    pub candidate_name: String,
    pub candidate_votes: u64
}