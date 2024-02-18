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
        let (n, c) = (scan.token::<usize>(), scan.token::<char>());
        let s = scan.token::<String>();

        if c == 'g' {
            writeln!(out, "0").unwrap();
            continue;
        }

        let mut s = s.chars().collect::<Vec<_>>();
        let mut ret = 0;

        if *s.last().unwrap() != 'g' {
            for i in 0..n {
                if s[i] == 'g' {
                    s.push(s[i]);
                    break;
                }

                s.push(s[i]);
            }
        }

        let mut idx_c = -1;

        for i in 0..s.len() {
            if s[i] == c && idx_c == -1 {
                idx_c = i as i64;
            } else if s[i] == 'g' {
                if idx_c >= 0 {
                    ret = ret.max(i as i64 - idx_c);
                    idx_c = -1;
                }
            }
        }

        writeln!(out, "{ret}").unwrap();
    }
}
