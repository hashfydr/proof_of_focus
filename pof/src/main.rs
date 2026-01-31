use std::env;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha256, Digest};

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Session {
    start_time: u64,
}

#[derive(Serialize, Deserialize)]
struct SessionRecord {
    start_time: u64,
    end_time: u64,
    duration: u64,
    hash: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return;
    }

    match args[1].as_str() {
        "start" => start_session(),
        "stop" => stop_session(),
        "status" => show_status(),
        "history" => show_history(),
        _ => print_help(),
    }

    fn session_file_path() -> PathBuf {
    let mut path = dirs::home_dir().expect("Could not find home directory");
    path.push(".pof");
    fs::create_dir_all(&path).unwrap();
    path.push("session.json");
    path
    }

    fn load_session() -> Option<Session> {
    let path = session_file_path();
    let data = fs::read_to_string(path).ok()?;
    serde_json::from_str(&data).ok()
    }

    fn save_session(session: &Session) {
    let path = session_file_path();
    let data = serde_json::to_string(session).unwrap();
    fs::write(path, data).unwrap();
    }

    fn delete_session() {
    let path = session_file_path();
    let _ = fs::remove_file(path);
    }

    fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
    }
    
    fn start_session() {
    if load_session().is_some() {
        println!("A focus session is already running.");
        return;
    }

    let session = Session {
        start_time: current_timestamp(),
    };

    save_session(&session);
    println!("Focus session started.");
    }

   fn stop_session() {
    if let Some(session) = load_session() {
        let end_time = current_timestamp();
        let duration = end_time - session.start_time;

        let hash = generate_hash(session.start_time, end_time, duration);

        let record = SessionRecord {
            start_time: session.start_time,
            end_time,
            duration,
            hash,
        };

        let mut history = load_history();
        history.push(record);
        save_history(&history);

        delete_session();

        println!("Focus session stopped.");
        println!("Duration: {} seconds", duration);
        } else {
        println!("No active focus session.");
        }
    }

    fn show_status() {
    if let Some(session) = load_session() {
        let elapsed = current_timestamp() - session.start_time;
        println!("Focus session running.");
        println!("Elapsed: {} seconds", elapsed);
    } else {
        println!("No active focus session.");
        }
    }

    fn print_help() {
    println!("Usage:");
    println!("  pof start   - Start a focus session");
    println!("  pof stop    - Stop the current session");
    println!("  pof status  - Show session status");
    }
    
    fn history_file_path() -> PathBuf {
    let mut path = dirs::home_dir().unwrap();
    path.push(".pof");
    path.push("history.json");
    path
    }

    fn load_history() -> Vec<SessionRecord> {
    let path = history_file_path();
    let data = fs::read_to_string(path).unwrap_or("[]".to_string());
    serde_json::from_str(&data).unwrap_or(Vec::new())
    }

    fn save_history(history: &Vec<SessionRecord>) {
    let path = history_file_path();
    let data = serde_json::to_string_pretty(history).unwrap();
    fs::write(path, data).unwrap();
    }

    fn generate_hash(start: u64, end: u64, duration: u64) -> String {
    let mut hasher = Sha256::new();
    hasher.update(format!("{}:{}:{}", start, end, duration));
    format!("{:x}", hasher.finalize())
    }
    
    fn verify_record(record: &SessionRecord) -> bool {
    let expected = generate_hash(
        record.start_time,
        record.end_time,
        record.duration,
        );

    expected == record.hash
    }

    fn show_history() {
    let history = load_history();

    if history.is_empty() {
        println!("No session history found.");
        return;
    }

    for (i, record) in history.iter().enumerate() {
        let valid = verify_record(record);

        println!(
            "#{} | Duration: {}s | Hash: {} | Valid: {}",
            i + 1,
            record.duration,
            record.hash,
            if valid { "YES" } else { "NO ❌" }
            );
        }
    }



}

