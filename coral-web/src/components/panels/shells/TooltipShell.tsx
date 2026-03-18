"use client";

export function TooltipShell({
  children,
  hover,
  minWidth = 320,
  maxWidth = 520,
}: {
  children: React.ReactNode;
  hover: boolean;
  minWidth?: number;
  maxWidth?: number;
}) {
  return (
    <div
      role="tooltip"
      className="absolute left-full ml-2 top-0 z-10 rounded-md border px-3 py-2 text-xs"
      style={{
        background: "var(--panel-bg)",
        borderColor: "var(--panel-border)",
        boxShadow: "var(--panel-shadow)",
        backdropFilter: "saturate(120%) blur(10px)",
        minWidth: `${minWidth}px`,
        maxWidth: `${maxWidth}px`,
        opacity: hover ? 1 : 0,
        transform: `translateY(${hover ? "0" : "4px"}) translateX(${
          hover ? "0" : "4px"
        })`,
        pointerEvents: "none",
        transitionProperty: "opacity, transform",
        transitionDuration: hover ? "220ms" : "160ms",
        transitionTimingFunction: hover
          ? "cubic-bezier(0.22, 0.61, 0.36, 1)"
          : "cubic-bezier(0.4, 0.0, 1, 1)",
      }}
    >
      {children}
    </div>
  );
}
