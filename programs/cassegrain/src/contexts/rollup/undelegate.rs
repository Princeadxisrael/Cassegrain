use anchor_lang::prelude::*;
use crate::consts::*;
use crate::state::*;
use crate::error::*;

// Magic Block SDK imports for commit and undelegate
use ephemeral_rollups_sdk::anchor::commit;
use ephemeral_rollups_sdk::ephem::commit_and_undelegate_accounts;

#[commit]
#[derive(Accounts)]
#[instruction(batch_id: [u8; 32], event_id: [u8; 32])]
pub struct UndelegateProduct<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    /// The delegated Product Batch account (to be undelegated)
    #[account(
        mut,
        seeds = [BATCH, batch_id.as_ref()],
        bump,
        constraint = product_batch.manufacturer == signer.key() 
            @ CassegrainError::Unauthorized,
    )]
    pub product_batch: Account<'info, ProductBatch>,

    /// The delegated Product Event account (to be undelegated)
    #[account(
        mut,
        seeds = [EVENT, event_id.as_ref()],
        bump,
        constraint = product_event.batch_id == batch_id 
            @ CassegrainError::InvalidBatchId,
        constraint = product_event.event_id == event_id 
            @ CassegrainError::InvalidEventId,
    )]
    pub product_event: Account<'info, ProductEvent>,
}

impl<'info> UndelegateProduct<'info> {
    pub fn undelegate(
        &mut self,
        batch_id: [u8; 32],
        event_id: [u8; 32],
    ) -> Result<()> {
      
        msg!("💾 Committing final state and undelegating from rollup...");

        let clock = Clock::get()?;
        
        commit_and_undelegate_accounts(
            &self.signer,
            vec![
                &self.product_batch.to_account_info(),
                &self.product_event.to_account_info(),
            ],
            &self.magic_context,
            &self.magic_program,
        )?;

        msg!("✅ Supply chain accounts successfully committed and undelegated!");
        msg!("🏠 Accounts are now back on Solana mainnet");
        msg!("📋 Final tracking data permanently recorded");

        // Emit completion event
        emit!(SupplyChainCompleted {
            batch_id,
            event_id,
            final_status: self.product_batch.status,
            final_order_status: self.product_event.order_status,
            verification_status: self.product_event.verification_status,
            total_events: self.product_batch.total_events,
            completed_by: self.signer.key(),
            completion_timestamp: clock.unix_timestamp,
        });

        Ok(())
    }

}


// Events for tracking completion
#[event]
pub struct SupplyChainCompleted {
    pub batch_id: [u8; 32],
    pub event_id: [u8; 32],
    pub final_status: ProductStatus,
    pub final_order_status: OrderStatus,
    pub verification_status: VerificationStatus,
    pub total_events: u32,
    pub completed_by: Pubkey,
    pub completion_timestamp: i64,
}

