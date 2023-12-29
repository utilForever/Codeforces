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
        let mut idx = 0;
        let mut ret = String::new();

        while idx < s.len() {
            if s[idx] == 'a' || s[idx] == 'e' || s[idx] == 'i' || s[idx] == 'o' || s[idx] == 'u' {
                if idx + 1 == s.len() || idx + 2 == s.len() {
                    if idx + 1 == s.len() {
                        ret.push(s[idx]);
                        idx += 1;
                    } else {
                        ret.push(s[idx]);
                        ret.push(s[idx + 1]);
                        idx += 2;
                    }
                } else if idx + 2 < s.len() && s[idx + 2] == 'a'
                    || s[idx + 2] == 'e'
                    || s[idx + 2] == 'i'
                    || s[idx + 2] == 'o'
                    || s[idx + 2] == 'u'
                {
                    ret.push(s[idx]);
                    ret.push('.');
                    ret.push(s[idx + 1]);
                    idx += 2;
                } else {
                    ret.push(s[idx]);
                    ret.push(s[idx + 1]);
                    ret.push('.');
                    ret.push(s[idx + 2]);
                    idx += 3;
                }
            } else {
                ret.push(s[idx]);
                idx += 1;
            }
        }

        writeln!(out, "{ret}").unwrap();
    }
}
