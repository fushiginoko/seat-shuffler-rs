use rand::seq::IteratorRandom;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    student_count: usize,
    seat_max: i32,
    front_seat_max: i32,
    seat_model: Vec<Vec<i32>>,
    student_names: Vec<String>,
    history: Vec<Vec<i32>>,
    front_student_num: Vec<i32>,
}

fn read_json() -> Result<Data, Box<dyn std::error::Error>> {
    let file = File::open("config.json")?;
    let reader = BufReader::new(file);
    let data: Data = serde_json::from_reader(reader)?;
    Ok(data)
}

struct Config {
    student_count: usize,
    seat_max: i32,
    front_seat_max: i32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data: Data = read_json()?;
    let config = Config {
        student_count: data.student_count,
        seat_max: data.seat_max,
        front_seat_max: data.front_seat_max,
    };

    // 設定
    // 座席の形
    let _seat_model: Vec<Vec<i32>> = data.seat_model;
    // 生徒名簿（出席番号順）
    let _student_names: Vec<String> = data.student_names;
    // 過去の座席配置の履歴
    let history: Vec<Vec<i32>> = data.history;
    // 前列指定の生徒の内部番号
    let front_student_num: Vec<i32> = data.front_student_num; // 内部番号なので、実際には+1した番号の名簿番号の人が対象

    let never_per_s = make_never_per_s(&make_history_per_s(&history, &config), &config);
    let front_per_s = make_front_per_s(&front_student_num, &config);
    let available_per_s = make_available_per_s(&never_per_s, &front_per_s);

    // 誰かの使用可能な座席が存在しないと、エラーメッセージを出して終了する
    if available_per_s.contains(&vec![]) {
        println!("この条件で使用可能な座席が存在しません！");
        return Ok(());
    }

    let max_retries = 100;
    let mut retry_count = 0;
    loop {
        if retry_count > max_retries {
            println!(
                "{}回リトライしましたが失敗しました。条件を見直してください",
                max_retries
            );
            return Ok(());
        }
        match make_seats(&available_per_s) {
            Ok(seats) => {
                if retry_count > 0 {
                    println!("{}回リトライしました", retry_count);
                }
                println!("完成した座席");
                println!("{:?}", seats);
                break;
            }
            Err(_) => retry_count += 1,
        }
    }
    Ok(())
}

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
