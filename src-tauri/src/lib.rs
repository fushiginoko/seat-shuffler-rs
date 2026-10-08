use rand::seq::IteratorRandom;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Student {
    pub student_no: String,
    pub name: String,
    pub romaji: String,
    pub gender: String,
    #[serde(default)]
    pub is_dorm: bool,
    #[serde(default)]
    pub is_abroad: bool,
    pub fixed_seat: Option<i32>,
}

// struct Data を pub にしただけ（フィールドは無変更）
#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub title: String,
    pub teacher_name: Option<String>,
    pub sub_teacher_name: Option<String>,
    pub front_rows: usize,
    pub seat_model: Vec<Vec<i32>>,
    pub students: Vec<Student>,
    pub history: Vec<Vec<i32>>,
    pub front_student_num: Vec<i32>,
    pub separation_groups: Vec<Vec<usize>>,
}

#[derive(Serialize)]
pub struct SolveResult {
    /// seats[i] = i 番目の生徒（students 配列順）の座席番号
    pub seats: Vec<i32>,
    pub retries: usize,
}

struct Config {
    student_count: usize,
    seat_max: i32,
    front_seat_max: i32,
}

// async にすることで、リトライ中も UI スレッドをブロックしない
#[tauri::command]
async fn solve_seats(data: Data) -> Result<SolveResult, String> {
    let n = data.students.len();
    if n == 0 {
        return Err("名簿が空です。生徒を追加してください。".into());
    }
    // 元コードは範囲外アクセスでパニックするため、入口で検証する
    if data.history.iter().any(|row| row.len() != n) {
        return Err("履歴の人数と名簿の人数が一致しません。履歴をクリアしてください。".into());
    }
    if data.separation_groups.iter().flatten().any(|&i| i >= n) {
        return Err("離すグループに存在しない生徒が含まれています。".into());
    }

    let config = Config {
        student_count: n,
        seat_max: data.seat_model.iter().flatten().copied().max().unwrap_or(0),
        front_seat_max: data
            .seat_model
            .iter()
            .take(data.front_rows)
            .flatten()
            .copied()
            .max()
            .unwrap_or(0),
    };

    let never_per_s = make_never_per_s(&make_history_per_s(&data.history, &config), &config);
    let front_per_s = make_front_per_s(&data.front_student_num, &config);
    let available_per_s = make_available_per_s(&never_per_s, &front_per_s);
    let available_per_s = apply_fixed_seats(&available_per_s, &data.students);
    let coord_map = build_coord_map(&data.seat_model);

    if available_per_s.iter().any(|v| v.is_empty()) {
        return Err("この条件で使用可能な座席が存在しない生徒がいます。前列指定・固定席・履歴を見直してください。".into());
    }

    let max_retries = 1000;
    for retry_count in 0..=max_retries {
        if let Ok(seats) = make_seats(&available_per_s) {
            if is_separation_valid(&seats, &data.separation_groups, &coord_map) {
                return Ok(SolveResult {
                    seats,
                    retries: retry_count,
                });
            }
        }
    }
    Err(format!(
        "{}回リトライしましたが失敗しました。条件を見直してください。",
        max_retries
    ))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .setup(|app| {
            // updater はデスクトップ専用。モバイルビルドでコンパイルエラーにならないよう cfg で限定する
            #[cfg(desktop)]
            app.handle()
                .plugin(tauri_plugin_updater::Builder::new().build())?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![solve_seats])
        .run(tauri::generate_context!())
        .expect("Tauri の起動に失敗しました");
}

// 「しなければならない」の条件に使う生成関数（配列を返す）
// 生徒ごとの座ったことのある座席の配列を作成
fn make_history_per_s(history: &[Vec<i32>], config: &Config) -> Vec<Vec<i32>> {
    let mut history_per_s: Vec<Vec<i32>> = Vec::new();
    for s_num in 0..config.student_count {
        let mut temp = Vec::new();
        for i in 0..history.len() {
            temp.push(history[i][s_num]);
        }
        history_per_s.push(temp);
    }
    history_per_s
}

// 生徒ごとの座ったことのない座席の配列を作成
fn make_never_per_s(history_per_s: &[Vec<i32>], config: &Config) -> Vec<Vec<i32>> {
    let mut never_per_s: Vec<Vec<i32>> = Vec::new();
    for history_one in history_per_s.iter() {
        let never_one: Vec<i32> = (0..(config.student_count as i32))
            .filter(|x| !history_one.contains(x))
            .collect();
        never_per_s.push(never_one);
    }
    never_per_s
}

// 目が悪い生徒は後ろの座席に座れない
fn make_front_per_s(front_student_num: &[i32], config: &Config) -> Vec<Vec<i32>> {
    let mut front_per_s: Vec<Vec<i32>> = Vec::new();
    for student_num in 0..(config.student_count as i32) {
        if front_student_num.contains(&student_num) {
            front_per_s.push((0..=config.front_seat_max).collect());
        } else {
            front_per_s.push((0..=config.seat_max).collect());
        }
    }
    front_per_s
}

// 条件ごとの座れる座席の共通部分を出す
fn make_available_per_s(never_per_s: &[Vec<i32>], front_per_s: &[Vec<i32>]) -> Vec<Vec<i32>> {
    let mut available_per_s: Vec<Vec<i32>> = Vec::new();
    for (never, front) in never_per_s.iter().zip(front_per_s.iter()) {
        let available_one: Vec<i32> = never
            .iter()
            .filter(|x| front.contains(x))
            .copied()
            .collect();
        available_per_s.push(available_one);
    }
    available_per_s
}

// available_per_sから一人一つ選ぶ
fn make_seats(available_per_s: &[Vec<i32>]) -> Result<Vec<i32>, String> {
    let mut rng = rand::rng();
    let mut seats: Vec<i32> = vec![-1; available_per_s.len()];

    // 要素数の少ない順に並べ替えられたインデックスを作成
    let mut indices: Vec<usize> = (0..available_per_s.len()).collect();
    indices.sort_by_key(|&i| available_per_s[i].len());

    // 要素数の少ない順に一つずつ座席を選んでいく
    for i in indices {
        let available = &available_per_s[i];
        let seat = *available
            .iter()
            .filter(|x| !seats.contains(x))
            .choose(&mut rng)
            // どこかで自分が座れる席はすべて取りつくされてしまったらエラーメッセージが出て終了する
            .ok_or("座席候補が空です。もう一度やり直してください。場合によっては、この条件で使用可能な座席が存在しないかもしれません。".to_string())?;
        seats[i] = seat
    }
    Ok(seats)
}

// 固定席ルールをドメインに反映する関数
fn apply_fixed_seats(available_per_s: &[Vec<i32>], students: &[Student]) -> Vec<Vec<i32>> {
    let all_fixed: Vec<i32> = students.iter().filter_map(|s| s.fixed_seat).collect();
    available_per_s
        .iter()
        .enumerate()
        .map(|(i, available)| {
            if let Some(fixed) = students[i].fixed_seat {
                vec![fixed] // 固定の生徒は1席だけ！
            } else {
                available
                    .iter()
                    .filter(|&&s| !all_fixed.contains(&s))
                    .copied()
                    .collect()
            }
        })
        .collect()
}

// 「してはならない」の条件で使う判定関数（真偽値を返す）

type CoordMap = HashMap<i32, (usize, usize)>;

fn build_coord_map(seat_model: &[Vec<i32>]) -> CoordMap {
    let mut map = HashMap::new();
    for (r, row) in seat_model.iter().enumerate() {
        for (c, &val) in row.iter().enumerate() {
            map.insert(val, (r, c));
        }
    }
    map
}

fn is_separation_valid(
    seats: &[i32],
    separation_groups: &[Vec<usize>],
    coord_map: &CoordMap,
) -> bool {
    separation_groups.iter().all(|group| {
        // 座標が見つからない座席があれば None → 不正として false にする
        let Some(coords) = group
            .iter()
            .map(|&i| coord_map.get(&seats[i]).copied())
            .collect::<Option<Vec<(usize, usize)>>>()
        else {
            return false;
        };

        // グループ内の全ペアのチェビシェフ距離が2以上か確認
        coords.iter().enumerate().all(|(i, &a)| {
            coords[i + 1..]
                .iter()
                .all(|&b| chebyshev_distance(a, b) >= 2)
        })
    })
}

fn chebyshev_distance((r1, c1): (usize, usize), (r2, c2): (usize, usize)) -> usize {
    r1.abs_diff(r2).max(c1.abs_diff(c2))
}
