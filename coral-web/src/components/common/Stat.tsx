import { colorJSX } from "@/lib/utils/general/colors";

type StatProps = { label: string; value: React.ReactNode; color?: string };

export function Stat({ label, value, color }: StatProps) {
  return (
    <div>
      <div
        className="text-xxxs tracking-wide opacity-70"
        style={{ fontFamily: "var(--font-inter)" }}
      >
        {label}
      </div>
      <div className="text-base font-mc">
        {typeof value === "string" && color
          ? colorJSX(
              `§${color.replace("var(--color-", "").replace(")", "")}${value}§r`
            )
          : typeof value === "string"
          ? colorJSX(`§7${value}`)
          : value}
      </div>
    </div>
  );
}
