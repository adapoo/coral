"use client";

import { useEffect, useRef, useState } from "react";
import { Search } from "lucide-react";
import { colorJSX } from "@/lib/utils/general/colors";

type PlayerItem = {
  uuid: string;
  name?: string | null;
  display?: string | null;
};

export function SearchSuggest({
  placeholder = "Search for a player...",
  inputHeightClass = "h-10",
  buttonSizeClass = "h-10 w-10",
  listMaxHeightClass = "max-h-[240px]",
  rowHeightClass = "h-10",
  imgSize = 24,
  scrollClass = "scroll-hidden",
  autoFocus = false,
}: {
  placeholder?: string;
  inputHeightClass?: string;
  buttonSizeClass?: string;
  listMaxHeightClass?: string;
  rowHeightClass?: string;
  imgSize?: number;
  scrollClass?: string;
  autoFocus?: boolean;
}) {
  const [q, setQ] = useState("");
  const [open, setOpen] = useState(false);
  const [preloaded, setPreloaded] = useState<PlayerItem[]>([]);
  const [suggestions, setSuggestions] = useState<PlayerItem[]>([]);
  const isNavigatingRef = useRef<boolean>(false);

  useEffect(() => {
    (async () => {
      try {
        const r = await fetch(`/api/top?limit=200`);
        if (!r.ok) return;
        const json = (await r.json()) as { players: PlayerItem[] };
        const list = json.players || [];
        setPreloaded(list);
        setSuggestions(list.filter((s) => !!s.display));
      } catch {}
    })();
  }, []);

  useEffect(() => {
    if (!open) return;
    const list = q.trim()
      ? preloaded.filter(
          (s) =>
            !!s.display &&
            !!s.name &&
            s.name.toLowerCase().startsWith(q.toLowerCase())
        )
      : preloaded.filter((s) => !!s.display);
    setSuggestions(list);
  }, [q, open, preloaded]);

  return (
    <div className="relative">
      <div className="surface-panel p-0">
        <input
          type="text"
          name="query"
          placeholder={placeholder}
          className={`w-full ${inputHeightClass} rounded-md pl-4 pr-10 bg-transparent outline-none`}
          style={{ fontFamily: "var(--font-inter)" }}
          aria-label="Player username or UUID"
          autoComplete="off"
          autoCorrect="off"
          autoCapitalize="none"
          spellCheck={false}
          value={q}
          onChange={(e) => setQ(e.target.value)}
          onFocus={() => setOpen(true)}
          onBlur={() => setTimeout(() => setOpen(false), 120)}
          autoFocus={autoFocus}
        />
        <button
          type="submit"
          className={`absolute right-0 top-1/2 -translate-y-1/2 ${buttonSizeClass} grid place-items-center rounded-md text-foreground disabled:opacity-60`}
          aria-label="Search"
        >
          <Search
            size={inputHeightClass === "h-9" ? 16 : 20}
            strokeWidth={2.25}
          />
        </button>
      </div>
      {open && suggestions.length > 0 ? (
        <div className="surface-panel absolute left-0 right-0 top-[calc(100%+4px)] z-20 rounded-md overflow-hidden">
          <div
            className={`${listMaxHeightClass} overflow-y-auto overscroll-contain ${scrollClass}`}
          >
            <ul>
              {(q.trim() ? suggestions : suggestions).slice(0, 15).map((s) => (
                <li
                  key={s.uuid}
                  className={`flex items-center gap-2 px-3 ${rowHeightClass} hover:bg-foreground/10 cursor-pointer`}
                  onClick={(e) => {
                    e.preventDefault();
                    if (isNavigatingRef.current) {
                      return;
                    }
                    isNavigatingRef.current = true;
                    window.location.href = `/player/${encodeURIComponent(
                      s.uuid
                    )}`;
                  }}
                >
                  {/* eslint-disable-next-line @next/next/no-img-element */}
                  <img
                    src={`https://vzge.me/face/256/${encodeURIComponent(
                      s.uuid
                    )}.png`}
                    alt="head"
                    width={imgSize}
                    height={imgSize}
                  />
                  <span className="text-base font-mc">
                    {colorJSX(s.display!)}
                  </span>
                </li>
              ))}
            </ul>
          </div>
        </div>
      ) : null}
    </div>
  );
}
