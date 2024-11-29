use std::fs;
use std::fs::File;
use std::io::Write;
use std::process::Command;
use serde::Deserialize;

#[derive(Deserialize)]
struct KittySession {
    tabs: Vec<KittyTab>,
}

#[derive(Deserialize)]
struct KittyTab {
    title: String,
    layout: String,
    windows: Vec<KittyWindow>,
}

#[derive(Deserialize)]
struct KittyWindow {
    cwd: String,
    foreground_processes: Vec<WindowForegroundProcess>,
}

#[derive(Deserialize)]
struct WindowForegroundProcess {
    cmdline: Vec<String>,
}

pub fn save_session(session_path: &str) {
    let output = Command::new("kitty")
        .args(["@", "ls"])
        .output()
        .expect("Failed to execute process.");

    let session_data: Vec<KittySession> = serde_json::from_str(
        String::from_utf8(output.stdout).unwrap().as_str()
    ).unwrap();

    write_data_to_file(session_data.first().unwrap(), session_path);
}

fn write_data_to_file(session: &KittySession, relative_session_path: &str) {
    let mut data: String = String::from("");

    for tab in session.tabs.as_slice() {
        data.push_str(build_tab_data(tab).as_str());

        for window in tab.windows.as_slice() {
            data.push_str(build_window_data(window).as_str());
        }
    }

    let mut file = File::create(
        format!("/home/grant/.config/kitty/sessions/{relative_session_path}.kitty")
    ).expect("Error creating file!");

    let _ = file.write(data.as_bytes()).expect("Error writing data!");
}

fn build_tab_data(tab: &KittyTab) -> String {
    format!("\
        new_tab {}\n\
        layout {}\n\n\
    ", tab.title, tab.layout)
}

fn build_window_data(window: &KittyWindow) -> String {
    let process_cmd: String = window
        .foreground_processes
        .first()
        .unwrap()
        .cmdline
        .join(" ");

    format!("\
        cd {}\n\
        launch {}\n\n\
    ", window.cwd, process_cmd)
}