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
        let _ = scan.token::<i64>();
        let s = scan.token::<String>();
        let s = s.chars().collect::<Vec<_>>();
        let mut pos = (0, 0);
        let mut ret = false;

        for c in s {
            match c {
                'L' => pos.0 -= 1,
                'R' => pos.0 += 1,
                'U' => pos.1 += 1,
                'D' => pos.1 -= 1,
                _ => unreachable!(),
            }

            if pos.0 == 1 && pos.1 == 1 {
                ret = true;
                break;
            }
        }

        writeln!(out, "{}", if ret { "YES" } else { "NO" }).unwrap();
    }
}
