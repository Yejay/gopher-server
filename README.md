# Gopher Server

A Rust implementation of a Gopher protocol server that provides static file serving capabilities and maps local filesystem content to Gopherspace.

## Features

- Static file serving with complete Gopher protocol support
- Support for multiple Gopher item types:
  - Text files (0)
  - Directories/Menus (1)
  - Image files (I)
  - Information messages (i)
- Automatic content type detection
- Directory listing with proper Gopher menu formatting
- Clean separation of concerns using Rust modules
- Proper error handling and logging
- Support for concurrent client connections

## Installation

### Prerequisites
- Rust (latest stable version)
- Cargo (comes with Rust)
- Root/sudo access (for binding to port 70)

### Building from source

1. Clone the repository:
```bash
git clone <your-repository-url>
cd gopher-server
```

2. Build the project:
```bash
cargo build --release
```

The compiled binary will be available at `target/release/gopher_server`

## Usage

The repository includes a `test-content` directory with sample files ready for testing.

Run the server pointing to the included test content:
```bash
sudo cargo run -- $(pwd)/test-content
```

The server will start listening on port 70 (requires sudo for port binding).

### Test Content Structure

The included test content contains:
```
test-content/
├── hello.txt           # Simple text file
├── documents/          # Directory with documents
│   └── test.txt       # Test document
└── images/            # Directory with images
    ├── sample.png     # Sample PNG image
    └── sample.gif     # Sample GIF image
```

## Project Structure

```
gopher-server/
├── src/
│   ├── main.rs      # Application entry point and command-line handling
│   ├── server.rs    # TCP server implementation
│   ├── handler.rs   # Request handling logic
│   └── gopher.rs    # Gopher protocol constants and utilities
├── test-content/    # Example content directory
│   ├── documents/   # Text files and documents
│   └── images/      # Image files (PNG, GIF, JPEG)
├── Cargo.toml       # Project configuration
└── README.md        # This file
```

## Protocol Implementation

The server implements the Gopher protocol as specified in RFC 1436. It supports:

- Directory listing with proper menu formatting
- File content serving
- Image file type detection and serving
- Proper selector string handling
- Error reporting

## Testing

The server can be tested with any Gopher client, including:
- Your own Gopher client implementation
- The Lynx text browser
- Other Gopher clients (Bombadillo, VF-1, etc.)

Example test using netcat:
```bash
echo "/" | nc localhost 70    # Get root directory listing
echo "/hello.txt" | nc localhost 70    # Get content of hello.txt
```
