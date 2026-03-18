"use client";

import { useState } from "react";
import { ProgressBar } from "./ProgressBar";
import { colorJSX } from "@/lib/utils/general";

type LevelingProgressProps = {
  // left display
  currentDisplay: string;
  // right display
  nextDisplay: string;
  // current value
  current: number;
  // required value
  required: number;
  // gradient colors
  gradientColors: {
    className: string;
    style: React.CSSProperties;
  };
  // tooltip content
  tooltipContent: {
    current: string;
    required: string;
  };
  // optional className
  className?: string;
};

export function LevelingProgress({
  currentDisplay,
  nextDisplay,
  current,
  required,
  gradientColors,
  tooltipContent,
  className = "",
}: LevelingProgressProps) {
  const [hover, setHover] = useState(false);
  const [mouseX, setMouseX] = useState(0);
  const [containerWidth, setContainerWidth] = useState(0);

  return (
    <div className={`space-y-2 ${className}`}>
      <div className="flex items-center gap-3">
        <div className="text-sm font-mc">{colorJSX(currentDisplay)}</div>
        <div
          className="flex-1 relative"
          onMouseEnter={() => setHover(true)}
          onMouseLeave={() => setHover(false)}
          onMouseMove={(e) => {
            const rect = e.currentTarget.getBoundingClientRect();
            setMouseX(e.clientX - rect.left);
            setContainerWidth(rect.width);
          }}
        >
          <ProgressBar
            current={Math.max(0, current)}
            max={required}
            showValues={false}
            barClassName={gradientColors.className}
            barStyle={gradientColors.style}
          />
          <div
            role="tooltip"
            className="absolute top-8 z-50 rounded-md border px-3 py-2 text-xs whitespace-nowrap"
            style={{
              background: "var(--panel-bg)",
              borderColor: "var(--panel-border)",
              boxShadow: "var(--panel-shadow)",
              backdropFilter: "saturate(120%) blur(10px)",
              left: `${mouseX}px`,
              transform: (() => {
                const edgeThreshold = 75;
                let translateX = "-50%";
                if (mouseX < edgeThreshold) {
                  translateX = "0%";
                } else if (mouseX > containerWidth - edgeThreshold) {
                  translateX = "-100%";
                }

                return `translateX(${translateX}) translateY(${
                  hover ? "0" : "-4px"
                })`;
              })(),
              opacity: hover ? 1 : 0,
              pointerEvents: "none",
              transitionProperty: "opacity, transform",
              transitionDuration: hover ? "220ms" : "160ms",
              transitionTimingFunction: hover
                ? "cubic-bezier(0.22, 0.61, 0.36, 1)"
                : "cubic-bezier(0.4, 0.0, 1, 1)",
            }}
          >
            <div className="text-left">
              <div className="font-mc">
                {colorJSX(tooltipContent.current)}{" "}
                <span>{colorJSX("§7/")}</span>{" "}
                {colorJSX(tooltipContent.required)}
              </div>
            </div>
          </div>
        </div>
        <div className="text-sm font-mc">{colorJSX(nextDisplay)}</div>
      </div>
    </div>
  );
}
