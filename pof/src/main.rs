use std::env;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha256, Digest};
use chrono::{DateTime, Local, TimeZone};
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
    prev_hash: String,
    hash: String,
}

#[derive(Serialize, Deserialize)]
struct Epoch {
    started_at: u64,
    ended_at: Option<u64>,
    records: Vec<SessionRecord>,
}

#[derive(Serialize, Deserialize)]
struct History {
    epochs: Vec<Epoch>,
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

        let mut history = load_history();

        let current_epoch = history.epochs.last_mut().unwrap();

        let prev_hash = current_epoch
            .records
            .last()
            .map(|r| r.hash.clone())
            .unwrap_or("GENESIS".to_string()); 
        
        let hash = generate_hash(session.start_time, end_time, duration, &prev_hash);

        let record = SessionRecord {
            start_time: session.start_time,
            end_time,
            duration,
            prev_hash,
            hash,
        };
        
        current_epoch.records.push(record);

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

fn load_history() -> History {
    let path = history_file_path();
    let data = fs::read_to_string(&path).unwrap_or_else(|_| {
        let initial_history = History {
            epochs: vec![Epoch {
                started_at: current_timestamp(),
                ended_at: None,
                records: Vec::new(),
            }],
        };
    serde_json::to_string_pretty(&initial_history).unwrap()
    });

    serde_json::from_str(&data).unwrap()
}

fn save_history(history: &History) {
    let path = history_file_path();
    let data = serde_json::to_string_pretty(history).unwrap();
    fs::write(path, data).unwrap();
}

fn generate_hash(
    start: u64,
    end: u64,
    duration: u64,
    prev_hash: &str,
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(format!(
        "{}:{}:{}:{}",
        start, end, duration, prev_hash
    ));
    format!("{:x}", hasher.finalize())
}
 
fn verify_record(record: &SessionRecord, expected_prev: &str) -> bool {
    // verify chain linkage
    if record.prev_hash != expected_prev{
        return false;
    }
    // verify hash integrity
    let expected_hash = generate_hash(
        record.start_time,
        record.end_time,
        record.duration,
        expected_prev,
        );

    expected_hash == record.hash
}

fn format_timestamp(ts: u64) -> String {
    let datetime: DateTime<Local> = Local.timestamp_opt(ts as i64, 0).unwrap();
    datetime.format("%d %b %Y, %H:%M").to_string()
}

fn show_history() {
    let history = load_history();

    if history.epochs.is_empty() {
        println!("No history found.");
        return;
    }
        let first_epoch = &history.epochs[0];

    println!("\n📊 Proof of Focus — History\n");

    println!(
    "User started using PoF on: {}\n",
    format_timestamp(first_epoch.started_at)
    );


    for (i, epoch) in history.epochs.iter().enumerate() {
        let is_current = epoch.ended_at.is_none();

        if is_current {
            println!("🟢 Epoch {} (Current)", i + 1);
        } else {
            println!("🔵 Epoch {}", i + 1);
        }

        println!("Started at: {}", format_timestamp(epoch.started_at));

        match epoch.ended_at {
            Some(t) => println!("Ended at:   {} (reset)", format_timestamp(t)),
            None => println!("Ended at:   —"),
        }

        println!("Sessions: {}", epoch.records.len());

        if epoch.records.is_empty() {
        println!("No sessions recorded.\n");
        continue;
        }

        let mut prev = "GENESIS".to_string();
        let mut valid_epoch = true;

        for record in &epoch.records {
            if !verify_record(record, &prev) {
                valid_epoch = false;
                break;
            }
            prev = record.hash.clone();
        }

        println!(
            "Status: {}\n",
            if valid_epoch { "✅ Trusted" } else { "❌ Untrusted" }
        );

        println!("Sessions:");
        for (j, record) in epoch.records.iter().enumerate() {
            println!(
                "  #{} | Duration: {}s | Started: {}",
                j + 1,
                record.duration,
                format_timestamp(record.start_time)
            );
        }

        println!("\n────────────────────────────\n");
    }
}

fn reset_chain() {
    let mut history = load_history();

    // End the current epoch
    if let Some(current_epoch) = history.epochs.last_mut() {
        current_epoch.ended_at = Some(current_timestamp());
    }

    // Start a new epoch
    history.epochs.push(Epoch {
        started_at: current_timestamp(),
        ended_at: None,
        records: Vec::new(),
    });

    save_history(&history);

    println!("✔ Trust chain reset.");
    println!("You are now starting from a new genesis.");
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
        "reset" => reset_chain(),
        _ => print_help(),
    }
}


