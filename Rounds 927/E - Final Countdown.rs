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
        let _ = scan.token::<usize>();
        let time = scan.token::<String>();

        // Remove 0s from the beginning
        let pos = time.trim_start_matches('0');
        let time = pos.chars().collect::<Vec<_>>();
        let len = time.len();

        let mut nums = vec![0; len];

        for i in 0..len {
            nums[i] = time[i] as i64 - '0' as i64;
        }

        for i in 1..len {
            nums[i] += nums[i - 1];
        }

        for i in (1..len).rev() {
            if nums[i] >= 10 {
                nums[i - 1] += nums[i] / 10;
                nums[i] %= 10;
            }
        }

        for num in nums {
            write!(out, "{num}").unwrap();
        }

        writeln!(out).unwrap();
    }
}
