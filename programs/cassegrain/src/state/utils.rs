use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct CassegrainConfig {
    pub authority: Pubkey,
    pub is_paused: bool,
    pub product_registration_fee: u64,
    pub fee_treasury: Pubkey,
    pub max_events_per_product: u32,
    pub max_products_per_manufacturer: u32,
    /// Minimum time between events (seconds) - For spam protection
    pub min_event_interval: i64,
    pub max_batch_size: u8,
    pub bump: u8, // Bump seed for PDA
}


#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct Location {
    pub latitude: f64,        
    pub longitude: f64,       
    #[max_len(32)]
    pub address: String,         
    #[max_len(32)]
    pub facility_id: String,    
}


#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct ShippingAddress {
    #[max_len(64)]
    pub recipient_name: String,  
    #[max_len(128)]
    pub street_address: String,   
    #[max_len(32)]
    pub city: String,            
    #[max_len(32)]
    pub state: String,           
    #[max_len(16)]
    pub postal_code: Option<String>,     
    #[max_len(32)]
    pub country: String,           
}


