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
        let (n, q) = (scan.token::<usize>(), scan.token::<i64>());

        let mut num_odd = 0;
        let mut num_even = 0;
        let mut sum = 0;

        for _ in 0..n {
            let val = scan.token::<i64>();
            sum += val;

            if val % 2 == 0 {
                num_even += 1;
            } else {
                num_odd += 1;
            }
        }

        for _ in 0..q {
            let (t, x) = (scan.token::<i64>(), scan.token::<i64>());

            if t == 0 {
                sum += x * num_even;

                if x % 2 == 1 {
                    num_odd += num_even;
                    num_even = 0;
                }
            } else {
                sum += x * num_odd;

                if x % 2 == 1 {
                    num_even += num_odd;
                    num_odd = 0;
                }
            }

            writeln!(out, "{sum}").unwrap();
        }
    }
}
