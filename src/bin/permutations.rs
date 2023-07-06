use std::io::{stdin, stdout, BufWriter, prelude::BufRead, prelude::Write};
use std::str;

/*
	Author: Koushik Sahu
	Created: 08:21:10 PM(20:21:10) IST(+05:30) 06-07-2023 Thu
*/

/*
 * 2 1 4 3
 * n 1 n-1 2 n-2 3... n-k k+1
 * n-2*k-1 = 1
 * k = (n-2) / 2
 * no problem for odd numbers
 * 4 1 3 2
 * odd in asc order even in reverse order
 * 1 3 5 2 4 6
 */

fn solve<R: BufRead, W: Write>(sc: &mut Scanner<R>, wr: &mut W) {
	let n: u32 = sc.next();
	if n != 1 && n < 4 {
		writeln!(wr, "{}", "NO SOLUTION").unwrap();
	} else {
		for i in 1..=n {
			if i % 2 == 0 {
				write!(wr, "{} ", i).unwrap();
			}
		}
		for i in 1..=n {
			if i % 2 == 1 {
				write!(wr, "{} ", i).unwrap();
			}
		}
	}
	writeln!(wr, "").unwrap();
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
