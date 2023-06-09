// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dpc::{
    base_dpc::{Options, DPC},
    fuel_dpc::FuelDPC,
};
mod read_game;

// use std::fs::File;
use std::path::Path;

#[tauri::command]
fn save_project(dir: &str) -> String {
    format!("Hello, {}!", dir)
}

#[tauri::command]
fn read_dir(dir: &str) -> Vec<String> {
    let entries = read_game::visit_dirs(&dir);
    let mut filenames: Vec<String> = Vec::new();
    for entry in entries {
        let path = entry
            .path()
            .to_str()
            .expect("couldn't convert path to string");
        filenames.push(String::from(path))
    }
    // println!("{:?}", filenames);
    return filenames;
}

#[tauri::command]
fn read_dpc(in_path: &str, out_path: &str) -> Result<String, u8> {
    let input_path = Path::new(in_path);
    if !input_path
        .extension()
        .unwrap()
        .to_str()
        .unwrap()
        .to_lowercase()
        .contains("dpc")
    {
        return Err(1);
    }

    let options: Options = Options {
        is_force: false,
        is_unsafe: true,
        is_lz: true,
        is_optimization: false,
        is_quiet: false,
        is_recursive: true,
    };
    let mut dpc = FuelDPC::new(&options, &Vec::new());
    let out_pathbuf = Path::new(out_path)
        .join(
            input_path
                .file_name()
                .expect("input file doesn't have a filename"),
        )
        .with_extension("DPC.d");
    let output_path = out_pathbuf.as_path();
    let res = dpc.extract(&input_path, &output_path);
    // output_path.to_str().unwrap()
    match res {
        Ok(_v) => {
            return Ok(String::from(
                out_pathbuf
                    .to_str()
                    .expect("couldn't convert path to string"),
            ))
        }
        Err(e) => {
            println!("{:?}", e);
            return Err(2);
        }
    };
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![save_project, read_dir, read_dpc,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
