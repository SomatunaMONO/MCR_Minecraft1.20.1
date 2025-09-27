// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::{Command, Child, Stdio};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::collections::VecDeque;
use serde::{Deserialize, Serialize};
use tokio::time::sleep;

slint::include_modules!();

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Config {
    launcher_paths: Vec<String>,
    local_addr: String,
    hostnames: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            launcher_paths: vec![],
            local_addr: "127.0.0.1:20100".to_string(),
            hostnames: vec!["minecraft.nitmcr.f5.si".to_string()],
        }
    }
}

const CONFIG_FILE: &str = "config.json";

#[cfg(windows)]
fn spawn_without_window(mut cmd: Command) -> std::io::Result<Child> {
    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x0800_0000;
    cmd.creation_flags(CREATE_NO_WINDOW);
    cmd.spawn()
}

#[cfg(not(windows))]
fn spawn_without_window(cmd: Command) -> std::io::Result<Child> {
    cmd.spawn()
}

fn invoke_ui<F>(ui_handle: slint::Weak<AppWindow>, f: F)
where
    F: FnOnce(AppWindow) + Send + 'static,
{
    let _ = slint::invoke_from_event_loop(move || {
        if let Some(ui) = ui_handle.upgrade() {
            f(ui);
        }
    });
}

// バリデーション関数
fn validate_launcher_path(path: &str) -> Result<String, String> {
    let trimmed = path.trim();
    
    if trimmed.is_empty() {
        return Err("ランチャーパスが空です".to_string());
    }
    
    if trimmed.contains('"') {
        return Err("ランチャーパスにダブルクォーテーションが含まれています。削除してください。".to_string());
    }
    
    // ダブルクォーテーションを自動で削除
    let cleaned = trimmed.replace('"', "");
    Ok(cleaned)
}

fn validate_hostname(hostname: &str) -> Result<String, String> {
    let trimmed = hostname.trim();
    
    if trimmed.is_empty() {
        return Err("ホスト名が空です".to_string());
    }
    
    if trimmed.contains(' ') {
        return Err("ホスト名にスペースが含まれています。削除してください。".to_string());
    }
    
    if trimmed.contains('\t') {
        return Err("ホスト名にタブ文字が含まれています。削除してください。".to_string());
    }
    
    // 基本的なホスト名のフォーマットチェック
    if !trimmed.chars().all(|c| c.is_alphanumeric() || c == '.' || c == '-' || c == '_') {
        return Err("ホスト名に無効な文字が含まれています。英数字、ドット、ハイフン、アンダースコアのみ使用できます。".to_string());
    }
    
    Ok(trimmed.to_string())
}

struct AppState {
    config: Config,
    launcher_process: Option<Child>,
    cloudflared_process: Option<Child>,
    log_buffer: VecDeque<String>,
}

impl AppState {
    fn new() -> Self {
        Self {
            config: Config::default(),
            launcher_process: None,
            cloudflared_process: None,
            log_buffer: VecDeque::new(),
        }
    }

    fn add_log(&mut self, message: String) {
        self.log_buffer.push_back(format!("[{}] {}", chrono::Local::now().format("%H:%M:%S"), message));
        if self.log_buffer.len() > 1000 {
            self.log_buffer.pop_front();
        }
    }

    fn get_logs(&self) -> String {
        self.log_buffer.iter().cloned().collect::<Vec<_>>().join("\n")
    }
}

fn load_or_create_config() -> Config {
    if Path::new(CONFIG_FILE).exists() {
        match fs::read_to_string(CONFIG_FILE) {
            Ok(content) => {
                match serde_json::from_str(&content) {
                    Ok(config) => config,
                    Err(e) => {
                        eprintln!("設定ファイルの読み込みエラー: {}", e);
                        Config::default()
                    }
                }
            }
            Err(e) => {
                eprintln!("設定ファイルの読み込みエラー: {}", e);
                Config::default()
            }
        }
    } else {
        Config::default()
    }
}

