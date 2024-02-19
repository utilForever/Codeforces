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
        let (n, mut k) = (scan.token::<i64>(), scan.token::<i64>());

        if (n % 2 == 0 && k <= n / 2) || (n % 2 != 0 && k <= n / 2 + 1) {
            writeln!(out, "{}", 2 * k - 1).unwrap();
            continue;
        }

        k -= if n % 2 == 0 { n / 2 } else { n / 2 + 1 };
        let mut a = 2;
        let mut d = 4;

        loop {
            let mut val = n / d * d + a;
            if val > n {
                val -= d;
            }

            let cnt = (val - a) / d + 1;

            if k <= cnt {
                writeln!(out, "{}", a + (k - 1) * d).unwrap();
                break;
            } else {
                k -= cnt;
                a *= 2;
                d *= 2;
            }
        }
    }
}
