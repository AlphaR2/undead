use anchor_lang::prelude::*;
use crate::{state::*, constants::*, error::*};
use session_keys::{SessionToken, Session};

#[derive(Accounts, Session)]
#[instruction(username: String)]
pub struct BuildUserProfile<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    /// CHECK: This is verified and is player account 
    pub player: AccountInfo<'info>,

    #[account(
        init_if_needed,
        payer = signer,
        space = ANCHOR_DISCRIMINATOR + UsernameRegistry::INIT_SPACE,
        seeds = [USER_REGISTRY, username.as_bytes()],
        bump
    )]
    pub user_registry: Account<'info, UsernameRegistry>,
    
    #[account(
        init_if_needed,
        payer = signer,
        space = ANCHOR_DISCRIMINATOR + UserProfile::INIT_SPACE,
        seeds = [USER_PROFILE, player.key().as_ref()],
        bump
    )]
    pub user_profile: Account<'info, UserProfile>,
    
    #[session(
        signer = signer,
        authority = player.key()
    )]
    pub session_token: Option<Account<'info, SessionToken>>,
    pub system_program: Program<'info, System>,
}

impl<'info> BuildUserProfile<'info> {
    pub fn build_user_profile(
        &mut self,
        username: String,
        persona: UserPersona,
        bumps: &BuildUserProfileBumps
    ) -> Result<()> {
        require!(username.len() <= 20, RustUndeadError::NameTooLong);
        require!(username.len() > 0, RustUndeadError::NameEmpty);
       
        let user_registry = &mut self.user_registry;
        if user_registry.claimed {
            return err!(RustUndeadError::UsernameAlreadyChoosen);
        }
        user_registry.claimed = true;
        user_registry.owner = self.player.key();
        user_registry.bump = bumps.user_registry;
        
        let user_profile = &mut self.user_profile;
        let is_new_account = user_profile.owner == Pubkey::default();
        
        if is_new_account {
            user_profile.owner = self.player.key();
            user_profile.warriors = 0;
            user_profile.username = Some(username);
            user_profile.user_persona = Some(persona);
            user_profile.achievement_level = AchievementLevel::None;
            user_profile.join_date = Clock::get()?.unix_timestamp;
            user_profile.bump = bumps.user_profile;
        } else {
            require!(user_profile.owner == self.player.key(), RustUndeadError::NotAuthorized);
            user_profile.username = Some(username);
            user_profile.user_persona = Some(persona);
        }
        
        Ok(())
    }
}