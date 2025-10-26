use anchor_lang::prelude::*;
use crate::constants::*;
use ephemeral_rollups_sdk::anchor::delegate;
use ephemeral_rollups_sdk::cpi::DelegateConfig;
use session_keys::{SessionToken, Session};

#[delegate]
#[derive(Accounts, Session)]
#[instruction(world_id: [u8; 32])]
pub struct WorldDelegate<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    /// CHECK: The undead world account we are delegating
    #[account(
        mut,
        del,
        seeds = [UNDEAD_WORLD, world_id.as_ref()],
        bump,
    )]
    pub undead_world: AccountInfo<'info>,

    #[session(
        signer = signer,
        authority = signer.key() 
    )]
    pub session_token: Option<Account<'info, SessionToken>>,
}

impl<'info> WorldDelegate<'info> {
    pub fn world_to_rollup(
        &mut self,
        world_id: [u8; 32],
    ) -> Result<()> {
        self.delegate_undead_world(
            &self.signer, 
            &[UNDEAD_WORLD, world_id.as_ref()],
            DelegateConfig::default()
        )?;
        
        Ok(())
    }
}