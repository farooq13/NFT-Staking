#![allow(unexpect_cfgs, decprecated)]
use anchor_lang::prelude::*;

mod state;
mod instructions;



declare_id!("AbhNMQwcSVSTycHrytJR1jePm6C2avH58h2YR77cm1w3");

#[program]
pub mod nft_staking {
    use super::*;

    pub fn initialize(ctx: Context<InitializeConfig>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}


