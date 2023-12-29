use io::Write;
use std::{io, str, collections::HashMap};

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
        let mut nums = vec![0; n + 1];
        let mut list_diff = HashMap::new();

        for i in 1..=n {
            nums[i] = scan.token::<i64>();
        }

        let mut sum_even = 0;
        let mut sum_odd = 0;
        let mut ret = false;

        for i in 1..=n {
            if i % 2 == 0 {
                sum_even += nums[i];
            } else {
                sum_odd += nums[i];
            }

            let diff = sum_odd - sum_even;

            if diff == 0 || list_diff.contains_key(&diff) {
                ret = true;
                break;
            } else {
                list_diff.insert(diff, i);
            }
        }

        writeln!(out, "{}", if ret { "YES" } else { "NO" }).unwrap();
    }
}
