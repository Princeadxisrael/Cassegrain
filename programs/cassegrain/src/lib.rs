use anchor_lang::prelude::*;
pub mod state;
pub use state::*;

pub mod contexts;
pub use contexts::*;
pub mod consts;
pub mod error;
// pub use contexts::*;
    

declare_id!("3QdPomKPoQwsFCRohMmQfsHd1LhoTN9ZrwwnsxbeFXnP");

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
    pub fn register_product(
        ctx: Context<RegisterProduct>,
        product_id: [u8; 32], 
        metadata_ipfs: String, 
        qr_code_hash: String,
        category: ProductCategory,
        batch_size: u8,
    ) -> Result<()> {
        ctx.accounts.register(
            product_id, 
            metadata_ipfs, 
            qr_code_hash, 
            category, 
            batch_size, 
            ctx.bumps
        )
    }

  

   
}
