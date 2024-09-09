use std::cmp::PartialEq;
use std::io;
use std::ops::BitAnd;
use clap::Parser;
use crate::Args;
use crate::flags::Flags;
use crate::input_reader::InputReader;
use crate::pipeline::{Pipeline, ShowEndsHandler};

pub struct Cat {
	suppress: bool,
	files: Vec<String>,
	pipeline: Pipeline
}


impl Cat {
	pub fn new(flags: Flags, files: Vec<String>) -> Self {
		Cat {
			suppress: flags.contains(Flags::SQUEEZE_BLANK),
			files,
			pipeline: Pipeline::from(flags)
		}
	}

	pub fn run(&mut self) -> io::Result<()> {
		let mut lines: Vec<String> = Vec::new();
		let mut result: Vec<String> = Vec::new();

		if self.files.is_empty() {
			let mut reader = InputReader::from_stdin();
			loop {
				if let Some(line) = reader.read_line()? {
					lines.push(line);
					for line in &lines {
						result.push(self.pipeline.execute(&line));
					}
					print!("{}", result.join(""));
					lines.clear();
					result.clear();
				} else {
					return Ok(())
				}
			}
		} else {
			for file in &self.files {
				let mut reader = InputReader::from_file(file)?;
				while let Some(line) = reader.read_line()? {
					if self.suppress {
						if lines.iter().last() == Some(&"\n".to_string()) && line == "\n" {} else {
							lines.push(line);
						}
					} else {
						lines.push(line);
					}
				}
			}
		}
		for line in lines {
			result.push(self.pipeline.execute(&line));
		}
		print!("{}", result.join(""));
		Ok(())
	}
}
