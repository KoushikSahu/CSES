use std::io::{stdin, stdout, BufWriter, prelude::BufRead, prelude::Write};
use std::str;

/*
	Author: Koushik Sahu
	Created: 01:44:43 AM(01:44:43) IST(+05:30) 04-07-2023 Tue
*/

fn solve<R: BufRead, W: Write>(sc: &mut Scanner<R>, wr: &mut W) {
	let n: i32 = sc.next();
	let mut x: Vec<i32> = (0..n).map(|_| sc.next()).collect();
	let mut ans: i64 = 0;
	for i in 1..n {
		let idx = i as usize;
		if x[idx] < x[idx-1] {
			ans = ans + (x[idx-1] - x[idx]) as i64;
			x[idx] = x[idx-1];
		}
	}
	writeln!(wr, "{}", ans).ok();
}

#[macro_export]
macro_rules! dbg{
	($($a:expr),*) => {
		#[cfg(debug_assertions)]
		eprintln!(
			concat!("{}:{}:{}: ",$(stringify!($a), " = {:?}, "),*),
			file!(), line!(), column!(), $($a),*
			);
		#[cfg(not(debug_assertions))]
		{};
	}
}

struct Scanner<R> {
	reader: R,
	buf_str: Vec<u8>,
	buf_iter: str::SplitWhitespace<'static>,
}

impl<R: BufRead> Scanner<R> {
	fn new(reader: R) -> Self {
		Self {
			reader,
			buf_str: vec![],
			buf_iter: "".split_whitespace(),
		}
	}
	fn next<T: str::FromStr>(&mut self) -> T {
		loop {
			if let Some(token) = self.buf_iter.next() {
				return token.parse().ok().expect("Failed parse");
			}
			self.buf_str.clear();
			self.reader
				.read_until(b'\n', &mut self.buf_str)
				.expect("Failed read");
			self.buf_iter = unsafe {
				let slice = str::from_utf8_unchecked(&self.buf_str);
				std::mem::transmute(slice.split_whitespace())
			}
		}
	}
}

fn main() {
	let (stdin, stdout) = (stdin(), stdout());
	let mut scan = Scanner::new(stdin.lock());
	let mut out = BufWriter::new(stdout.lock());
	#[allow(unused_variables)]
	let t: i32 = 1;
	//let t: i32 = scan.next(); 
	for _ in 0..t {
		solve(&mut scan, &mut out);
	}
}
