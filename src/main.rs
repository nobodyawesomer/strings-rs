use std::fs;

use clap::Parser;
use eyre::Result;
use regex::bytes::Regex;

#[derive(Parser)]
struct Cli {
	/// Print sequences of characters that are least MIN-LEN long,
	/// instead of the default 4.
	#[arg(short = 'n', long, id = "MIN-LEN")]
	bytes: Option<u32>,

	/// Files to input to the program
	files: Vec<String>,
}

fn main() -> Result<()> {
	let cli = Cli::parse();
	let min_len = cli.bytes.unwrap_or(4);

	eprintln!("Filtering for a length of {min_len}");
	let re = {
		// ASCII-compatible mode
		// Capture sequence of printable characters, ending in Null
		// Substitute in with format!-escaped minimum length
		let re = format!(r"(?ms)([\s\x20-\x7e]{{{min_len},}})\x00");
		Regex::new(&re).unwrap()
	};

	for file in cli.files {
		let bytes = fs::read(file)?;
		for cap in re.captures_iter(&bytes) {
			let s = String::from_utf8_lossy(&cap[1]); // guaranteed 1 capture on match
										  // println!("++++++++++++++++++++++++\n{s}\n------------------------");
			println!("{s}");
		}
	}

	Ok(())
}
