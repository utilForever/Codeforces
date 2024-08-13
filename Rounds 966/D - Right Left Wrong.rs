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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let n = scan.token::<usize>();
        let mut scores = vec![0; n];
        let mut prefix_sum = vec![0; n + 1];

        for i in 0..n {
            scores[i] = scan.token::<i64>();
        }

        for i in 1..=n {
            prefix_sum[i] = prefix_sum[i - 1] + scores[i - 1];
        }

        let directions = scan.token::<String>();
        let mut stack_left = Vec::new();
        let mut stack_right = Vec::new();

        for (idx, c) in directions.chars().enumerate() {
            if c == 'L' {
                stack_left.push(idx);
            } else {
                stack_right.push(idx);
            }
        }

        stack_left.reverse();

        let mut ret = 0;

        while !stack_left.is_empty()
            && !stack_right.is_empty()
            && stack_left.last().unwrap() <= stack_right.last().unwrap()
        {
            let val_left = stack_left.pop().unwrap();
            let val_right = stack_right.pop().unwrap();

            ret += prefix_sum[val_right + 1] - prefix_sum[val_left];
        }

        writeln!(out, "{ret}").unwrap();
    }
}
