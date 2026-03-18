import { GameModePanelShell } from "../shells/GameModePanelShell";
import {
  StatDisplay,
  StatRow,
  StatSection,
} from "@/components/common/StatDisplay";
import { Stat } from "../../common/Stat";
import { ratio, duration, colorJSX, getNumber } from "@/lib/utils/general";
import { formatPitLevelByXp } from "@/lib/utils/hypixel/gamemodes/pit";

export function PitPanel({ hypixel }: { hypixel: any }) {
  const dirt = "/items/dirt.webp";
  const stats = hypixel?.stats?.Pit ?? {};
  const ptl = stats.pit_stats_ptl ?? {};
  const profile = stats.profile ?? {};

  const xp = getNumber(profile, "xp");
  const levelFormatted = formatPitLevelByXp(xp);
  const kills = getNumber(ptl, "kills");
  const deaths = getNumber(ptl, "deaths");
  const streak = getNumber(ptl, "max_streak");
  const gold = getNumber(profile, "cash");
  const goldEarned = getNumber(ptl, "cash_earned");
  const playtimeMinutes = getNumber(ptl, "playtime_minutes");

  return (
    <GameModePanelShell
      icon={dirt}
      title="Pit"
      headerRight={
        <>
          <Stat label="Level" value={colorJSX(levelFormatted)} />
          <Stat label="Kills" value={kills.toLocaleString()} />
          <Stat label="K/D" value={ratio(kills, deaths)} />
          <Stat
            label="Gold"
            value={colorJSX(
              `§6${gold.toLocaleString(undefined, {
                minimumFractionDigits: 0,
                maximumFractionDigits: 0,
              })}g§r`
            )}
          />
          <Stat label="Playtime" value={duration(playtimeMinutes)} />
        </>
      }
    >
      <StatDisplay>
        <StatSection columns={2}>
          <StatRow label="Level" value={colorJSX(levelFormatted)} />
          <StatRow label="Highest Streak" value={streak.toLocaleString()} />
          <StatRow label="Kills" value={kills.toLocaleString()} />
          <StatRow label="K/D" value={ratio(kills, deaths)} />
          <StatRow
            label="Gold"
            value={colorJSX(
              `§6${gold.toLocaleString(undefined, {
                minimumFractionDigits: 0,
                maximumFractionDigits: 0,
              })}g§r`
            )}
          />
          <StatRow
            label="Lifetime Gold"
            value={colorJSX(
              `§6${goldEarned.toLocaleString(undefined, {
                minimumFractionDigits: 0,
                maximumFractionDigits: 0,
              })}g§r`
            )}
          />
          <StatRow label="Playtime" value={duration(playtimeMinutes)} />
        </StatSection>
      </StatDisplay>
    </GameModePanelShell>
  );
}
