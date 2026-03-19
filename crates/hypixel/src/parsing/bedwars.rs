use super::player::{calculate_network_level, color_code, extract_rank_prefix};
use serde_json::Value;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Mode {
    #[default]
    Overall,
    Core,
    Solos,
    Doubles,
    Threes,
    Fours,
    FourTeamModes,
    FourVFour,
}

impl Mode {
    pub fn display_name(&self) -> &'static str {
        match self {
            Mode::Overall => "Overall",
            Mode::Core => "Core",
            Mode::Solos => "Solos",
            Mode::Doubles => "Doubles",
            Mode::Threes => "Threes",
            Mode::Fours => "Fours",
            Mode::FourTeamModes => "4 Team Modes",
            Mode::FourVFour => "4v4",
        }
    }

    pub fn all() -> &'static [Mode] {
        &[
            Mode::Overall,
            Mode::Core,
            Mode::Solos,
            Mode::Doubles,
            Mode::Threes,
            Mode::Fours,
            Mode::FourTeamModes,
            Mode::FourVFour,
        ]
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "overall" => Some(Mode::Overall),
            "core" => Some(Mode::Core),
            "solos" => Some(Mode::Solos),
            "doubles" => Some(Mode::Doubles),
            "threes" => Some(Mode::Threes),
            "fours" => Some(Mode::Fours),
            "4 team modes" | "4_team_modes" | "fourteammodes" => Some(Mode::FourTeamModes),
            "4v4" | "fourvfour" => Some(Mode::FourVFour),
            _ => None,
        }
    }
}

#[derive(Clone, Default)]
pub struct GuildInfo {
    pub name: Option<String>,
    pub tag: Option<String>,
    pub tag_color: Option<String>,
    pub rank: Option<String>,
    pub joined: Option<i64>,
    pub weekly_gexp: Option<u64>,
}

impl GuildInfo {
    pub fn tag_with_color(&self) -> Option<String> {
        let tag = self.tag.as_ref()?;
        let color = self.tag_color.as_deref().unwrap_or("GRAY");
        Some(format!("{}[{}]", color_code(color), tag))
    }
}

pub fn ratio(numerator: u64, denominator: u64) -> f64 {
    if denominator == 0 {
        numerator as f64
    } else {
        numerator as f64 / denominator as f64
    }
}

#[derive(Clone, Default)]
pub struct ModeStats {
    pub wins: u64,
    pub losses: u64,
    pub kills: u64,
    pub deaths: u64,
    pub final_kills: u64,
    pub final_deaths: u64,
    pub beds_broken: u64,
    pub beds_lost: u64,
    pub winstreak: Option<u64>,
}

impl ModeStats {
    pub fn wlr(&self) -> f64 {
        ratio(self.wins, self.losses)
    }

    pub fn kdr(&self) -> f64 {
        ratio(self.kills, self.deaths)
    }

    pub fn fkdr(&self) -> f64 {
        ratio(self.final_kills, self.final_deaths)
    }

    pub fn bblr(&self) -> f64 {
        ratio(self.beds_broken, self.beds_lost)
    }
}

#[derive(Clone, Default)]
pub struct SlumberInfo {
    pub tickets: u64,
    pub total_tickets_earned: u64,
    pub doublers: u64,
}

#[derive(Clone)]
pub struct Stats {
    pub username: String,
    pub display_name: String,
    pub rank_prefix: Option<String>,
    pub experience: u64,
    pub level: u32,
    pub games_played: u64,
    pub network_level: f64,
    pub achievement_points: u64,
    pub ranks_gifted: u64,
    pub first_login: Option<i64>,
    pub guild: GuildInfo,
    pub slumber: SlumberInfo,
    pub overall: ModeStats,
    pub solos: ModeStats,
    pub doubles: ModeStats,
    pub threes: ModeStats,
    pub fours: ModeStats,
    pub four_v_four: ModeStats,
}

impl Stats {
    pub fn get_mode_stats(&self, mode: Mode) -> ModeStats {
        match mode {
            Mode::Overall => self.overall.clone(),
            Mode::Core => self.core_stats(),
            Mode::Solos => self.solos.clone(),
            Mode::Doubles => self.doubles.clone(),
            Mode::Threes => self.threes.clone(),
            Mode::Fours => self.fours.clone(),
            Mode::FourTeamModes => self.four_team_modes_stats(),
            Mode::FourVFour => self.four_v_four.clone(),
        }
    }

