use anchor_lang::prelude::*;
use crate::{state::*, constants::*, error::*};
use ephemeral_rollups_sdk::anchor::commit;
use ephemeral_rollups_sdk::ephem::commit_accounts;


#[commit]
#[derive(Accounts)]
#[instruction(world_id: [u8; 32])]
pub struct SubmitQuiz<'info> {
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
        bump,
    )]
    pub undead_world: Account<'info, UndeadWorld>,
}

impl<'info> SubmitQuiz<'info> {
    pub fn submit_quiz(
        &mut self,
        score: u8,
        world_id: [u8; 32],
    ) -> Result<()> {
        let profile = &mut self.gamer_profile;
        let world = &mut self.undead_world;
        require!(score <= 100, RustUndeadError::InvalidScore);
        require!(world_id ==world.world_id, RustUndeadError::InvalidRoomId);

        // Update quiz stats
        profile.total_quiz_score = profile.total_quiz_score.saturating_add(score as u32);
        profile.quizzes_taken = profile.quizzes_taken.saturating_add(1);
        profile.undead_score = profile.total_quiz_score / profile.quizzes_taken as u32;

        // Update world leaderboard if top score
        if profile.undead_score > world.highest_undead_score_average {
            world.highest_undead_score_average = profile.undead_score;
            world.top_commander = self.player.key();
        }

        msg!("Quiz submitted: Score {}, Average {}", score, profile.undead_score);

        // Commit to rollup
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



// cargo clean
// rm -rf Cargo.lock
// cargo update
// cargo build-bpf