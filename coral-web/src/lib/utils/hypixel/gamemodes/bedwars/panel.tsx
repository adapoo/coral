import { getNumber, ratio, formatInt } from "@/lib/utils/general";

export interface BedwarsModeStats {
  wins: number;
  losses: number;
  kills: number;
  deaths: number;
  finalKills: number;
  finalDeaths: number;
  bedsBroken: number;
  bedsLost: number;
}

export function getBedwarsModes(stats: Record<string, unknown>) {
  return [
    {
      mode: "Solo",
      wins: getNumber(stats, "eight_one_wins_bedwars"),
      losses: getNumber(stats, "eight_one_losses_bedwars"),
      kills: getNumber(stats, "eight_one_kills_bedwars"),
      deaths: getNumber(stats, "eight_one_deaths_bedwars"),
      finalKills: getNumber(stats, "eight_one_final_kills_bedwars"),
      finalDeaths: getNumber(stats, "eight_one_final_deaths_bedwars"),
      bedsBroken: getNumber(stats, "eight_one_beds_broken_bedwars"),
      bedsLost: getNumber(stats, "eight_one_beds_lost_bedwars"),
    },
    {
      mode: "Doubles",
      wins: getNumber(stats, "eight_two_wins_bedwars"),
      losses: getNumber(stats, "eight_two_losses_bedwars"),
      kills: getNumber(stats, "eight_two_kills_bedwars"),
      deaths: getNumber(stats, "eight_two_deaths_bedwars"),
      finalKills: getNumber(stats, "eight_two_final_kills_bedwars"),
      finalDeaths: getNumber(stats, "eight_two_final_deaths_bedwars"),
      bedsBroken: getNumber(stats, "eight_two_beds_broken_bedwars"),
      bedsLost: getNumber(stats, "eight_two_beds_lost_bedwars"),
    },
    {
      mode: "3v3v3v3",
      wins: getNumber(stats, "four_three_wins_bedwars"),
      losses: getNumber(stats, "four_three_losses_bedwars"),
      kills: getNumber(stats, "four_three_kills_bedwars"),
      deaths: getNumber(stats, "four_three_deaths_bedwars"),
      finalKills: getNumber(stats, "four_three_final_kills_bedwars"),
      finalDeaths: getNumber(stats, "four_three_final_deaths_bedwars"),
      bedsBroken: getNumber(stats, "four_three_beds_broken_bedwars"),
      bedsLost: getNumber(stats, "four_three_beds_lost_bedwars"),
    },
    {
      mode: "4v4v4v4",
      wins: getNumber(stats, "four_four_wins_bedwars"),
      losses: getNumber(stats, "four_four_losses_bedwars"),
      kills: getNumber(stats, "four_four_kills_bedwars"),
      deaths: getNumber(stats, "four_four_deaths_bedwars"),
      finalKills: getNumber(stats, "four_four_final_kills_bedwars"),
      finalDeaths: getNumber(stats, "four_four_final_deaths_bedwars"),
      bedsBroken: getNumber(stats, "four_four_beds_broken_bedwars"),
      bedsLost: getNumber(stats, "four_four_beds_lost_bedwars"),
    },
  ].map((mode) => ({
    mode: mode.mode,
    kills: formatInt(mode.kills),
    deaths: formatInt(mode.deaths),
    kd: ratio(mode.kills, mode.deaths),
    finalKills: formatInt(mode.finalKills),
    finalDeaths: formatInt(mode.finalDeaths),
    fkd: ratio(mode.finalKills, mode.finalDeaths),
    wins: formatInt(mode.wins),
    losses: formatInt(mode.losses),
    wl: ratio(mode.wins, mode.losses),
    bedsBroken: formatInt(mode.bedsBroken),
    bedsLost: formatInt(mode.bedsLost),
    bbl: ratio(mode.bedsBroken, mode.bedsLost),
  }));
}