    fn core_stats(&self) -> ModeStats {
        ModeStats {
            wins: self.overall.wins.saturating_sub(self.four_v_four.wins),
            losses: self.overall.losses.saturating_sub(self.four_v_four.losses),
            kills: self.overall.kills.saturating_sub(self.four_v_four.kills),
            deaths: self.overall.deaths.saturating_sub(self.four_v_four.deaths),
            final_kills: self
                .overall
                .final_kills
                .saturating_sub(self.four_v_four.final_kills),
            final_deaths: self
                .overall
                .final_deaths
                .saturating_sub(self.four_v_four.final_deaths),
            beds_broken: self
                .overall
                .beds_broken
                .saturating_sub(self.four_v_four.beds_broken),
            beds_lost: self
                .overall
                .beds_lost
                .saturating_sub(self.four_v_four.beds_lost),
            winstreak: self.overall.winstreak,
        }
    }

    fn four_team_modes_stats(&self) -> ModeStats {
        ModeStats {
            wins: self.threes.wins + self.fours.wins,
            losses: self.threes.losses + self.fours.losses,
            kills: self.threes.kills + self.fours.kills,
            deaths: self.threes.deaths + self.fours.deaths,
            final_kills: self.threes.final_kills + self.fours.final_kills,
            final_deaths: self.threes.final_deaths + self.fours.final_deaths,
            beds_broken: self.threes.beds_broken + self.fours.beds_broken,
            beds_lost: self.threes.beds_lost + self.fours.beds_lost,
            winstreak: None,
        }
    }
}

pub fn extract(username: &str, player: &Value, guild: Option<GuildInfo>) -> Option<Stats> {
    let bw = player.get("stats")?.get("Bedwars")?;

    let experience = bw.get("Experience").and_then(|v| v.as_u64()).unwrap_or(0);

    let level = player
        .get("achievements")
        .and_then(|a| a.get("bedwars_level"))
        .and_then(|v| v.as_u64())
        .unwrap_or_else(|| calculate_level(experience)) as u32;

    let display_name = player
        .get("displayname")
        .and_then(|v| v.as_str())
        .unwrap_or(username)
        .to_string();

    let rank_prefix = extract_rank_prefix(player);

    let network_exp = player
        .get("networkExp")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);
    let network_level = calculate_network_level(network_exp);

    let achievement_points = player
        .get("achievementPoints")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);

    let ranks_gifted = player
        .get("giftingMeta")
        .and_then(|g| g.get("ranksGiven"))
        .and_then(|v| v.as_u64())
        .unwrap_or(0);

    let first_login = player.get("firstLogin").and_then(|v| v.as_i64());

    let guild_info = guild.unwrap_or_default();

    let slumber = bw
        .get("slumber")
        .map(|s| SlumberInfo {
            tickets: s.get("tickets").and_then(|v| v.as_u64()).unwrap_or(0),
            total_tickets_earned: s
                .get("total_tickets_earned")
                .and_then(|v| v.as_u64())
                .unwrap_or(0),
            doublers: s.get("doublers").and_then(|v| v.as_u64()).unwrap_or(0),
        })
        .unwrap_or_default();

    Some(Stats {
        username: username.to_string(),
        display_name,
        rank_prefix,
        experience,
        level,
        games_played: bw
            .get("games_played_bedwars")
            .and_then(|v| v.as_u64())
            .unwrap_or(0),
        network_level,
        achievement_points,
        ranks_gifted,
        first_login,
        guild: guild_info,
        slumber,
        overall: extract_mode_stats(bw, ""),
        solos: extract_mode_stats(bw, "eight_one_"),
        doubles: extract_mode_stats(bw, "eight_two_"),
        threes: extract_mode_stats(bw, "four_three_"),
        fours: extract_mode_stats(bw, "four_four_"),
        four_v_four: extract_mode_stats(bw, "two_four_"),
    })
}

