extern crate db;
extern crate chain;
extern crate ethcore_devtools as devtools;
extern crate test_data;
extern crate time;

mod database;

use time::{PreciseTime, Duration};
use std::io::Write;
use std::str;

#[derive(Default)]
pub struct Benchmark {
	start: Option<PreciseTime>,
	end: Option<PreciseTime>,
}

impl Benchmark {
	pub fn start(&mut self) {
		self.start = Some(PreciseTime::now());
	}

	pub fn stop(&mut self) {
		self.end = Some(PreciseTime::now());
	}

	pub fn evaluate(&self) -> Duration {
		self.start.expect("benchmarch never ended").to(self.end.expect("benchmark never started"))
	}
}

fn decimal_mark(s: String) -> String {
    let bytes: Vec<_> = s.bytes().rev().collect();
    let chunks: Vec<_> = bytes.chunks(3).map(|chunk| str::from_utf8(chunk).unwrap()).collect();
    let result: Vec<_> = chunks.join(",").bytes().rev().collect();
    String::from_utf8(result).unwrap()
}


fn run_benchmark<F>(name: &str, f: F) where F: FnOnce(&mut Benchmark) {
	print!("{}: ", name);
	::std::io::stdout().flush().unwrap();

	let mut benchmark = Benchmark::default();
	f(&mut benchmark);
	println!("{} ns", decimal_mark(format!("{}", benchmark.evaluate().num_nanoseconds().unwrap())));
}

macro_rules! benchmark {
    ($t:expr) => {
    	run_benchmark(stringify!($t), $t);
    };
}

fn main() {
	benchmark!(database::fetch);
	benchmark!(database::write);
	benchmark!(database::reorg_short);
	benchmark!(database::write_heavy);
}