export function getBedwarsSpecialModes(stats: Record<string, unknown>) {
  const allSpecialModes: (BedwarsModeStats & { mode: string })[] = [
    {
      mode: "4v4",
      wins: getNumber(stats, "two_four_wins_bedwars"),
      losses: getNumber(stats, "two_four_losses_bedwars"),
      kills: getNumber(stats, "two_four_kills_bedwars"),
      deaths: getNumber(stats, "two_four_deaths_bedwars"),
      finalKills: getNumber(stats, "two_four_final_kills_bedwars"),
      finalDeaths: getNumber(stats, "two_four_final_deaths_bedwars"),
      bedsBroken: getNumber(stats, "two_four_beds_broken_bedwars"),
      bedsLost: getNumber(stats, "two_four_beds_lost_bedwars"),
    },
    {
      mode: "Rush Doubles",
      wins: getNumber(stats, "eight_two_rush_wins_bedwars"),
      losses: getNumber(stats, "eight_two_rush_losses_bedwars"),
      kills: getNumber(stats, "eight_two_rush_kills_bedwars"),
      deaths: getNumber(stats, "eight_two_rush_deaths_bedwars"),
      finalKills: getNumber(stats, "eight_two_rush_final_kills_bedwars"),
      finalDeaths: getNumber(stats, "eight_two_rush_final_deaths_bedwars"),
      bedsBroken: getNumber(stats, "eight_two_rush_beds_broken_bedwars"),
      bedsLost: getNumber(stats, "eight_two_rush_beds_lost_bedwars"),
    },
    {
      mode: "Rush 4v4v4v4",
      wins: getNumber(stats, "four_four_rush_wins_bedwars"),
      losses: getNumber(stats, "four_four_rush_losses_bedwars"),
      kills: getNumber(stats, "four_four_rush_kills_bedwars"),
      deaths: getNumber(stats, "four_four_rush_deaths_bedwars"),
      finalKills: getNumber(stats, "four_four_rush_final_kills_bedwars"),
      finalDeaths: getNumber(stats, "four_four_rush_final_deaths_bedwars"),
      bedsBroken: getNumber(stats, "four_four_rush_beds_broken_bedwars"),
      bedsLost: getNumber(stats, "four_four_rush_beds_lost_bedwars"),
    },
    {
      mode: "Ultimate Doubles",
      wins: getNumber(stats, "eight_two_ultimate_wins_bedwars"),
      losses: getNumber(stats, "eight_two_ultimate_losses_bedwars"),
      kills: getNumber(stats, "eight_two_ultimate_kills_bedwars"),
      deaths: getNumber(stats, "eight_two_ultimate_deaths_bedwars"),
      finalKills: getNumber(stats, "eight_two_ultimate_final_kills_bedwars"),
      finalDeaths: getNumber(stats, "eight_two_ultimate_final_deaths_bedwars"),
      bedsBroken: getNumber(stats, "eight_two_ultimate_beds_broken_bedwars"),
      bedsLost: getNumber(stats, "eight_two_ultimate_beds_lost_bedwars"),
    },
    {
      mode: "Ultimate 4v4v4v4",
      wins: getNumber(stats, "four_four_ultimate_wins_bedwars"),
      losses: getNumber(stats, "four_four_ultimate_losses_bedwars"),
      kills: getNumber(stats, "four_four_ultimate_kills_bedwars"),
      deaths: getNumber(stats, "four_four_ultimate_deaths_bedwars"),
      finalKills: getNumber(stats, "four_four_ultimate_final_kills_bedwars"),
      finalDeaths: getNumber(stats, "four_four_ultimate_final_deaths_bedwars"),
      bedsBroken: getNumber(stats, "four_four_ultimate_beds_broken_bedwars"),
      bedsLost: getNumber(stats, "four_four_ultimate_beds_lost_bedwars"),
    },
    {
      mode: "Lucky Doubles",
      wins: getNumber(stats, "eight_two_lucky_wins_bedwars"),
      losses: getNumber(stats, "eight_two_lucky_losses_bedwars"),
      kills: getNumber(stats, "eight_two_lucky_kills_bedwars"),
      deaths: getNumber(stats, "eight_two_lucky_deaths_bedwars"),
      finalKills: getNumber(stats, "eight_two_lucky_final_kills_bedwars"),
      finalDeaths: getNumber(stats, "eight_two_lucky_final_deaths_bedwars"),
      bedsBroken: getNumber(stats, "eight_two_lucky_beds_broken_bedwars"),
      bedsLost: getNumber(stats, "eight_two_lucky_beds_lost_bedwars"),
    },
    {
      mode: "Lucky 4v4v4v4",
      wins: getNumber(stats, "four_four_lucky_wins_bedwars"),
      losses: getNumber(stats, "four_four_lucky_losses_bedwars"),
      kills: getNumber(stats, "four_four_lucky_kills_bedwars"),
      deaths: getNumber(stats, "four_four_lucky_deaths_bedwars"),
      finalKills: getNumber(stats, "four_four_lucky_final_kills_bedwars"),
      finalDeaths: getNumber(stats, "four_four_lucky_final_deaths_bedwars"),
      bedsBroken: getNumber(stats, "four_four_lucky_beds_broken_bedwars"),
      bedsLost: getNumber(stats, "four_four_lucky_beds_lost_bedwars"),
    },
    {
      mode: "Voidless Doubles",
      wins: getNumber(stats, "eight_two_voidless_wins_bedwars"),
      losses: getNumber(stats, "eight_two_voidless_losses_bedwars"),
      kills: getNumber(stats, "eight_two_voidless_kills_bedwars"),
      deaths: getNumber(stats, "eight_two_voidless_deaths_bedwars"),
      finalKills: getNumber(stats, "eight_two_voidless_final_kills_bedwars"),
      finalDeaths: getNumber(stats, "eight_two_voidless_final_deaths_bedwars"),
      bedsBroken: getNumber(stats, "eight_two_voidless_beds_broken_bedwars"),
      bedsLost: getNumber(stats, "eight_two_voidless_beds_lost_bedwars"),
    },
    {
      mode: "Voidless 4v4v4v4",
      wins: getNumber(stats, "four_four_voidless_wins_bedwars"),
      losses: getNumber(stats, "four_four_voidless_losses_bedwars"),
      kills: getNumber(stats, "four_four_voidless_kills_bedwars"),
      deaths: getNumber(stats, "four_four_voidless_deaths_bedwars"),
      finalKills: getNumber(stats, "four_four_voidless_final_kills_bedwars"),
      finalDeaths: getNumber(stats, "four_four_voidless_final_deaths_bedwars"),
      bedsBroken: getNumber(stats, "four_four_voidless_beds_broken_bedwars"),
      bedsLost: getNumber(stats, "four_four_voidless_beds_lost_bedwars"),
    },
    {
      mode: "Armed Doubles",
      wins: getNumber(stats, "eight_two_armed_wins_bedwars"),
      losses: getNumber(stats, "eight_two_armed_losses_bedwars"),
      kills: getNumber(stats, "eight_two_armed_kills_bedwars"),
      deaths: getNumber(stats, "eight_two_armed_deaths_bedwars"),
      finalKills: getNumber(stats, "eight_two_armed_final_kills_bedwars"),
      finalDeaths: getNumber(stats, "eight_two_armed_final_deaths_bedwars"),
      bedsBroken: getNumber(stats, "eight_two_armed_beds_broken_bedwars"),
      bedsLost: getNumber(stats, "eight_two_armed_beds_lost_bedwars"),
    },
    {
      mode: "Armed 4v4v4v4",
      wins: getNumber(stats, "four_four_armed_wins_bedwars"),
      losses: getNumber(stats, "four_four_armed_losses_bedwars"),
      kills: getNumber(stats, "four_four_armed_kills_bedwars"),
      deaths: getNumber(stats, "four_four_armed_deaths_bedwars"),
      finalKills: getNumber(stats, "four_four_armed_final_kills_bedwars"),
      finalDeaths: getNumber(stats, "four_four_armed_final_deaths_bedwars"),
      bedsBroken: getNumber(stats, "four_four_armed_beds_broken_bedwars"),
      bedsLost: getNumber(stats, "four_four_armed_beds_lost_bedwars"),
    },
    {
      mode: "Swappage Doubles",
      wins: getNumber(stats, "eight_two_swap_wins_bedwars"),
      losses: getNumber(stats, "eight_two_swap_losses_bedwars"),
      kills: getNumber(stats, "eight_two_swap_kills_bedwars"),
      deaths: getNumber(stats, "eight_two_swap_deaths_bedwars"),
      finalKills: getNumber(stats, "eight_two_swap_final_kills_bedwars"),
      finalDeaths: getNumber(stats, "eight_two_swap_final_deaths_bedwars"),
      bedsBroken: getNumber(stats, "eight_two_swap_beds_broken_bedwars"),
      bedsLost: getNumber(stats, "eight_two_swap_beds_lost_bedwars"),
    },
    {
      mode: "Swappage 4v4v4v4",
      wins: getNumber(stats, "four_four_swap_wins_bedwars"),
      losses: getNumber(stats, "four_four_swap_losses_bedwars"),
      kills: getNumber(stats, "four_four_swap_kills_bedwars"),
      deaths: getNumber(stats, "four_four_swap_deaths_bedwars"),
      finalKills: getNumber(stats, "four_four_swap_final_kills_bedwars"),
      finalDeaths: getNumber(stats, "four_four_swap_final_deaths_bedwars"),
      bedsBroken: getNumber(stats, "four_four_swap_beds_broken_bedwars"),
      bedsLost: getNumber(stats, "four_four_swap_beds_lost_bedwars"),
    },
    {
      mode: "Underworld Doubles",
      wins: getNumber(stats, "eight_two_underworld_wins_bedwars"),
      losses: getNumber(stats, "eight_two_underworld_losses_bedwars"),
      kills: getNumber(stats, "eight_two_underworld_kills_bedwars"),
      deaths: getNumber(stats, "eight_two_underworld_deaths_bedwars"),
      finalKills: getNumber(stats, "eight_two_underworld_final_kills_bedwars"),
      finalDeaths: getNumber(
        stats,
        "eight_two_underworld_final_deaths_bedwars"
      ),
      bedsBroken: getNumber(stats, "eight_two_underworld_beds_broken_bedwars"),
      bedsLost: getNumber(stats, "eight_two_underworld_beds_lost_bedwars"),
    },
    {
      mode: "Underworld 4v4v4v4",
      wins: getNumber(stats, "four_four_underworld_wins_bedwars"),
      losses: getNumber(stats, "four_four_underworld_losses_bedwars"),
      kills: getNumber(stats, "four_four_underworld_kills_bedwars"),
      deaths: getNumber(stats, "four_four_underworld_deaths_bedwars"),
      finalKills: getNumber(stats, "four_four_underworld_final_kills_bedwars"),
      finalDeaths: getNumber(
        stats,
        "four_four_underworld_final_deaths_bedwars"
      ),
      bedsBroken: getNumber(stats, "four_four_underworld_beds_broken_bedwars"),
      bedsLost: getNumber(stats, "four_four_underworld_beds_lost_bedwars"),
    },
    {
      mode: "Castle",
      wins: getNumber(stats, "castle_wins_bedwars"),
      losses: getNumber(stats, "castle_losses_bedwars"),
      kills: getNumber(stats, "castle_kills_bedwars"),
      deaths: getNumber(stats, "castle_deaths_bedwars"),
      finalKills: getNumber(stats, "castle_final_kills_bedwars"),
      finalDeaths: getNumber(stats, "castle_final_deaths_bedwars"),
      bedsBroken: getNumber(stats, "castle_beds_broken_bedwars"),
      bedsLost: getNumber(stats, "castle_beds_lost_bedwars"),
    },
  ];

  return allSpecialModes
    .filter((mode) =>
      [
        mode.wins,
        mode.losses,
        mode.kills,
        mode.deaths,
        mode.finalKills,
        mode.finalDeaths,
        mode.bedsBroken,
        mode.bedsLost,
      ].some((v) => v > 0)
    )
    .map((mode) => ({
      mode: mode.mode,
      kills: formatInt(mode.kills),
      deaths: formatInt(mode.deaths),
      kd: ratio(mode.kills, mode.deaths),
      finalKills: formatInt(mode.finalKills),
      finalDeaths: formatInt(mode.finalDeaths),
      fkd: ratio(mode.finalKills, mode.finalDeaths),
      wins: formatInt(mode.wins),
      losses: formatInt(mode.losses),
      wl: ratio(mode.wins, mode.losses),
      bedsBroken: formatInt(mode.bedsBroken),
      bedsLost: formatInt(mode.bedsLost),
      bbl: ratio(mode.bedsBroken, mode.bedsLost),
    }));
}

