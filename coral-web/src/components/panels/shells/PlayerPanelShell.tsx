type PlayerPanelShellProps = {
  children?: React.ReactNode;
};

export function PlayerPanelShell({ children }: PlayerPanelShellProps) {
  return (
    <div className="surface-panel p-4 self-start">
      <div className="space-y-2 text-sm">{children}</div>
    </div>
  );
}
