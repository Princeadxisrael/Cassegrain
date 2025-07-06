use anchor_lang::prelude::*;
use crate::consts::*;
use crate::state::*;

// Magic Block SDK imports for commit
use ephemeral_rollups_sdk::anchor::commit;
use ephemeral_rollups_sdk::ephem::commit_accounts;

#[commit]
#[derive(Accounts)]
#[instruction(batch_id: [u8; 32], event_id: [u8; 32])]
pub struct RollupEventLog<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    /// The delegated Product Batch account (already on rollup)
    #[account(
        mut,
        seeds = [BATCH, batch_id.as_ref()],
        bump,
    )]
    pub product_batch: Account<'info, ProductBatch>,

    /// The delegated Product Event account (already on rollup)
    #[account(
        mut,
        seeds = [EVENT, event_id.as_ref()],
        bump,
    )]
    pub product_event: Account<'info, ProductEvent>,
}

impl<'info> RollupEventLog<'info> {
    pub fn update_supply_chain_state(
        &mut self,
        _batch_id: [u8; 32],
        _event_id: [u8; 32],
        // Fields to update
        new_product_status: Option<ProductStatus>,
        new_order_status: Option<OrderStatus>, 
        new_event_type: Option<EventType>,
        previous_event: Option<Pubkey>,
        next_event: Option<Pubkey>,
        metadata_ipfs: Option<String>,
    ) -> Result<()> {
        let clock = Clock::get()?;
        
        msg!("🔄 Updating supply chain state on rollup...");

      

        // 1. Update ProductBatch status if provided
        if let Some(status) = new_product_status {
            self.product_batch.status = status;
            msg!("📦 Batch status updated to: {:?}", status);
        }

        // 2. Update ProductEvent fields
        if let Some(event_type) = new_event_type {
            self.product_event.product_event_type = event_type;
            msg!("📝 Event type updated to: {:?}", event_type);
        }

        if let Some(order_status) = new_order_status {
            self.product_event.order_status = order_status;
            msg!("📋 Order status updated to: {:?}", order_status);
        }

        if let Some(prev_event) = previous_event {
            self.product_event.previous_event = Some(prev_event);
            msg!("🔗 Previous event linked: {:?}", prev_event);
        }

        if let Some(next_event) = next_event {
            self.product_event.next_event = Some(next_event);
            msg!("🔗 Next event linked: {:?}", next_event);
        }

        if let Some(ipfs) = metadata_ipfs {
            self.product_event.metadata_ipfs = Some(ipfs.clone());
            msg!("📎 Metadata IPFS updated: {}", ipfs);
        }

        // 3. Update timestamps
        self.product_event.timestamp = clock.unix_timestamp;
        self.product_batch.last_updated = clock.unix_timestamp;

        // 4. Increment event counter
        self.product_batch.total_events += 1;

        // Log current state
        msg!("📊 Updated State Summary:");
        msg!("   Batch Status: {:?}", self.product_batch.status);
        msg!("   Event Type: {:?}", self.product_event.product_event_type);
        msg!("   Order Status: {:?}", self.product_event.order_status);
        msg!("   Total Events: {}", self.product_batch.total_events);
        msg!("   Timestamp: {}", clock.unix_timestamp);

        // Commit all changes back to mainnet
        msg!("💾 Committing state updates to mainnet...");
        
        commit_accounts(
            &self.signer,
            vec![
                &self.product_batch.to_account_info(),
                &self.product_event.to_account_info(),
            ],
            &self.magic_context,
            &self.magic_program,
        )?;

        msg!("✅ Supply chain state successfully updated and committed!");
        
        Ok(())
    }
}

// Event for tracking state updates
#[event]
pub struct StateUpdated {
    pub batch_id: [u8; 32],
    pub event_id: [u8; 32],
    pub updated_by: Pubkey,
    pub batch_status: ProductStatus,
    pub order_status: OrderStatus,
    pub event_type: EventType,
    pub timestamp: i64,
}