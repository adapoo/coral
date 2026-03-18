"use client";

import Link from "next/link";
import Image from "next/image";
import { SearchSuggest } from "@/components/common/SearchSuggest";

export function PlayerNav() {
  return (
    <header>
      <div className="max-w-7xl mx-auto flex items-center gap-3 p-3">
        <Link href="/" className="flex items-center gap-2">
          <Image src="/logo.png" alt="Urchin" width={20} height={20} />
          <span className="flex items-baseline gap-2">
            <span
              className="text-xl font-bold"
              style={{ fontFamily: "var(--font-inter)" }}
            >
              Coral
            </span>
            <span
              className="text-sm opacity-70"
              style={{ fontFamily: "var(--font-inter)" }}
            >
              by Urchin
            </span>
          </span>
        </Link>
        <form
          action="/search"
          method="GET"
          autoComplete="off"
          className="ml-auto w-full max-w-[24rem]"
        >
          <SearchSuggest
            placeholder="Search player..."
            inputHeightClass="h-10"
            buttonSizeClass="h-10 w-10"
            listMaxHeightClass="max-h-[200px]"
            rowHeightClass="h-10"
            imgSize={22}
            scrollClass="scroll-hidden"
            autoFocus={false}
          />
        </form>
      </div>
    </header>
  );
}
