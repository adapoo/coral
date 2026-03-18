use chrono::{DateTime, Utc};

use super::bedwars::{Mode, WinstreakSnapshot};

const MIN_STREAK_THRESHOLD: u64 = 15;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum StreakSource {
    Urchin,
    Antisniper,
}

pub struct Streak {
    pub value: u64,
    pub approximate: bool,
    pub timestamp: DateTime<Utc>,
    pub source: StreakSource,
}

pub struct WinstreakHistory {
    pub streaks: Vec<Streak>,
}

pub fn calculate(
    snapshots: &[(DateTime<Utc>, WinstreakSnapshot)],
    mode: Mode,
) -> WinstreakHistory {
    if snapshots.is_empty() {
        return WinstreakHistory {
            streaks: Vec::new(),
        };
    }

    let mut streaks = Vec::new();
    let mut streak_start: Option<usize> = None;
    let mut peak_api_winstreak: Option<u64> = None;

    for (i, (_timestamp, stats)) in snapshots.iter().enumerate() {
        let (wins, losses) = mode_wins_losses(stats, mode);
        let api_ws = api_winstreak(stats, mode);

        let delta_losses = if i > 0 {
            let (_, prev_losses) = mode_wins_losses(&snapshots[i - 1].1, mode);
            losses.saturating_sub(prev_losses)
        } else {
            0
        };

        if let Some(ws) = api_ws {
            peak_api_winstreak = Some(peak_api_winstreak.map_or(ws, |peak| peak.max(ws)));
        }

        if streak_start.is_some() && delta_losses > 0 {
            let prev_idx = i - 1;
            let prev_timestamp = snapshots[prev_idx].0;
            let start_idx = streak_start.unwrap();
            let (start_wins, _) = mode_wins_losses(&snapshots[start_idx].1, mode);

            let (prev_wins, _) = mode_wins_losses(&snapshots[prev_idx].1, mode);

            let (value, approximate) = if let Some(peak) = peak_api_winstreak {
                let delta_wins = wins.saturating_sub(prev_wins);

                let mut best = peak;
                if delta_losses == 1 {
                    let wins_after_loss = api_ws.unwrap_or(0);
                    let total_streak_wins = wins
                        .saturating_sub(start_wins)
                        .saturating_sub(wins_after_loss);
                    best = best.max(total_streak_wins);
                } else {
                    let observed_streak_wins = prev_wins.saturating_sub(start_wins);
                    best = best.max(observed_streak_wins);
                }

                (best, delta_wins >= 2 || best > peak)
            } else {
                (prev_wins.saturating_sub(start_wins), true)
            };

            if value >= MIN_STREAK_THRESHOLD {
                streaks.push(Streak {
                    value,
                    approximate,
                    timestamp: prev_timestamp,
                    source: StreakSource::Urchin,
                });
            }

            streak_start = None;
            peak_api_winstreak = None;
        }

        if streak_start.is_none() && (delta_losses > 0 || i == 0) {
            streak_start = Some(i);
            peak_api_winstreak = None;

            if let Some(ws) = api_ws {
                peak_api_winstreak = Some(ws);
            }
        }
    }

    streaks.sort_by(|a, b| b.value.cmp(&a.value));
    WinstreakHistory { streaks }
}

fn mode_wins_losses(stats: &WinstreakSnapshot, mode: Mode) -> (u64, u64) {
    match mode {
        Mode::Overall | Mode::Core => {
            let wins =
                stats.solos.wins + stats.doubles.wins + stats.threes.wins + stats.fours.wins;
            let losses = stats.solos.losses
                + stats.doubles.losses
                + stats.threes.losses
                + stats.fours.losses;
            (wins, losses)
        }
        Mode::Solos => (stats.solos.wins, stats.solos.losses),
        Mode::Doubles => (stats.doubles.wins, stats.doubles.losses),
        Mode::Threes => (stats.threes.wins, stats.threes.losses),
        Mode::Fours => (stats.fours.wins, stats.fours.losses),
        Mode::FourTeamModes => (
            stats.threes.wins + stats.fours.wins,
            stats.threes.losses + stats.fours.losses,
        ),
        Mode::FourVFour => (stats.four_v_four.wins, stats.four_v_four.losses),
    }
}

fn api_winstreak(stats: &WinstreakSnapshot, mode: Mode) -> Option<u64> {
    match mode {
        Mode::Overall | Mode::Core => stats.overall.winstreak,
        Mode::Solos => stats.solos.winstreak,
        Mode::Doubles => stats.doubles.winstreak,
        Mode::Threes => stats.threes.winstreak,
        Mode::Fours => stats.fours.winstreak,
        Mode::FourTeamModes => None,
        Mode::FourVFour => stats.four_v_four.winstreak,
    }
}
