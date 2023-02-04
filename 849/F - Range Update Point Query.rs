use io::Write;
use std::{collections::BTreeSet, io, str};

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

fn find_neighbors(tree: &BTreeSet<usize>, val: usize) -> (Option<&usize>, Option<&usize>) {
    use std::ops::Bound::*;

    let mut before = tree.range((Unbounded, Included(val)));
    let mut after = tree.range((Included(val), Unbounded));

    (before.next_back(), after.next())
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (n, q) = (scan.token::<usize>(), scan.token::<i64>());
        let mut nums = vec![0; n + 1];
        let mut lazy = BTreeSet::new();

        for i in 1..=n {
            nums[i] = scan.token::<i64>();

            if nums[i] >= 10 {
                lazy.insert(i);
            }
        }

        for _ in 0..q {
            let num = scan.token::<i64>();

            if num == 1 {
                let (l, r) = (scan.token::<usize>(), scan.token::<usize>());
                let mut left = l;

                while !lazy.is_empty() {
                    let next = find_neighbors(&lazy, left).1;

                    if next == None || *next.unwrap() > r {
                        break;
                    }

                    let idx = *next.unwrap();
                    let mut num = nums[*next.unwrap()];
                    let mut sum = 0;

                    while num > 0 {
                        sum += num % 10;
                        num /= 10;
                    }

                    nums[idx] = sum;

                    if sum < 10 {
                        lazy.remove(&idx);
                    }

                    left = idx + 1;
                }
            } else {
                let x = scan.token::<usize>();
                writeln!(out, "{}", nums[x]).unwrap();
            }
        }
    }
}
