pub const ANCHOR_DISCRIMINATOR: usize = 8;

// PDA Seeds
pub const USER_PROFILE: &[u8] = b"user_profile";
pub const USER_REGISTRY: &[u8] = b"user_registry";
pub const USER_GAME_PROFILE: &[u8] = b"user_game_profile";
pub const UNDEAD_WARRIOR: &[u8] = b"undead_warrior";
pub const UNDEAD_WORLD: &[u8] = b"undead_world";
pub const GAME_CONFIG: &[u8] = b"game_config";

// IPFS folder hashes
pub const GUARDIAN_FOLDER_HASH: &str = "bafybeieg4s45fshekdmtqssax4c2tw3ro5z6rmv4ka5dnit7x66f4tmsby";
pub const VALIDATOR_FOLDER_HASH: &str = "bafybeibs2qb55efetumvknddcbizjwg4kvzdnyojys6jtovclfodxhec2a";
pub const ORACLE_FOLDER_HASH: &str = "bafybeia3ukkyzpqo6sjtjxj3iqdtfnpyh5szffxdo3avov2muo2wnzs6dy";
pub const DAEMON_FOLDER_HASH: &str = "bafybeicyzvtflal64zu5jrfhveuuqaoizhiaulvyax7s5c3jnoxyyendxu";

// Gateway URL
pub const IPFS_GATEWAY: &str = "https://gateway.pinata.cloud/ipfs";

// Image counts per rarity
pub const COMMON_COUNT: u8 = 10;
pub const UNCOMMON_COUNT: u8 = 6;
pub const RARE_COUNT: u8 = 4;

// Battle rewards
pub const WINNER_XP: u8 = 20;
pub const LOSER_XP: u8 = 10;