"use client";
import { SearchSuggest } from "@/components/common/SearchSuggest";

export default function Home() {
  const sp = new URLSearchParams(
    typeof window !== "undefined" ? window.location.search : ""
  );
  const e = sp.get("e") || undefined;
  const error =
    e === "inv"
      ? "Invalid player/UUID."
      : e === "np"
      ? "This player has never played on Hypixel before."
      : e === "iapikey"
      ? "Internal API error. Please try again later."
      : undefined;

  return (
    <div className="min-h-screen flex items-center justify-center p-6">
      <div className="w-full max-w-2xl text-center">
        <div className="mb-6 flex items-center justify-center gap-2">
          <img src="/logo.png" alt="Urchin" width={40} height={40} />
          <div className="flex items-baseline gap-2">
            <h1
              className="text-4xl font-bold"
              style={{ fontFamily: "var(--font-inter)" }}
            >
              Coral
            </h1>
            <span
              className="text-sm opacity-70"
              style={{ fontFamily: "var(--font-inter)" }}
            >
              by Urchin
            </span>
          </div>
        </div>
        <form
          action="/search"
          method="GET"
          autoComplete="off"
          className="mx-auto max-w-2xl"
        >
          <SearchSuggest
            placeholder="Search for a player..."
            inputHeightClass="h-12"
            buttonSizeClass="h-12 w-12"
            listMaxHeightClass="max-h-[280px]"
            rowHeightClass="h-11"
            imgSize={26}
            scrollClass="scroll-hidden"
            autoFocus={false}
          />
        </form>
        {error ? (
          <div className="mt-3 flex justify-center">
            <div className="rounded-md border border-red-300 bg-red-500/15 text-red-300 px-4 py-2 text-sm shadow-sm">
              {error}
            </div>
          </div>
        ) : null}
      </div>
    </div>
  );
}
