"use client";

import { useRef, useState, useEffect } from "react";
import { ChevronDown } from "lucide-react";
import { colorJSX } from "@/lib/utils/general/colors";

type GameModePanelShellProps = {
  icon: string;
  title: string;
  headerRight?: React.ReactNode;
  children?: React.ReactNode;
  defaultOpen?: boolean;
};

export function GameModePanelShell({
  icon,
  title,
  headerRight,
  children,
  defaultOpen = false,
}: GameModePanelShellProps) {
  const [open, setOpen] = useState(defaultOpen);
  const contentRef = useRef<HTMLDivElement | null>(null);
  const [height, setHeight] = useState(0);

  useEffect(() => {
    const el = contentRef.current;
    if (!el) return;
    if (!open) {
      setHeight(0);
      return;
    }
    setHeight(el.scrollHeight);
  }, [open]);

  useEffect(() => {
    const el = contentRef.current;
    if (!el || !open) return;
    setHeight(el.scrollHeight);
  }, [children, open]);

  return (
    <div className="surface-panel overflow-hidden">
      <button
        type="button"
        aria-expanded={open}
        onClick={() => setOpen((v) => !v)}
        className="w-full cursor-pointer select-none px-4 py-2.5 focus:outline-none"
      >
        <div className="flex items-center justify-between gap-4">
          <div className="flex items-center gap-2">
            <img
              alt={title}
              src={icon}
              width={20}
              height={20}
              className="shrink-0"
            />
            <span className="text-base font-mc">{colorJSX(`§f${title}`)}</span>
          </div>
          <div className="flex items-center gap-4">
            {headerRight ? (
              <div className="hidden sm:flex flex-wrap items-center gap-5 overflow-x-auto text-right">
                {headerRight}
              </div>
            ) : null}
            <ChevronDown
              size={16}
              strokeWidth={2.25}
              className={`transition-transform ${
                open ? "rotate-180" : "rotate-0"
              }`}
              style={{
                transitionDuration: open ? "220ms" : "160ms",
                transitionTimingFunction: open
                  ? "cubic-bezier(0.22, 0.61, 0.36, 1)"
                  : "cubic-bezier(0.4, 0.0, 1, 1)",
              }}
            />
          </div>
        </div>
      </button>
      <div
        className="content transition-[height]"
        style={{
          height,
          overflow: "hidden",
          transitionDuration: open ? "220ms" : "160ms",
          transitionTimingFunction: open
            ? "cubic-bezier(0.22, 0.61, 0.36, 1)"
            : "cubic-bezier(0.4, 0.0, 1, 1)",
        }}
      >
        <div ref={contentRef} className="px-4 pb-4">
          {children}
        </div>
      </div>
    </div>
  );
}
