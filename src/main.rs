use std::env;
use std::fs;

fn main() {
	let args: Vec<String> = env::args().collect();

	if args.len() < 2 {
		eprintln!("Usage: textstats <file>");
		return;
	}
	
	let file_path = &args[1];
	let contents = fs::read_to_string(file_path);

	match contents {
		Ok(data) => {
			let lines = data.lines().count();
			let words = data.split_whitespace().count();
			let characters = data.chars().count();

			println!("Lines: {}", lines);
			println!("Words: {}", words);
			println!("Characters: {}", characters);
		}
		Err(e) => {
			eprintln!("Failed to read file: {}", e);
		}
	}
}
