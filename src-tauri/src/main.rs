// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;

fn get_home_dir() -> String {
    let base_dirs = directories::BaseDirs::new().unwrap();
    let home_dir = base_dirs.home_dir().to_str().unwrap();
    return home_dir.to_string();
}

fn resolve_data_path(data_path: &str) -> String {
    if data_path != "default" {
        return data_path.to_string();
    }

    let home_dir = get_home_dir();
    let path = format!("{}/.config/skypeforlinux", home_dir);

    path.to_string()
}

fn kill_process(pid: i32) -> bool {
    let output = std::process::Command::new("kill")
        .arg("-9")
        .arg(format!("{}", pid))
        .output()
        .expect("Failed to execute command");

    return output.status.success();
}

#[tauri::command]
fn has_skype() -> bool {
    return std::process::Command::new("which")
        .arg("skypeforlinux")
        .spawn()
        .is_ok();
}

#[tauri::command]
fn is_skype_running(data_path: &str) -> bool {
    let data_path = resolve_data_path(data_path);
    let cookies_path = format!("{}/Cookies", data_path);
    return std::process::Command::new("lsof")
        .arg(cookies_path)
        .spawn()
        .is_ok();
}

#[tauri::command]
fn stop_skype(data_path: &str) {
    let data_path = resolve_data_path(data_path);
    let cookies_path = format!("{}/Cookies", data_path);
    let output = std::process::Command::new("lsof")
        .arg(cookies_path)
        .output()
        .unwrap();

    if !output.status.success() {
        return;
    }
    let stdout = std::str::from_utf8(&output.stdout).unwrap();

    let pids: Vec<&str> = stdout
        .lines()
        .skip(1)
        .map(|line| line.split_whitespace().nth(1).unwrap())
        .collect();

    let output = Command::new("ps")
        .arg("-o")
        .arg("ppid=")
        .arg(format!("{}", pids[0]))
        .output()
        .expect("Failed to execute command");

    let ppid: Vec<i32> = std::str::from_utf8(&output.stdout)
        .unwrap()
        .trim()
        .lines()
        .map(|line| line.parse::<i32>().unwrap_or(-1))
        .collect();

    for pid in ppid {
        kill_process(pid);
    }
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn launch_skype(data_path: &str) {
    let mut command = std::process::Command::new("skypeforlinux");

    if data_path != "default" {
        command.args(["--secondary", "--datapath", data_path]);
    }

    command.spawn().unwrap();
}

#[tauri::command]
fn get_skypes() -> Vec<String> {
    let home_dir = get_home_dir();
    let glob_pattern = format!("{}{}", home_dir, "/.skype*");
    return glob::glob(glob_pattern.as_str())
        .expect("Error")
        .map(|e| -> String {
            let path_buf = e.unwrap().to_path_buf();
            let path_string = path_buf.to_str().unwrap().to_string();
            path_string
        })
        .collect();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            launch_skype,
            get_skypes,
            has_skype,
            is_skype_running,
            stop_skype
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
