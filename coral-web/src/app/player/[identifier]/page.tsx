import { Metadata } from "next";
import { notFound, redirect, permanentRedirect } from "next/navigation";
import { internalGetJson } from "@/lib/utils/server/internal";
import { PlayerNav } from "@/components/PlayerNav";
import {
  BedwarsPanel,
  DuelsPanel,
  SkywarsPanel,
  PitPanel,
  QuakePanel,
} from "@/components/panels";
import { GeneralPanel } from "@/components/panels/player/GeneralPanel";
import { PlayerHeader } from "@/components/PlayerHeader";
import { isUuidLike } from "@/lib/utils/general/validate";
import { recordPlayerPopularity } from "@/lib/utils/server/popular";
import {
  getDisplayName,
  getPlusColor,
  getRank,
} from "@/lib/utils/hypixel/player/rank";

export async function generateMetadata({
  params,
}: {
  params: Promise<{ identifier: string }>;
}): Promise<Metadata> {
  const { identifier } = await params;
  let displayName: string | undefined;
  try {
    const pResp = await internalGetJson<{
      success: boolean;
      player: { id: string; username: string } | null;
    }>(`/api/playerdb?identifier=${encodeURIComponent(identifier)}`);
    displayName = pResp?.player?.username || undefined;
  } catch {}
  const title = displayName ? `${displayName}'s Stats` : `Player's Stats`;
  const desc = displayName
    ? `${displayName}'s Hypixel stats on Coral by Urchin.`
    : `Player's Hypixel stats on Coral by Urchin.`;
  const url = `/player/${encodeURIComponent(identifier)}`;
  return {
    title,
    description: desc,
    alternates: { canonical: url },
    openGraph: {
      title,
      description: desc,
      url,
      images: [
        {
          url: `/api/og/player?name=${encodeURIComponent(
            displayName || identifier
          )}`,
          width: 1200,
          height: 630,
        },
      ],
    },
    twitter: {
      card: "summary_large_image",
      title,
      description: desc,
      images: [
        `/api/og/player?name=${encodeURIComponent(displayName || identifier)}`,
      ],
    },
  };
}

export default async function PlayerPage({
  params,
}: {
  params: Promise<{ identifier: string }>;
}) {
  const { identifier } = await params;
  const looksLikeUuid = isUuidLike(identifier);
  let player: { id: string; username: string } | null = null;
  {
    const playerdb = await internalGetJson<{
      success: boolean;
      player: { id: string; username: string } | null;
    }>(`/api/playerdb?identifier=${encodeURIComponent(identifier)}`);
    player = playerdb?.player ?? null;
    if (!player && looksLikeUuid) {
      // fallback: allow viewing by raw UUID even if PlayerDB fails
      player = { id: identifier, username: identifier };
    }
  }
  if (!player) notFound();

  // redirect to uuid slug
  if (!looksLikeUuid && player.id) {
    permanentRedirect(`/player/${encodeURIComponent(player.id)}`);
  }

  const hypixelResponse = await internalGetJson<{
    success: boolean;
    player?: unknown;
  }>(`/api/hypixel?uuid=${encodeURIComponent(player.id)}`);
  const hypixelPlayer = hypixelResponse?.success
    ? (hypixelResponse.player as any | null)
    : null;

  if (hypixelResponse?.success === true && hypixelPlayer === null) {
    // never played on hypixel
    redirect(`/?e=np`);
  }
  if (hypixelResponse?.success === false) {
    // invalid/missing api key
    redirect(`/?e=iapikey`);
  }

  try {
    const resolvedRank = getRank(hypixelPlayer);
    const plusColorName =
      (hypixelPlayer as any)?.rankPlusColor ||
      (hypixelPlayer as any)?.monthlyRankColor;
    const plusColorCode = getPlusColor(resolvedRank, plusColorName);
    const nameFromHypixel =
      ((hypixelPlayer as any)?.displayname as string | undefined) ||
      player.username;
    const formattedDisplay = getDisplayName(
      nameFromHypixel,
      resolvedRank,
      plusColorCode
    );
    await recordPlayerPopularity(
      player.id,
      (hypixelPlayer as any)?.displayname || player.username,
      formattedDisplay
    );
  } catch {}

  return (
    <div className="min-h-screen">
      <PlayerNav />
      <div className="max-w-7xl mx-auto px-4 py-10">
        <meta name="coral:player:uuid" content={player.id} />
        <PlayerHeader
          hypixel={hypixelPlayer}
          username={player.username}
          uuid={player.id}
        />

        <div className="mt-8 grid grid-cols-1 lg:grid-cols-10 gap-6">
          <div className="lg:col-span-3">
            <GeneralPanel hypixel={hypixelPlayer} />
          </div>

          <div className="lg:col-span-7 space-y-4">
            <BedwarsPanel hypixel={hypixelPlayer} />
            <DuelsPanel hypixel={hypixelPlayer} />
            <SkywarsPanel hypixel={hypixelPlayer} />
            <PitPanel hypixel={hypixelPlayer} />
            <QuakePanel hypixel={hypixelPlayer} />
          </div>
        </div>
      </div>
    </div>
  );
}
