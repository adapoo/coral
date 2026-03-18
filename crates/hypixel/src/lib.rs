mod guild;
pub mod parsing;
mod player;
mod stats;

pub use guild::{Guild, GuildMember};
pub use player::Player;
pub use stats::{BedwarsStats, GameStats};

pub use parsing::bedwars::{GuildInfo, Mode, ModeStats, SlumberInfo, Stats as BedwarsPlayerStats};
pub use parsing::bedwars::{
    WinstreakModeData, WinstreakSnapshot, calculate_level, experience_for_level,
    extract as extract_bedwars_stats, extract_winstreak_snapshot, level_progress, ratio,
};
pub use parsing::delta::{SessionPlayerStats, SessionStats};
pub use parsing::player::{calculate_network_level, color_code, extract_rank_prefix};
pub use parsing::winstreaks::{Streak, StreakSource, WinstreakHistory};