export function createSummaryRow(label: string, data: BedwarsModeStats[]) {
  const totalWins = data.reduce((sum, mode) => sum + mode.wins, 0);
  const totalLosses = data.reduce((sum, mode) => sum + mode.losses, 0);
  const totalKills = data.reduce((sum, mode) => sum + mode.kills, 0);
  const totalDeaths = data.reduce((sum, mode) => sum + mode.deaths, 0);
  const totalFinalKills = data.reduce((sum, mode) => sum + mode.finalKills, 0);
  const totalFinalDeaths = data.reduce(
    (sum, mode) => sum + mode.finalDeaths,
    0
  );
  const totalBedsBroken = data.reduce((sum, mode) => sum + mode.bedsBroken, 0);
  const totalBedsLost = data.reduce((sum, mode) => sum + mode.bedsLost, 0);

  return {
    mode: <span className="font-bold">{label}</span>,
    kills: <span className="font-bold">{formatInt(totalKills)}</span>,
    deaths: <span className="font-bold">{formatInt(totalDeaths)}</span>,
    kd: <span className="font-bold">{ratio(totalKills, totalDeaths)}</span>,
    finalKills: (
      <span className="font-bold">{formatInt(totalFinalKills)}</span>
    ),
    finalDeaths: (
      <span className="font-bold">{formatInt(totalFinalDeaths)}</span>
    ),
    fkd: (
      <span className="font-bold">{ratio(totalFinalKills, totalFinalDeaths)}</span>
    ),
    wins: <span className="font-bold">{formatInt(totalWins)}</span>,
    losses: <span className="font-bold">{formatInt(totalLosses)}</span>,
    wl: <span className="font-bold">{ratio(totalWins, totalLosses)}</span>,
    bedsBroken: (
      <span className="font-bold">{formatInt(totalBedsBroken)}</span>
    ),
    bedsLost: <span className="font-bold">{formatInt(totalBedsLost)}</span>,
    bbl: (
      <span className="font-bold">{ratio(totalBedsBroken, totalBedsLost)}</span>
    ),
  };
}

