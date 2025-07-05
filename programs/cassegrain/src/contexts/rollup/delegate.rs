use anchor_lang::prelude::*;
use crate::consts::*;

// Magic Block SDK imports
use ephemeral_rollups_sdk::anchor::delegate;
use ephemeral_rollups_sdk::cpi::DelegateConfig;

#[delegate]
#[derive(Accounts)]
#[instruction(batch_id: [u8; 32], event_id: [u8; 32])]
pub struct DelegateProduct<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    /// CHECK: The Product Batch account we are delegating to ER
    #[account(
        mut,
        del,
        seeds = [BATCH, batch_id.as_ref()],
        bump,
    )]
    pub product_batch: AccountInfo<'info>,

    /// CHECK: The Product Event account we are delegating to ER
    #[account(
        mut,
        del,
        seeds = [EVENT, event_id.as_ref()],
        bump,
    )]
    pub product_event: AccountInfo<'info>,

}

impl<'info> DelegateProduct<'info> {
    pub fn delegate_to_rollup(
        &mut self,
        batch_id: [u8; 32],
        event_id: [u8; 32],
    ) -> Result<()> {
        msg!("Delegating supply chain accounts to Magic Block Ephemeral Rollup...");
        
        // Delegate Product Batch account
        self.delegate_product_batch(
            &self.signer,
            &[BATCH, batch_id.as_ref()],
            DelegateConfig::default(),
        )?;
        
        // Delegate Product Event account  
        self.delegate_product_event(
            &self.signer,
            &[EVENT, event_id.as_ref()],
            DelegateConfig::default(),
        )?;
        
        msg!(
            "Successfully delegated batch and event to ephemeral rollup",
        );
        Ok(())
    }
}