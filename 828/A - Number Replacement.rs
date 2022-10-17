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
        let mut arr = vec![0; n];

        for i in 0..n {
            arr[i] = scan.token::<i64>();
        }

        let s = scan.token::<String>();
        let s = s.chars().collect::<Vec<char>>();

        let mut new_arr = vec![Vec::new(); 51];
        let mut ret = true;

        for i in 0..n {
            new_arr[arr[i] as usize].push(s[i]);
        }

        for i in 1..=50 {
            new_arr[i].sort();
            new_arr[i].dedup();
        }

        for elem in new_arr.iter() {
            if elem.len() > 1 {
                ret = false;
                break;
            }
        }

        writeln!(out, "{}", if ret { "YES" } else { "NO" }).unwrap();
    }
}
