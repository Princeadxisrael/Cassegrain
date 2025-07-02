use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Copy, PartialEq)]
pub enum ProductStatus {
    Registered,
    Created,
    Manufactured,
    InTransit,
    InWarehouse,
    ForSale,
    Sold,
    Delivered,
    Recalled,
    Destroyed,
}

impl Space for ProductStatus {
    const INIT_SPACE: usize = 1; 
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Copy, PartialEq)]
pub enum ProductCategory {
    Electronics,
    Automotive,
    Pharmaceuticals,
    Food,
    Textiles,
    Luxury,
    Industrial,
    Other,
}

impl Space for ProductCategory {
    const INIT_SPACE: usize = 1; 
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Copy, PartialEq)]
pub enum EventType {
    Manufactured,
    QualityCheck,
    Packaged,
    Shipped,
    InTransit,
    Delivered,
    Sold,
    Recalled,
    QualityFailed,
    OwnershipTransfer,
    LocationUpdate,
    CustomsCleared,
}

impl Space for EventType {
    const INIT_SPACE: usize = 1; 
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Copy, PartialEq)]
pub enum VerificationStatus {
    Pending,
    Verified,
    Failed,
    Disputed,
}

impl Space for VerificationStatus {
    const INIT_SPACE: usize = 1; 
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Copy, PartialEq)]
pub enum OrderStatus {
    Pending,
    Confirmed,
    Processing,
    Shipped,
    InTransit,
    Delivered,
    Completed,
    Cancelled,
    Disputed,
    Refunded,
}

impl Space for OrderStatus {
    const INIT_SPACE: usize = 1; 
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Copy, PartialEq)]
pub enum PaymentStatus {
    Pending,
    Authorized,
    Captured,
    InEscrow,
    Released,
    Refunded,
    Failed,
}

impl Space for PaymentStatus {
    const INIT_SPACE: usize = 1; 
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Copy, PartialEq)]
pub enum VerificationMethod {
    QRCode,
    NFCTag,
    Manual,
    API,
    Batch,
}

impl Space for VerificationMethod {
    const INIT_SPACE: usize = 1;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Copy, PartialEq)]
pub enum CertificationLevel {
    Basic,
    Standard,
    Premium,
    Enterprise,
}

impl Space for CertificationLevel {
    const INIT_SPACE: usize = 1;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Copy, PartialEq)]
pub enum BusinessType {
    Manufacturer,
    Distributor,
    Retailer,
    LogisticsProvider,
    QualityInspector,
    Consumer,
}

impl Space for BusinessType {
    const INIT_SPACE: usize = 1; 
}



