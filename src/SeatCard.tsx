import { Pin } from "lucide-react";
import type { Student } from "./types";

// 印刷時にも確実に色が乗るようにインラインスタイルで完全保証
const getCardStyle = (g: string) => {
  if (g === "男") {
    return {
      backgroundColor: "#dbeafe", // 確実な薄青 (Tailwind blue-100)
      WebkitPrintColorAdjust: "exact" as const,
      printColorAdjust: "exact" as const,
    };
  }
  if (g === "女") {
    return {
      backgroundColor: "#fce7f3", // 確実な薄ピンク (Tailwind pink-100)
      WebkitPrintColorAdjust: "exact" as const,
      printColorAdjust: "exact" as const,
    };
  }
  return {
    backgroundColor: "#ffffff",
    WebkitPrintColorAdjust: "exact" as const,
    printColorAdjust: "exact" as const,
  };
};

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
      style={getCardStyle(student.gender)}
      // 🚨 print:bg-white を完全削除！ 枠線も印刷時にハッキリ出るように調整
      className="relative flex min-w-0 flex-col justify-between rounded-md border border-slate-400 px-2 py-1.5 print:rounded-none print:border-slate-800"
    >
      <div className="grid grid-cols-[auto_1fr_auto] items-center gap-1 text-xs text-slate-700 print:text-[8pt] print:text-black">
        <span className="font-bold tabular-nums">{student.student_no}</span>

        {/* 🚨 ローマ字: min-w-0追加、print:text-[6.5pt]に縮小、tracking-tighterで絶対に見切れない */}
        <span className="min-w-0 max-w-full truncate text-center tracking-normal text-[11px] text-slate-600 print:text-[6.5pt] print:tracking-tighter print:text-black">
          {student.romaji}
        </span>

        <span className="font-bold text-slate-800 print:text-black">{student.gender}</span>
      </div>

      <div className="truncate text-center text-[clamp(15px,1.8vw,24px)] font-bold leading-tight text-slate-900 print:text-[13pt] print:text-black my-0.5">
        {student.name}
      </div>

      <div className="flex h-4 items-center justify-between text-[11px] font-semibold text-slate-700 print:text-[8pt] print:text-black">
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
