use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("REPLACE_WITH_PROGRAM_ID");

#[program]
pub mod ticket_fees {
    use super::*;

    pub fn pay_fee(
        ctx: Context<PayFee>,
        ticket_amount: u16,
        sol_equivalent_lamports: u64,
    ) -> Result<()> {
        let allowed = ticket_amount == 100 || ticket_amount == 250 || ticket_amount == 500;
        require!(allowed, CustomError::InvalidTicketAmount);

        let fee_receiver = &ctx.accounts.fee_receiver;
        let payer = &ctx.accounts.payer;

        **payer.try_borrow_mut_lamports()? -= sol_equivalent_lamports;
        **fee_receiver.try_borrow_mut_lamports()? += sol_equivalent_lamports;

        emit!(FeePaid {
            payer: payer.key(),
            tickets: ticket_amount,
            amount: sol_equivalent_lamports,
        });

        Ok(())
    }
}

#[derive(Accounts)]
pub struct PayFee<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub fee_receiver: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[event]
pub struct FeePaid {
    pub payer: Pubkey,
    pub tickets: u16,
    pub amount: u64,
}

#[error_code]
pub enum CustomError {
    #[msg("Invalid ticket amount")]
    InvalidTicketAmount,
}
