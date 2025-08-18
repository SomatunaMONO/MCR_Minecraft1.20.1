use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
use std::thread;
use std::time::Duration;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    launcher_path: String,
    local_addr: String,
    hostname: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            launcher_path: String::new(),
            local_addr: "127.0.0.1:20100".to_string(),
            hostname: "minecraft.nitmcr.f5.si".to_string(),
        }
    }
}

const CONFIG_FILE: &str = "config.json";

fn main() {
    println!("=== Minecraft Launcher with Cloudflared ===");    
    let config = load_or_create_config();    
    if config.launcher_path.is_empty() {
        println!("初回設定が必要です。");
        let updated_config = setup_initial_config(config);
        save_config(&updated_config);
        run_launcher(&updated_config);
    } else {
        println!("設定を読み込みました:");
        println!("  ランチャーパス: {}", config.launcher_path);
        println!("  ローカルアドレス: {}", config.local_addr);
        println!("  ホスト名: {}", config.hostname);      
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
    loop {
        print!("Minecraftランチャーのパスを入力してください: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let path = input.trim().to_string();
        if path.is_empty() {
            println!("パスを入力してください。");
            continue;
        }
        if !Path::new(&path).exists() {
            println!("ファイルが存在しません: {}", path);
            print!("それでも続行しますか？ (y/n): ");
            io::stdout().flush().unwrap();
            let mut confirm = String::new();
            io::stdin().read_line(&mut confirm).unwrap();
            if confirm.trim().to_lowercase() != "y" {
                continue;
            }
        }
        config.launcher_path = path;
        break;
    }
    print!("ローカルアドレス (現在: {}): ", config.local_addr);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let addr = input.trim();
    if !addr.is_empty() {
        config.local_addr = addr.to_string();
    }
    print!("ホスト名 (現在: {}): ", config.hostname);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let hostname = input.trim();
    if !hostname.is_empty() {
        config.hostname = hostname.to_string();
    }  
    config
}

fn run_launcher(config: &Config) {
    println!("\n=== 実行開始 ===");
    let launcher_path = &config.launcher_path;
    println!("Minecraftランチャーを起動しています...");    
    let result = if launcher_path.ends_with(".url") {
        println!("ショートカット (.url) を起動します: {}", launcher_path);
        Command::new("cmd")
            .args(&["/C", "start", "", launcher_path])
            .spawn()
    } else {
        Command::new(launcher_path).spawn()
    };
    match result {
        Ok(_) => {
            println!("ランチャーを起動しました。");
        }
        Err(e) => {
            println!("ランチャーの起動エラー: {}", e);
            println!("パスを確認してください: {}", launcher_path);
            return;
        }
    }
    println!("5秒待機してからcloudflaredを起動します...");
    thread::sleep(Duration::from_secs(5));
    println!("cloudflaredを起動しています...");
    println!("コマンド: cloudflared access tcp --hostname {} --url {}", config.hostname, config.local_addr);
    match Command::new("cloudflared")
        .args(&[
            "access",
            "tcp",
            "--hostname",
            &config.hostname,
            "--url",
            &config.local_addr,
        ])
        .spawn()
    {
        Ok(mut child) => {
            println!("cloudflaredを起動しました。");
            println!("プロセスID: {}", child.id());
            println!("終了するには Ctrl+C を押してください。");
            match child.wait() {
                Ok(status) => {
                    println!("cloudflaredが終了しました。ステータス: {}", status);
                }
                Err(e) => {
                    println!("プロセス待機エラー: {}", e);
                }
            }
        }
        Err(e) => {
            println!("cloudflaredの起動エラー: {}", e);
            println!("cloudflaredがインストールされているか確認してください。");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.local_addr, "127.0.0.1:20100");
        assert_eq!(config.hostname, "minecraft.nitmcr.f5.si");
        assert!(config.launcher_path.is_empty());
    }
    
    #[test]
    fn test_config_serialization() {
        let config = Config {
            launcher_path: "C:\\test\\launcher.exe".to_string(),
            local_addr: "192.168.1.10:25565".to_string(),
            hostname: "test.example.com".to_string(),
        };
        
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: Config = serde_json::from_str(&json).unwrap();
        
        assert_eq!(config.launcher_path, deserialized.launcher_path);
        assert_eq!(config.local_addr, deserialized.local_addr);
        assert_eq!(config.hostname, deserialized.hostname);
    }
}