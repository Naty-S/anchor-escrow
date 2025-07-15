#![allow(unexpected_cfgs, deprecated)]
use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

pub use instructions::*;

declare_id!("CVVB7FwsvjpqsdvT1hNuhgWMYgQ3g4SQkBUzDU4tdiKf");

#[program]
pub mod anchor_escrow {
    use super::*;

    pub fn make(
          ctx: Context<Make>
        , seed: u64
        , receive: u64
        , deposit: u64
    ) -> Result<()> {
        ctx.accounts.init_escrow(seed, receive, &ctx.bumps)?;
        ctx.accounts.deposit(deposit)
    }
}
