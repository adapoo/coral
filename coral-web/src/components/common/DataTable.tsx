import React from "react";

type DataTableProps = {
  columns: Array<{
    key: string;
    label: string;
    align?: "left" | "center" | "right";
    className?: string;
  }>;
  data: Array<Record<string, React.ReactNode> | React.ReactElement>;
  className?: string;
};

export function DataTable({ columns, data, className = "" }: DataTableProps) {
  return (
    <div className={`overflow-x-auto ${className}`}>
      <table className="w-full border-collapse">
        <thead>
          <tr className="border-b border-gray-800">
            {columns.map((column, index) => (
              <th
                key={column.key}
                className={`py-1 text-xs font-medium text-gray-400 ${
                  index === 0
                    ? "pr-2"
                    : index === columns.length - 1
                    ? "pl-2"
                    : "px-2"
                } ${
                  column.align === "center"
                    ? "text-center"
                    : column.align === "right"
                    ? "text-right"
                    : "text-left"
                } ${column.className || ""}`}
                style={{ fontFamily: "var(--font-inter)" }}
              >
                {column.label}
              </th>
            ))}
          </tr>
        </thead>
        <tbody>
          {data.map((row, index) => {
            if (React.isValidElement(row) && row.type === TableSpacer) {
              return (
                <tr key={index} className="border-b-0">
                  <td colSpan={columns.length} className="p-0">
                    <div className="h-4" />
                  </td>
                </tr>
              );
            }

            const dataRow = row as Record<string, React.ReactNode>;
            return (
              <tr key={index} className="border-b border-gray-800/50">
                {columns.map((column, colIndex) => (
                  <td
                    key={column.key}
                    className={`py-1 text-xs ${
                      colIndex === 0
                        ? "pr-2"
                        : colIndex === columns.length - 1
                        ? "pl-2"
                        : "px-2"
                    } ${
                      column.align === "center"
                        ? "text-center"
                        : column.align === "right"
                        ? "text-right"
                        : "text-left"
                    } ${column.className || ""}`}
                  >
                    {dataRow[column.key]}
                  </td>
                ))}
              </tr>
            );
          })}
        </tbody>
      </table>
    </div>
  );
}

type DataTableSectionProps = {
  title?: string;
  children: React.ReactNode;
};

export function DataTableSection({ title, children }: DataTableSectionProps) {
  return (
    <section className="space-y-3">
      {title && (
        <h3
          className="text-xs font-bold opacity-80"
          style={{ fontFamily: "var(--font-inter)" }}
        >
          {title}
        </h3>
      )}
      {children}
    </section>
  );
}

export function TableSpacer() {
  return null;
}
