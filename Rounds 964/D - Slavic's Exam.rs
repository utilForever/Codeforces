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

fn is_subsequence(s: &Vec<char>, t: &Vec<char>) -> bool {
    let mut idx_t = 0;

    for c in s {
        if idx_t == t.len() {
            break;
        }

        if *c == t[idx_t] {
            idx_t += 1;
        }
    }

    idx_t == t.len()
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let s = scan.token::<String>().chars().collect::<Vec<_>>();
        let t = scan.token::<String>().chars().collect::<Vec<_>>();

        let mut idx_t = 0;
        let mut ret = Vec::new();

        for c in s.iter() {
            if *c == '?' {
                if idx_t < t.len() {
                    ret.push(t[idx_t]);
                    idx_t += 1;
                } else {
                    ret.push('a');
                }
            } else {
                ret.push(*c);

                if idx_t < t.len() && *c == t[idx_t] {
                    idx_t += 1;
                }
            }
        }

        if idx_t == t.len() && is_subsequence(&ret, &t) {
            writeln!(out, "YES").unwrap();
            writeln!(out, "{}", ret.iter().collect::<String>()).unwrap();
        } else {
            writeln!(out, "NO").unwrap();
        }
    }
}
