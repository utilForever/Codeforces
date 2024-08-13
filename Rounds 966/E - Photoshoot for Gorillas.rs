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
        let (n, m, k) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
        );
        let w = scan.token::<usize>();
        let mut heights = vec![0; w];

        for i in 0..w {
            heights[i] = scan.token::<i64>();
        }

        let mut height = vec![0; n + 1];
        let mut width = vec![0; m + 1];

        for i in 0..n - k + 1 {
            height[i] += 1;
            height[i + k] -= 1;
        }

        for i in 0..m - k + 1 {
            width[i] += 1;
            width[i + k] -= 1;
        }

        for i in 1..n {
            height[i] += height[i - 1];
        }

        for i in 1..m {
            width[i] += width[i - 1];
        }

        let mut cnts = Vec::with_capacity(n * m);

        for i in 0..n {
            for j in 0..m {
                cnts.push(height[i] * width[j]);
            }
        }

        heights.sort_by(|a, b| b.cmp(a));
        cnts.sort_by(|a, b| b.cmp(a));

        let mut ret = 0;

        for i in 0..w {
            ret += heights[i] * cnts[i];
        }

        writeln!(out, "{ret}").unwrap();
    }
}
