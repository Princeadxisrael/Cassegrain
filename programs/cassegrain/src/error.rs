use anchor_lang::prelude::*;

#[error_code]

pub enum CassegrainError {
    #[msg("Unauthorized action")]
    Unauthorized,
    #[msg("Program is paused")]
    ProgramPaused,

    #[msg("Product not found")]
    ProductNotFound,

    #[msg("Manufacturer not found")]
    ManufacturerNotFound,

    #[msg("Invalid product category")]
    InvalidProductCategory,

    #[msg("Invalid product status")]
    InvalidProductStatus,

    #[msg("Invalid business type")]
    InvalidBusinessType,

    #[msg("Insufficient funds for registration fee")]
    InsufficientRegistrationFee,

    #[msg("Event limit exceeded for this product")]
    EventLimitExceeded,

    #[msg("Product already exists")]
    ProductAlreadyExists,

    #[msg("Manufacturer profile already exists")]
    ManufacturerProfileExists,

    #[msg("Invalid shipping address format")]
    InvalidShippingAddressFormat,

    #[msg("Location data is invalid or incomplete")]
    InvalidLocationData,
    #[msg("Manufacturer not verified")]
    ManufacturerNotVerified,
    
    #[msg("Invalid batch size")]
    InvalidBatchSize,
    
    #[msg("Invalid IPFS hash")]
    InvalidIPFSHash,
    
    #[msg("Batch not found")]
    BatchNotFound,
      #[msg("Events logged too frequently")]
    EventTooFrequent,
    
    #[msg("Unauthorized to perform quality checks")]
    UnauthorizedQualityCheck,
    
    #[msg("Unauthorized to log customs events")]
    UnauthorizedCustomsEvent,
    
    #[msg("Event already exists")]
    EventAlreadyExists,
    
    #[msg("Invalid batch ID")]
    InvalidBatchId,
    
    #[msg("Invalid event ID")]
    InvalidEventId,
}