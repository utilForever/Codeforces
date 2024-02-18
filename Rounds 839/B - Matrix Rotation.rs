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
        let mut matrix = [0; 4];

        for i in 0..4 {
            matrix[i] = scan.token::<i64>();
        }

        let cond1 = matrix[0] < matrix[1]
            && matrix[2] < matrix[3]
            && matrix[0] < matrix[2]
            && matrix[1] < matrix[3];
        let cond2 = matrix[2] < matrix[0]
            && matrix[3] < matrix[1]
            && matrix[2] < matrix[3]
            && matrix[0] < matrix[1];
        let cond3 = matrix[3] < matrix[2]
            && matrix[1] < matrix[0]
            && matrix[3] < matrix[1]
            && matrix[2] < matrix[0];
        let cond4 = matrix[1] < matrix[3]
            && matrix[0] < matrix[2]
            && matrix[1] < matrix[0]
            && matrix[3] < matrix[2];

        writeln!(
            out,
            "{}",
            if cond1 || cond2 || cond3 || cond4 {
                "YES"
            } else {
                "NO"
            }
        )
        .unwrap();
    }
}
