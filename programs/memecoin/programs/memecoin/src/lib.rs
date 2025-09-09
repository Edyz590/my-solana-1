use anchor_lang::prelude::*;

declare_id!("YourProgramPublicKeyWillGoHere");

#[program]
pub mod memecoin {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Memecoin initialized!");
        Ok(())
    }

    pub fn whitelist_wallet(ctx: Context<Whitelist>, wallet: Pubkey) -> Result<()> {
        msg!("Wallet whitelisted: {}", wallet);
        Ok(())
    }

    pub fn block_sell(ctx: Context<BlockSell>, wallet: Pubkey) -> Result<()> {
        msg!("Sell blocked for: {}", wallet);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct Whitelist {}

#[derive(Accounts)]
pub struct BlockSell {}
