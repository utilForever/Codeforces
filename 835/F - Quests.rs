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
        let (n, c, d) = (
            scan.token::<usize>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );
        let mut rewards = vec![0; n];

        for i in 0..n {
            rewards[i] = scan.token::<i64>();
        }

        rewards.sort_by(|a, b| b.cmp(a));

        if rewards[0..n.min(d as usize)].iter().sum::<i64>() >= c {
            writeln!(out, "Infinity").unwrap();
        } else if rewards[0] * d < c {
            writeln!(out, "Impossible").unwrap();
        } else {
            let mut left = 0;
            let mut right = d;
            let mut ret = 0;

            while left <= right {
                let mid = (left + right) / 2;
                let (q, r) = (d / (mid + 1), d % (mid + 1));

                let sum = if mid < n as i64 {
                    rewards[0..(mid + 1) as usize].iter().sum::<i64>() * q
                        + rewards[0..r as usize].iter().sum::<i64>()
                } else {
                    rewards.iter().sum::<i64>() * q
                        + rewards[0..n.min(r as usize) as usize].iter().sum::<i64>()
                };

                if sum >= c {
                    ret = mid;
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }

            writeln!(out, "{ret}").unwrap();
        }
    }
}
