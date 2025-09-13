use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
use std::thread;
use std::time::Duration;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
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

fn main() {
    println!("=== Minecraft Launcher with Cloudflared ===");
    let config = load_or_create_config();
    if config.launcher_paths.is_empty() || config.hostnames.is_empty() {
        println!("初回設定が必要です。");
        let updated_config = setup_initial_config(config);
        save_config(&updated_config);
        run_launcher(&updated_config);
    } else {
        println!("設定を読み込みました:");
        println!("  ランチャーパス: {:?}", config.launcher_paths);
        println!("  ローカルアドレス: {}", config.local_addr);
        println!("  ホスト名: {:?}", config.hostnames);
        println!("\n1. 実行");
        println!("2. 設定変更");
        print!("選択してください (1 or 2): ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "2" => {
                let updated_config = setup_initial_config(config);
                save_config(&updated_config);
                run_launcher(&updated_config);
            }
            _ => {
                run_launcher(&config);
            }
        }
    }
}

fn load_or_create_config() -> Config {
    if Path::new(CONFIG_FILE).exists() {
        match fs::read_to_string(CONFIG_FILE) {
            Ok(content) => {
                match serde_json::from_str(&content) {
                    Ok(config) => config,
                    Err(e) => {
                        println!("設定ファイルの読み込みエラー: {}", e);
                        Config::default()
                    }
                }
            }
            Err(e) => {
                println!("設定ファイルの読み込みエラー: {}", e);
                Config::default()
            }
        }
    } else {
        Config::default()
    }
}

fn save_config(config: &Config) {
    match serde_json::to_string_pretty(config) {
        Ok(json) => {
            if let Err(e) = fs::write(CONFIG_FILE, json) {
                println!("設定ファイルの保存エラー: {}", e);
            } else {
                println!("設定を保存しました。");
            }
        }
        Err(e) => {
            println!("設定のシリアライズエラー: {}", e);
        }
    }
}

fn setup_initial_config(mut config: Config) -> Config {
    println!("\n=== 設定 ===");
    config.launcher_paths.clear();
    loop {
        print!("Minecraftランチャーのパスを追加（終了は空欄でEnter）: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let path = input.trim().to_string();
        if path.is_empty() {
            break;
        }
        config.launcher_paths.push(path);
    }
    print!("ローカルアドレス (現在: {}): ", config.local_addr);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let addr = input.trim();
    if !addr.is_empty() {
        config.local_addr = addr.to_string();
    }
    config.hostnames.clear();
    loop {
        print!("ホスト名を追加（終了は空欄でEnter）: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let hostname = input.trim().to_string();
        if hostname.is_empty() {
            break;
        }
        config.hostnames.push(hostname);
    }
    config
}

fn run_launcher(config: &Config) {
    println!("\n=== 実行開始 ===");
    // ランチャー選択
    println!("Minecraftランチャーを選択してください:");
    for (i, path) in config.launcher_paths.iter().enumerate() {
        println!("  {}. {}", i + 1, path);
    }
    print!("番号を入力: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let launcher_idx = input.trim().parse::<usize>().unwrap_or(1) - 1;
    let launcher_path = config.launcher_paths.get(launcher_idx).unwrap_or(&config.launcher_paths[0]);

    // ホスト名選択
    println!("ホスト名を選択してください:");
    for (i, hostname) in config.hostnames.iter().enumerate() {
        println!("  {}. {}", i + 1, hostname);
    }
    print!("番号を入力: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let hostname_idx = input.trim().parse::<usize>().unwrap_or(1) - 1;
    let hostname = config.hostnames.get(hostname_idx).unwrap_or(&config.hostnames[0]);

    // ランチャー起動
    let result = if launcher_path.ends_with(".url") {
        Command::new("cmd")
            .args(&["/C", "start", "", launcher_path])
            .spawn()
    } else {
        Command::new(launcher_path).spawn()
    };
    match result {
        Ok(_) => {}
        Err(e) => {
            println!("ランチャーの起動エラー: {}", e);
            println!("パスを確認してください: {}", launcher_path);
            return;
        }
    }
    thread::sleep(Duration::from_secs(5));
    // cloudflared起動
    match Command::new("cloudflared")
        .args(&[
            "access",
            "tcp",
            "--hostname",
            hostname,
            "--url",
            &config.local_addr,
        ])
        .spawn()
    {
        Ok(mut child) => {
            println!("cloudflaredを起動しました。プロセスID: {}", child.id());
            match child.wait() {
                Ok(_) => {}
                Err(e) => {
                    println!("プロセス待機エラー: {}", e);
                }
            }
        }
        Err(e) => {
            println!("cloudflaredの起動エラー: {}", e);
        }
    }
}
