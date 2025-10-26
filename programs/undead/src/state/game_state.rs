use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct GameConfig {
    pub authority: Pubkey,
    pub released_chapters: u8,
    pub total_warriors: u32,
    pub boss_battles_enabled: bool,
    pub paused: bool,
    pub bump: u8,
}


/* User Persona */
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum UserPersona {
TreasureHunter,
BoneSmith,
ObsidianProphet,
GraveBaron,
Demeter, 
Collector, 
CovenCaller, 
SeerOfAsh,
Cerberus 
}
impl Space for UserPersona {
const INIT_SPACE: usize = 1;
}


/*User Profile, stays on base layer */
#[account]
#[derive(InitSpace)]
pub struct UserProfile {
    pub owner: Pubkey,
    #[max_len(20)]
    pub username: Option<String>,
    pub user_persona: Option<UserPersona>,
    pub warriors: u32,
    pub achievement_level: AchievementLevel,  
    pub join_date: i64,
    pub bump: u8,                      
}
/*Username Registry to ensure single Usernames */
#[account]
#[derive(InitSpace)]
pub struct UsernameRegistry {
    pub claimed: bool,
    pub owner: Pubkey,
    pub bump: u8
}

// gems and in-game stuff coming in soon

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Copy, PartialEq, Eq)]
pub enum AchievementLevel {
    None,
    Bronze,
    Silver,
    Gold,
    Platinum,
    Diamond,
}


impl Space for AchievementLevel {
	const INIT_SPACE: usize = 1;
 }

 /*game profile permanent to the ER */
#[account]
#[derive(InitSpace)]
pub struct GamerProfile {
    pub owner: Pubkey,
    pub character_class: WarriorClass,
    pub current_chapter: u16,
    pub chapters_completed: u16,
    pub current_position: u32,
    pub total_battles_won: u64,
    pub total_battles_lost: u64,
    pub total_battles_fought: u64,
    pub quizzes_taken: u16,
    pub total_quiz_score: u32,
    pub undead_score: u32, // Average: total_quiz_score / quizzes_taken
    pub bump: u8,
    pub created_at: i64,
}


 
/*Warrior Class */
 #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum WarriorClass {
	Validator,
	Oracle,
	Guardian,
	Daemon
}

impl Space for WarriorClass {
	const INIT_SPACE : usize = 1;
}

impl std::fmt::Display for WarriorClass {
	fn fmt(
		&self,
		f: &mut std::fmt::Formatter<'_>
	) -> std::fmt::Result{
		match self {
						WarriorClass::Validator => write!(f, "Validator Warrior"),
						WarriorClass::Oracle => write!(f, "Oracle Warrior"),
						WarriorClass::Guardian => write!(f, "Guardian Warrior"),
						WarriorClass::Daemon => write!(f, "Daemon Warrior"),
        }
	}
 }


 /*Image Rarity */
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImageRarity {
	Common,
	Uncommon,
	Rare
}

impl Space for ImageRarity {
	const INIT_SPACE: usize = 1;
}

impl std::fmt::Display for ImageRarity {
	fn fmt(
		&self,
		f: &mut std::fmt::Formatter<'_>
	) -> std::fmt::Result{
		match self {
            ImageRarity::Common => write!(f, "common"),
            ImageRarity::Uncommon => write!(f, "uncommon"),
            ImageRarity::Rare => write!(f, "rare"),
        }
	}
 }


/*Warrior Account*/
#[account]
#[derive(InitSpace)]
pub struct UndeadWarrior {
	#[max_len(32)]
  pub name: String,
	pub address : Pubkey,
	pub owner: Pubkey,
	pub dna: [u8; 8],
	pub created_at: i64,
	pub base_attack: u16,
	pub base_defense: u16,
	pub base_knowledge: u16,
	pub current_hp: u16,
  pub max_hp: u16,                
	pub warrior_class: WarriorClass,
	pub battles_won: u32,
	pub battles_lost: u32,
	pub experience_points: u64,
	pub level: u16,
	pub last_battle_at: i64,  
  pub cooldown_expires_at: i64,
	pub bump: u8,

	//img fields
	pub image_rarity: ImageRarity,
  pub image_index: u8,
  #[max_len(200)]
  pub image_uri: String,
}

/*World State */
#[account]
#[derive(InitSpace)]
pub struct UndeadWorld {
    pub world_id: [u8; 32],
    pub active_players: u16,
    pub total_players: u32,
    pub total_completions: u32,
    pub highest_undead_score_average: u32,
    pub top_commander: Pubkey,
    pub created_at: i64, 
    pub bump: u8
}
