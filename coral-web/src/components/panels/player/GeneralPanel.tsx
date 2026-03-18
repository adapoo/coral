import { getNetworkLevel } from "@/lib/utils/hypixel/player/network";
import { formatInt } from "@/lib/utils/general";
import { colorJSX } from "@/lib/utils/general/colors";
import { PlayerPanelShell } from "../shells/PlayerPanelShell";

export function GeneralPanel({ hypixel }: { hypixel: any }) {
  const firstLogin = hypixel?.firstLogin
    ? new Date(hypixel.firstLogin)
        .toLocaleString("en-US", {
          timeZone: "America/New_York",
          year: "numeric",
          month: "2-digit",
          day: "2-digit",
        })
        .replace(
          /(\d{2})\/(\d{2})\/(\d{4}),\s*/,
          (_m, mm, dd, yyyy) => `${yyyy}-${mm}-${dd}, `
        )
    : "Hidden";
  const lastLogin = hypixel?.lastLogin
    ? new Date(hypixel.lastLogin)
        .toLocaleString("en-US", {
          timeZone: "America/New_York",
          year: "numeric",
          month: "2-digit",
          day: "2-digit",
        })
        .replace(
          /(\d{2})\/(\d{2})\/(\d{4}),\s*/,
          (_m, mm, dd, yyyy) => `${yyyy}-${mm}-${dd}, `
        )
    : "Hidden";

  const networkLevel = getNetworkLevel(hypixel?.networkExp ?? 0);
  const coinMultiplier = Math.max(
    1,
    Math.min(10, Math.floor(networkLevel / 25) + 1)
  );
  const karma = hypixel?.karma ?? 0;
  const achievementPoints = hypixel?.achievementPoints ?? 0;
  const questsCompleted = hypixel?.achievementsOneTime?.length ?? 0;
  const challengesCompleted =
    hypixel?.challenges?.all_time &&
    typeof hypixel.challenges.all_time === "object"
      ? Object.values(
          hypixel.challenges.all_time as Record<string, number>
        ).reduce(
          (sum, value) => sum + (typeof value === "number" ? value : 0),
          0
        )
      : 0;

  function Stat({
    label,
    value,
    color,
  }: {
    label: string;
    value: React.ReactNode;
    color?: string;
  }) {
    return (
      <div className="flex items-baseline justify-between gap-4">
        <div
          className="text-xxxs tracking-wide opacity-70"
          style={{ fontFamily: "var(--font-inter)" }}
        >
          {label}
        </div>
        <div className="text-base font-mc">
          {typeof value === "string" && color
            ? colorJSX(
                `§${color
                  .replace("var(--color-", "")
                  .replace(")", "")}${value}§r`
              )
            : typeof value === "string"
            ? colorJSX(`§7${value}`)
            : value}
        </div>
      </div>
    );
  }

  return (
    <PlayerPanelShell>
      <div className="space-y-2 text-sm">
        <Stat label="Hypixel Level" value={colorJSX(`§a${networkLevel}`)} />
        <Stat label="Karma" value={formatInt(karma)} color="var(--color-d)" />
        <Stat
          label="Coin Multiplier"
          value={colorJSX(
            `§6x${coinMultiplier}§7 (Level ${Math.floor(networkLevel)})`
          )}
        />
        <Stat
          label="Achievement Points"
          value={formatInt(achievementPoints)}
          color="var(--color-e)"
        />
        <Stat
          label="Quests Completed"
          value={formatInt(questsCompleted)}
          color="var(--color-b)"
        />
        <Stat
          label="Challenges Completed"
          value={formatInt(challengesCompleted)}
          color="var(--color-3)"
        />
        <Stat label="First Login" value={firstLogin} color="var(--color-e)" />
        <Stat
          label="Last Login"
          value={hypixel?.lastLogin ? lastLogin : "Hidden"}
          color={hypixel?.lastLogin ? "var(--color-e)" : "var(--color-c)"}
        />
      </div>
    </PlayerPanelShell>
  );
}
