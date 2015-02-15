#![feature(io)]
use std::old_io;

fn main() {
	let mut output = old_io::stdout();
	let mut input = old_io::stdin();

	loop {
		let line = input.read_line().unwrap();
		match line.as_bytes()[0] {
			//"!quit" => break,
			b'!' => output.write_all(line[1..].as_bytes()).unwrap(),
			_ => output.write_all(line.as_bytes()).unwrap(),
		}
	}
}
