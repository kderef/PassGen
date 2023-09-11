#![allow(non_snake_case)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // no console

const SHUFFLE_TWICE: bool = false;

use rand::{seq::SliceRandom, Rng};

#[tauri::command]
fn generate_password(len: u16, use_nums: bool, use_special_characters: bool) -> String {
    const NUMS_WEIGHT: f32 = 0.25;
    const SPEC_WEIGHT: f32 = 0.15;
    const UPPER_WEIGHT: f32 = 0.25;

    // pools
    const LOWER: &str = "abcdefghijklmnopqrstuvwxyz";
    const UPPER: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const NUMS: &str = "0123456789";
    const SPEC: &str = "*+-=:()[]&";

    // generate password

    let len_nums = if use_nums {
        (len as f32 * NUMS_WEIGHT).ceil() as u16
    } else {
        0
    };
    let len_spec = if use_special_characters {
        (len as f32 * SPEC_WEIGHT).ceil() as u16
    } else {
        0
    };
    let len_upper = (len as f32 * UPPER_WEIGHT).ceil() as u16;
    let len_lower = len - len_nums - len_spec - len_upper;

    let mut generated = String::with_capacity(len as usize);
    let mut rng = rand::thread_rng();

    for _ in 0..len_nums {
        generated.push((NUMS.chars().nth(rng.gen_range(0..NUMS.len()))).unwrap());
    }
    for _ in 0..len_spec {
        generated.push(SPEC.chars().nth(rng.gen_range(0..SPEC.len())).unwrap());
    }
    for _ in 0..len_lower {
        generated.push(LOWER.chars().nth(rng.gen_range(0..LOWER.len())).unwrap());
    }
    for _ in 0..len_upper {
        generated.push(UPPER.chars().nth(rng.gen_range(0..UPPER.len())).unwrap());
    }

    let mut new = generated.clone().chars().collect::<Vec<char>>();

    drop(generated);

    new.shuffle(&mut rng);
    if SHUFFLE_TWICE {
        new.shuffle(&mut rng);
    }

    new.iter().collect()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![generate_password])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
