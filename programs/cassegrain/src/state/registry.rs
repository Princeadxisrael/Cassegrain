use anchor_lang::prelude::*;
use crate::state::{ProductCategory, ProductStatus, BusinessType};


#[account]
#[derive(InitSpace)]
pub struct ProductBatch{
    pub batch_id: [u8; 32],
    #[max_len(32)]
    pub manufacturer_name: String,  
    pub status: ProductStatus,   
    pub created_at: i64,             
    pub last_updated: i64, 
    #[max_len(32)]         
    pub metadata_ipfs: Option<String>,
    pub authenticity_verified: bool,  
    pub category: ProductCategory,    
    pub manufacturer: Pubkey,
    pub event_account: Option<Pubkey>,
    pub total_events: u32,
    pub batch_size: u8,
    pub bump: u8,
}

// // redundant for first batch mvp, will be usefull later
// #[account]
// #[derive(InitSpace)]
// pub struct Product {
//     pub product_id: [u8; 32],  
//     pub batch_id: [u8; 32], 
//     pub created_at: i64,             
//     pub last_updated: i64, 
//     #[max_len(32)]         
//     pub metadata_ipfs: Option<String>,
//     pub current_owner: Option<Pubkey>,        
//     pub bump: u8,
// }


#[account]
#[derive(InitSpace)]
pub struct ManufacturerProfile {
    #[max_len(32)]
    pub company_name: String,
    pub business_type: BusinessType,
    pub owner: Pubkey,
    #[max_len(32)]
    pub certifications: String,
    pub is_verified: bool,
    pub bump: u8,
}
