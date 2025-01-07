mod gopher;
mod handler;
mod server;

use std::path::PathBuf;

fn main() {
    // Get root directory from command line argument
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <root_directory>", args[0]);
        std::process::exit(1);
    }

    let root_dir = PathBuf::from(&args[1]);
    if !root_dir.is_dir() {
        eprintln!("Error: {} is not a directory", args[1]);
        std::process::exit(1);
    }

    let server = server::GopherServer::new(root_dir);
    if let Err(e) = server.run() {
        eprintln!("Server error: {}", e);
        std::process::exit(1);
    }
}