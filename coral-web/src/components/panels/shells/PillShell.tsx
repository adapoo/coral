"use client";

export function PillShell({ children }: { children: React.ReactNode }) {
  return (
    <div
      className="inline-flex items-center gap-2 rounded-md border px-2 py-1 text-xs cursor-pointer select-none"
      style={{
        background: "var(--panel-bg)",
        borderColor: "var(--panel-border)",
      }}
    >
      {children}
    </div>
  );
}
