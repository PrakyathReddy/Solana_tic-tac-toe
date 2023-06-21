use anchor_lang::prelude::*;

declare_id!("BwAT2NVQuxS4wuvzSd4MjPUbxMZm4yv791C7E62yYJUp");

#[program]
pub mod tic_tac_toe_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
