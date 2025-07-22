use anchor_lang::prelude::*;
use anchor_spl::{
  associated_token::AssociatedToken,
  token_interface::{
      Mint
    , TokenAccount
    , TokenInterface
    , CloseAccount
    , TransferChecked
    , transfer_checked
    , close_account
  },
};

use crate::Escrow;

#[derive(Accounts)]
pub struct Take<'info> {

  #[account(mut)]
  pub maker: SystemAccount<'info>, // Just need to know is an account own by the Sys program

  #[account(mut)]
  pub taker: Signer<'info>,

  #[account(mint::token_program = token_program)]
  // Token to send from maker
  pub mint_a: InterfaceAccount<'info, Mint>,

  #[account(mint::token_program = token_program)]
  // Token to receive from taker
  pub mint_b: InterfaceAccount<'info, Mint>,

  #[account(
    init_if_needed,
    payer = taker,
    associated_token::mint = mint_b,
    associated_token::authority = maker,
    associated_token::token_program = token_program,
  )]
  // To where transfer 'b' tokens from Taker
  pub maker_ata_b: InterfaceAccount<'info, TokenAccount>,

  #[account(
    init_if_needed,
    payer = taker,
    associated_token::mint = mint_a,
    associated_token::authority = taker,
    associated_token::token_program = token_program,
  )]
  // To where transfer 'a' tokens from Maker (vault)
  pub taker_ata_a: InterfaceAccount<'info, TokenAccount>,

  #[account(
    mut,
    associated_token::mint = mint_b,
    associated_token::authority = taker,
    associated_token::token_program = token_program,
  )]
  // From where to transfer 'b' tokens to Maker
  pub taker_ata_b: InterfaceAccount<'info, TokenAccount>,

  #[account(
    mut,
    associated_token::mint = mint_a,
    associated_token::authority = escrow,
    associated_token::token_program = token_program,
  )]
  // From where to transfer 'a' tokens to Taker
  pub vault: InterfaceAccount<'info, TokenAccount>,

  #[account(
    mut,
    close = maker,
    has_one = maker,
    has_one = mint_a,
    has_one = mint_b,
    seeds = [
      b"escrow",
      escrow.maker.key().as_ref(), // make sure is not any random maker
      escrow.seed.to_le_bytes().as_ref()
    ],
    bump = escrow.bump
  )]
  pub escrow: Account<'info, Escrow>,

  pub associated_token_program: Program<'info, AssociatedToken>,
  pub token_program: Interface<'info, TokenInterface>,
  pub system_program: Program<'info, System>,
}

impl<'info> Take<'info> {

  // Transfer 'b' tokens from Taker to Maker
  pub fn transfer_b_to_maker(&mut self) -> Result<()> {
    
    let transfer_accounts = TransferChecked {
        from: self.taker_ata_b.to_account_info(),
        mint: self.mint_b.to_account_info(),
        to: self.maker_ata_b.to_account_info(),
        authority: self.taker.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), transfer_accounts);

    transfer_checked(cpi_ctx, self.escrow.receive, self.mint_b.decimals)
  }

  // Transfer 'a' tokens from Maker (vault) to Taker
  pub fn transfer_a_to_taker(&mut self) -> Result<()> {
    
    let signer_seeds: [&[&[u8]]; 1] = [&[
      b"escrow",
      self.maker.to_account_info().key.as_ref(),
      &self.escrow.seed.to_le_bytes()[..],
      &[self.escrow.bump],
    ]];

    let accounts = TransferChecked {
      from: self.vault.to_account_info(),
      mint: self.mint_a.to_account_info(),
      to: self.taker_ata_a.to_account_info(),
      authority: self.escrow.to_account_info(),
    };

    let ctx = CpiContext::new_with_signer(
      self.token_program.to_account_info(),
      accounts,
      &signer_seeds,
    );

    transfer_checked(ctx, self.vault.amount, self.mint_a.decimals)?;

    // Make the token program close the vault (token account)
    let accounts = CloseAccount {
      account: self.vault.to_account_info(),
      destination: self.maker.to_account_info(), // To where the rent will go
      authority: self.escrow.to_account_info(),
    };

    let ctx = CpiContext::new_with_signer(
      self.token_program.to_account_info(),
      accounts,
      &signer_seeds,
    );

    close_account(ctx)
  }
}
