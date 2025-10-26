use anchor_lang::prelude::*;
use crate::{state::*, constants::*};

/* Initialize Game Config - One time setup by admin */
#[derive(Accounts)]
pub struct InitializeGameConfig<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = ANCHOR_DISCRIMINATOR + GameConfig::INIT_SPACE,
        seeds = [GAME_CONFIG, authority.key().as_ref()],
        bump
    )]
    pub game_config: Account<'info, GameConfig>,
    
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeGameConfig<'info> {
    pub fn initialize_game_config(
        &mut self,
        released_chapters: u8,
        bumps: &InitializeGameConfigBumps
    ) -> Result<()> {
      self.game_config.set_inner(
        GameConfig { 
          authority: self.authority.key(), 
          total_warriors: 0,
          released_chapters: released_chapters, 
          boss_battles_enabled: false, 
          paused: false, 
          bump: bumps.game_config 
        }
      );        
        Ok(())
    }
}