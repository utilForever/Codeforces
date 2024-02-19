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
        let n = scan.token::<usize>();
        let mut grid = vec![vec![0; n]; n];

        for i in 0..n {
            let s = scan.token::<String>();

            for (j, c) in s.chars().enumerate() {
                grid[i][j] = c.to_digit(10).unwrap();
            }
        }

        let mut prev = 0;
        let mut is_triangle = false;

        for i in 0..n {
            let mut sum = 0;

            for j in 0..n {
                sum += grid[i][j];
            }

            if sum != 0 {
                if sum != prev && prev != 0 {
                    is_triangle = true;
                    break;
                } else {
                    prev = sum;
                }
            }
        }

        writeln!(out, "{}", if is_triangle { "TRIANGLE" } else { "SQUARE" }).unwrap();
    }
}
