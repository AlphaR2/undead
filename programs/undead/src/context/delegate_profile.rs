use anchor_lang::prelude::*;
use crate::constants::*;
use ephemeral_rollups_sdk::anchor::delegate;
use ephemeral_rollups_sdk::cpi::DelegateConfig;


#[delegate]
#[derive(Accounts)]
#[instruction(player: Pubkey)]
pub struct GamingDelegate<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    /// CHECK: The gaming profile account we are delegating
    #[account(
        mut,
        del,
        seeds = [USER_GAME_PROFILE, player.key().as_ref()],
        bump,
    )]
    pub user_game_profile: AccountInfo<'info>,
}

impl<'info> GamingDelegate<'info> {
    pub fn game_profile_to_rollup(
        &mut self,
        player: Pubkey,
    ) -> Result<()> {
        self.delegate_user_game_profile(
            &self.signer, 
            &[USER_GAME_PROFILE, player.key().as_ref()],
            DelegateConfig::default()
        )?;
        
        Ok(())
    }
}