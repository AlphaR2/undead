use anchor_lang::prelude::*;
pub mod state;
pub mod constants;
pub mod error;
pub mod helpers;
pub mod context;

pub use {state::*, error::RustUndeadError, context::*, helpers::*, constants::*};
pub use session_keys::{ session_auth_or, Session, SessionError, SessionToken };
use ephemeral_vrf_sdk::anchor::vrf;
use ephemeral_vrf_sdk::instructions::{create_request_randomness_ix, RequestRandomnessParams};
use ephemeral_vrf_sdk::types::SerializableAccountMeta;

use ephemeral_rollups_sdk::anchor::ephemeral;

declare_id!("rst6o6UC9WGZn9fTjicAatTqyRFVKBE2c6th6AYAUs4");

#[ephemeral]
#[program]
pub mod rust_undead {
    use super::*;

    pub fn initialize_game_config(
        ctx: Context<InitializeGameConfig>,
        released_chapters: u8,
    ) -> Result<()> {
        ctx.accounts.initialize_game_config(released_chapters, &ctx.bumps)
    }

    #[session_auth_or(
        ctx.accounts.user_profile.owner == ctx.accounts.player.key() || 
        ctx.accounts.user_profile.owner == Pubkey::default(),
        RustUndeadError::NotAuthorized
    )]
    pub fn build_user_profile(
        ctx: Context<BuildUserProfile>,
        username: String,
        persona: UserPersona,
    ) -> Result<()> {
        ctx.accounts.build_user_profile(username, persona, &ctx.bumps)
    }

    #[session_auth_or(
        ctx.accounts.gamer_profile.owner == ctx.accounts.player.key() || 
        ctx.accounts.gamer_profile.owner == Pubkey::default(),
        RustUndeadError::NotAuthorized
    )]
    pub fn build_gaming_profile(
        ctx: Context<BuildGamingProfile>,
        character_class: WarriorClass,
    ) -> Result<()> {
        ctx.accounts.build_gaming_profile(character_class, &ctx.bumps)
    }

    pub fn initialize_undead_world(
        ctx: Context<InitializeUndeadWorld>,
        world_id: [u8; 32],
    ) -> Result<()> {
        ctx.accounts.initialize_undead_world(world_id, &ctx.bumps)
    }

    // ============== WARRIOR CREATION ==============
    
    #[session_auth_or(
        ctx.accounts.user_profile.owner == ctx.accounts.player.key(),
        RustUndeadError::NotAuthorized
    )]
    pub fn create_warrior(
        ctx: Context<CreateWarrior>,
        name: String, 
        dna: [u8; 8], 
        class: WarriorClass,
        client_seed: u8,
        no_vrf: bool,
    ) -> Result<()> {
        require!(name.len() <= 32, RustUndeadError::NameTooLong);
        require!(name.len() > 0, RustUndeadError::NameEmpty);
        
        let player_key = ctx.accounts.player.key();
        let warrior_key = ctx.accounts.warrior.key();
        let user_profile = ctx.accounts.user_profile;

        user_profile.warriors = user_profile.warriors.saturating_add(1);

        {
            let warrior = &mut ctx.accounts.warrior;

            warrior.name = name.clone();
            warrior.owner = player_key;
            warrior.dna = dna;
            warrior.created_at = Clock::get()?.unix_timestamp;
            warrior.warrior_class = class;
            warrior.last_battle_at = 0;
            warrior.cooldown_expires_at = 0;
            warrior.address = warrior_key;
            warrior.bump = ctx.bumps.warrior;
            warrior.max_hp = 100;       
            warrior.current_hp = 100;
            warrior.battles_won = 0;
            warrior.battles_lost = 0;
            warrior.experience_points = 0;
            warrior.level = 1;

            // ALWAYS set fallback stats first so vrf can override if available
            match temp_stats_rand(player_key, dna, client_seed, class) {
                Ok((attack, defense, knowledge)) => {
                    warrior.base_attack = attack;
                    warrior.base_defense = defense;
                    warrior.base_knowledge = knowledge;
                },
                Err(_) => {
                    let (default_attack, default_defense, default_knowledge) = match class {
                        WarriorClass::Daemon => (120, 50, 75),
                        WarriorClass::Guardian => (50, 120, 75),
                        WarriorClass::Oracle => (75, 60, 120),
                        WarriorClass::Validator => (80, 80, 60),
                    };
                    
                    warrior.base_attack = default_attack;
                    warrior.base_defense = default_defense;
                    warrior.base_knowledge = default_knowledge;
                }
            }

            // ALWAYS set fallback image first
            match temp_img_rand(player_key, dna, client_seed, class) {
                Ok((rarity, index, url)) => {
                    warrior.image_rarity = rarity;
                    warrior.image_index = index; 
                    warrior.image_uri = url.clone();
                },
                Err(_) => {
                    warrior.image_rarity = ImageRarity::Common;
                    warrior.image_index = 1;
                    warrior.image_uri = format!(
                        "{}/{}/c1.png", 
                        IPFS_GATEWAY,
                        get_class_folder_hash(class)
                    );
                }
            }
        }

        ctx.accounts.config.total_warriors = ctx.accounts.config.total_warriors.saturating_add(1);

        let user_profile = &mut ctx.accounts.user_profile;
        user_profile.warriors_created = user_profile.warriors_created.saturating_add(1);
        user_profile.total_points = user_profile.total_points.saturating_add(100);
        
        let user_achievements = &mut ctx.accounts.user_achievements;
        if user_achievements.owner == Pubkey::default() {
            user_achievements.owner = player_key;
            user_achievements.overall_achievements = AchievementLevel::None;
            user_achievements.warrior_achivement = AchievementLevel::Bronze;
            user_achievements.winner_achievement = AchievementLevel::None;
            user_achievements.battle_achievement = AchievementLevel::None;
            user_achievements.first_warrior_date = Clock::get()?.unix_timestamp;
            user_achievements.bump = ctx.bumps.user_achievements;
            user_achievements.warrior_achivement = calculate_warrior_achievement(user_profile.warriors_created);
        } else {
            user_achievements.warrior_achivement = calculate_warrior_achievement(user_profile.warriors_created);
        }
        
        user_profile.total_points = user_profile.total_points.saturating_add(100);
        user_achievements.overall_achievements = calculate_overall_achievement(user_profile.total_points);

        // Request VRF if enabled (will override fallback stats via callback)
        if !no_vrf {
            let ix = create_request_randomness_ix(RequestRandomnessParams { 
                payer: ctx.accounts.signer.key(),
                oracle_queue: ctx.accounts.oracle_queue.key(), 
                callback_program_id: ID, 
                callback_discriminator: instruction::CallbackWarriorStats::DISCRIMINATOR.to_vec(),
                caller_seed: [client_seed; 32], 
                accounts_metas: Some(vec![SerializableAccountMeta{
                    pubkey: warrior_key,
                    is_signer: false,
                    is_writable: true
                }]), 
                ..Default::default()
            });

            ctx.accounts.invoke_signed_vrf(&ctx.accounts.signer.to_account_info(), &ix)?;
        }

        emit!(WariorCreatedEvent{
            name: ctx.accounts.warrior.name.clone(),
            class: ctx.accounts.warrior.warrior_class,
            attack: ctx.accounts.warrior.base_attack, 
            defense: ctx.accounts.warrior.base_defense, 
            knowledge: ctx.accounts.warrior.base_knowledge,
            image_url: ctx.accounts.warrior.image_uri.clone(),
            image_rarity: ctx.accounts.warrior.image_rarity,
            current_hp: ctx.accounts.warrior.current_hp,
            max_hp: ctx.accounts.warrior.max_hp
        });
       
        Ok(())
    }

    pub fn callback_warrior_stats(
        ctx: Context<CallbackWarriorStats>,
        randomness: [u8; 32],
    ) -> Result<()> {
        let warrior = &mut ctx.accounts.warrior;
        let class = warrior.warrior_class;

        // Generate stats based on class specialization with FIXED ranges
        let (attack, defense, knowledge) = match class {
            WarriorClass::Validator => {
                let attack = ephemeral_vrf_sdk::rnd::random_u8_with_range(&randomness, 60, 100);   
                let defense = ephemeral_vrf_sdk::rnd::random_u8_with_range(&randomness, 40, 100);  
                let knowledge = ephemeral_vrf_sdk::rnd::random_u8_with_range(&randomness, 40, 80); 
                (attack, defense, knowledge) 
            },
            WarriorClass::Oracle => {
                let attack = ephemeral_vrf_sdk::rnd::random_u8_with_range(&randomness, 50, 100);   
                let defense = ephemeral_vrf_sdk::rnd::random_u8_with_range(&randomness, 40, 80);  
                let knowledge = ephemeral_vrf_sdk::rnd::random_u8_with_range(&randomness, 100, 141);
                (attack, defense, knowledge)
            },
            WarriorClass::Guardian => {
                let attack = ephemeral_vrf_sdk::rnd::random_u8_with_range(&randomness, 40, 61);
                let defense = ephemeral_vrf_sdk::rnd::random_u8_with_range(&randomness, 100, 141);
                let knowledge = ephemeral_vrf_sdk::rnd::random_u8_with_range(&randomness, 50, 100); 
                (attack, defense, knowledge)
            },
            WarriorClass::Daemon => {
                let attack = ephemeral_vrf_sdk::rnd::random_u8_with_range(&randomness, 100, 141);
                let defense = ephemeral_vrf_sdk::rnd::random_u8_with_range(&randomness, 40, 61);
                let knowledge = ephemeral_vrf_sdk::rnd::random_u8_with_range(&randomness, 50, 100); 
                (attack, defense, knowledge)
            },
        };

        warrior.base_attack = attack as u16;
        warrior.base_defense = defense as u16;
        warrior.base_knowledge = knowledge as u16;

        match generate_warrior_image(&randomness, class) {
            Ok((rarity, index, url)) => {
                warrior.image_rarity = rarity;
                warrior.image_index = index;
                warrior.image_uri = url;
            }
            Err(_e) => {
                warrior.image_rarity = ImageRarity::Common;
                warrior.image_index = 1;
                warrior.image_uri = format!(
                    "{}/{}/c1.png", 
                    IPFS_GATEWAY,
                    get_class_folder_hash(class)
                );
            }
        }
        
        Ok(())
    }

    // ============== DELEGATION INSTRUCTIONS ==============
    
    #[session_auth_or(
        true,
        RustUndeadError::NotAuthorized
    )]
    pub fn game_profile_to_rollup(
        ctx: Context<GamingDelegate>,
        player: Pubkey,
    ) -> Result<()> {
        ctx.accounts.game_profile_to_rollup(player)
    }

    #[session_auth_or(
        true,
        RustUndeadError::NotAuthorized
    )]
    pub fn world_to_rollup(
        ctx: Context<WorldDelegate>,
        world_id: [u8; 32],
    ) -> Result<()> {
        ctx.accounts.world_to_rollup(world_id)
    }

    // ============== ER GAMEPLAY INSTRUCTIONS ==============
    
    #[session_auth_or(
        ctx.accounts.gamer_profile.owner == ctx.accounts.player.key(),
        RustUndeadError::NotAuthorized
    )]
    pub fn start_chapter(
        ctx: Context<StartChapter>,
        player: Pubkey,
        chapter_number: u16,
        world_id: [u8; 32],
    ) -> Result<()> {
        ctx.accounts.start_chapter(player, chapter_number, world_id)
    }

    #[session_auth_or(
        ctx.accounts.gamer_profile.owner == ctx.accounts.player.key(),
        RustUndeadError::NotAuthorized
    )]
    pub fn update_position(
        ctx: Context<UpdatePosition>,
        player: Pubkey,
        position: u32,
    ) -> Result<()> {
        ctx.accounts.update_position(player, position)
    }

    #[session_auth_or(
        ctx.accounts.gamer_profile.owner == ctx.accounts.player.key(),
        RustUndeadError::NotAuthorized
    )]
    pub fn submit_quiz(
        ctx: Context<SubmitQuiz>,
        player: Pubkey,
        score: u8,
        world_id: [u8; 32],
    ) -> Result<()> {
        ctx.accounts.submit_quiz(player, score, world_id)
    }
}


