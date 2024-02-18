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

fn process(left: &mut Vec<i64>, right: &mut Vec<i64>, len: usize, cnt: &mut i64) {
    if len < 1 || *cnt == -1 {
        return;
    }

    if left[0] > right[0] {
        *cnt += 1;
        std::mem::swap(left, right);
    }

    let mut left_clone = left.clone();
    let mut right_clone = right.clone();

    left_clone.sort();
    right_clone.sort();

    for i in 0..len {
        if left_clone[i] + len as i64 != right_clone[i] {
            *cnt = -1;
            return;
        }
    }

    process(&mut left[0..len / 2].to_vec(), &mut left[len / 2..len].to_vec(), len / 2, cnt);
    process(&mut right[0..len / 2].to_vec(), &mut right[len / 2..len].to_vec(), len / 2, cnt);
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<usize>();

    for _ in 0..t {
        let m = scan.token::<usize>();
        let mut arr = vec![0; m];

        for i in 0..m {
            arr[i] = scan.token::<i64>();
        }

        if m == 1 {
            writeln!(out, "0").unwrap();
        } else {
            let mut cnt = 0;
            process(&mut arr[0..m / 2].to_vec(), &mut arr[m / 2..m].to_vec(), m / 2, &mut cnt);
    
            writeln!(out, "{cnt}").unwrap();
        }
    }
}
