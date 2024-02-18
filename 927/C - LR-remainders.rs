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
        let (n, m) = (scan.token::<usize>(), scan.token::<i64>());
        let mut nums = vec![0; n];

        for i in 0..n {
            nums[i] = scan.token::<i64>();
        }

        let s = scan.token::<String>();
        let mut commands = Vec::new();
        let mut left = 0;
        let mut right = n - 1;

        for c in s.chars() {
            if c == 'L' {
                commands.push(left);
                left += 1;
            } else {
                commands.push(right);
                right -= 1;
            }
        }

        let mut ret = Vec::new();
        let mut val = 1;

        for command in commands.iter().rev() {
            val = (val * nums[*command]) % m;
            ret.push(val);
        }

        ret.reverse();

        for val in ret {
            write!(out, "{val} ").unwrap();
        }

        writeln!(out).unwrap();
    }
}
