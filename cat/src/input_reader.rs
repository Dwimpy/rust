use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub struct InputReader<T: BufRead> {
	reader: T
}

impl InputReader<BufReader<io::Stdin>> {

	pub fn from_stdin() -> Self {
		InputReader {
			reader: BufReader::new(io::stdin())
		}
	}
}

impl InputReader<BufReader<File>> {

	pub fn from_file<P: AsRef<Path>>(file_path: P) -> io::Result<Self> {
		let file = File::open(file_path)?;
		Ok(
			InputReader {
			reader: BufReader::new(file)
		})
	}
}

impl<R: BufRead> InputReader<R> {

	pub fn read_line(&mut self) -> io::Result<Option<String>> {
		let mut line = String::new();
		let bytes_read = self.reader.read_line(&mut line)?;
		match bytes_read == 0 {
			true => Ok(None),
			false => Ok(Some(line))
		}
	}
}