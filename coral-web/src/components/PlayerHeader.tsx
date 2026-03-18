import {
  getDisplayName,
  getPlusColor,
  getRank,
} from "@/lib/utils/hypixel/player/rank";
import { colorJSX } from "@/lib/utils/general/colors";
import { PlayerPills } from "./panels/pills/PlayerPills";

export function PlayerHeader({
  hypixel,
  username,
  uuid,
}: {
  hypixel: any;
  username: string;
  uuid: string;
}) {
  const resolvedRank = getRank(hypixel);
  const plusColorName =
    (hypixel?.rankPlusColor as string | undefined) ||
    (hypixel?.monthlyRankColor as string | undefined);
  const plusColorCode = getPlusColor(resolvedRank, plusColorName);
  const nameFromHypixel =
    (hypixel?.displayname as string | undefined) || username;
  const displayName = getDisplayName(
    nameFromHypixel,
    resolvedRank,
    plusColorCode
  );

  return (
    <div className="flex items-center gap-4">
      <img
        src={`https://vzge.me/full/384/${encodeURIComponent(uuid)}.png`}
        alt={`${username}'s skin`}
        width={64}
        height={64}
        className="rounded"
      />
      <div>
        <h1 className="text-3xl font-mc">
          <span>{colorJSX(displayName)}</span>
        </h1>
        <div className="mt-1 flex flex-wrap items-center gap-2">
          <PlayerPills />
        </div>
      </div>
    </div>
  );
}
