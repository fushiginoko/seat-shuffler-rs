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
      <div className="flex items-center justify-center rounded-md border border-dashed border-slate-300 text-xs text-slate-400 tabular-nums print:border-slate-500 print:text-slate-500">
        {seatNo}
      </div>
    );
  }
  const sub =
    student.is_dorm && student.is_abroad ? "寮（留学中）" : student.is_dorm ? "寮" : student.is_abroad ? "留学中" : "";
  return (
    <div
      className={`relative flex min-w-0 flex-col justify-between rounded-md border px-1.5 py-1 print:rounded-none print:border-black print:bg-white ${tone(student.gender)}`}
    >
      <div className="grid grid-cols-[1fr_auto_1fr] items-start gap-1 text-[10px] leading-tight text-slate-600 print:text-[7pt] print:text-black">
        <span className="tabular-nums">{student.student_no}</span>
        <span className="max-w-full truncate tracking-wide">{student.romaji}</span>
        <span className="text-right">{student.gender}</span>
      </div>
      <div className="truncate text-center text-[clamp(14px,1.7vw,22px)] font-bold leading-tight text-slate-900 print:text-[13pt] print:text-black">
        {student.name}
      </div>
      <div className="flex h-3.5 items-center justify-between text-[10px] text-slate-600 print:text-[7pt] print:text-black">
        <span className="w-3" />
        <span>{sub}</span>
        <span className="w-3">{student.fixed_seat != null && <Pin className="size-3 text-slate-700 print:text-black" aria-label="固定席" />}</span>
      </div>
    </div>
  );
}
