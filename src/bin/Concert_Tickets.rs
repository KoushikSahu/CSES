use std::collections::BTreeMap;
use std::io::{prelude::BufRead, prelude::Write, stdin, stdout, BufWriter};
use std::str;

/*
    Author: Koushik Sahu
    Created: 19:34:46 IST 29-12-2023 Friday
*/

fn solve<R: BufRead, W: Write>(sc: &mut Scanner<R>, wr: &mut W) {
    let n: i32 = sc.next();
    let m: i32 = sc.next();
    let h: Vec<i32> = (0..n).map(|_| sc.next()).collect::<Vec<i32>>();
    let t: Vec<i32> = (0..m).map(|_| sc.next()).collect::<Vec<i32>>();
    let mut cntr: BTreeMap<i32, i32> = BTreeMap::new();
    for i in h.iter() {
        cntr.entry(i.to_owned())
            .and_modify(|x| *x += 1)
            .or_insert(1);
    }
    for i in t.iter() {
        let p = cntr.range_mut(..=i).rev().next();
        let mut r: Option<i32> = None;
        match p {
            Some((price, cnt)) => {
                if *cnt > 0 {
                    writeln!(wr, "{} ", price).unwrap();
                    *cnt -= 1;
                }
                if *cnt == 0 {
                    r = Some(price.clone());
                }
            }
            None => {
                writeln!(wr, "-1 ").unwrap();
            }
        }
        match r {
            Some(key) => {
                cntr.remove(&key);
            }
            None => (),
        }
    }
    writeln!(wr, "").unwrap();
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
