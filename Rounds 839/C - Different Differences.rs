use io::Write;
use std::{io, str};

pub struct UnsafeScanner<R> {
    reader: R,
    buf_str: Vec<u8>,
    buf_iter: str::SplitAsciiWhitespace<'static>,
}

impl<R: io::BufRead> UnsafeScanner<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buf_str: vec![],
            buf_iter: "".split_ascii_whitespace(),
        }
    }

    pub fn token<T: str::FromStr>(&mut self) -> T {
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
                std::mem::transmute(slice.split_ascii_whitespace())
            }
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (k, n) = (scan.token::<usize>(), scan.token::<usize>());
        let mut arr = vec![0; k];

        for i in 0..k {
            arr[i] = i + 1;
        }

        let mut acc = k;
        let mut add = 2;
        let mut idx = 0;

        while acc < n {
            idx += 1;
            acc += add;
            add += 1;
        }

        let mut add = 1;

        for i in ((k as i64 - idx as i64).max(0) as usize)..k {
            arr[i] += add;
            add += i - (k - idx - 1) + 1;
        }

        for i in 0..k {
            write!(out, "{} ", arr[i]).unwrap();
        }

        writeln!(out).unwrap();
    }
}
