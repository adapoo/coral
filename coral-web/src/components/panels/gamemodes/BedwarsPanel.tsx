import {
  ratio,
  getNumber,
  getString,
  getOptionalNumber,
  formatInt,
  C,
} from "@/lib/utils/general";
import {
  getBedwarsLevel,
  formatBedwarsStar,
  getBedwarsExpRequirement,
  getBedwarsProgressBarColors,
  getBedwarsPrestige,
  getBedwarsHighestDoor,
  getBedwarsWalletCapacity,
} from "@/lib/utils/hypixel/gamemodes/bedwars";
import {
  getBedwarsModes,
  getBedwarsSpecialModes,
  createSummaryRow,
} from "@/lib/utils/hypixel/gamemodes/bedwars/panel";
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
import { LevelingProgress } from "@/components/common/LevelingProgress";

export function BedwarsPanel({ hypixel }: { hypixel: any }) {
  const bed = "/items/bed.webp";
  const stats = hypixel?.stats?.Bedwars ?? {};
  const slumberHotel = stats.slumber ?? {};

  const exp = getNumber(stats, "Experience");
  const wins = getNumber(stats, "wins_bedwars");
  const losses = getNumber(stats, "losses_bedwars");
  const kills = getNumber(stats, "kills_bedwars");
  const deaths = getNumber(stats, "deaths_bedwars");
  const fk = getNumber(stats, "final_kills_bedwars");
  const fd = getNumber(stats, "final_deaths_bedwars");
  const bb = getNumber(stats, "beds_broken_bedwars");
  const bl = getNumber(stats, "beds_lost_bedwars");
  const ws = getOptionalNumber(stats, "winstreak");
  const winstreakDisplay = ws === null ? "§c?" : formatInt(ws);

  const tokens = getNumber(stats, "coins");
  const slumberTickets = getNumber(slumberHotel, "tickets");
  const lifetimeSlumberTickets = getNumber(
    slumberHotel,
    "total_tickets_earned"
  );

  const highestDoor = getBedwarsHighestDoor(slumberHotel?.room ?? {});
  const walletCapacity = getBedwarsWalletCapacity(
    getString(slumberHotel, "bag_type") || ""
  );
  const ironCollected = getNumber(stats, "iron_resources_collected_bedwars");
  const goldCollected = getNumber(stats, "gold_resources_collected_bedwars");
  const diamondsCollected = getNumber(
    stats,
    "diamond_resources_collected_bedwars"
  );
  const emeraldsCollected = getNumber(
    stats,
    "emerald_resources_collected_bedwars"
  );
  const shopPurchases = getNumber(stats, "items_purchased_bedwars");

  const level = getBedwarsLevel(exp);
  const star = formatBedwarsStar(Math.floor(level));
  const prestige = getBedwarsPrestige(level);

  const currentLevel = Math.floor(level);
  const nextLevel = currentLevel + 1;
  const prestiges = Math.floor(exp / 487000);
  let expIntoCurrentPrestige = exp - prestiges * 487000;

  let expUsedInPrestige = 0;
  for (let i = 0; i < currentLevel % 100; i++) {
    expUsedInPrestige += getBedwarsExpRequirement(i);
  }

  const currentLevelExp = expIntoCurrentPrestige - expUsedInPrestige;
  const requiredForNext = getBedwarsExpRequirement(currentLevel % 100);
  const modes = getBedwarsModes(stats);
  const specialModes = getBedwarsSpecialModes(stats);

  return (
    <GameModePanelShell
      icon={bed}
      title="Bed Wars"
      headerRight={
        <>
          <Stat label="Level" value={star} />
          <Stat label="K/D" value={ratio(kills, deaths)} />
          <Stat label="F. Kills" value={formatInt(fk)} />
          <Stat label="FK/D" value={ratio(fk, fd)} />
          <Stat label="Wins" value={formatInt(wins)} />
          <Stat label="W/L" value={ratio(wins, losses)} />
        </>
      }
    >
      <div className="space-y-6">
        <LevelingProgress
          currentDisplay={star}
          nextDisplay={formatBedwarsStar(nextLevel)}
          current={Math.max(0, currentLevelExp)}
          required={requiredForNext}
          gradientColors={getBedwarsProgressBarColors(currentLevel)}
          tooltipContent={{
            current: `${C.GREEN}${formatInt(Math.max(0, currentLevelExp))}`,
            required: `${C.AQUA}${formatInt(requiredForNext)} XP`,
          }}
        />

        <StatDisplay>
          <div className="grid grid-cols-3 gap-6">
            <StatSection columns={1}>
              <StatRow label="Level" value={formatInt(currentLevel)} />
              <StatRow label="Prestige" value={prestige} />
              <StatSpacer />
              <StatRow label="Kills" value={formatInt(kills)} />
              <StatRow label="Deaths" value={formatInt(deaths)} />
              <StatRow label="Kill/Death Ratio" value={ratio(kills, deaths)} />
              <StatSpacer />
              <StatRow label="Final Kills" value={formatInt(fk)} />
              <StatRow label="Final Deaths" value={formatInt(fd)} />
              <StatRow label="Final Kill/Death Ratio" value={ratio(fk, fd)} />
            </StatSection>

            <StatSection columns={1}>
              <StatRow label="Tokens" value={`§2${formatInt(tokens)}`} />
              <StatRow label="Winstreak" value={winstreakDisplay} />
              <StatSpacer />
              <StatRow label="Wins" value={formatInt(wins)} />
              <StatRow label="Losses" value={formatInt(losses)} />
              <StatRow label="Win/Loss Ratio" value={ratio(wins, losses)} />
              <StatSpacer />
              <StatRow label="Beds Broken" value={formatInt(bb)} />
              <StatRow label="Beds Lost" value={formatInt(bl)} />
              <StatRow label="Beds Broken/Lost Ratio" value={ratio(bb, bl)} />
            </StatSection>

            <StatSection columns={1}>
              <StatRow label="Door Unlocked" value={`§a${highestDoor}`} />
              <StatRow
                label="Slumber Tickets"
                value={`§b${formatInt(slumberTickets)}§7/${formatInt(
                  walletCapacity
                )}`}
              />
              <StatRow
                label="Lifetime Slumber Tickets"
                value={`§3${formatInt(lifetimeSlumberTickets)}§7`}
              />
              <StatSpacer />
              <StatRow
                label="Shop Purchases"
                value={`§e${formatInt(shopPurchases)}`}
              />
              <StatSpacer />
              <StatRow
                label="Iron Collected"
                value={`§7${formatInt(ironCollected)}`}
              />
              <StatRow
                label="Gold Collected"
                value={`§6${formatInt(goldCollected)}`}
              />
              <StatRow
                label="Diamonds Collected"
                value={`§b${formatInt(diamondsCollected)}`}
              />
              <StatRow
                label="Emeralds Collected"
                value={`§a${formatInt(emeraldsCollected)}`}
              />
            </StatSection>
          </div>
        </StatDisplay>

        <DataTableSection>
          <DataTable
            columns={[
              { key: "mode", label: "Mode", align: "left" },
              { key: "kills", label: "K", align: "right" },
              { key: "deaths", label: "D", align: "right" },
              { key: "kd", label: "K/D", align: "right" },
              { key: "finalKills", label: "FK", align: "right" },
              { key: "finalDeaths", label: "FD", align: "right" },
              { key: "fkd", label: "FK/D", align: "right" },
              { key: "wins", label: "W", align: "right" },
              { key: "losses", label: "L", align: "right" },
              { key: "wl", label: "W/L", align: "right" },
              { key: "bedsBroken", label: "BB", align: "right" },
              { key: "bedsLost", label: "BL", align: "right" },
              { key: "bbl", label: "BB/L", align: "right" },
            ]}
            data={[
              ...modes,
              createSummaryRow("Core Modes", [
                {
                  wins: getNumber(stats, "eight_one_wins_bedwars"),
                  losses: getNumber(stats, "eight_one_losses_bedwars"),
                  kills: getNumber(stats, "eight_one_kills_bedwars"),
                  deaths: getNumber(stats, "eight_one_deaths_bedwars"),
                  finalKills: getNumber(stats, "eight_one_final_kills_bedwars"),
                  finalDeaths: getNumber(
                    stats,
                    "eight_one_final_deaths_bedwars"
                  ),
                  bedsBroken: getNumber(stats, "eight_one_beds_broken_bedwars"),
                  bedsLost: getNumber(stats, "eight_one_beds_lost_bedwars"),
                },
                {
                  wins: getNumber(stats, "eight_two_wins_bedwars"),
                  losses: getNumber(stats, "eight_two_losses_bedwars"),
                  kills: getNumber(stats, "eight_two_kills_bedwars"),
                  deaths: getNumber(stats, "eight_two_deaths_bedwars"),
                  finalKills: getNumber(stats, "eight_two_final_kills_bedwars"),
                  finalDeaths: getNumber(
                    stats,
                    "eight_two_final_deaths_bedwars"
                  ),
                  bedsBroken: getNumber(stats, "eight_two_beds_broken_bedwars"),
                  bedsLost: getNumber(stats, "eight_two_beds_lost_bedwars"),
                },
                {
                  wins: getNumber(stats, "four_three_wins_bedwars"),
                  losses: getNumber(stats, "four_three_losses_bedwars"),
                  kills: getNumber(stats, "four_three_kills_bedwars"),
                  deaths: getNumber(stats, "four_three_deaths_bedwars"),
                  finalKills: getNumber(
                    stats,
                    "four_three_final_kills_bedwars"
                  ),
                  finalDeaths: getNumber(
                    stats,
                    "four_three_final_deaths_bedwars"
                  ),
                  bedsBroken: getNumber(
                    stats,
                    "four_three_beds_broken_bedwars"
                  ),
                  bedsLost: getNumber(stats, "four_three_beds_lost_bedwars"),
                },
                {
                  wins: getNumber(stats, "four_four_wins_bedwars"),
                  losses: getNumber(stats, "four_four_losses_bedwars"),
                  kills: getNumber(stats, "four_four_kills_bedwars"),
                  deaths: getNumber(stats, "four_four_deaths_bedwars"),
                  finalKills: getNumber(stats, "four_four_final_kills_bedwars"),
                  finalDeaths: getNumber(
                    stats,
                    "four_four_final_deaths_bedwars"
                  ),
                  bedsBroken: getNumber(stats, "four_four_beds_broken_bedwars"),
                  bedsLost: getNumber(stats, "four_four_beds_lost_bedwars"),
                },
              ]),
              <TableSpacer key="spacer" />,
              ...specialModes,
              createSummaryRow("Overall", [
                {
                  wins: getNumber(stats, "eight_one_wins_bedwars"),
                  losses: getNumber(stats, "eight_one_losses_bedwars"),
                  kills: getNumber(stats, "eight_one_kills_bedwars"),
                  deaths: getNumber(stats, "eight_one_deaths_bedwars"),
                  finalKills: getNumber(stats, "eight_one_final_kills_bedwars"),
                  finalDeaths: getNumber(
                    stats,
                    "eight_one_final_deaths_bedwars"
                  ),
                  bedsBroken: getNumber(stats, "eight_one_beds_broken_bedwars"),
                  bedsLost: getNumber(stats, "eight_one_beds_lost_bedwars"),
                },
                {
                  wins: getNumber(stats, "eight_two_wins_bedwars"),
                  losses: getNumber(stats, "eight_two_losses_bedwars"),
                  kills: getNumber(stats, "eight_two_kills_bedwars"),
                  deaths: getNumber(stats, "eight_two_deaths_bedwars"),
                  finalKills: getNumber(stats, "eight_two_final_kills_bedwars"),
                  finalDeaths: getNumber(
                    stats,
                    "eight_two_final_deaths_bedwars"
                  ),
                  bedsBroken: getNumber(stats, "eight_two_beds_broken_bedwars"),
                  bedsLost: getNumber(stats, "eight_two_beds_lost_bedwars"),
                },
                {
                  wins: getNumber(stats, "four_three_wins_bedwars"),
                  losses: getNumber(stats, "four_three_losses_bedwars"),
                  kills: getNumber(stats, "four_three_kills_bedwars"),
                  deaths: getNumber(stats, "four_three_deaths_bedwars"),
                  finalKills: getNumber(
                    stats,
                    "four_three_final_kills_bedwars"
                  ),
                  finalDeaths: getNumber(
                    stats,
                    "four_three_final_deaths_bedwars"
                  ),
                  bedsBroken: getNumber(
                    stats,
                    "four_three_beds_broken_bedwars"
                  ),
                  bedsLost: getNumber(stats, "four_three_beds_lost_bedwars"),
                },
                {
                  wins: getNumber(stats, "four_four_wins_bedwars"),
                  losses: getNumber(stats, "four_four_losses_bedwars"),
                  kills: getNumber(stats, "four_four_kills_bedwars"),
                  deaths: getNumber(stats, "four_four_deaths_bedwars"),
                  finalKills: getNumber(stats, "four_four_final_kills_bedwars"),
                  finalDeaths: getNumber(
                    stats,
                    "four_four_final_deaths_bedwars"
                  ),
                  bedsBroken: getNumber(stats, "four_four_beds_broken_bedwars"),
                  bedsLost: getNumber(stats, "four_four_beds_lost_bedwars"),
                },
                {
                  wins: getNumber(stats, "two_four_wins_bedwars"),
                  losses: getNumber(stats, "two_four_losses_bedwars"),
                  kills: getNumber(stats, "two_four_kills_bedwars"),
                  deaths: getNumber(stats, "two_four_deaths_bedwars"),
                  finalKills: getNumber(stats, "two_four_final_kills_bedwars"),
                  finalDeaths: getNumber(
                    stats,
                    "two_four_final_deaths_bedwars"
                  ),
                  bedsBroken: getNumber(stats, "two_four_beds_broken_bedwars"),
                  bedsLost: getNumber(stats, "two_four_beds_lost_bedwars"),
                },
              ]),
            ]}
          />
        </DataTableSection>
      </div>
    </GameModePanelShell>
  );
}
