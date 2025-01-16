use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::{fs, process::{Child, Command, Stdio}, thread};
use tauri::Emitter;

use lazy_static::lazy_static;

lazy_static! {
    static ref PROCESSES: Mutex<Vec<Child>> = Mutex::new(Vec::new());
}

const DUMMY_RS_CONTENT: &str = include_str!("dummy.rs");
const TEMP_DIR: &str = "C:\\Temp\\zarzul";

fn ensure_temp_dir() -> PathBuf {
    let temp_path = Path::new(TEMP_DIR).to_path_buf();
    if !temp_path.exists() {
        fs::create_dir_all(&temp_path).expect("Failed to create temporary directory");
    }
    temp_path
}

fn check_rust_in_path() -> Result<(), String> {
    let check = Command::new("powershell")
        .arg("-Command")
        .arg("Get-Command rustc | Out-Null; if ($?) { exit 0 } else { exit 1 }")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    if let Ok(status) = check {
        if status.success() {
            return Ok(());
        }
    }
    Err("`rustc` was not found in PATH. Make sure Rust is installed and available.".to_string())
}

fn create_dummy_executables_sync() -> Result<Vec<PathBuf>, String> {
    check_rust_in_path()?;

    let dummy_process_names = vec![
        "vmware",
        "vmware-authd",
        "vmware-vmx",
        "vmware-hostd",
    ];

    let temp_path = ensure_temp_dir();
    let mut compiled_paths = Vec::new();

    for process_name in dummy_process_names {
        let dummy_file = temp_path.join(format!("{}.rs", process_name));
        fs::write(&dummy_file, &DUMMY_RS_CONTENT)
            .map_err(|e| format!("Failed to create source file {:?}: {:?}", dummy_file, e))?;

        let output_path = temp_path.join(process_name.to_owned() + ".exe");
        let output = Command::new("rustc")
            .args(&[dummy_file.to_str().unwrap(), "-o", output_path.to_str().unwrap()])
            .output()
            .map_err(|e| format!("Failed to compile {:?}: {:?}", dummy_file, e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Compilation failed for {:?} with output: {}", dummy_file, stderr));
        }

        compiled_paths.push(output_path);
        let _ = fs::remove_file(dummy_file);
    }

    Ok(compiled_paths)
}

#[tauri::command]
pub fn start_background_processes(window: tauri::Window) {
    thread::spawn(move || {
        match create_dummy_executables_sync() {
            Ok(dummy_process_paths) => {
                for process_path in dummy_process_paths {
                    match Command::new(process_path.clone()).spawn() {
                        Ok(child) => {
                            println!("Spawned dummy process: {:?}", process_path);
                            PROCESSES.lock().unwrap().push(child);
                        }
                        Err(e) => {
                            let error_message = format!("Failed to spawn process {:?}: {:?}", process_path, e);
                            println!("{}", error_message);
                            window.emit("error-log", error_message).unwrap();
                        }
                    }
                }
                window.emit("success-log", "Processes are running.".to_string()).unwrap();
            }
            Err(e) => {
                println!("{}", e);
                window.emit("error-log", e).unwrap();
            }
        }
    });
}

#[tauri::command]
pub fn stop_background_processes(window: tauri::Window) {
    thread::spawn(move || {
        let mut processes = PROCESSES.lock().unwrap();
        while let Some(mut child) = processes.pop() {
            if let Err(e) = child.kill() {
                let error_message = format!("Failed to kill process: {:?}", e);
                println!("{}", error_message);
                window.emit("error-log", error_message).unwrap();
            } else {
                println!("Process killed successfully");
            }
        }

        thread::sleep(std::time::Duration::from_secs(1));
        if let Err(error) = fs::remove_dir_all(TEMP_DIR).map_err(|error| format!("Failed to clean up temporary directory {:?}: {:?}", TEMP_DIR, error)) {
            println!("{}", error);
            window.emit("error-log", error).unwrap();
        } else {
            println!("Cleaned up temporary directory: {:?}", TEMP_DIR);
        }

        window.emit("success-log", "Processes stopped and cleaned up successfully.".to_string()).unwrap();
    });
}
