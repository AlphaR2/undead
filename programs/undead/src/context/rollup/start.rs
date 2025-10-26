use anchor_lang::prelude::*;
use crate::{state::*, constants::*, error::*, events::*};
use ephemeral_rollups_sdk::anchor::commit;
use ephemeral_rollups_sdk::ephem::commit_accounts;
use session_keys::{SessionToken, Session};

#[commit]
#[derive(Accounts, Session)]
#[instruction(player: Pubkey, chapter_number: u16, world_id: [u8; 32])]
pub struct StartChapter<'info> {
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

    #[account(
        mut,
        seeds = [UNDEAD_WORLD, world_id.as_ref()],
        bump = undead_world.bump,
    )]
    pub undead_world: Account<'info, UndeadWorld>,

  #[session(
      signer = signer,
      authority = player.key() 
    )]
    pub session_token: Option<Account<'info, SessionToken>>,
}

impl<'info> StartChapter<'info> {
    pub fn start_chapter(
        &mut self,
        player: Pubkey,
        chapter_number: u16,
        world_id: [u8; 32],
    ) -> Result<()> {
        let profile = &mut self.gamer_profile;
        let world = &mut self.undead_world;

      
        // Set current chapter
        profile.current_chapter = chapter_number;
        profile.current_position = 0;

        // Increment active players
        world.active_players = world.active_players.saturating_add(1);
        
        msg!("Player {} started Chapter {}", player, chapter_number);

        // Commit changes to rollup
        commit_accounts(
            &self.signer,
            vec![
                &self.gamer_profile.to_account_info(),
                &self.undead_world.to_account_info(),
            ],
            &self.magic_context,
            &self.magic_program,
        )?;

        Ok(())
    }
}