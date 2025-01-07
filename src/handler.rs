use std::fs::{self, File};
use std::io::{self, Write, BufReader, BufRead};
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::time::Duration;

use crate::gopher::{ITEM_DIRECTORY, ITEM_FILE, ITEM_INFO, ITEM_IMAGE, create_menu_line};

pub fn handle_request(mut stream: TcpStream, root_dir: &Path) -> io::Result<()> {
    // Set read timeout to prevent hanging
    stream.set_read_timeout(Some(Duration::from_secs(5)))?;
    
    // Create a buffered reader
    let mut reader = BufReader::new(&stream);
    let mut selector = String::new();
    
    // Read until we get a line ending
    match reader.read_line(&mut selector) {
        Ok(_) => {
            // Clean up the selector
            let selector = selector.trim_end();
            println!("Received selector: '{}'", selector);
            
            // Convert selector to filesystem path
            let path = root_dir.join(selector.trim_start_matches('/'));
            println!("Looking up path: {}", path.display());
            
            if path.is_dir() {
                println!("Serving directory: {}", path.display());
                serve_directory(&path, &mut stream, root_dir)?;
            } else if path.is_file() {
                println!("Serving file: {}", path.display());
                serve_file(&path, &mut stream)?;
            } else {
                println!("Resource not found: {}", path.display());
                let error = format!("{}Resource not found: {}\r\n", ITEM_INFO, selector);
                stream.write_all(error.as_bytes())?;
            }
        }
        Err(e) => {
            eprintln!("Error reading from client: {}", e);
            let error = format!("{}Error reading request\r\n", ITEM_INFO);
            stream.write_all(error.as_bytes())?;
        }
    }
    
    Ok(())
}

pub fn serve_file(file_path: &Path, stream: &mut TcpStream) -> io::Result<()> {
    println!("Sending file contents...");
    let mut file = File::open(file_path)?;
    io::copy(&mut file, stream)?;
    println!("File sent successfully");
    Ok(())
}

pub fn serve_directory(dir_path: &Path, stream: &mut TcpStream, root_dir: &Path) -> io::Result<()> {
    println!("Sending directory listing...");
    // Send directory header
    let relative_path = dir_path.strip_prefix(root_dir)
        .unwrap_or(dir_path)
        .display()
        .to_string();
    
    let header = format!("{}Directory listing of {}\r\n", ITEM_INFO, relative_path);
    stream.write_all(header.as_bytes())?;

    // List directory contents
    let entries = fs::read_dir(dir_path)?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        let name = entry.file_name();
        let display_name = name.to_string_lossy();
        
        // Create relative selector
        let selector = path.strip_prefix(root_dir)
            .unwrap_or(&path)
            .to_string_lossy()
            .into_owned();

        // Determine item type
        let item_type = if path.is_dir() {
            ITEM_DIRECTORY
        } else {
            // Check file extension for image types
            if let Some(extension) = path.extension() {
                match extension.to_str().unwrap_or("").to_lowercase().as_str() {
                    "gif" | "png" | "jpg" | "jpeg" => ITEM_IMAGE,
                    _ => ITEM_FILE
                }
            } else {
                ITEM_FILE
            }
        };

        let menu_line = create_menu_line(item_type, &display_name, &selector);
        stream.write_all(menu_line.as_bytes())?;
    }
    
    println!("Directory listing sent successfully");
    Ok(())
}