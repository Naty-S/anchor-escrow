// Maker: creates the deal for the exchage
use anchor_lang::prelude::*;
use anchor_spl::{
  associated_token::AssociatedToken,
  token_interface::{
      Mint
    , TokenAccount
    , TokenInterface
    , TransferChecked
    , transfer_checked
  }
};

use crate::state::Escrow;


#[derive(Accounts)]
#[instruction(seed: u64)]
// Make context
pub struct Make<'info> {
  
  #[account(mut)]
  // Creates the deal (starts the escrow)
  pub maker: Signer<'info>,

  #[account(mint::token_program = token_program)]
  // Token to send
  pub mint_a: InterfaceAccount<'info, Mint>,

  #[account(mint::token_program = token_program)]
  // Token to receive
  pub mint_b: InterfaceAccount<'info, Mint>,

  #[account(
    // no init, 'cuz since the maker is depositing 'a' tokens means already has the ATA for it
    mut,
    associated_token::mint = mint_a,
    associated_token::authority = maker,
    associated_token::token_program = token_program
  )]
  pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,

  #[account(
    init,
    payer = maker,
    associated_token::mint = mint_a, // tokens maker sends
    associated_token::authority = escrow,
    associated_token::token_program = token_program // owner. Allows the escrow to sign
  )]
  // Holds the tokens to trade, the ones the maker sends
  pub vault: InterfaceAccount<'info, TokenAccount>,

  #[account(
    init,
    payer = maker,
    seeds = [
      b"escrow",
      maker.key().as_ref(),
      seed.to_le_bytes().as_ref() // Allows create multiple escrows
    ],
    space = 8 + Escrow::INIT_SPACE,
    bump
  )]
  pub escrow: Account<'info, Escrow>,

  pub associated_token_program: Program<'info, AssociatedToken>,
  pub token_program: Interface<'info, TokenInterface>,
  pub system_program: Program<'info, System>
}


impl<'info> Make<'info> {
  
  // Initialize escrow
  pub fn init_escrow(
    &mut self,
    seed: u64,
    receive: u64,
    bumps: &MakeBumps
  ) -> Result<()> {

    self.escrow.set_inner(
      Escrow { 
        seed, 
        maker: self.maker.key(), 
        mint_a: self.mint_a.key(), 
        mint_b: self.mint_b.key(), 
        receive, 
        bump: bumps.escrow 
      }
    );

    Ok(())
  }
  
  // Deposit 'a' tokens to vault
  pub fn deposit(&mut self, deposit: u64) -> Result<()> {

    let transfer_accounts = TransferChecked {
      from: self.maker_ata_a.to_account_info(),
      mint: self.mint_a.to_account_info(),
      to: self.vault.to_account_info(),
      authority: self.maker.to_account_info()
    };

    let cpi_ctx = CpiContext::new(
      self.token_program.to_account_info(),
      transfer_accounts
    );

    transfer_checked(cpi_ctx, deposit, self.mint_a.decimals)
  }
}
