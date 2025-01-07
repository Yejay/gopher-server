use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::io;

use crate::handler;
use crate::gopher::PORT;

pub struct GopherServer {
    root_dir: PathBuf,
}

impl GopherServer {
    pub fn new<P: AsRef<Path>>(root_dir: P) -> Self {
        GopherServer {
            root_dir: root_dir.as_ref().to_path_buf(),
        }
    }

    pub fn run(&self) -> io::Result<()> {
        let addr = format!("0.0.0.0:{}", PORT);
        let listener = TcpListener::bind(&addr)?;
        
        println!("Gopher server listening on {}", addr);
        println!("Serving content from: {}", self.root_dir.display());
        
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("New connection established!");
                    if let Err(e) = handler::handle_request(stream, &self.root_dir) {
                        eprintln!("Error handling connection: {}", e);
                    }
                }
                Err(e) => eprintln!("Connection failed: {}", e),
            }
        }

        Ok(())
    }
}