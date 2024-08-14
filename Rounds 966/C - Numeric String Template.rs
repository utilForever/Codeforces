use io::Write;
use std::{collections::HashMap, io, str};

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
        let n = scan.token::<usize>();
        let mut idxes = vec![0; n];

        for i in 0..n {
            idxes[i] = scan.token::<i64>();
        }

        let m = scan.token::<i64>();

        for _ in 0..m {
            let mut alphabet = vec![i64::MAX; 26];
            let s = scan.token::<String>();
            let mut map = HashMap::new();
            let mut check = true;

            if s.len() != n {
                writeln!(out, "NO").unwrap();
                continue;
            }

            for (idx, c) in s.chars().enumerate() {
                let idx_c = c as usize - 'a' as usize;

                if map.contains_key(&idxes[idx]) && *map.get(&idxes[idx]).unwrap() != c {
                    check = false;
                    break;
                }

                if alphabet[idx_c] != i64::MAX && alphabet[idx_c] != idxes[idx] {
                    check = false;
                    break;
                }

                alphabet[idx_c] = idxes[idx];
                map.insert(idxes[idx], c);
            }

            writeln!(out, "{}", if check { "YES" } else { "NO" }).unwrap();
        }
    }
}
