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
        let mut sequence = vec![vec![0; n - 1]; n];

        for i in 0..n {
            for j in 0..n - 1 {
                sequence[i][j] = scan.token::<i64>();
            }
        }

        let idx1 = 0;
        let mut idx2 = 0;

        for i in 1..n {
            if sequence[idx1][0] != sequence[i][0] {
                idx2 = i;
                break;
            }
        }

        let mut cnt_idx1 = 0;

        for i in 0..n {
            if sequence[i][0] == sequence[idx1][0] {
                cnt_idx1 += 1;
            }
        }

        if cnt_idx1 == 1 {
            write!(out, "{} ", sequence[idx2][0]).unwrap();

            for i in 0..n - 1 {
                write!(out, "{} ", sequence[idx1][i]).unwrap();
            }
        } else {
            write!(out, "{} ", sequence[idx1][0]).unwrap();

            for i in 0..n - 1 {
                write!(out, "{} ", sequence[idx2][i]).unwrap();
            }
        }

        writeln!(out).unwrap();
    }
}
