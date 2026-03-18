import React from "react";

type ProgressBarProps = {
  current: number;
  max: number;
  label?: string;
  showValues?: boolean;
  className?: string;
  barClassName?: string;
  barStyle?: React.CSSProperties;
  textClassName?: string;
};

export function ProgressBar({
  current,
  max,
  label,
  showValues = true,
  className = "",
  barClassName = "",
  barStyle = {},
  textClassName = "",
}: ProgressBarProps) {
  const percentage = Math.min((current / max) * 100, 100);

  return (
    <div className={`space-y-1 ${className}`}>
      {(label || showValues) && (
        <div className="flex justify-between items-center text-sm">
          {label && (
            <span className={`opacity-80 ${textClassName}`}>{label}</span>
          )}
          {showValues && (
            <span className={`text-[var(--color-7)] ${textClassName}`}>
              {current.toLocaleString()} / {max.toLocaleString()}
            </span>
          )}
        </div>
      )}
      <div className="relative h-2 bg-gray-800 rounded-full overflow-hidden">
        <div
          className={`h-full transition-all duration-300 ease-out ${
            barClassName || "bg-gradient-to-r from-blue-500 to-blue-400"
          }`}
          style={{ width: `${percentage}%`, ...barStyle }}
        />
      </div>
    </div>
  );
}

type LevelProgressProps = {
  currentLevel: number;
  nextLevel: number;
  currentXp: number;
  requiredXp: number;
  className?: string;
};

export function LevelProgress({
  currentLevel,
  nextLevel,
  currentXp,
  requiredXp,
  className = "",
}: LevelProgressProps) {
  return (
    <div className={`space-y-2 ${className}`}>
      <div className="flex justify-between items-center text-sm">
        <span className="opacity-80">Leveling Progress</span>
        <span className="text-[var(--color-7)] font-mono">
          {currentLevel} → {nextLevel}
        </span>
      </div>
      <ProgressBar
        current={currentXp}
        max={requiredXp}
        showValues={false}
        barClassName="from-yellow-500 to-yellow-400"
      />
    </div>
  );
}
