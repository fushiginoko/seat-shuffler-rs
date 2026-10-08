import { Pin } from "lucide-react";
import type { Student } from "./types";

const tone = (g: string) =>
  g === "男"
    ? "border-slate-400 bg-slate-100"
    : g === "女"
      ? "border-rose-300 bg-rose-50"
      : "border-slate-300 bg-white";

export function SeatCard({ student, seatNo }: { student?: Student; seatNo: number }) {
  if (!student) {
    return (
      <div className="flex items-center justify-center rounded-md border border-dashed border-slate-300 text-sm font-medium text-slate-400 tabular-nums print:border-slate-500 print:text-slate-500">
        {seatNo}
      </div>
    );
  }
  const sub =
    student.is_dorm && student.is_abroad ? "寮（留学中）" : student.is_dorm ? "寮" : student.is_abroad ? "留学中" : "";
  return (
    <div
      className={`relative flex min-w-0 flex-col justify-between rounded-md border px-2 py-1.5 print:rounded-none print:border-black print:bg-white ${tone(student.gender)}`}
    >
      <div className="grid grid-cols-[auto_1fr_auto] items-center gap-1.5 text-xs text-slate-700 print:text-[8.5pt] print:text-black">
        <span className="font-bold tabular-nums">{student.student_no}</span>
        <span className="max-w-full truncate text-center tracking-wide text-[11px] text-slate-500 print:text-[8pt] print:text-black">
          {student.romaji}
        </span>
        <span className="font-medium text-slate-800 print:text-black">{student.gender}</span>
      </div>

      <div className="truncate text-center text-[clamp(15px,1.8vw,24px)] font-bold leading-tight text-slate-900 print:text-[14pt] print:text-black my-0.5">
        {student.name}
      </div>

      <div className="flex h-4 items-center justify-between text-[11px] font-semibold text-slate-700 print:text-[8.5pt] print:text-black">
        <span className="w-3" />
        <span className="tracking-wider">{sub}</span>
        <span className="w-3">
          {student.fixed_seat != null && (
            <Pin className="size-3.5 text-slate-700 print:text-black" aria-label="固定席" />
          )}
        </span>
      </div>
    </div>
  );
}
