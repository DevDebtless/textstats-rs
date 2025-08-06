# textstats-rs  

A command-line tool written in Rust that provies detailed statistics about a text file. This project is part of my journey to master systems-level programming and build tools from scratch using high-performance languages.

## Features  

- Word count  
- Character count  
- Line count  
- Average word length  
- Longest word  
- Most common word (coming soon)  

## Why Rust?  

Rust offers memory safety, speed, and modern tooling, making it ideal for building reliable and efficient command-line utilities. This version of 'TextStats' is my first step into systems programming and tooling with Rust.  

## Getting Started  

### Prerequisites  

- [Rust](https://www.rust-lang.org/tools/install)  

### Clone the Repository  

```bash  
git clone https://github.com/DevDebtless/textstats-rs.git  
cd textstats-rs  
```
Build the Project  
`cargo build --release`

Run it  
`cargo run -- <file_path>`

Example  
`cargo run -- sample.txt`

Project Structure
textstats-rs/
├── Cargo.toml       # Project metadata and dependencies
└── src/
    └── main.rs      # Application entry point
