use anchor_lang::prelude::*;
use crate::state::{EventType, VerificationStatus, OrderStatus};

#[account]
#[derive(InitSpace)]
pub struct SupplyChainEvent {
    pub event_id: [u8; 32],           // Unique event identifier
    pub product_id: [u8; 32],          // Links to product
    pub event_type: EventType,        // From your enum
    pub actor: Pubkey,               // Who performed the action
    pub timestamp: i64,              // time of action
    // pub location: Option<Location>,   // From your struct
    #[max_len(32)]
    pub metadata_ipfs: String,       // Detailed event data on IPFS
    pub verification_status: VerificationStatus, // From your enum
    pub order_status: OrderStatus,    // From your enum  
    pub previous_event: Option<Pubkey>, // Chain events together
    pub next_event: Option<Pubkey>,     // Bidirectional linking
    // pub real_time_data: Option<RealTimeData>, // IoT sensor data
}