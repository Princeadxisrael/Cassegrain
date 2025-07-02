use anchor_lang::prelude::*;
use crate::state::*;
use crate::consts::*;
use crate::error::*;

#[derive(Accounts)]
pub struct RegisterProfile<'info> {
  #[account(mut)]
  pub signer : Signer<'info>,

  /// CHECK : The authority of this program
  pub authority: UncheckedAccount<'info>,
   #[account(
    mut,
    seeds = [CONFIG, authority.key().as_ref()],
    bump,
    constraint = cassegrain_config.authority == authority.key() @CassegrainError::Unauthorized,
    constraint = !cassegrain_config.is_paused @CassegrainError::ProgramPaused  
  )]
  pub cassegrain_config: Account<'info, CassegrainConfig>,

  #[account(
    init,
    payer = signer,
    space = ANCHOR_DISCRIMINATOR + ManufacturerProfile::INIT_SPACE,
    seeds = [MANUFACTURER, signer.key().as_ref()],
    bump
  )]
  pub manufacturer: Account<'info, ManufacturerProfile>,

  pub system_program: Program<'info, System>,
}

impl <'info> RegisterProfile<'info> {
  pub fn register(
    &mut self,
    company_name: String,
    business_type: BusinessType,
    certifications: String,
    bumps: RegisterProfileBumps
  ) -> Result<()> {

    self.manufacturer.set_inner(
      ManufacturerProfile { 
        company_name, 
        business_type, 
        owner: self.signer.key(),
        certifications, 
        is_verified: false, 
        bump: bumps.manufacturer
      });

    Ok(())
  }
}