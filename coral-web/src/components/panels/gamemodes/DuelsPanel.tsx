import {
  getDuelsDivision,
  getMostPlayedDuelsMode,
  getAllDuelsStats,
  getDuelsOverallWinstreak,
  DUELS_MODES,
} from "@/lib/utils/hypixel/gamemodes/duels";
import {
  buildModesByCategory,
  renderDivision,
  formatWinstreak,
  getMeleeHitRate,
  getArrowHitRate,
  getGoals,
  getBridgeKills,
  getBridgeDeaths,
  getOverallMeleeSwings,
  getOverallMeleeHits,
  getOverallMeleeHitRate,
  getOverallArrowShots,
  getOverallArrowHits,
  getOverallArrowHitRate,
  getCategoryMeleeHitRate,
  getCategoryArrowHitRate,
  getCategoryWinstreak,
  getTokens,
  DuelsModeStats,
} from "@/lib/utils/hypixel/gamemodes/duels/panel";
import { ratio, getNumber, formatInt } from "@/lib/utils/general";
import { GameModePanelShell } from "../shells/GameModePanelShell";
import { Stat } from "../../common/Stat";
import {
  StatDisplay,
  StatRow,
  StatSection,
  StatSpacer,
} from "@/components/common/StatDisplay";
import {
  DataTable,
  DataTableSection,
  TableSpacer,
} from "@/components/common/DataTable";

