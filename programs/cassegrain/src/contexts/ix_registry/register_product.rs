use anchor_lang::prelude::*;
use crate::state::*;
use crate::error::*;
use crate::consts::*;

#[derive(Accounts)]
#[instruction(batch_id: [u8; 32])]
pub struct RegisterProduct<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    
    /// CHECK: The authority of this program
    pub authority: UncheckedAccount<'info>,
    
    #[account(
        init_if_needed,
        payer = signer,
        space = ANCHOR_DISCRIMINATOR + ProductBatch::INIT_SPACE,
        seeds = [BATCH, batch_id.as_ref()],
        bump,
    )]
    pub product_batch: Account<'info, ProductBatch>,
    
    #[account(
        mut,
        seeds = [CONFIG, authority.key().as_ref()],
        bump,
        constraint = cassegrain_config.authority == authority.key() 
            @ CassegrainError::Unauthorized,
        constraint = !cassegrain_config.is_paused 
            @ CassegrainError::ProgramPaused,
    )]
    pub cassegrain_config: Account<'info, CassegrainConfig>,

    #[account(
        mut,
        seeds = [MANUFACTURER, signer.key().as_ref()],
        bump,
        constraint = manufacturer.owner == signer.key() 
            @ CassegrainError::Unauthorized,
        constraint = manufacturer.is_verified 
            @ CassegrainError::ManufacturerNotVerified,
    )]
    pub manufacturer: Account<'info, ManufacturerProfile>,

    pub system_program: Program<'info, System>,
}

impl<'info> RegisterProduct<'info> {
    pub fn register(
        &mut self,
        batch_id: [u8; 32],
        metadata_ipfs: Option<String>,
        category: ProductCategory,
        batch_size: u8,
        bumps: RegisterProductBumps,
    ) -> Result<()> {
        let clock = Clock::get()?;
        let config = &self.cassegrain_config;
        
        // Validation checks
        require!(
            batch_size > 0 && batch_size <= config.max_batch_size,
            CassegrainError::InvalidBatchSize
        );
        

        
      
        if self.product_batch.batch_size == 0 {
            self.product_batch.set_inner(ProductBatch {
                batch_id,
                manufacturer_name: self.manufacturer.company_name.clone(),
                status: ProductStatus::Created,
                created_at: clock.unix_timestamp,
                last_updated: clock.unix_timestamp,
                metadata_ipfs: metadata_ipfs.clone(),
                authenticity_verified: false,
                category,
                event_account: None,
                manufacturer: self.manufacturer.owner,
                total_events: 0,
                batch_size,
                bump: bumps.product_batch,
            });
        }
        Ok(())
    }
}