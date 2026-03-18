"use client";

import { useState } from "react";
import Image from "next/image";
import { PillShell } from "@/components/panels/shells/PillShell";
import { TooltipShell } from "@/components/panels/shells/TooltipShell";
import { relativeTime } from "@/lib/utils/general/format";

export function UrchinTagPill({
  type,
  label,
  reason,
  addedBy,
  addedOn,
}: {
  type: string;
  label: string;
  reason?: string;
  addedBy?: string | null;
  addedOn?: string | null;
}) {
  const [hover, setHover] = useState(false);
  const MIN_DETAILS_WIDTH = 320;
  const MAX_DETAILS_WIDTH = 520;

  return (
    <div
      className="mt-1 relative inline-flex items-center"
      onMouseEnter={() => setHover(true)}
      onMouseLeave={() => setHover(false)}
    >
      <PillShell>
        <Image
          src={`/urchin/${type}.webp`}
          alt={label}
          width={14}
          height={14}
        />
        <span
          className="font-semibold"
          style={{ fontFamily: "var(--font-inter)" }}
        >
          {label}
        </span>
      </PillShell>
      <TooltipShell
        hover={hover}
        minWidth={MIN_DETAILS_WIDTH}
        maxWidth={MAX_DETAILS_WIDTH}
      >
        {reason ? (
          <div className="leading-snug">
            {reason.length > 1000 ? `${reason.slice(0, 1000)}…` : reason}
          </div>
        ) : null}
        {addedBy || addedOn ? (
          <div className="opacity-75 mt-1 whitespace-nowrap">
            {addedBy ? (
              <>
                Added by <span className="opacity-90">@{addedBy}</span>
                {addedOn
                  ? ` · ${
                      relativeTime(
                        addedOn && !addedOn.endsWith("Z")
                          ? `${addedOn}Z`
                          : addedOn
                      ) ?? ""
                    }`
                  : ""}
              </>
            ) : (
              <>
                Added{" "}
                {relativeTime(
                  addedOn && !addedOn.endsWith("Z") ? `${addedOn}Z` : addedOn
                )}
              </>
            )}
          </div>
        ) : null}
      </TooltipShell>
    </div>
  );
}
