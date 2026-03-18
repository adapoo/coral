import {
  ratio,
  colorJSX,
  getNumber,
  formatInt,
  getOptionalString,
} from "@/lib/utils/general";
import {
  getSkywarsLevel,
  formatSkywarsLevel,
} from "@/lib/utils/hypixel/gamemodes/skywars";
import { GameModePanelShell } from "../shells/GameModePanelShell";
import { Stat } from "../../common/Stat";
import {
  StatDisplay,
  StatRow,
  StatSection,
} from "@/components/common/StatDisplay";

export function SkywarsPanel({ hypixel }: { hypixel: any }) {
  const stats = hypixel?.stats?.SkyWars ?? {};
  const xp = getNumber(stats, "skywars_experience");
  const level = getSkywarsLevel(xp);
  const star_intended = getOptionalString(stats, "levelFormattedWithBrackets");
  const star = formatSkywarsLevel(
    level,
    getOptionalString(stats, "active_scheme") ?? undefined,
    getOptionalString(stats, "active_emblem") ?? undefined,
    false,
    false,
    false
  );
  const eye = "/items/ender_eye.webp";

  const wins = getNumber(stats, "wins");
  const losses = getNumber(stats, "losses");
  const kills = getNumber(stats, "kills");
  const deaths = getNumber(stats, "deaths");

  return (
    <GameModePanelShell
      icon={eye}
      title="SkyWars"
      headerRight={
        <>
          <Stat label="Level" value={colorJSX(star_intended ?? star)} />
          <Stat label="Kills" value={formatInt(kills)} />
          <Stat label="K/D" value={ratio(kills, deaths)} />
          <Stat label="Wins" value={formatInt(wins)} />
          <Stat label="W/L" value={ratio(wins, losses)} />
        </>
      }
    >
      <StatDisplay>
        <StatSection columns={2}>
          <StatRow label="Level" value={colorJSX(star_intended ?? star)} />
          <StatRow label="Kills" value={formatInt(kills)} />
          <StatRow label="K/D" value={ratio(kills, deaths)} />
          <StatRow label="Wins" value={formatInt(wins)} />
          <StatRow label="W/L" value={ratio(wins, losses)} />
        </StatSection>
      </StatDisplay>
    </GameModePanelShell>
  );
}
