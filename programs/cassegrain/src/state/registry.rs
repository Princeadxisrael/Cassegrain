use anchor_lang::prelude::*;
use crate::state::{ProductCategory, ProductStatus, BusinessType};

//I am thinking of batch situations -- discuss 
#[account]
#[derive(InitSpace)]
pub struct Product {
    pub product_id: [u8; 32],  
    #[max_len(32)]
    pub manufacturer_name: String,  
    #[max_len(32)]  
    // pub current_location: Option<String>,     
    pub status: ProductStatus,       
    pub created_at: i64,             
    pub last_updated: i64, 
    #[max_len(32)]         
    pub metadata_ipfs: String,       // IPFS hash for detailed metadata
   #[max_len(128)]
    pub qr_code_hash: String,        // For consumer verification
    pub authenticity_verified: bool,  
    pub category: ProductCategory,    
    pub manufacturer: Pubkey,
    pub current_owner: Option<Pubkey>,        
    pub total_events: u32,
    pub batch_size: u8, // Number of items in this batch
    pub bump: u8, // Bump seed for PDA


}


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
    pub bump: u8, // Bump seed for PDA
}
