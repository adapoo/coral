import { colorJSX } from "@/lib/utils/general/colors";

type StatDisplayProps = {
  children?: React.ReactNode;
};

export function StatDisplay({ children }: StatDisplayProps) {
  return <div className="space-y-4">{children}</div>;
}

type StatSectionProps = {
  title?: string;
  children?: React.ReactNode;
  columns?: 1 | 2 | 3;
};

export function StatSection({
  title,
  children,
  columns = 2,
}: StatSectionProps) {
  return (
    <section>
      {title ? (
        <h3
          className="mb-1.5 text-xs font-bold opacity-80"
          style={{ fontFamily: "var(--font-inter)" }}
        >
          {title}
        </h3>
      ) : null}
      <div
        className={`grid gap-1.5 ${
          columns === 1
            ? "grid-cols-1"
            : columns === 2
            ? "grid-cols-2"
            : "grid-cols-3"
        }`}
      >
        {children}
      </div>
    </section>
  );
}

type StatRowProps = {
  label: string;
  value: React.ReactNode;
};

export function StatRow({ label, value }: StatRowProps) {
  return (
    <div className="flex items-baseline justify-between gap-4 text-xs leading-4">
      <div className="font-medium">{label}</div>
      <div className="font-mc">
        {typeof value === "string" ? colorJSX(`§7${value}`) : value}
      </div>
    </div>
  );
}

export function StatSpacer() {
  return (
    <div className="flex items-baseline justify-between gap-4">
      <div className="h-[0.5rem]" />
    </div>
  );
}
