use anchor_lang::prelude::*;



#[account]
#[drive(InitSpace)]
pub struct UserAccount {
  pub points: u32,
  pub amount_staked: u8,
  pub bump: u8,
}