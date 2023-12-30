use std::io::{prelude::BufRead, prelude::Write, stdin, stdout, BufWriter};
use std::str;

/*
    Author: Koushik Sahu
    Created: 23:32:43 IST 30-12-2023 Saturday
*/

fn solve<R: BufRead, W: Write>(sc: &mut Scanner<R>, wr: &mut W) {
    let n: usize = sc.next();
    let mut a: Vec<(i32, i32)> = (0..n)
        .map(|_| {
            let x: i32 = sc.next();
            let y: i32 = sc.next();
            (x, y)
        })
        .collect();
    a.sort_by(|a, b| {
        if a.1 == b.1 {
            return a.0.partial_cmp(&b.0).unwrap();
        } else {
            return a.1.partial_cmp(&b.1).unwrap();
        }
    });
    let mut ans: i32 = 0;
    let mut curr: i32 = -1;
    for i in 0..a.len() {
        let l = a[i].0;
        if curr == -1 {
            ans += 1;
            curr += 1;
        } else {
            if l >= a[curr as usize].1 {
                ans += 1;
                curr = i as i32;
            }
        }
    }
    writeln!(wr, "{}", ans).unwrap();
}

#[macro_export]
macro_rules! dbg{
    ($(a:expr),*) => {
        #[cfg(debug_assertions)]
        eprintln!(
            concat!("{}:{}:{}: ",$(stringify!(a), " = {:?}, "),*),
            file!(), line!(), column!(), $(a),*
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