#[vrf]
#[derive(Accounts, Session)]
#[instruction(name: String, class: WarriorClass)]
pub struct CreateWarrior<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    /// CHECK: This is verified and is player account 
    pub player: AccountInfo<'info>,

    #[account(
        init,
        payer = signer,
        space = ANCHOR_DISCRIMINATOR + UndeadWarrior::INIT_SPACE,
        seeds = [UNDEAD_WARRIOR, player.key().as_ref(), name.as_bytes()],
        bump
    )]
    pub warrior: Account<'info, UndeadWarrior>,
    
    #[account(
        mut,
        seeds = [USER_PROFILE, player.key().as_ref()],
        bump = user_profile.bump,
        constraint = user_profile.owner == player.key() @ RustUndeadError::NotAuthorized,
    )]
    pub user_profile: Account<'info, UserProfile>,
    
    #[account(
        mut,
        seeds = [USER_GAME_PROFILE, player.key().as_ref()],
        bump = gamer_profile.bump,
    )]
    pub gamer_profile: Account<'info, GamerProfile>,

    /// CHECK: The oracle queue
    #[account(
        mut,
        address = ephemeral_vrf_sdk::consts::DEFAULT_QUEUE
    )]
    pub oracle_queue: AccountInfo<'info>,

    #[session(
        signer = signer,
        authority = player.key() 
    )]
    pub session_token: Option<Account<'info, SessionToken>>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CallbackWarriorStats<'info> {
    /// This check ensures that the vrf_program_identity (which is a PDA) is a signer
    /// enforcing the callback is executed by the VRF program through CPI
    #[account(
        address = ephemeral_vrf_sdk::consts::VRF_PROGRAM_IDENTITY
    )]
    pub vrf_program_identity: Signer<'info>,
    
    #[account(mut)]
    pub warrior: Account<'info, UndeadWarrior>,
}
