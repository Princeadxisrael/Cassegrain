use anchor_lang::prelude::*;
use crate::consts::*;
use crate::state::*;
use crate::error::*;

#[derive(Accounts)]
#[instruction(batch_id: [u8; 32], event_id: [u8; 32])]
pub struct CreateEvent<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    /// CHECK: The authority of this program  
    pub authority: UncheckedAccount<'info>,

    #[account(
        init,
        payer = signer,
        space = ANCHOR_DISCRIMINATOR + ProductEvent::INIT_SPACE,
        seeds = [EVENT, event_id.as_ref()],
        bump,
    )]
    pub events: Account<'info, ProductEvent>,

    #[account(
        seeds = [BATCH, batch_id.as_ref()],
        bump,
        constraint = product_batch.manufacturer == signer.key() 
            @ CassegrainError::Unauthorized,
    )]
    pub product_batch: Account<'info, ProductBatch>,

    #[account(
        seeds = [CONFIG, authority.key().as_ref()],
        bump,
        constraint = cassegrain_config.authority == authority.key() 
            @ CassegrainError::Unauthorized,
        constraint = !cassegrain_config.is_paused 
            @ CassegrainError::ProgramPaused,
    )]
    pub cassegrain_config: Account<'info, CassegrainConfig>,

    #[account(
        seeds = [MANUFACTURER, signer.key().as_ref()],
        bump,
        constraint = manufacturer.owner == signer.key() 
            @ CassegrainError::Unauthorized,
    )]
    pub manufacturer: Account<'info, ManufacturerProfile>,

    pub system_program: Program<'info, System>,
}

impl<'info> CreateEvent<'info> {
    pub fn create_event(
        &mut self,
        batch_id: [u8; 32],
        event_id: [u8; 32],
        event_type: EventType,
        metadata_ipfs: Option<String>,
        order_status: OrderStatus,
        previous_event: Option<Pubkey>,
        bumps: CreateEventBumps,
    ) -> Result<()> {
        let clock = Clock::get()?;
        let config = &self.cassegrain_config;
        
        // Basic validation checks only
        if let Some(ref ipfs) = metadata_ipfs {
            require!(
                ipfs.len() <= 32, 
                CassegrainError::InvalidIPFSHash
            );
        }

        // Rate limiting check to prevent spam
        let time_since_last_event = clock.unix_timestamp - self.product_batch.last_updated;
        require!(
            time_since_last_event >= config.min_event_interval,
            CassegrainError::EventTooFrequent
        );


        // Create the event ONLY - no batch updates
        self.events.set_inner(ProductEvent {
            event_id,
            batch_id,
            product_event_type: event_type,
            actor: self.signer.key(),
            timestamp: clock.unix_timestamp,
            metadata_ipfs,
            verification_status: VerificationStatus::Pending,
            order_status,
            previous_event,
            next_event: None,
            bumps: bumps.product_batch
        });

        // Emit event for off-chain tracking
        emit!(EventCreated {
            event_id,
            batch_id,
            event_type,
            actor: self.signer.key(),
            timestamp: clock.unix_timestamp,
        });

        Ok(())
    }

  }


// Event emission for pure event creation
#[event]
pub struct EventCreated {
    pub event_id: [u8; 32],
    pub batch_id: [u8; 32],
    pub event_type: EventType,
    pub actor: Pubkey,
    pub timestamp: i64,
}