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
        let n = scan.token::<usize>();
        let mut cards = vec![String::new(); 2 * n];
        let trump = scan.token::<char>();

        for i in 0..2 * n {
            cards[i] = scan.token::<String>();
        }

        let cards = cards
            .iter()
            .map(|card| (card.chars().nth(0).unwrap(), card.chars().nth(1).unwrap()))
            .collect::<Vec<(char, char)>>();

        let mut cards_club = cards
            .iter()
            .filter(|(_, suit)| *suit == 'C')
            .map(|(value, _)| value)
            .collect::<Vec<&char>>();
        let mut cards_diamond = cards
            .iter()
            .filter(|(_, suit)| *suit == 'D')
            .map(|(value, _)| value)
            .collect::<Vec<&char>>();
        let mut cards_heart = cards
            .iter()
            .filter(|(_, suit)| *suit == 'H')
            .map(|(value, _)| value)
            .collect::<Vec<&char>>();
        let mut cards_spade = cards
            .iter()
            .filter(|(_, suit)| *suit == 'S')
            .map(|(value, _)| value)
            .collect::<Vec<&char>>();

        cards_club.sort();
        cards_diamond.sort();
        cards_heart.sort();
        cards_spade.sort();

        let mut cnt_club = cards_club.len();
        let mut cnt_diamond = cards_diamond.len();
        let mut cnt_heart = cards_heart.len();
        let mut cnt_spade = cards_spade.len();

        let mut check_club = false;
        let mut check_diamond = false;
        let mut check_heart = false;
        let mut check_spade = false;

        match trump {
            'C' => {
                if cnt_club > 0 && cnt_diamond % 2 == 1 {
                    cnt_diamond -= 1;
                    cnt_club -= 1;
                    check_diamond = true;
                }

                if cnt_club > 0 && cnt_heart % 2 == 1 {
                    cnt_heart -= 1;
                    cnt_club -= 1;
                    check_heart = true;
                }

                if cnt_club > 0 && cnt_spade % 2 == 1 {
                    cnt_spade -= 1;
                    cnt_club -= 1;
                    check_spade = true;
                }

                if cnt_club % 2 == 1 || cnt_diamond % 2 == 1 || cnt_heart % 2 == 1 || cnt_spade % 2 == 1 {
                    writeln!(out, "IMPOSSIBLE").unwrap();
                    continue;
                } else {
                    let mut idx_other = 0;
                    let mut idx_trump = 0;

                    if check_diamond {
                        writeln!(out, "{}D {}C", cards_diamond[0], cards_club[0]).unwrap();
                        idx_other += 1;
                        idx_trump += 1;
                    }

                    for i in (idx_other..cards_diamond.len()).step_by(2) {
                        writeln!(out, "{}D {}D", cards_diamond[i], cards_diamond[i + 1]).unwrap();
                    }

                    idx_other = 0;

                    if check_heart {
                        writeln!(out, "{}H {}C", cards_heart[0], cards_club[idx_trump]).unwrap();
                        idx_other += 1;
                        idx_trump += 1;
                    }

                    for i in (idx_other..cards_heart.len()).step_by(2) {
                        writeln!(out, "{}H {}H", cards_heart[i], cards_heart[i + 1]).unwrap();
                    }

                    idx_other = 0;

                    if check_spade {
                        writeln!(out, "{}S {}C", cards_spade[0], cards_club[idx_trump]).unwrap();
                        idx_other += 1;
                        idx_trump += 1;
                    }

                    for i in (idx_other..cards_spade.len()).step_by(2) {
                        writeln!(out, "{}S {}S", cards_spade[i], cards_spade[i + 1]).unwrap();
                    }

                    for i in (idx_trump..cards_club.len()).step_by(2) {
                        writeln!(out, "{}C {}C", cards_club[i], cards_club[i + 1]).unwrap();
                    }
                }
            }
            'D' => {
                if cnt_diamond > 0 && cnt_club % 2 == 1 {
                    cnt_club -= 1;
                    cnt_diamond -= 1;
                    check_club = true;
                }

                if cnt_diamond > 0 && cnt_heart % 2 == 1 {
                    cnt_heart -= 1;
                    cnt_diamond -= 1;
                    check_heart = true;
                }

                if cnt_diamond > 0 && cnt_spade % 2 == 1 {
                    cnt_spade -= 1;
                    cnt_diamond -= 1;
                    check_spade = true;
                }

                if cnt_club % 2 == 1 || cnt_diamond % 2 == 1 || cnt_heart % 2 == 1 || cnt_spade % 2 == 1 {
                    writeln!(out, "IMPOSSIBLE").unwrap();
                    continue;
                } else {
                    let mut idx_other = 0;
                    let mut idx_trump = 0;

                    if check_club {
                        writeln!(out, "{}C {}D", cards_club[0], cards_diamond[0]).unwrap();
                        idx_other += 1;
                        idx_trump += 1;
                    }

                    for i in (idx_other..cards_club.len()).step_by(2) {
                        writeln!(out, "{}C {}C", cards_club[i], cards_club[i + 1]).unwrap();
                    }

                    idx_other = 0;

                    if check_heart {
                        writeln!(out, "{}H {}D", cards_heart[0], cards_diamond[idx_trump]).unwrap();
                        idx_other += 1;
                        idx_trump += 1;
                    }

                    for i in (idx_other..cards_heart.len()).step_by(2) {
                        writeln!(out, "{}H {}H", cards_heart[i], cards_heart[i + 1]).unwrap();
                    }

                    idx_other = 0;

                    if check_spade {
                        writeln!(out, "{}S {}D", cards_spade[0], cards_diamond[idx_trump]).unwrap();
                        idx_other += 1;
                        idx_trump += 1;
                    }

                    for i in (idx_other..cards_spade.len()).step_by(2) {
                        writeln!(out, "{}S {}S", cards_spade[i], cards_spade[i + 1]).unwrap();
                    }

                    for i in (idx_trump..cards_diamond.len()).step_by(2) {
                        writeln!(out, "{}D {}D", cards_diamond[i], cards_diamond[i + 1]).unwrap();
                    }
                }
            }
            'H' => {
                if cnt_heart > 0 && cnt_club % 2 == 1 {
                    cnt_club -= 1;
                    cnt_heart -= 1;
                    check_club = true;
                }

                if cnt_heart > 0 && cnt_diamond % 2 == 1 {
                    cnt_diamond -= 1;
                    cnt_heart -= 1;
                    check_diamond = true;
                }

                if cnt_heart > 0 && cnt_spade % 2 == 1 {
                    cnt_spade -= 1;
                    cnt_heart -= 1;
                    check_spade = true;
                }

                if cnt_club % 2 == 1 || cnt_diamond % 2 == 1 || cnt_heart % 2 == 1 || cnt_spade % 2 == 1 {
                    writeln!(out, "IMPOSSIBLE").unwrap();
                    continue;
                } else {
                    let mut idx_other = 0;
                    let mut idx_trump = 0;

                    if check_club {
                        writeln!(out, "{}C {}H", cards_club[0], cards_heart[0]).unwrap();
                        idx_other += 1;
                        idx_trump += 1;
                    }

                    for i in (idx_other..cards_club.len()).step_by(2) {
                        writeln!(out, "{}C {}C", cards_club[i], cards_club[i + 1]).unwrap();
                    }

                    idx_other = 0;

                    if check_diamond {
                        writeln!(out, "{}D {}H", cards_diamond[0], cards_heart[idx_trump]).unwrap();
                        idx_other += 1;
                        idx_trump += 1;
                    }

                    for i in (idx_other..cards_diamond.len()).step_by(2) {
                        writeln!(out, "{}D {}D", cards_diamond[i], cards_diamond[i + 1]).unwrap();
                    }

                    idx_other = 0;

                    if check_spade {
                        writeln!(out, "{}S {}H", cards_spade[0], cards_heart[idx_trump]).unwrap();
                        idx_other += 1;
                        idx_trump += 1;
                    }

                    for i in (idx_other..cards_spade.len()).step_by(2) {
                        writeln!(out, "{}S {}S", cards_spade[i], cards_spade[i + 1]).unwrap();
                    }

                    for i in (idx_trump..cards_heart.len()).step_by(2) {
                        writeln!(out, "{}H {}H", cards_heart[i], cards_heart[i + 1]).unwrap();
                    }
                }
            }
            'S' => {
                if cnt_spade > 0 && cnt_club % 2 == 1 {
                    cnt_club -= 1;
                    cnt_spade -= 1;
                    check_club = true;
                }

                if cnt_spade > 0 && cnt_diamond % 2 == 1 {
                    cnt_diamond -= 1;
                    cnt_spade -= 1;
                    check_diamond = true;
                }

                if cnt_spade > 0 && cnt_heart % 2 == 1 {
                    cnt_heart -= 1;
                    cnt_spade -= 1;
                    check_heart = true;
                }

                if cnt_club % 2 == 1 || cnt_diamond % 2 == 1 || cnt_heart % 2 == 1 || cnt_spade % 2 == 1 {
                    writeln!(out, "IMPOSSIBLE").unwrap();
                    continue;
                } else {
                    let mut idx_other = 0;
                    let mut idx_trump = 0;

                    if check_club {
                        writeln!(out, "{}C {}S", cards_club[0], cards_spade[0]).unwrap();
                        idx_other += 1;
                        idx_trump += 1;
                    }

                    for i in (idx_other..cards_club.len()).step_by(2) {
                        writeln!(out, "{}C {}C", cards_club[i], cards_club[i + 1]).unwrap();
                    }

                    idx_other = 0;

                    if check_diamond {
                        writeln!(out, "{}D {}S", cards_diamond[0], cards_spade[idx_trump]).unwrap();
                        idx_other += 1;
                        idx_trump += 1;
                    }

                    for i in (idx_other..cards_diamond.len()).step_by(2) {
                        writeln!(out, "{}D {}D", cards_diamond[i], cards_diamond[i + 1]).unwrap();
                    }

                    idx_other = 0;

                    if check_heart {
                        writeln!(out, "{}H {}S", cards_heart[0], cards_spade[idx_trump]).unwrap();
                        idx_other += 1;
                        idx_trump += 1;
                    }

                    for i in (idx_other..cards_heart.len()).step_by(2) {
                        writeln!(out, "{}H {}H", cards_heart[i], cards_heart[i + 1]).unwrap();
                    }

                    for i in (idx_trump..cards_spade.len()).step_by(2) {
                        writeln!(out, "{}S {}S", cards_spade[i], cards_spade[i + 1]).unwrap();
                    }
                }
            }
            _ => unreachable!(),
        }
    }
}
