use std::env;

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
        _ => print_help(),
    }
}

fn start_session() {
    println!("Focus session started.");
}

fn stop_session() {
    println!("Focus session stopped.");
}

fn show_status() {
    println!("No active focus session.");
}

fn print_help() {
    println!("Usage:");
    println!("  pof start   - Start a focus session");
    println!("  pof stop    - Stop the current session");
    println!("  pof status  - Show session status");
}

