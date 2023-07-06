use std::io::{stdin, stdout, BufWriter, prelude::BufRead, prelude::Write};
use std::str;
use std::cmp::max;

/*
	Author: Koushik Sahu
	Created: 01:10:59 AM(01:10:59) IST(+05:30) 04-07-2023 Tue
*/

fn solve<R: BufRead, W: Write>(sc: &mut Scanner<R>, wr: &mut W) {
	let s: Vec<char> = sc.next::<String>().chars().collect();
	let mut ans: i32 = 1;
	let mut cnt: i32 = 1;
	for (idx, &c) in s.iter().enumerate().skip(1) {
		if s[idx-1] == c {
			cnt = cnt + 1;
			ans = max(ans, cnt);
		} else {
			cnt = 1;
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
