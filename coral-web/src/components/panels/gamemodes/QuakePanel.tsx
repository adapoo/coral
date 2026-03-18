import { GameModePanelShell } from "../shells/GameModePanelShell";
import {
  StatDisplay,
  StatRow,
  StatSection,
} from "@/components/common/StatDisplay";
import { Stat } from "../../common/Stat";
import { ratio, colorJSX, getString, formatInt } from "@/lib/utils/general";
import {
  getQuakePrefix,
  getQuakeTrigger,
} from "@/lib/utils/hypixel/gamemodes/quake";
import { sumNumbers } from "@/lib/utils/general";

export function QuakePanel({ hypixel }: { hypixel: any }) {
  const quake = "/items/firework_rocket.webp";
  const stats = hypixel?.stats?.Quake ?? {};

  const totalKills = sumNumbers(stats, "kills", "kills_teams");
  const totalDeaths = sumNumbers(stats, "deaths", "deaths_teams");
  const totalWins = sumNumbers(stats, "wins", "wins_teams");
  const totalKillstreaks = sumNumbers(
    stats,
    "killstreaks",
    "killstreaks_teams"
  );
  const prefix = getQuakePrefix(totalKills);
  const trigger = getQuakeTrigger(getString(stats, "trigger"));

  return (
    <GameModePanelShell
      icon={quake}
      title="Quakecraft"
      headerRight={
        <>
          <Stat label="Prefix" value={colorJSX(prefix)} />
          <Stat label="Kills" value={formatInt(totalKills)} />
          <Stat label="K/D" value={ratio(totalKills, totalDeaths)} />
          <Stat label="Wins" value={formatInt(totalWins)} />
          <Stat label="KS" value={formatInt(totalKillstreaks)} />
          <Stat label="Trigger" value={`${trigger.toFixed(1)}s`} />
        </>
      }
    >
      <StatDisplay>
        <StatSection columns={2}>
          <StatRow label="Prefix" value={colorJSX(prefix)} />
          <StatRow label="Kills" value={formatInt(totalKills)} />
          <StatRow label="K/D" value={ratio(totalKills, totalDeaths)} />
          <StatRow label="Wins" value={formatInt(totalWins)} />
          <StatRow label="Killstreaks" value={formatInt(totalKillstreaks)} />
          <StatRow label="Trigger" value={`${trigger.toFixed(1)}s`} />
        </StatSection>
      </StatDisplay>
    </GameModePanelShell>
  );
}
