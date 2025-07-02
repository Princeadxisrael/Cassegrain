use anchor_lang::prelude::*;
use crate::state::*;
use crate::consts::*;
use crate::error::*;


#[derive(Accounts)]
pub struct RegisterProduct<'info> {
  #[account(mut)]
  pub signer : Signer<'info>,
  /// CHECK : The authority of this program
  pub authority: UncheckedAccount<'info>,
  #[account(
    init,
    payer = signer,
    space =  ANCHOR_DISCRIMINATOR + Product::INIT_SPACE ,
    seeds = [PRODUCT, signer.key().as_ref()],
    bump
  )] 
  pub product : Account<'info, Product>,
  #[account(
    mut,
    seeds = [CONFIG, authority.key().as_ref()],
    bump,
    constraint = cassegrain_config.authority == authority.key() @CassegrainError::Unauthorized,
    constraint = cassegrain_config.is_paused == false @CassegrainError::ProgramPaused  
  )]
  pub cassegrain_config: Account<'info, CassegrainConfig>,

  #[account(
    mut,
    seeds = [MANUFACTURER, signer.key().as_ref()],
    bump,
    constraint = manufacturer.owner == signer.key() @ CassegrainError::Unauthorized,
    constraint = manufacturer.is_verified == true @ CassegrainError::ManufacturerNotVerified // must verify manufacturer before registering products
  )]
  pub manufacturer: Account<'info, ManufacturerProfile>,

  pub system_program: Program<'info, System>,

}

impl <'info> RegisterProduct<'info> {
  pub fn register(
    &mut self,
    product_id: [u8; 32], 
    metadata_ipfs: String, 
    qr_code_hash: String,
    category: ProductCategory,
    batch_size: u8,
    bumps: RegisterProductBumps
  ) -> Result<()> {

 // we can do some checks here

    let clock = Clock::get()?;

    self.product.set_inner(
      Product { 
        product_id, 
        manufacturer_name: self.manufacturer.company_name.clone(), 
        status: ProductStatus::Registered, 
        created_at: clock.unix_timestamp, 
        last_updated: clock.unix_timestamp, 
        metadata_ipfs, 
        qr_code_hash, 
        authenticity_verified: false, 
        category, 
        manufacturer: self.manufacturer.key(), 
        current_owner: Some(self.manufacturer.key()), 
        total_events: 0, 
        batch_size, 
        bump: bumps.product,
      });

    Ok(())
  }
}