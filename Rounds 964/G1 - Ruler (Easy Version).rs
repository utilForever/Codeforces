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
    let stdin = io::stdin();
    let mut scan = UnsafeScanner::new(stdin.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let mut left = 2;
        let mut right = 999;

        loop {
            if left == right {
                println!("! {left}");
                break;
            }

            let mid1 = left + (right - left) / 3;
            let mid2 = left + 2 * (right - left) / 3;

            println!("? {mid1} {mid2}");

            let ret = scan.token::<i64>();

            if ret == mid1 * mid2 {
                left = mid2 + 1;
            } else if ret == mid1 * (mid2 + 1) {
                left = mid1 + 1;
                right = mid2;
            } else {
                right = mid1;
            }
        }
    }
}
