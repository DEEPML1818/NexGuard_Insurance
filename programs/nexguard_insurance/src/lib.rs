use anchor_lang::prelude::*;
declare_id!("5VyTJTmfBBLaNRYVXxEfrxUyeVvdUtBohouPDTjZXGCG");

#[program]
pub mod nexguard_insurance {
    use super::*;

    pub fn initialize_policy(
        ctx: Context<InitializePolicy>,
        policy_id: String,
        premium: u64,
        coverage_amount: u64,
    ) -> Result<()> {
        let policy = &mut ctx.accounts.policy;
        policy.policy_id = policy_id;
        policy.premium = premium;
        policy.coverage_amount = coverage_amount;
        policy.claimed = false;
        Ok(())
    }

    pub fn pay_premium(ctx: Context<PayPremium>, amount: u64) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        user_account.balance += amount;
        Ok(())
    }

    pub fn process_claim(ctx: Context<ProcessClaim>, claim_amount: u64) -> Result<()> {
        let policy = &mut ctx.accounts.policy;
        require!(!policy.claimed, InsuranceError::AlreadyClaimed);
        require!(claim_amount <= policy.coverage_amount, InsuranceError::InsufficientCoverage);
        policy.claimed = true;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializePolicy<'info> {
    #[account(init, payer = user, space = 8 + 64)]
    pub policy: Account<'info, Policy>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PayPremium<'info> {
    #[account(mut)]
    pub user_account: Account<'info, UserAccount>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct ProcessClaim<'info> {
    #[account(mut)]
    pub policy: Account<'info, Policy>,
    #[account(mut)]
    pub user_account: Account<'info, UserAccount>,
}

#[account]
pub struct Policy {
    pub policy_id: String,
    pub premium: u64,
    pub coverage_amount: u64,
    pub claimed: bool,
}

#[account]
pub struct UserAccount {
    pub balance: u64,
}

#[error_code]
pub enum InsuranceError {
    #[msg("This policy has already been claimed.")]
    AlreadyClaimed,
    #[msg("Claim amount exceeds coverage.")]
    InsufficientCoverage,
}

