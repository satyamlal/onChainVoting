use anchor_lang::prelude::*;

declare_id!("GUTMvs4uirowQBJstmRsiHkvKL2UdS6F3FTTeq29tqoC");

#[program]
pub mod on_chain_voting {
    use super::*;

    pub fn init_poll(ctx: Context<InitPoll>, _poll_id: u64, start: u64, end: u64, name: String, description: String) -> Result<()> {
        let poll = &mut ctx.accounts.poll_account;
        poll.poll_name = name;
        poll.poll_description = description;
        poll.poll_start = start;
        poll.poll_end = end;
        Ok(())
    }

    pub fn initialize_candidate(ctx: Context<InitializeCandidate>, _poll_id: u64, candidate: String) -> Result<()> {
        ctx.accounts.candidate_account.candidate_name = candidate;
        ctx.accounts.poll_account.poll_option_index += 1;
        OK(())
    }
}

#[derive(Accounts)]
#[instruction(poll_id: u64)]
pub struct InitPoll<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = 8 + PollAccount::INIT_SPACE,
        seeds = [b"poll".as_ref(), poll_id.to_le_bytes().as_ref()],
        bump        
    )]
    pub poll_account: Account<'info, PollAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(poll_id: u64, candidate: String)]
pub struct InitCandidate<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = 8 + PollAccount::INIT_SPACE,
        seeds = [b"poll".as_ref(), poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll_account: Account<'info, PollAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct PollAccount {
    #[max_len(32)]
    pub poll_name: String,

    #[max_len(280)]
    pub poll_description: String,
    pub poll_start: u64,
    pub poll_end: u64,
    pub poll_option_index: u64
}

#[account]
#[derive(InitSpace)]
pub struct CandidateAccount {
    #[max_len(32)]
    pub candidate_name: String,
    pub candidate_votes: u64,
}