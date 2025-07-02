use anchor_lang:: prelude::*;
use crate::state::*;
use crate::consts::*;

#[derive(Accounts)]
pub struct Initialize<'info>{
  #[account(mut)]
  pub authority: Signer<'info>,
  #[account(
    init,
    payer = authority,
    space = ANCHOR_DISCRIMINATOR + CassegrainConfig::INIT_SPACE,
    seeds = [CONFIG, authority.key().as_ref()],
    bump,
  )]
  pub cassegrain_config: Account<'info, CassegrainConfig>,
  pub system_program: Program<'info, System>
}

impl <'info> Initialize<'info> {
  pub fn initialize(
    &mut self,
    product_registration_fee: u64, 
    max_events_per_product: u32, 
    max_products_per_manufacturer: u32, 
    min_event_interval: i64, 
    max_batch_size: u8,
    bumps: InitializeBumps
  ) -> Result<()> {

    self.cassegrain_config.set_inner(
      CassegrainConfig { 
        authority: self.authority.key(), 
        is_paused: false, 
        product_registration_fee, 
        fee_treasury: self.authority.key(), 
        max_events_per_product, 
        max_products_per_manufacturer, 
        min_event_interval, 
        max_batch_size, 
        bump: bumps.cassegrain_config
       });

    Ok(())
  }
}