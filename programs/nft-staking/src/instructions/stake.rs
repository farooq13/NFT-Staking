use anchor_lang::prelude::*;
use anchor_spl::{
  metadata::{
    npl_token_metadata::intructions::{
      FreezeDelegatedAccountCpi,
      FreezeDelegatedAccountCpiAccounts,
    },
    MasterEditAccount,
    Metadata,
    MetadataAccount,
  }, 
  token::{
    approve,
    Approve,
    Mint,
    Token,
    TokenAccount
  },
};

use crate::state::{StakeAccount, StakeConfig, UserAccount};


#[derive(Accounts)]
pub struct Stake<'info> {
  #[account(mut)]
  pub user: Signer<'info>,
  pub mint: Account<'info, Mint>,
  pub collection_mint: Account<'info, Mint>,

  #[account(
    mut,
    associate_token::mint = mint,
    associate_token::authority = user,
  )]
  pub mint_ata: Account<'info, TokenAccount>,

  #[account(
    seeds = [b"metadata", metadata_program.key().as_ref(), mint.key().as_ref()],
    seeds::program = metadata_program.key(),
    bump,
    constraint = metadata.colletion.as_ref().unwrap() == collection_mint.key().as_ref(),
    constraint = metadata.colletion.as_ref().unwrap().varified == true,
  )]
  pub metadata: Account<'info, MetaAccount>,

  #[account(
    seeds = [b"metadata", metadata_program.key().as_ref(), mint.key().as_ref(), b"edition"],
    seeds::program = metadata_program.key(),
    bump,
  )]
  pub edition: Account<'info, MasterEditAccount>,

  #[account(
    seeds = [b"config".as_ref()],
    bump,
  )]
  pub config: Account<'info, StakeConfig>,

  #[account(
    init,
    payer = user,
    space = 8 + StakeAccount::INIT_SPACE,
    seeds = [b"stake".as_ref(), mint.key().as_ref(), config.key().as_ref()],
    bump,
  )]
  pub stake_account: Account<'info, StakeAccount>

  #[account(
    mut,
    seeds = [b"user".as_ref(), user.key().as_ref()],
    bump = user_account.bump,
  )]
  pub user_account: Account<'info, UserAccount>,

  pub system_program: Program<'info, System>,
  pub token_program: Program<'info, Token>

  pub metadata_program: Program<'info, Metadata>,
}

impl<'info> Stake<'info> {
  pub fn stake(&mut self, bump: &StakeBumps) -> Result<()> {
    assert!(self.user_account.amount_staked < self.config.max_stake);

    self.stake_account.set_inner(StakeAccount {
      owner: self.user.key(),
      mint: self.mint.key(),
      staked_at: Clock::get()?.unix_timestamp,
      bump: bumps.stake_account,
    });

    let cpi_program = self.token_program.to_account_info(),

    let cpi_accounts = Approve {
      to: self.mint_ata.to_account_info(),
      delegate: self.stake_account.to_account_info(),
      authority: self.user.to_account_info(),
    };

    let cpi_context = CpiContext::new(cpi_program, cpi_accounts);

    approve(cpi_context)?;

    let seeds = &[
      b"stake",
      self.mint.to_account_info().key.as_ref(),
      self.config.to_account_info().key.as_ref(),
      &[self.stake_account.bump]
    ];

    let siger_seeds = &[&seeds[..]];

    let delegate = &self.stake_account.to_account_info();
    let token_account = &self.mint_ata.to_account_info();
    let edition = &self.editon.to_account_info();
    let mint = &self.mint.to_account_info();
    let token_program = &self.token_program.to_account_info();
    let metadata_program = &self.metadata_program.to_account_info();

    FreezeDelegatedAccountCpi::new(
      metadata_program,
      FreezeDelegatedAccountCpiAccounts {
        delegate,
        token_account,
        edition,
        mint,
        token_program,
      },
    ).invoke_signed(signer_seeds)?;

    self.user_account.amount_staked += 1;

    Ok(())
  }
}