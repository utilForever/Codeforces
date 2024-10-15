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

struct PermutationTree {
    size: usize,
    data: Vec<(i64, i64)>,
    lazy: Vec<i64>,
}

impl PermutationTree {
    pub fn new(n: usize) -> Self {
        let mut real_n = 1;
        while real_n < n {
            real_n *= 2;
        }

        Self {
            size: n,
            data: vec![(0, 0); real_n * 4],
            lazy: vec![0; real_n * 4],
        }
    }

    fn merge(left: &(i64, i64), right: &(i64, i64)) -> (i64, i64) {
        if left.0 < right.0 {
            (left.0, left.1)
        } else if left.0 > right.0 {
            (right.0, right.1)
        } else {
            (left.0, left.1 + right.1)
        }
    }

    pub fn construct(&mut self, start: usize, end: usize) {
        self.construct_internal(1, start, end);
    }

    fn construct_internal(&mut self, node: usize, start: usize, end: usize) {
        if start == end {
            self.data[node] = (start as i64, 1);
            return;
        } else {
            let mid = (start + end) / 2;

            self.construct_internal(node * 2, start, mid);
            self.construct_internal(node * 2 + 1, mid + 1, end);

            self.data[node] =
                PermutationTree::merge(&self.data[node * 2], &self.data[node * 2 + 1]);
        }
    }

    fn propagate(&mut self, node: usize, start: usize, end: usize) {
        if self.lazy[node] == 0 {
            return;
        }

        self.data[node * 2].0 += self.lazy[node];
        self.data[node * 2 + 1].0 += self.lazy[node];

        if start != end {
            self.lazy[node * 2] += self.lazy[node];
            self.lazy[node * 2 + 1] += self.lazy[node];
        }

        self.lazy[node] = 0;
    }

    pub fn update(&mut self, start: usize, end: usize, val: i64) {
        self.update_internal(start, end, val, 1, 1, self.size);
    }

    fn update_internal(
        &mut self,
        start: usize,
        end: usize,
        val: i64,
        node: usize,
        node_start: usize,
        node_end: usize,
    ) {
        if start == node_start && node_end == end {
            self.data[node].0 += val;
            self.lazy[node] += val;
            return;
        }

        self.propagate(node, node_start, node_end);

        let mid = (node_start + node_end) / 2;

        if end <= mid {
            self.update_internal(start, end, val, node * 2, node_start, mid);
        } else if start > mid {
            self.update_internal(start, end, val, node * 2 + 1, mid + 1, node_end);
        } else {
            self.update_internal(start, mid, val, node * 2, node_start, mid);
            self.update_internal(mid + 1, end, val, node * 2 + 1, mid + 1, node_end);
        }

        self.data[node] = PermutationTree::merge(&self.data[node * 2], &self.data[node * 2 + 1]);
    }

    pub fn query(&mut self, start: usize, end: usize) -> (i64, i64) {
        self.query_internal(start, end, 1, 1, self.size)
    }

    fn query_internal(
        &mut self,
        start: usize,
        end: usize,
        node: usize,
        node_start: usize,
        node_end: usize,
    ) -> (i64, i64) {
        if start == node_start && node_end == end {
            return self.data[node];
        }

        self.propagate(node, node_start, node_end);

        let mid = (node_start + node_end) / 2;

        if end <= mid {
            self.query_internal(start, end, node * 2, node_start, mid)
        } else if start > mid {
            self.query_internal(start, end, node * 2 + 1, mid + 1, node_end)
        } else {
            let left = self.query_internal(start, mid, node * 2, node_start, mid);
            let right = self.query_internal(mid + 1, end, node * 2 + 1, mid + 1, node_end);

            PermutationTree::merge(&left, &right)
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut queries = vec![0; n + 1];

    for _ in 0..n {
        let (a, b) = (scan.token::<usize>(), scan.token::<usize>());
        queries[a] = b;
    }

    let mut tree = PermutationTree::new(n);
    tree.construct(1, n);

    let mut max = Vec::new();
    let mut min = Vec::new();
    let mut ret = 0;

    for i in 1..=n {
        while !max.is_empty() && queries[i] > queries[*max.last().unwrap()] {
            let val = max.pop().unwrap();

            tree.update(
                if max.is_empty() {
                    1
                } else {
                    *max.last().unwrap() + 1
                },
                val,
                (queries[i] - queries[val]) as i64,
            );
        }

        max.push(i);

        while !min.is_empty() && queries[i] < queries[*min.last().unwrap()] {
            let val = min.pop().unwrap();

            tree.update(
                if min.is_empty() {
                    1
                } else {
                    *min.last().unwrap() + 1
                },
                val,
                (queries[val] - queries[i]) as i64,
            );
        }

        min.push(i);

        let val = tree.query(1, i);
        ret += if val.0 == i as i64 { val.1 } else { 0 };
    }

    writeln!(out, "{ret}").unwrap();
}
