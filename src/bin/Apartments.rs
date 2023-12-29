use std::io::{prelude::BufRead, prelude::Write, stdin, stdout, BufWriter};
use std::str;

/*
    Author: Koushik Sahu
    Created: 11:16:32 IST 27-12-2023 Wednesday
*/

fn solve<R: BufRead, W: Write>(sc: &mut Scanner<R>, wr: &mut W) {
    /*
    45 60 60 80
    30 60 75
     */
    let n = sc.next::<i32>();
    let m = sc.next::<i32>();
    let k = sc.next::<i32>();
    let mut a: Vec<i32> = (0..n).map(|_| sc.next::<i32>()).collect();
    let mut b: Vec<i32> = (0..m).map(|_| sc.next::<i32>()).collect();
    a.sort();
    b.sort();
    let mut ptr1 = 0usize;
    let mut ptr2 = 0usize;
    let mut ans = 0;
    while ptr1 < n as usize && ptr2 < m as usize {
        if b[ptr2] >= a[ptr1] - k && b[ptr2] <= a[ptr1] + k {
            ans += 1;
            ptr1 += 1;
            ptr2 += 1;
        } else if b[ptr2] < a[ptr1] - k {
            ptr2 += 1;
        } else if b[ptr2] > a[ptr1] + k {
            ptr1 += 1;
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
