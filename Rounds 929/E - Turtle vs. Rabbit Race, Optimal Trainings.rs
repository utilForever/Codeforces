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

fn calculate_performance(prefix_sum: &Vec<i64>, l: usize, r: usize, u: i64) -> i64 {
    let total_sections = prefix_sum[r] - prefix_sum[l - 1];
    let positive_increases = std::cmp::min(total_sections, u);
    let negative_increases = total_sections - positive_increases - 1;

    let positive_sum = if positive_increases > 0 {
        (2 * u - positive_increases + 1) * positive_increases / 2
    } else {
        0
    };

    let negative_sum = if negative_increases > 0 {
        negative_increases * (negative_increases + 1) / 2
    } else {
        0
    };

    positive_sum - negative_sum
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let n = scan.token::<usize>();
        let mut nums = vec![0; n + 1];
        let mut prefix_sum = vec![0; n + 1];

        for i in 1..=n {
            nums[i] = scan.token::<i64>();
            prefix_sum[i] = prefix_sum[i - 1] + nums[i];
        }

        let q = scan.token::<i64>();

        for _ in 0..q {
            let (l, u) = (scan.token::<usize>(), scan.token::<i64>());
            let mut left = l;
            let mut right = n;

            while right - left > 2 {
                let mid1 = left + (right - left) / 3;
                let mid2 = right - (right - left) / 3;

                if calculate_performance(&prefix_sum, l, mid1, u)
                    > calculate_performance(&prefix_sum, l, mid2, u)
                {
                    right = mid2;
                } else {
                    left = mid1;
                }
            }

            let mut ret_r = left;
            let mut ret_val = calculate_performance(&prefix_sum, l, left, u);

            for r in left..=right {
                let val = calculate_performance(&prefix_sum, l, r, u);

                if val > ret_val {
                    ret_val = val;
                    ret_r = r;
                }
            }

            write!(out, "{ret_r} ").unwrap();
        }

        writeln!(out).unwrap();
    }
}
