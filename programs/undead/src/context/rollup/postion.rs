use anchor_lang::prelude::*;
use crate::{state::*, constants::*};
use ephemeral_rollups_sdk::anchor::commit;
use ephemeral_rollups_sdk::ephem::commit_accounts;


#[commit]
#[derive(Accounts)]
pub struct UpdatePosition<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    /// CHECK: Player account
    pub player: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [USER_GAME_PROFILE, player.key().as_ref()],
        bump = gamer_profile.bump,
    )]
    pub gamer_profile: Account<'info, GamerProfile>,
}

impl<'info> UpdatePosition<'info> {
    pub fn update_position(
        &mut self,
        position: u32,
    ) -> Result<()> {
        let profile = &mut self.gamer_profile;

        // Update current position only
        profile.current_position = position;

        // Commit to rollup
        commit_accounts(
            &self.signer,
            vec![&self.gamer_profile.to_account_info()],
            &self.magic_context,
            &self.magic_program,
        )?;

        Ok(())
    }
}