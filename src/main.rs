use clap::Parser;

#[derive(Parser)]
struct Cli {
	/// Print sequences of characters that are least MIN-LEN long,
	/// instead of the default 4.
	#[arg(short = 'n', long, id = "MIN-LEN")]
	bytes: Option<u32>,

	/// Files to input to the program
	files: Vec<String>,
}

fn main() {
	let cli = Cli::parse();
	let min_len = cli.bytes.unwrap_or(4);

	eprintln!("Filtering for a length of {min_len}");
}