fn extract_mode_stats(bw: &Value, prefix: &str) -> ModeStats {
    let get_stat = |suffix: &str| -> u64 {
        let key = format!("{}{}", prefix, suffix);
        bw.get(&key).and_then(|v| v.as_u64()).unwrap_or(0)
    };

    let winstreak_key = if prefix.is_empty() {
        "winstreak".to_string()
    } else {
        format!("{}winstreak", prefix)
    };

    ModeStats {
        wins: get_stat("wins_bedwars"),
        losses: get_stat("losses_bedwars"),
        kills: get_stat("kills_bedwars"),
        deaths: get_stat("deaths_bedwars"),
        final_kills: get_stat("final_kills_bedwars"),
        final_deaths: get_stat("final_deaths_bedwars"),
        beds_broken: get_stat("beds_broken_bedwars"),
        beds_lost: get_stat("beds_lost_bedwars"),
        winstreak: bw.get(&winstreak_key).and_then(|v| v.as_u64()),
    }
}

#[derive(Clone, Default)]
pub struct WinstreakModeData {
    pub wins: u64,
    pub losses: u64,
    pub winstreak: Option<u64>,
}

#[derive(Clone, Default)]
pub struct WinstreakSnapshot {
    pub overall: WinstreakModeData,
    pub solos: WinstreakModeData,
    pub doubles: WinstreakModeData,
    pub threes: WinstreakModeData,
    pub fours: WinstreakModeData,
    pub four_v_four: WinstreakModeData,
}

pub fn extract_winstreak_snapshot(player: &Value) -> Option<WinstreakSnapshot> {
    let bw = player.get("stats")?.get("Bedwars")?;

    fn mode(bw: &Value, wins: &str, losses: &str, ws: &str) -> WinstreakModeData {
        WinstreakModeData {
            wins: bw.get(wins).and_then(|v| v.as_u64()).unwrap_or(0),
            losses: bw.get(losses).and_then(|v| v.as_u64()).unwrap_or(0),
            winstreak: bw.get(ws).and_then(|v| v.as_u64()),
        }
    }

    Some(WinstreakSnapshot {
        overall: mode(bw, "wins_bedwars", "losses_bedwars", "winstreak"),
        solos: mode(
            bw,
            "eight_one_wins_bedwars",
            "eight_one_losses_bedwars",
            "eight_one_winstreak",
        ),
        doubles: mode(
            bw,
            "eight_two_wins_bedwars",
            "eight_two_losses_bedwars",
            "eight_two_winstreak",
        ),
        threes: mode(
            bw,
            "four_three_wins_bedwars",
            "four_three_losses_bedwars",
            "four_three_winstreak",
        ),
        fours: mode(
            bw,
            "four_four_wins_bedwars",
            "four_four_losses_bedwars",
            "four_four_winstreak",
        ),
        four_v_four: mode(
            bw,
            "two_four_wins_bedwars",
            "two_four_losses_bedwars",
            "two_four_winstreak",
        ),
    })
}

pub fn calculate_level(experience: u64) -> u64 {
    let level = 100 * (experience / 487000);
    let exp = experience % 487000;

    if exp < 500 {
        return level;
    }
    if exp < 1500 {
        return level + 1;
    }
    if exp < 3500 {
        return level + 2;
    }
    if exp < 7000 {
        return level + 3;
    }

    level + 4 + (exp - 7000) / 5000
}

pub fn experience_for_level(level: u64) -> u64 {
    let prestige = level / 100;
    let within = level % 100;

    let base = prestige * 487000;
    match within {
        0 => base,
        1 => base + 500,
        2 => base + 1500,
        3 => base + 3500,
        l => base + 7000 + (l - 4) * 5000,
    }
}

pub fn level_progress(experience: u64) -> f64 {
    let exp = experience % 487000;
    let raw = if exp < 500 {
        exp as f64 / 500.0 / 100.0
    } else if exp < 1500 {
        (1.0 + (exp - 500) as f64 / 1000.0) / 100.0
    } else if exp < 3500 {
        (2.0 + (exp - 1500) as f64 / 2000.0) / 100.0
    } else if exp < 7000 {
        (3.0 + (exp - 3500) as f64 / 3500.0) / 100.0
    } else {
        let remaining = exp - 7000;
        let levels = remaining / 5000;
        let progress = (remaining % 5000) as f64 / 5000.0;
        ((4 + levels) as f64 + progress) / 100.0
    };
    raw.fract()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn experience_for_level_roundtrip() {
        for level in [0, 1, 2, 3, 4, 50, 99, 100, 150, 200, 500, 1000] {
            let xp = experience_for_level(level);
            assert_eq!(
                calculate_level(xp),
                level,
                "roundtrip failed for level {level}"
            );
        }
    }
}
