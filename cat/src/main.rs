mod input_reader;
mod line_processor;

use crate::input_reader::InputReader;
use clap::Parser;
use std::fmt::Debug;
use std::io;

#[derive(Parser, Debug)]
#[command(author="Robu Andrei", version="1.0", about="Concatenate FILE(s) to standard output.\n\nWith no FILE or when FILE is -, the program reads from standard input.")]
struct Args {
	#[arg(short='A', long="show-all", help="equivalent to -vET", default_value = "false")]
	show_all: bool,

	#[arg(short='b', long="number-nonblank", help="number nonempty output lines, overrides -n", default_value = "false")]
	number_nonblank: bool,

	#[arg(short='e', help="equivalent to -vE", default_value = "false")]
	ve: bool,

	#[arg(short='E', long="show-ends", default_value = "false", help = "display $ at end of each line")]
	show_ends: bool,

	#[arg(short='n', long="number", help="number all output lines", default_value = "0")]
	number: u32,

	#[arg(short='s', long="squeeze-blank", help="suppress repeated empty output lines", default_value = "false")]
	squeeze_blank: bool,

	#[arg(short='t', help="equivalent to -vT", default_value = "false")]
	vt: bool,

	#[arg(short='T', long="show-tabs", help="display TAB characters as ^I", default_value = "false")]
	show_tabs: bool,

	#[arg(short='u', default_value = "false", help = "(ignored)")]
	ignored: bool,

	#[arg(short='v', long="show-nonprinting", help="use ^ and M- notation, except for LFD and TAB", default_value = "false")]
	show_nonprinting: bool,

	#[arg(help = "The file contents to display", num_args = 1..)]
	file: Vec<String>
}


fn main() -> io::Result<()>{
	let args = Args::parse();
	match args { Args { .. } => {
			process_lines(&args.file)?;
		}
	}
	Ok(())
}

fn  process_lines(vec: &Vec<String>) -> io::Result<()>{
	for file in vec {
		let mut reader = InputReader::from_file(&file)?;
		loop {
			match reader.read_line()? {
				None => { println!("EOF Reached"); break ;},
				Some(line) => print!("{}", line)
			}
		}
	}
	Ok(())
}
