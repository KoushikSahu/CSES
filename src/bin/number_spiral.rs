use std::io::{stdin, stdout, BufWriter, prelude::BufRead, prelude::Write};
use std::str;
use std::cmp::{max};

/*
	Author: Koushik Sahu
	Created: 09:38:30 PM(21:38:30) IST(+05:30) 06-07-2023 Thu
*/

/*
 * 1 3 5 .. 2i + 1 = i**2
 *  1 + (2*i*(i+1))/2 = i**2 + i + 1
*/

fn solve<R: BufRead, W: Write>(sc: &mut Scanner<R>, wr: &mut W) {
	let mut y: i32 = sc.next();
	let mut x: i32 = sc.next();
	y = y - 1;
	x = x - 1;
	let i: i128 = max(x as i128, y as i128);
	let mut ans: i128 = i*i + i + 1;
	y = y - (i as i32);
	x = x - (i as i32);
	if i%2 == 0 {
		if x != 0 {
			ans = ans + (x as i128);
		} else {
			ans = ans - (y as i128);
		}
	} else {
		if x != 0 {
			ans = ans - (x as i128);
		} else {
			ans = ans + (y as i128);
		}
	}
	writeln!(wr, "{}", ans).unwrap();
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
	let t: i32 = scan.next(); 
	for _ in 0..t {
		solve(&mut scan, &mut out);
	}
}