fn save_config(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(config)?;
    fs::write(CONFIG_FILE, json)?;
    Ok(())
}

async fn start_launcher_async(
    launcher_path: String,
    hostname: String,
    local_addr: String,
    state: Arc<Mutex<AppState>>,
    ui_handle: slint::Weak<AppWindow>,
) {
    // ランチャー起動
    {
        let mut state_guard = state.lock().unwrap();
        state_guard.add_log(format!("ランチャーを起動中: {}", launcher_path));
    }
    
    let launcher_result = if launcher_path.ends_with(".url") {
        let mut cmd = Command::new("cmd");
        cmd.arg("/C")
            .arg("start")
            .arg("")
            .arg("/B")
            .arg(&launcher_path);
        spawn_without_window(cmd)
    } else {
        let cmd = Command::new(&launcher_path);
        spawn_without_window(cmd)
    };

    match launcher_result {
        Ok(child) => {
            {
                let mut state_guard = state.lock().unwrap();
                state_guard.launcher_process = Some(child);
                state_guard.add_log("ランチャーを起動しました".to_string());
            }
            
            let logs = state.lock().unwrap().get_logs();
            invoke_ui(ui_handle.clone(), move |ui| {
                ui.set_log_output(logs.clone().into());
            });
        }
        Err(e) => {
            {
                let mut state_guard = state.lock().unwrap();
                state_guard.add_log(format!("ランチャーの起動エラー: {}", e));
            }
            
            let logs = state.lock().unwrap().get_logs();
            invoke_ui(ui_handle.clone(), move |ui| {
                ui.set_log_output(logs.clone().into());
                ui.set_is_running(false);
            });
            return;
        }
    }

    // 5秒待機
    sleep(Duration::from_secs(5)).await;

    // cloudflared起動
    {
        let mut state_guard = state.lock().unwrap();
        state_guard.add_log(format!("cloudflaredを起動中: {}", hostname));
    }

    let mut cloudflared_cmd = Command::new("cloudflared");
    cloudflared_cmd
        .args(&[
            "access",
            "tcp",
            "--hostname",
            &hostname,
            "--url",
            &local_addr,
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    match spawn_without_window(cloudflared_cmd) {
        Ok(mut child) => {
            let stdout = child.stdout.take();
            let stderr = child.stderr.take();
            {
                let mut state_guard = state.lock().unwrap();
                state_guard.cloudflared_process = Some(child);
                state_guard.add_log("cloudflaredを起動しました".to_string());
            }

            let logs = state.lock().unwrap().get_logs();
            invoke_ui(ui_handle.clone(), move |ui| {
                ui.set_log_output(logs.clone().into());
            });

            if let Some(stdout) = stdout {
                let state_clone = Arc::clone(&state);
                let ui_handle_clone = ui_handle.clone();
                std::thread::spawn(move || {
                    let reader = BufReader::new(stdout);
                    for line in reader.lines() {
                        match line {
                            Ok(line) => {
                                let logs = {
                                    let mut state_guard = state_clone.lock().unwrap();
                                    state_guard.add_log(format!("cloudflared: {}", line));
                                    state_guard.get_logs()
                                };
                                invoke_ui(ui_handle_clone.clone(), move |ui| {
                                    ui.set_log_output(logs.clone().into());
                                });
                            }
                            Err(e) => {
                                let logs = {
                                    let mut state_guard = state_clone.lock().unwrap();
                                    state_guard.add_log(format!("cloudflared stdout 読み取りエラー: {}", e));
                                    state_guard.get_logs()
                                };
                                invoke_ui(ui_handle_clone.clone(), move |ui| {
                                    ui.set_log_output(logs.clone().into());
                                });
                                break;
                            }
                        }
                    }
                });
            }

            if let Some(stderr) = stderr {
                let state_clone = Arc::clone(&state);
                let ui_handle_clone = ui_handle.clone();
                std::thread::spawn(move || {
                    let reader = BufReader::new(stderr);
                    for line in reader.lines() {
                        match line {
                            Ok(line) => {
                                let logs = {
                                    let mut state_guard = state_clone.lock().unwrap();
                                    state_guard.add_log(format!("cloudflared: {}", line));
                                    state_guard.get_logs()
                                };
                                invoke_ui(ui_handle_clone.clone(), move |ui| {
                                    ui.set_log_output(logs.clone().into());
                                });
                            }
                            Err(e) => {
                                let logs = {
                                    let mut state_guard = state_clone.lock().unwrap();
                                    state_guard.add_log(format!("cloudflared stderr 読み取りエラー: {}", e));
                                    state_guard.get_logs()
                                };
                                invoke_ui(ui_handle_clone.clone(), move |ui| {
                                    ui.set_log_output(logs.clone().into());
                                });
                                break;
                            }
                        }
                    }
                });
            }
        }
        Err(e) => {
            {
                let mut state_guard = state.lock().unwrap();
                state_guard.add_log(format!("cloudflaredの起動エラー: {}", e));
            }

            let logs = state.lock().unwrap().get_logs();
            invoke_ui(ui_handle.clone(), move |ui| {
                ui.set_log_output(logs.clone().into());
                ui.set_is_running(false);
            });
        }
    }
}

fn stop_processes(state: Arc<Mutex<AppState>>) {
    let mut state_guard = state.lock().unwrap();
    
    if let Some(mut child) = state_guard.cloudflared_process.take() {
        let _ = child.kill();
        state_guard.add_log("cloudflaredを停止しました".to_string());
    }
    
    if let Some(mut child) = state_guard.launcher_process.take() {
        let _ = child.kill();
        state_guard.add_log("ランチャーを停止しました".to_string());
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ui = AppWindow::new()?;
    let state = Arc::new(Mutex::new(AppState::new()));
    
    // 設定を初期読み込み
    {
        let mut state_guard = state.lock().unwrap();
        state_guard.config = load_or_create_config();
        state_guard.add_log("アプリケーションを開始しました".to_string());
    }

    // UIの初期設定
    {
        let state_guard = state.lock().unwrap();
        let launcher_paths: Vec<LauncherPath> = state_guard.config.launcher_paths
            .iter()
            .map(|path| LauncherPath { path: path.clone().into() })
            .collect();
        ui.set_launcher_paths(launcher_paths.as_slice().into());

        let hostnames: Vec<Hostname> = state_guard.config.hostnames
            .iter()
            .map(|name| Hostname { name: name.clone().into() })
            .collect();
        ui.set_hostnames(hostnames.as_slice().into());

        ui.set_local_addr(state_guard.config.local_addr.clone().into());
        ui.set_log_output(state_guard.get_logs().into());
    }

    // コールバック設定
    {
        let state_clone = Arc::clone(&state);
        let ui_handle = ui.as_weak();
        ui.on_add_launcher_path(move |path| {
            let mut state_guard = state_clone.lock().unwrap();
            
            match validate_launcher_path(&path) {
                Ok(validated_path) => {
                    state_guard.config.launcher_paths.push(validated_path.clone());
                    state_guard.add_log(format!("ランチャーパスを追加: {}", validated_path));
                    
                    if let Some(ui) = ui_handle.upgrade() {
                        let launcher_paths: Vec<LauncherPath> = state_guard.config.launcher_paths
                            .iter()
                            .map(|path| LauncherPath { path: path.clone().into() })
                            .collect();
                        let _ = ui.set_launcher_paths(launcher_paths.as_slice().into());
                        let _ = ui.set_status_message("ランチャーパスを追加しました".into());
                        let logs = state_guard.get_logs();
                        let _ = ui.set_log_output(logs.into());
                    }
                }
                Err(error_msg) => {
                    state_guard.add_log(format!("ランチャーパスエラー: {}", error_msg));
                    
                    if let Some(ui) = ui_handle.upgrade() {
                        let _ = ui.set_status_message(format!("エラー: {}", error_msg).into());
                        let logs = state_guard.get_logs();
                        let _ = ui.set_log_output(logs.into());
                    }
                }
            }
        });
    }

    {
        let state_clone = Arc::clone(&state);
        let ui_handle = ui.as_weak();
        ui.on_remove_launcher_path(move |index| {
            let mut state_guard = state_clone.lock().unwrap();
            if (index as usize) < state_guard.config.launcher_paths.len() {
                let removed = state_guard.config.launcher_paths.remove(index as usize);
                state_guard.add_log(format!("ランチャーパスを削除: {}", removed));
                
                if let Some(ui) = ui_handle.upgrade() {
                    let launcher_paths: Vec<LauncherPath> = state_guard.config.launcher_paths
                        .iter()
                        .map(|path| LauncherPath { path: path.clone().into() })
                        .collect();
                    let _ = ui.set_launcher_paths(launcher_paths.as_slice().into());
                    let logs = state_guard.get_logs();
                    let _ = ui.set_log_output(logs.into());
                }
            }
        });
    }

    {
        let state_clone = Arc::clone(&state);
        let ui_handle = ui.as_weak();
        ui.on_add_hostname(move |hostname| {
            let mut state_guard = state_clone.lock().unwrap();
            
            match validate_hostname(&hostname) {
                Ok(validated_hostname) => {
                    state_guard.config.hostnames.push(validated_hostname.clone());
                    state_guard.add_log(format!("ホスト名を追加: {}", validated_hostname));
                    
                    if let Some(ui) = ui_handle.upgrade() {
                        let hostnames: Vec<Hostname> = state_guard.config.hostnames
                            .iter()
                            .map(|name| Hostname { name: name.clone().into() })
                            .collect();
                        let _ = ui.set_hostnames(hostnames.as_slice().into());
                        let _ = ui.set_status_message("ホスト名を追加しました".into());
                        let logs = state_guard.get_logs();
                        let _ = ui.set_log_output(logs.into());
                    }
                }
                Err(error_msg) => {
                    state_guard.add_log(format!("ホスト名エラー: {}", error_msg));
                    
                    if let Some(ui) = ui_handle.upgrade() {
                        let _ = ui.set_status_message(format!("エラー: {}", error_msg).into());
                        let logs = state_guard.get_logs();
                        let _ = ui.set_log_output(logs.into());
                    }
                }
            }
        });
    }

    {
        let state_clone = Arc::clone(&state);
        let ui_handle = ui.as_weak();
        ui.on_remove_hostname(move |index| {
            let mut state_guard = state_clone.lock().unwrap();
            if (index as usize) < state_guard.config.hostnames.len() {
                let removed = state_guard.config.hostnames.remove(index as usize);
                state_guard.add_log(format!("ホスト名を削除: {}", removed));
                
                if let Some(ui) = ui_handle.upgrade() {
                    let hostnames: Vec<Hostname> = state_guard.config.hostnames
                        .iter()
                        .map(|name| Hostname { name: name.clone().into() })
                        .collect();
                    let _ = ui.set_hostnames(hostnames.as_slice().into());
                    let logs = state_guard.get_logs();
                    let _ = ui.set_log_output(logs.into());
                }
            }
        });
    }

    {
        let state_clone = Arc::clone(&state);
        let ui_handle = ui.as_weak();
        ui.on_update_local_addr(move |addr| {
            let mut state_guard = state_clone.lock().unwrap();
            state_guard.config.local_addr = addr.to_string();
            state_guard.add_log(format!("ローカルアドレスを更新: {}", addr));
            
            if let Some(ui) = ui_handle.upgrade() {
                let _ = ui.set_local_addr(addr);
                let logs = state_guard.get_logs();
                let _ = ui.set_log_output(logs.into());
            }
        });
    }

    {
        let state_clone = Arc::clone(&state);
        let ui_handle = ui.as_weak();
        ui.on_save_config(move || {
            let mut state_guard = state_clone.lock().unwrap();
            state_guard.add_log("設定を保存中...".to_string());
            match save_config(&state_guard.config) {
                Ok(_) => {
                    state_guard.add_log("設定を保存しました".to_string());
                    if let Some(ui) = ui_handle.upgrade() {
                        let _ = ui.set_status_message("設定を保存しました".into());
                        let logs = state_guard.get_logs();
                        let _ = ui.set_log_output(logs.into());
                    }
                }
                Err(e) => {
                    state_guard.add_log(format!("設定保存エラー: {}", e));
                    if let Some(ui) = ui_handle.upgrade() {
                        let _ = ui.set_status_message(format!("設定保存エラー: {}", e).into());
                        let logs = state_guard.get_logs();
                        let _ = ui.set_log_output(logs.into());
                    }
                }
            }
        });
    }

    {
        let state_clone = Arc::clone(&state);
        let ui_handle = ui.as_weak();
        ui.on_load_config(move || {
            let mut state_guard = state_clone.lock().unwrap();
            state_guard.config = load_or_create_config();
            state_guard.add_log("設定を再読み込みしました".to_string());
            
            if let Some(ui) = ui_handle.upgrade() {
                let launcher_paths: Vec<LauncherPath> = state_guard.config.launcher_paths
                    .iter()
                    .map(|path| LauncherPath { path: path.clone().into() })
                    .collect();
                let _ = ui.set_launcher_paths(launcher_paths.as_slice().into());

                let hostnames: Vec<Hostname> = state_guard.config.hostnames
                    .iter()
                    .map(|name| Hostname { name: name.clone().into() })
                    .collect();
                let _ = ui.set_hostnames(hostnames.as_slice().into());

                let _ = ui.set_local_addr(state_guard.config.local_addr.clone().into());
                let _ = ui.set_log_output(state_guard.get_logs().into());
            }
        });
    }

    {
        let state_clone = Arc::clone(&state);
        let ui_handle = ui.as_weak();
        ui.on_start_launcher(move || {
            let state_guard = state_clone.lock().unwrap();
            let launcher_index = ui_handle.upgrade().map(|ui| ui.get_selected_launcher_index()).unwrap_or(0) as usize;
            let hostname_index = ui_handle.upgrade().map(|ui| ui.get_selected_hostname_index()).unwrap_or(0) as usize;
            
            if launcher_index < state_guard.config.launcher_paths.len() 
                && hostname_index < state_guard.config.hostnames.len() {
                
                let launcher_path = state_guard.config.launcher_paths[launcher_index].clone();
                let hostname = state_guard.config.hostnames[hostname_index].clone();
                let local_addr = state_guard.config.local_addr.clone();
                
                drop(state_guard); // ロックを解放
                
                if let Some(ui) = ui_handle.upgrade() {
                    let _ = ui.set_is_running(true);
                    let _ = ui.set_status_message("起動中...".into());
                }
                
                let state_clone_2 = Arc::clone(&state_clone);
                let ui_handle_2 = ui_handle.clone();
                tokio::spawn(async move {
                    start_launcher_async(launcher_path, hostname, local_addr, state_clone_2, ui_handle_2).await;
                });
            }
        });
    }

    {
        let state_clone = Arc::clone(&state);
        let ui_handle = ui.as_weak();
        ui.on_stop_processes(move || {
            stop_processes(Arc::clone(&state_clone));
            if let Some(ui) = ui_handle.upgrade() {
                let _ = ui.set_is_running(false);
                let _ = ui.set_status_message("停止しました".into());
                let logs = state_clone.lock().unwrap().get_logs();
                let _ = ui.set_log_output(logs.into());
            }
        });
    }

    ui.run()?;

    Ok(())
}
