use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("V1st1ng111111111111111111111111111111111112");

@program
pub mod solana_token_vesting_minimal {
    use super::*;

    pub fn create_vesting_schedule(
        ctx: Context<CreateVestingSchedule>,
        amount: u64,
        start_time: i64,
        end_time: i64,
        cliff_time: i64,
    ) -> Result<()> {
        let schedule = &mut ctx.accounts.vesting_schedule;
        schedule.initializer = ctx.accounts.initializer.key();
        schedule.beneficiary = ctx.accounts.beneficiary.key();
        schedule.token_vault = ctx.accounts.token_vault.key();
        schedule.total_amount = amount;
        schedule.released_amount = 0;
        schedule.start_time = start_time;
        schedule.end_time = end_time;
        schedule.cliff_time = cliff_time;

        let cpi_ctx = Context::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.initializer_token_account.to_account_info(),
                to: ctx.accounts.token_vault.to_account_info(),
                authority: ctx.accounts.initializer.to_account_info(),
            },
        );
        token::transfer(cpi_ctx, amount)?;
        Ok(())
    }

    pub fn claim(ctx: Context<ClaimTokens>) -> Result<()> {
        let schedule = &mut ctx.accounts.vesting_schedule;
        let current_time = Clock::get()?.unix_timestamp;

        require!(current_time >= schedule.cliff_time, VestingError::CliffNotReached);

        let vested_amount = if current_time >= schedule.end_time {
            schedule.total_amount
        } else {
            schedule.total_amount
                .checked_mul((current_time - schedule.start_time) as u64).unwrap()
                .checked_div((schedule.end_time - schedule.start_time) as u64).unwrap()
        };

        let claimable = vested_amount.checked_sub(schedule.released_amount).unwrap();
        require!(claimable > 0, VestingError::NothingToClaim);

        schedule.released_amount = schedule.released_amount.checked_add(claimable).unwrap();

        let bump = ctx.bumps.vesting_schedule;
        let signer_seeds: &[&[&[u8]]] = &[&[
            b"vesting",
            schedule.initializer.as_ref(),
            schedule.beneficiary.as_ref(),
            &[bump],
        ]];

        let cpi_ctx = Context::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.token_vault.to_account_info(),
                to: ctx.accounts.beneficiary_token_account.to_account_info(),
                authority: schedule.to_account_info(),
            },
            signer_seeds,
        );
        token::transfer(cpi_ctx, claimable)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateVestingSchedule<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,
    /// CHECK: Safe beneficiary key
    pub beneficiary: AccountInfo<'info>,
    #[account(mut)]
    pub initializer_token_account: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = initializer,
        space = 8 + 32 + 32 + 32 + 8 + 8 + 8 + 8 + 8,
        seeds = [b"vesting", initializer.key().as_ref(), beneficiary.key().as_ref()],
        bump
    )]
    pub vesting_schedule: Account<'info, VestingSchedule>,
    #[account(mut)]
    pub token_vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ClaimTokens<'info> {
    #[account(mut)]
    pub beneficiary: Signer<'info>,
    #[account(
        mut,
        seeds = [b"vesting", vesting_schedule.initializer.as_ref(), beneficiary.key().as_ref()],
        bump
    )]
    pub vesting_schedule: Account<'info, VestingSchedule>,
    #[account(mut)]
    pub token_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub beneficiary_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct VestingSchedule {
    pub initializer: Pubkey,
    pub beneficiary: Pubkey,
    pub token_vault: Pubkey,
    pub total_amount: u64,
    pub released_amount: u64,
    pub start_time: i64,
    pub end_time: i64,
    pub cliff_time: i64,
}

#[error_code]
pub enum VestingError {
    #[msg("The cliff period has not ended yet.")]
    CliffNotReached,
    #[msg("There are no fully vested tokens available to claim.")]
    NothingToClaim,
}
