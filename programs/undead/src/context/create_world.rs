use anchor_lang::prelude::*;
use crate::{state::*, constants::*, error::*};

/* Initialize Undead World - Admin only */
#[derive(Accounts)]
#[instruction(world_id: [u8; 32])]
pub struct InitializeUndeadWorld<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        seeds = [GAME_CONFIG, authority.key().as_ref()],
        bump = game_config.bump,
        constraint = game_config.authority == authority.key() @ RustUndeadError::NotAuthorized
    )]
    pub game_config: Account<'info, GameConfig>,

    #[account(
        init,
        payer = authority,
        space = ANCHOR_DISCRIMINATOR + UndeadWorld::INIT_SPACE,
        seeds = [UNDEAD_WORLD, world_id.as_ref()],
        bump
    )]
    pub undead_world: Account<'info, UndeadWorld>,
    
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeUndeadWorld<'info> {
    pub fn initialize_undead_world(
        &mut self,
        world_id: [u8; 32],
        bumps: &InitializeUndeadWorldBumps
    ) -> Result<()> {
      self.undead_world.set_inner(
        UndeadWorld { 
          world_id : world_id, 
          active_players: 0, 
          total_players: 0, 
          total_completions: 0, 
          highest_undead_score_average: 0, 
          top_commander: Pubkey::default(), 
          created_at: Clock::get()?.unix_timestamp, 
          bump: bumps.undead_world
        });
    
        Ok(())
    }
}