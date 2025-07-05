use anchor_lang::prelude::*;
use crate::state::*;

#[account]
#[derive(InitSpace)]
pub struct ProductEvent {
    pub event_id: [u8; 32],           
    pub batch_id: [u8; 32],          
    pub product_event_type: EventType,     
    pub actor: Pubkey,              
    pub timestamp: i64,       
    #[max_len(32)]
    pub metadata_ipfs: Option<String>, 
    pub verification_status: VerificationStatus,
    pub order_status: OrderStatus, 
    pub previous_event: Option<Pubkey>, 
    pub next_event: Option<Pubkey>, 
    pub bumps: u8    
}
