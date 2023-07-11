#![allow(non_snake_case)]
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rand::{seq::SliceRandom, Rng};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn generate_password(len: u16, use_nums: bool, use_special_characters: bool) -> String {
    const NUMS_WEIGHT: f32 = 0.25;
    const SPEC_WEIGHT: f32 = 0.15;
    const UPPER_WEIGHT: f32 = 0.25;

    // pools
    const P_LOWER: &str = "abcdefghijklmnopqrstuvwxyz";
    const P_UPPER: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const P_NUMS: &str = "0123456789";
    const P_SPEC: &str = "*+-=:()[]&";

    const P_LOWER_LEN: usize = P_LOWER.len();
    const P_UPPER_LEN: usize = P_UPPER.len();
    const P_NUMS_LEN: usize = P_NUMS.len();
    const P_SPEC_LEN: usize = P_SPEC.len();

    // generate password

    let len_nums = if use_nums {
        (len as f32 * NUMS_WEIGHT).ceil() as u16
    } else {
        0
    };
    let len_spec = if use_special_characters { (len as f32 * SPEC_WEIGHT).ceil() as u16 } else { 0 };
    let len_upper = (len as f32 * UPPER_WEIGHT).ceil() as u16;
    let len_lower = len - len_nums - len_spec - len_upper;

    let mut generated = String::new();

    let mut rng = rand::thread_rng();

    for _ in 0..len_nums {
        generated.push((P_NUMS.chars().nth(rng.gen_range(0..P_NUMS_LEN))).unwrap());
    }
    for _ in 0..len_spec {
        generated.push(P_SPEC.chars().nth(rng.gen_range(0..P_SPEC_LEN)).unwrap());
    }
    for _ in 0..len_lower {
        generated.push(P_LOWER.chars().nth(rng.gen_range(0..P_LOWER_LEN)).unwrap());
    }
    for _ in 0..len_upper {
        generated.push(P_UPPER.chars().nth(rng.gen_range(0..P_UPPER_LEN)).unwrap());
    }

    let mut new = generated.clone().chars().collect::<Vec<char>>();
    new.shuffle(&mut rng);
    generated = new.iter().collect(); // shuffled

    generated
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![generate_password])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
