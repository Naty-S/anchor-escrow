use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Escrow {
  
  pub seed: u64, // Allows the maker to set up (create) multiple escrows
  pub maker: Pubkey,

  // Make sure the scrow goes between 'a' and 'b' token
  pub mint_a: Pubkey,
  pub mint_b: Pubkey,
  
  pub receive: u64, // ammount of tokens that maker requires from taker. how much maker wants to receive
  pub bump: u8 // Always store for signatures when signing
}
