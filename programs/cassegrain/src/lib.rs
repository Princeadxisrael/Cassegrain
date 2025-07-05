use anchor_lang::prelude::*;
pub mod state;
pub use state::*;

pub mod contexts;
pub use contexts::*;
pub mod consts;
pub mod error;
// pub use contexts::*;

use ephemeral_rollups_sdk::anchor::ephemeral;
    

declare_id!("3QdPomKPoQwsFCRohMmQfsHd1LhoTN9ZrwwnsxbeFXnP");

#[ephemeral]
#[program]
pub mod cassegrain {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        product_registration_fee: u64, 
        max_events_per_product: u32, 
        max_products_per_manufacturer: u32, 
        min_event_interval: i64, 
        max_batch_size: u8,
    ) -> Result<()> {
        ctx.accounts.initialize(product_registration_fee, max_events_per_product, max_products_per_manufacturer, min_event_interval, max_batch_size, ctx.bumps)
    }

    pub fn register_manufacturer (
        ctx: Context<RegisterProfile>,
        company_name: String,
        business_type: BusinessType,
        certifications: String,
    ) -> Result<()> {
        ctx.accounts.register(company_name, business_type, certifications, ctx.bumps)

    }

    /// Register a new product
    pub fn register_product_batch(
        ctx: Context<RegisterProduct>,
        batch_id: [u8; 32],
        metadata_ipfs: Option<String>, 
        category: ProductCategory,
        batch_size: u8,
    ) -> Result<()> {
        ctx.accounts.register(
            batch_id, 
            metadata_ipfs, 
            category, 
            batch_size, 
            ctx.bumps
        )
    }

    //create event 

    pub fn create_event(
        ctx: Context<CreateEvent>,
        batch_id: [u8; 32],
        event_id: [u8; 32],
        event_type: EventType,
        metadata_ipfs: Option<String>,
        order_status: OrderStatus,
        previous_event: Option<Pubkey>,
    ) -> Result<()> {
       ctx.accounts.create_event(batch_id, event_id, event_type, metadata_ipfs, order_status, previous_event, ctx.bumps)
    }

    /// delegate event 
    /// 

    pub fn delegate_product(
        ctx: Context<DelegateProduct>,
        batch_id: [u8; 32],
        event_id: [u8; 32],
    ) -> Result<()> {
       ctx.accounts.delegate_to_rollup(batch_id, event_id)
    }

    //event log 

     pub fn event_log (
        ctx: Context<RollupEventLog>,
        batch_id: [u8; 32],
        event_id: [u8; 32],
        new_product_status: Option<ProductStatus>,
        new_order_status: Option<OrderStatus>, 
        new_event_type: Option<EventType>,
        previous_event: Option<Pubkey>,
        next_event: Option<Pubkey>,
        metadata_ipfs: Option<String>,
    ) -> Result<()> {
      ctx.accounts.update_supply_chain_state(batch_id, event_id, new_product_status, new_order_status, new_event_type, previous_event, next_event, metadata_ipfs)
    }
    
     pub fn undelegate_product(
        ctx: Context<UndelegateProduct>,
        batch_id: [u8; 32],
        event_id: [u8; 32],
    ) -> Result<()> {
       ctx.accounts.undelegate(batch_id, event_id)
    }
 
}
