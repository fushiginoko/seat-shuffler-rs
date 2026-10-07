// Rust の Data / Student と 1:1 対応（フィールド名は snake_case のまま）
export interface Student {
  student_no: string;
  name: string;
  romaji: string;
  gender: string;
  is_dorm: boolean;
  is_abroad: boolean;
  fixed_seat: number | null;
}

export interface Data {
  title: string;
  teacher_name: string | null;
  sub_teacher_name: string | null;
  front_rows: number;
  seat_model: number[][]; // -1 = 通路
  students: Student[];
  history: number[][]; // history[回][生徒index] = 座席番号
  front_student_num: number[]; // 生徒index（名簿順 0 始まり）
  separation_groups: number[][]; // 生徒index の配列
}

export interface SolveResult {
  seats: number[]; // seats[生徒index] = 座席番号
  retries: number;
}

export type View = "student" | "teacher";

/** 通路列（0 始まり）を除いて、座席番号を 0 から行順に振る */
export function makeSeatModel(rows: number, cols: number, aisleCols: number[]): number[][] {
  let n = 0;
  return Array.from({ length: rows }, () =>
    Array.from({ length: cols }, (_, c) => (aisleCols.includes(c) ? -1 : n++)),
  );
}

export const seatCount = (m: number[][]) => m.flat().filter((v) => v >= 0).length;

export const newStudent = (no: string): Student => ({
  student_no: no,
  name: "",
  romaji: "",
  gender: "男",
  is_dorm: false,
  is_abroad: false,
  fixed_seat: null,
});
