use std::sync::atomic::{AtomicUsize, Ordering};
use clap::builder::Str;
use crate::flags::Flags;

// Declare a static variable to keep track of the line number
static LINE_NUMBER: AtomicUsize = AtomicUsize::new(1);

pub trait Handler {
	fn handle(&self, line: &mut String);
}

pub struct ShowEndsHandler;
pub struct ShowTabsHandler;
pub struct NumberNonBlankHandler;
pub struct ShowNonPrintingHandler;
pub struct NumberHandler;

impl Handler for ShowEndsHandler {
	fn handle(&self, line: &mut String) {
		if line.contains("\r\n") {
			*line = line.replace("\r\n", "$\r\n");
		}
		else if line.contains("\n") {
			*line = line.replace("\n", "$\n");
		}
	}
}

impl Handler for ShowTabsHandler {
	fn handle(&self, line: &mut String) {
		*line = line.replace("\t", "^I");
	}
}

impl Handler for NumberNonBlankHandler {
	fn handle(&self, line: &mut String) {
		if line != "$\n" && line != "\n" {
			let current_line = LINE_NUMBER.fetch_add(1, Ordering::SeqCst);
			*line = format!("{:6}  {}", current_line, line);
		}
	}
}

impl Handler for NumberHandler {
	fn handle(&self, line: &mut String) {
		let current_line = LINE_NUMBER.fetch_add(1, Ordering::SeqCst);
		*line = format!("{:6}  {}", current_line, line);
	}
}

impl Handler for ShowNonPrintingHandler {
	fn handle(&self, line: &mut String) {
		*line = line.chars().map(|c| {
			match c {
				'\n' => '\n'.to_string(),
				'\t' => '\t'.to_string(),
				'\r' => '\r'.to_string(),
				c if c.is_control() => format!("^{}", (c as u8 + 64) as char),  // Control chars to caret notation
				_ => c.to_string(),        // Other characters remain the same
			}
		}).collect::<String>();
	}
}

pub struct PipelineObject {
	handler: Box<dyn Handler>
}

impl PipelineObject {
	pub fn new<H: Handler + 'static>(handler: H) -> Self {
		PipelineObject {
			handler: Box::new(handler)
		}
	}
}

pub struct Pipeline {
	pipeline: Vec<PipelineObject>
}

impl From<Flags> for Pipeline {
	fn from(value: Flags) -> Self {
		let mut pipeline = Pipeline::new();
		if value.contains(Flags::SHOW_ENDS) {
			pipeline.add_handler(ShowEndsHandler);
		}
		if value.contains(Flags::SHOW_TABS) {
			pipeline.add_handler(ShowTabsHandler);
		}
		if value.contains(Flags::NUMBER_NONBLANK) {
			pipeline.add_handler(NumberNonBlankHandler);
		}
		if value.contains(Flags::NUMBER) {
			pipeline.add_handler(NumberHandler);
		}
		if value.contains(Flags::SHOW_NONPRINTING) {
			pipeline.add_handler(ShowNonPrintingHandler);
		}
		pipeline
	}
}

impl Pipeline {

	pub fn new() -> Self {
		Pipeline {
			pipeline: Vec::with_capacity(8)
		}
	}

	pub fn add_handler<H: Handler + 'static>(&mut self, handler: H) {
		self.pipeline.push(PipelineObject::new(handler));
	}

	pub fn execute(&mut self, line: &String) -> String {
		if self.pipeline.len() > 0 {
			let mut line= String::from(line);
			for obj in &self.pipeline {
				obj.handler.handle(&mut line);
			}
			return line
		}
		String::from(line)
	}
}
