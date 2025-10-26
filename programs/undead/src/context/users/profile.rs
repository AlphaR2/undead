use anchor_lang::prelude::*;
use crate::{state::*, constants::*};
use session_keys::{SessionToken, Session};

/*Gaming Profile that is delegated to the ER for Updates */
#[derive(Accounts, Session)]
pub struct BuildGamingProfile<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    /// CHECK: This is verified and is player account 
    pub player: AccountInfo<'info>,

    #[account(
        init_if_needed,
        payer = signer,
        space = ANCHOR_DISCRIMINATOR + GamerProfile::INIT_SPACE,
        seeds = [USER_GAME_PROFILE, player.key().as_ref()],
        bump
    )]
    pub gamer_profile: Account<'info, GamerProfile>,
    
    #[session(
        signer = signer,
        authority = player.key()
    )]
    pub session_token: Option<Account<'info, SessionToken>>,
    pub system_program: Program<'info, System>,
}

impl<'info> BuildGamingProfile<'info> {
    pub fn build_gaming_profile(
        &mut self,
        character_class: WarriorClass,
        bumps: &BuildGamingProfileBumps
    ) -> Result<()> {
        let gamer_profile = &mut self.gamer_profile;
        let is_new_account = gamer_profile.owner == Pubkey::default();
        
        if is_new_account {
            gamer_profile.owner = self.player.key();
            gamer_profile.character_class = character_class;
            gamer_profile.current_chapter = 0;
            gamer_profile.chapters_completed = 0;
            gamer_profile.total_distance = 0;
            gamer_profile.current_position = 0;
            gamer_profile.total_playtime = 0;
            gamer_profile.total_battles_won = 0;
            gamer_profile.total_battles_lost = 0;
            gamer_profile.total_battles_fought = 0;
            gamer_profile.quizzes_taken = 0;
            gamer_profile.total_quiz_score = 0;
            gamer_profile.undead_score = 0;
            gamer_profile.bump = bumps.gamer_profile;
            gamer_profile.created_at = Clock::get()?.unix_timestamp;
        } else {
            gamer_profile.character_class = character_class;
        }
        
        Ok(())
    }
}