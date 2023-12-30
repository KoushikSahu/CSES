use std::cmp::max;
use std::io::{prelude::BufRead, prelude::Write, stdin, stdout, BufWriter};
use std::str;

/*
    Author: Koushik Sahu
    Created: 21:26:01 IST 29-12-2023 Friday
*/

struct Event {
    tme: i32,
    typ: i32,
}

fn solve<R: BufRead, W: Write>(sc: &mut Scanner<R>, wr: &mut W) {
    let n: i32 = sc.next();
    let mut v: Vec<Event> = Vec::new();
    for _ in 0..n as usize {
        let s: i32 = sc.next();
        let e: i32 = sc.next();
        v.push(Event { tme: s, typ: 1 });
        v.push(Event { tme: e, typ: -1 });
    }
    v.sort_by(|a, b| {
        if a.tme == b.tme {
            return a.typ.partial_cmp(&b.typ).unwrap();
        } else {
            return a.tme.partial_cmp(&b.tme).unwrap();
        }
    });
    let mut ans = i32::MIN;
    let mut curr = 0;
    for x in v.iter() {
        if x.typ == 1 {
            curr += 1;
        } else {
            curr -= 1;
        }
        ans = max(ans, curr);
    }
    writeln!(wr, "{:}", ans).unwrap();
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