export function DuelsPanel({ hypixel }: { hypixel: any }) {
  const rod = "/items/fishing_rod.webp";
  const stats = hypixel?.stats?.Duels ?? {};

  const kills = getNumber(stats, "kills");
  const deaths = getNumber(stats, "deaths");
  const wins = getNumber(stats, "wins");
  const losses = getNumber(stats, "losses");
  const mostMode = getMostPlayedDuelsMode(stats);
  const { formatted } = getDuelsDivision(wins);

  const modesByCategory: Record<string, DuelsModeStats[]> =
    buildModesByCategory(stats);
  const overallStats = getAllDuelsStats(stats);

  return (
    <GameModePanelShell
      icon={rod}
      title="Duels"
      headerRight={
        <>
          <Stat label="Division" value={formatted} />
          <Stat label="Most Played" value={`§f${mostMode}`} />
          <Stat label="K/D" value={ratio(kills, deaths)} />
          <Stat label="Wins" value={formatInt(wins)} />
          <Stat label="W/L" value={ratio(wins, losses)} />
        </>
      }
    >
      <div className="space-y-6">
        <StatDisplay>
          <div className="grid grid-cols-3 gap-6">
            <StatSection columns={1}>
              <StatRow label="Division" value={formatted} />
              <StatRow label="Most Played Mode" value={mostMode} />
              <StatRow label="Games Played" value={formatInt(wins + losses)} />
              <StatSpacer />
              <StatRow label="Kills" value={formatInt(kills)} />
              <StatRow label="Deaths" value={formatInt(deaths)} />
              <StatRow label="Kill/Death Ratio" value={ratio(kills, deaths)} />
            </StatSection>

            <StatSection columns={1}>
              <StatRow
                label="Tokens"
                value={`§2${formatInt(getTokens(stats))}`}
              />
              <StatSpacer />
              <StatRow label="Wins" value={formatInt(wins)} />
              <StatRow label="Losses" value={formatInt(losses)} />
              <StatRow label="Win/Loss Ratio" value={ratio(wins, losses)} />
              <StatSpacer />
              <StatRow
                label="Current Winstreak"
                value={formatWinstreak(
                  getDuelsOverallWinstreak(stats, "current")
                )}
              />
              <StatRow
                label="Best Winstreak"
                value={formatWinstreak(getDuelsOverallWinstreak(stats, "best"))}
              />
            </StatSection>

            <StatSection columns={1}>
              <StatRow
                label="Melee Swings"
                value={formatInt(getOverallMeleeSwings(stats))}
              />
              <StatRow
                label="Melee Hits"
                value={formatInt(getOverallMeleeHits(stats))}
              />
              <StatRow
                label="Melee Hit Accuracy"
                value={getOverallMeleeHitRate(stats)}
              />
              <StatSpacer />
              <StatRow
                label="Arrows Shot"
                value={formatInt(getOverallArrowShots(stats))}
              />
              <StatRow
                label="Arrows Hit"
                value={formatInt(getOverallArrowHits(stats))}
              />
              <StatRow
                label="Arrow Hit Accuracy"
                value={getOverallArrowHitRate(stats)}
              />
            </StatSection>
          </div>
        </StatDisplay>

        <DataTableSection>
          <DataTable
            columns={[
              { key: "mode", label: "Mode", align: "left" },
              { key: "division", label: "Div", align: "center" },
              { key: "kills", label: "K", align: "right" },
              { key: "deaths", label: "D", align: "right" },
              { key: "kd", label: "K/D", align: "right" },
              { key: "wins", label: "W", align: "right" },
              { key: "losses", label: "L", align: "right" },
              { key: "wl", label: "W/L", align: "right" },
              { key: "meleeHM", label: "M%", align: "right" },
              { key: "arrowHM", label: "A%", align: "right" },
              { key: "goals", label: "Goals", align: "right" },
              { key: "currentWinstreak", label: "CWS", align: "right" },
              { key: "bestWinstreak", label: "BWS", align: "right" },
            ]}
            data={[
              ...Object.entries(modesByCategory).flatMap(
                ([category, modes]) => [
                  ...modes.map((mode) => {
                    const modeKey =
                      Object.keys(DUELS_MODES).find(
                        (key) =>
                          DUELS_MODES[key as keyof typeof DUELS_MODES].label ===
                          mode.mode
                      ) || "";
                    const bridgeKills = getBridgeKills(stats, modeKey);
                    const bridgeDeaths = getBridgeDeaths(stats, modeKey);
                    return {
                      mode: mode.mode,
                      division: renderDivision(
                        modes.reduce((sum, m) => sum + m.wins, 0)
                      ),
                      kills: formatInt(bridgeKills),
                      deaths: formatInt(bridgeDeaths),
                      kd: ratio(bridgeKills, bridgeDeaths),
                      wins: formatInt(mode.wins),
                      losses: formatInt(mode.losses),
                      wl: ratio(mode.wins, mode.losses),
                      meleeHM: getMeleeHitRate(stats, modeKey),
                      arrowHM: getArrowHitRate(stats, modeKey),
                      goals: getGoals(stats, modeKey),
                      currentWinstreak: formatWinstreak(mode.currentWinstreak),
                      bestWinstreak: formatWinstreak(mode.bestWinstreak),
                    };
                  }),
                  {
                    mode: <span className="font-bold">{category} Overall</span>,
                    division: renderDivision(
                      modes.reduce((sum, m) => sum + m.wins, 0)
                    ),
                    kills: (
                      <span className="font-bold">
                        {formatInt(
                          modes.reduce((sum, m) => {
                            const modeKey =
                              Object.keys(DUELS_MODES).find(
                                (key) =>
                                  DUELS_MODES[key as keyof typeof DUELS_MODES]
                                    .label === m.mode
                              ) || "";
                            return sum + getBridgeKills(stats, modeKey);
                          }, 0)
                        )}
                      </span>
                    ),
                    deaths: (
                      <span className="font-bold">
                        {formatInt(
                          modes.reduce((sum, m) => {
                            const modeKey =
                              Object.keys(DUELS_MODES).find(
                                (key) =>
                                  DUELS_MODES[key as keyof typeof DUELS_MODES]
                                    .label === m.mode
                              ) || "";
                            return sum + getBridgeDeaths(stats, modeKey);
                          }, 0)
                        )}
                      </span>
                    ),
                    kd: (
                      <span className="font-bold">
                        {(() => {
                          const totalKills = modes.reduce((sum, m) => {
                            const modeKey =
                              Object.keys(DUELS_MODES).find(
                                (key) =>
                                  DUELS_MODES[key as keyof typeof DUELS_MODES]
                                    .label === m.mode
                              ) || "";
                            return sum + getBridgeKills(stats, modeKey);
                          }, 0);
                          const totalDeaths = modes.reduce((sum, m) => {
                            const modeKey =
                              Object.keys(DUELS_MODES).find(
                                (key) =>
                                  DUELS_MODES[key as keyof typeof DUELS_MODES]
                                    .label === m.mode
                              ) || "";
                            return sum + getBridgeDeaths(stats, modeKey);
                          }, 0);
                          return ratio(totalKills, totalDeaths);
                        })()}
                      </span>
                    ),
                    wins: (
                      <span className="font-bold">
                        {formatInt(modes.reduce((sum, m) => sum + m.wins, 0))}
                      </span>
                    ),
                    losses: (
                      <span className="font-bold">
                        {formatInt(modes.reduce((sum, m) => sum + m.losses, 0))}
                      </span>
                    ),
                    wl: (
                      <span className="font-bold">
                        {ratio(
                          modes.reduce((sum, m) => sum + m.wins, 0),
                          modes.reduce((sum, m) => sum + m.losses, 0)
                        )}
                      </span>
                    ),
                    meleeHM: (
                      <span className="font-bold">
                        {getCategoryMeleeHitRate(stats, modes)}
                      </span>
                    ),
                    arrowHM: (
                      <span className="font-bold">
                        {getCategoryArrowHitRate(stats, modes)}
                      </span>
                    ),
                    goals: <span className="font-bold">-</span>,
                    currentWinstreak: (
                      <span className="font-bold">
                        {getCategoryWinstreak(stats, category, "current")}
                      </span>
                    ),
                    bestWinstreak: (
                      <span className="font-bold">
                        {getCategoryWinstreak(stats, category, "best")}
                      </span>
                    ),
                  },
                  <TableSpacer key={`spacer-${category}`} />,
                ]
              ),
              {
                mode: <span className="font-bold">Overall</span>,
                division: renderDivision(overallStats.wins),
                kills: (
                  <span className="font-bold">
                    {formatInt(overallStats.kills)}
                  </span>
                ),
                deaths: (
                  <span className="font-bold">
                    {formatInt(overallStats.deaths)}
                  </span>
                ),
                kd: (
                  <span className="font-bold">
                    {ratio(overallStats.kills, overallStats.deaths)}
                  </span>
                ),
                wins: (
                  <span className="font-bold">
                    {formatInt(overallStats.wins)}
                  </span>
                ),
                losses: (
                  <span className="font-bold">
                    {formatInt(overallStats.losses)}
                  </span>
                ),
                wl: (
                  <span className="font-bold">
                    {ratio(overallStats.wins, overallStats.losses)}
                  </span>
                ),
                meleeHM: (
                  <span className="font-bold">
                    {getOverallMeleeHitRate(stats)}
                  </span>
                ),
                arrowHM: (
                  <span className="font-bold">
                    {getOverallArrowHitRate(stats)}
                  </span>
                ),
                goals: <span className="font-bold">-</span>,
                currentWinstreak: (
                  <span className="font-bold">
                    {formatWinstreak(
                      getDuelsOverallWinstreak(stats, "current")
                    )}
                  </span>
                ),
                bestWinstreak: (
                  <span className="font-bold">
                    {formatWinstreak(getDuelsOverallWinstreak(stats, "best"))}
                  </span>
                ),
              },
            ]}
          />
        </DataTableSection>
      </div>
    </GameModePanelShell>
  );
}
