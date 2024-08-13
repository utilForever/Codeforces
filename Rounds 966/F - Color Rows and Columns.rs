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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (n, k) = (scan.token::<usize>(), scan.token::<usize>());
        let mut rectangles = vec![(0, 0); n + 1];
        let mut operations = vec![vec![i64::MAX / 4; k + 1]; n + 1];

        for i in 1..=n {
            rectangles[i] = (scan.token::<i64>(), scan.token::<i64>());
        }

        for i in 1..=n {
            operations[i][0] = 0;

            for j in 1..=k {
                for r in 0..=j {
                    let c = j - r;

                    if r as i64 > rectangles[i].0 || c as i64 > rectangles[i].1 {
                        continue;
                    }

                    operations[i][j] = operations[i][j].min(
                        r as i64 * rectangles[i].1 + c as i64 * rectangles[i].0
                            - r as i64 * c as i64,
                    );
                }
            }
        }

        let mut dp = vec![vec![i64::MAX / 4; k + 1]; n + 1];
        dp[0][0] = 0;

        for i in 0..n {
            for j in 0..=k {
                for l in j..=k {
                    dp[i + 1][l] = dp[i + 1][l].min(dp[i][j] + operations[i + 1][l - j]);
                }
            }
        }

        writeln!(
            out,
            "{}",
            if dp[n][k] == i64::MAX / 4 {
                -1
            } else {
                dp[n][k]
            }
        )
        .unwrap();
    }
}
