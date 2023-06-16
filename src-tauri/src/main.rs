// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dpc::{
    base_dpc::{Options, DPC},
    fuel_dpc::FuelDPC,
    walle_dpc::WALLEDPC,
};
use image::io::Reader as ImageReader;
mod read_game;

// use std::fs::File;
use std::{io::Error, path::Path};

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
fn get_dpc_out_path(in_path: &str, out_path: &str) -> String {
    let out_pathbuf = Path::new(out_path)
        .join(
            Path::new(in_path)
                .file_name()
                .expect("input file doesn't have a filename"),
        )
        .with_extension("DPC.d");

    String::from(
        out_pathbuf
            .to_str()
            .expect("couldn't convert path to string"),
    )
}

#[tauri::command]
fn path_exists(path: &str) -> bool {
    Path::new(path)
        .try_exists()
        .expect("can't check if path exists")
}

#[tauri::command]
fn is_dir(path: &str) -> bool {
    Path::new(path).is_dir()
}

#[tauri::command]
fn read_dpc(in_path: &str, out_path: &str, game: &str) -> Result<(), u8> {
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
        is_force: true,
        is_unsafe: true,
        is_lz: true,
        is_optimization: false,
        is_quiet: true,
        is_recursive: false,
    };
    let output_path = Path::new(out_path);
    let res: Result<(), Error>;
    match game {
        "walle" => {
            let mut dpc = WALLEDPC::new(&options, &Vec::new());
            res = dpc.extract(&input_path, &output_path);
        }
        _ => {
            let mut dpc = FuelDPC::new(&options, &Vec::new());
            res = dpc.extract(&input_path, &output_path);
        }
    }
    match res {
        Ok(_v) => return Ok(()),
        Err(e) => {
            println!("{:?}", e);
            return Err(2);
        }
    };
}

#[tauri::command]
fn read_object(in_path: &str, out_path: &str, game: &str) -> Result<(), u8> {
    let input_path = Path::new(in_path);
    if !input_path
        .extension()
        .unwrap()
        .to_str()
        .unwrap()
        .to_lowercase()
        .contains("_z")
    {
        return Err(1);
    }

    let options: Options = Options {
        is_force: true,
        is_unsafe: true,
        is_lz: true,
        is_optimization: false,
        is_quiet: false,
        is_recursive: false,
    };
    let output_path = Path::new(out_path);
    let res: Result<(Vec<u32>, Vec<u32>), Error>;
    match game {
        "walle" => {
            let dpc = WALLEDPC::new(&options, &Vec::new());
            res = dpc.fmt_extract(&input_path, &output_path);
        }
        _ => {
            let dpc = FuelDPC::new(&options, &Vec::new());
            res = dpc.fmt_extract(&input_path, &output_path);
        }
    }
    match res {
        Ok(_v) => return Ok(()),
        Err(e) => {
            println!("{:?}", e);
            return Err(2);
        }
    };
}

#[tauri::command]
fn save_png(dds_path: &str) -> Result<(), String> {
    let res = ImageReader::open(dds_path)
        .unwrap()
        .decode()
        .unwrap()
        .save(Path::new(dds_path).with_extension("png"));
    match res {
        Ok(_) => return Ok(()),
        Err(e) => return Err(e.to_string()),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            save_project,
            read_dir,
            read_dpc,
            read_object,
            save_png,
            get_dpc_out_path,
            path_exists,
            is_dir
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
