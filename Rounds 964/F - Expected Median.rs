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

pub struct Combination {
    fact: Vec<usize>,
    inv_fact: Vec<usize>,
    modulo: usize,
}

impl Combination {
    pub fn new(max: usize, modulo: usize) -> Self {
        let mut inv = vec![0; max + 1];
        let mut fact = vec![0; max + 1];
        let mut inv_fact = vec![0; max + 1];

        inv[1] = 1;

        for i in 2..(max + 1) {
            inv[i] = inv[modulo % i] * (modulo - modulo / i) % modulo;
        }

        fact[0] = 1;
        inv_fact[0] = 1;

        for i in 0..max {
            fact[i + 1] = fact[i] * (i + 1) % modulo;
        }

        for i in 0..max {
            inv_fact[i + 1] = inv_fact[i] * inv[i + 1] % modulo;
        }

        Self {
            fact,
            inv_fact,
            modulo,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> usize {
        if x < y {
            return 0;
        }

        self.fact[x] * self.inv_fact[y] % self.modulo * self.inv_fact[x - y] % self.modulo
    }

    pub fn h(&self, n: usize, r: usize) -> usize {
        self.get(n + r - 1, r)
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i32>();

    for _ in 0..t {
        let (n, k) = (scan.token::<usize>(), scan.token::<usize>());
        let comb = Combination::new(n, 1_000_000_007);
        let mut cnt_one = 0;
        let mut cnt_zero = 0;

        for _ in 0..n {
            let val = scan.token::<i32>();

            if val == 1 {
                cnt_one += 1;
            } else {
                cnt_zero += 1;
            }
        }

        let mut ret = 0;

        for i in (k + 1) / 2..=k {
            ret += comb.get(cnt_one, i) * comb.get(cnt_zero, k - i) % 1_000_000_007;
            ret %= 1_000_000_007;
        }

        writeln!(out, "{ret}").unwrap();
    }
}
