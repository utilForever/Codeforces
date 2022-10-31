use io::Write;
use std::{
    cmp::{Ord, Ordering, PartialOrd},
    io, str,
};

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

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl std::fmt::Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Rank::Two => "2",
                Rank::Three => "3",
                Rank::Four => "4",
                Rank::Five => "5",
                Rank::Six => "6",
                Rank::Seven => "7",
                Rank::Eight => "8",
                Rank::Nine => "9",
                Rank::Ten => "T",
                Rank::Jack => "J",
                Rank::Queen => "Q",
                Rank::King => "K",
                Rank::Ace => "A",
            }
        )
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Suit {
    Diamond,
    Club,
    Heart,
    Spade,
}

impl std::fmt::Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Suit::Diamond => "D",
                Suit::Club => "C",
                Suit::Heart => "H",
                Suit::Spade => "S",
            }
        )
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Card {
    rank: Rank,
    suit: Suit,
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank.cmp(&other.rank)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let convert_card = |raw_card: &str| -> Card {
        let suit = match &raw_card[1..2] {
            "D" => Suit::Diamond,
            "C" => Suit::Club,
            "H" => Suit::Heart,
            "S" => Suit::Spade,
            _ => panic!("Invalid suit"),
        };
        let rank = match &raw_card[0..1] {
            "2" => Rank::Two,
            "3" => Rank::Three,
            "4" => Rank::Four,
            "5" => Rank::Five,
            "6" => Rank::Six,
            "7" => Rank::Seven,
            "8" => Rank::Eight,
            "9" => Rank::Nine,
            "T" => Rank::Ten,
            "J" => Rank::Jack,
            "Q" => Rank::Queen,
            "K" => Rank::King,
            "A" => Rank::Ace,
            _ => panic!("Invalid rank"),
        };

        Card { rank, suit }
    };

    let ranks = [
        Rank::Two,
        Rank::Three,
        Rank::Four,
        Rank::Five,
        Rank::Six,
        Rank::Seven,
        Rank::Eight,
        Rank::Nine,
        Rank::Ten,
        Rank::Jack,
        Rank::Queen,
        Rank::King,
        Rank::Ace,
    ];
    let suits = [Suit::Diamond, Suit::Club, Suit::Heart, Suit::Spade];

    let t = scan.token::<i64>();

    for _ in 0..t {
        let mut cards_alice = Vec::new();
        let mut cards_bob = Vec::new();

        for _ in 0..2 {
            cards_alice.push(convert_card(&scan.token::<String>()));
        }

        for _ in 0..2 {
            cards_bob.push(convert_card(&scan.token::<String>()));
        }

        cards_alice.sort_by(|a, b| b.rank.cmp(&a.rank));
        cards_bob.sort_by(|a, b| b.rank.cmp(&a.rank));

        // How to make Alice win
        write!(out, "YES ").unwrap();

        if cards_alice[0].rank == cards_bob[0].rank && cards_alice[1].rank == cards_bob[1].rank {
            let mut cnt = 0;

            // Random different rank cards
            for rank in ranks.iter() {
                if cnt == 6 {
                    break;
                }

                if cards_alice[0].rank == *rank {
                    continue;
                }

                cnt += 1;

                write!(out, "{}{} ", rank, suits[cnt % 4]).unwrap();
            }

            writeln!(out).unwrap();
        } else {
            let mut vec_rank = vec![Vec::new(); 13];

            for card in cards_alice.iter() {
                vec_rank[card.rank as usize].push(card.suit.clone());
            }

            for card in cards_bob.iter() {
                if vec_rank[card.rank as usize].len() == 2 {
                    vec_rank[card.rank as usize].push(card.suit.clone());
                } else if vec_rank[card.rank as usize].len() == 1 {
                    vec_rank[card.rank as usize].clear();
                }
            }

            let mut cnt = 0;

            for (i, rank) in vec_rank.iter().enumerate() {
                if cnt == 6 {
                    break;
                }

                if rank.is_empty() {
                    continue;
                }

                for suit in suits.iter() {
                    if cnt == 6 {
                        break;
                    }

                    if rank.contains(suit) {
                        continue;
                    }

                    write!(out, "{}{} ", ranks[i], suit).unwrap();
                    cnt += 1;
                }
            }

            // Random different rank cards
            for rank in ranks.iter() {
                if cnt == 6 {
                    break;
                }

                if cards_alice[0].rank == *rank
                    || cards_alice[1].rank == *rank
                    || cards_bob[0].rank == *rank
                    || cards_bob[1].rank == *rank
                {
                    continue;
                }

                cnt += 1;

                write!(out, "{}{} ", rank, suits[cnt % 4]).unwrap();
            }

            writeln!(out).unwrap();
        }

        // How to make Bob win
        if cards_alice[0].rank == Rank::Ace
            && cards_alice[1].rank == Rank::Ace
            && cards_bob[0].rank == Rank::Ace
            && cards_bob[1].rank != Rank::Ace
        {
            if cards_bob[0].suit != cards_bob[1].suit {
                if cards_bob[1].rank == Rank::Six
                    || cards_bob[1].rank == Rank::Seven
                    || cards_bob[1].rank == Rank::Eight
                    || cards_bob[1].rank == Rank::Nine
                {
                    writeln!(out, "NO").unwrap();
                } else if cards_bob[1].rank == Rank::Ten
                    || cards_bob[1].rank == Rank::Jack
                    || cards_bob[1].rank == Rank::Queen
                    || cards_bob[1].rank == Rank::King
                {
                    // Straight
                    write!(out, "YES ").unwrap();

                    let mut cnt = 0;

                    for rank in [Rank::Ten, Rank::Jack, Rank::Queen, Rank::King].iter() {
                        if cards_bob[1].rank == *rank {
                            continue;
                        }

                        write!(out, "{}{} ", rank, suits[cnt % 4]).unwrap();
                        write!(out, "{}{} ", rank, suits[(cnt + 1) % 4]).unwrap();

                        cnt += 2;
                    }

                    writeln!(out).unwrap();
                } else {
                    // Straight
                    write!(out, "YES ").unwrap();

                    let mut cnt = 0;

                    for rank in [Rank::Two, Rank::Three, Rank::Four, Rank::Five].iter() {
                        if cards_bob[1].rank == *rank {
                            continue;
                        }

                        write!(out, "{}{} ", rank, suits[cnt % 4]).unwrap();
                        write!(out, "{}{} ", rank, suits[(cnt + 1) % 4]).unwrap();

                        cnt += 2;
                    }

                    writeln!(out).unwrap();
                }
            } else {
                // Flush
                write!(out, "YES ").unwrap();

                let mut cnt = 0;

                for rank in ranks.iter() {
                    if cnt == 6 {
                        break;
                    }

                    if cards_bob[0].rank == *rank
                        || cards_bob[1].rank == *rank
                        || cards_alice[0].rank == *rank
                        || cards_alice[1].rank == *rank
                    {
                        continue;
                    }

                    cnt += 1;

                    write!(out, "{}{} ", rank, cards_bob[0].suit).unwrap();
                }

                writeln!(out).unwrap();
            }
        } else if cards_alice[0].rank == cards_alice[1].rank
            && cards_alice[1].rank == cards_bob[0].rank
            && cards_bob[0].rank == cards_bob[1].rank
        {
            writeln!(out, "NO").unwrap();
        } else if cards_alice[0].rank != cards_alice[1].rank
            && cards_bob[0].rank != cards_bob[1].rank
            && cards_alice[0].rank == cards_bob[0].rank
            && cards_alice[1].rank == cards_bob[1].rank
        {
            if cards_bob[0].suit != cards_bob[1].suit {
                writeln!(out, "NO").unwrap();
            } else {
                // Flush
                write!(out, "YES ").unwrap();

                let mut cnt = 0;

                for rank in ranks.iter() {
                    if cnt == 6 {
                        break;
                    }

                    if cards_bob[0].rank == *rank
                        || cards_bob[1].rank == *rank
                        || cards_alice[0].rank == *rank
                        || cards_alice[1].rank == *rank
                    {
                        continue;
                    }

                    cnt += 1;

                    write!(out, "{}{} ", rank, cards_bob[0].suit).unwrap();
                }

                writeln!(out).unwrap();
            }
        } else {
            write!(out, "YES ").unwrap();

            if cards_bob[0].suit == cards_bob[1].suit {
                if cards_alice[0].suit == cards_alice[1].suit
                    && cards_bob[0].suit == cards_alice[0].suit
                {
                    let mut vec_rank = vec![Vec::new(); 13];

                    for card in cards_bob.iter() {
                        vec_rank[card.rank as usize].push(card.suit.clone());
                    }

                    for card in cards_alice.iter() {
                        if vec_rank[card.rank as usize].len() == 2 {
                            vec_rank[card.rank as usize].push(card.suit.clone());
                        } else if vec_rank[card.rank as usize].len() == 1 {
                            vec_rank[card.rank as usize].clear();
                        }
                    }

                    let mut cnt = 0;

                    for (i, rank) in vec_rank.iter().enumerate() {
                        if cnt == 6 {
                            break;
                        }

                        if rank.is_empty() {
                            continue;
                        }

                        for suit in suits.iter() {
                            if cnt == 6 {
                                break;
                            }

                            if rank.contains(suit) {
                                continue;
                            }

                            write!(out, "{}{} ", ranks[i], suit).unwrap();
                            cnt += 1;
                        }
                    }

                    // Random different rank cards
                    for rank in ranks.iter() {
                        if cnt == 6 {
                            break;
                        }

                        if cards_alice[0].rank == *rank
                            || cards_alice[1].rank == *rank
                            || cards_bob[0].rank == *rank
                            || cards_bob[1].rank == *rank
                        {
                            continue;
                        }

                        cnt += 1;

                        write!(out, "{}{} ", rank, suits[cnt % 4]).unwrap();
                    }

                    writeln!(out).unwrap();
                } else {
                    // Flush
                    let mut cnt = 0;

                    for rank in ranks.iter() {
                        if cnt == 6 {
                            break;
                        }

                        if cards_bob[0].rank == *rank
                            || cards_bob[1].rank == *rank
                            || cards_alice[0].rank == *rank
                            || cards_alice[1].rank == *rank
                        {
                            continue;
                        }

                        cnt += 1;

                        write!(out, "{}{} ", rank, cards_bob[0].suit).unwrap();
                    }

                    writeln!(out).unwrap();
                }
            } else {
                let mut vec_rank = vec![Vec::new(); 13];

                for card in cards_bob.iter() {
                    vec_rank[card.rank as usize].push(card.suit.clone());
                }

                for card in cards_alice.iter() {
                    if vec_rank[card.rank as usize].len() == 2 {
                        vec_rank[card.rank as usize].push(card.suit.clone());
                    } else if vec_rank[card.rank as usize].len() == 1 {
                        vec_rank[card.rank as usize].clear();
                    }
                }

                let mut cnt = 0;

                for (i, rank) in vec_rank.iter().enumerate() {
                    if cnt == 6 {
                        break;
                    }

                    if rank.is_empty() {
                        continue;
                    }

                    for suit in suits.iter() {
                        if cnt == 6 {
                            break;
                        }

                        if rank.contains(suit) {
                            continue;
                        }

                        write!(out, "{}{} ", ranks[i], suit).unwrap();
                        cnt += 1;
                    }
                }

                // Random different rank cards
                for rank in ranks.iter() {
                    if cnt == 6 {
                        break;
                    }

                    if cards_alice[0].rank == *rank
                        || cards_alice[1].rank == *rank
                        || cards_bob[0].rank == *rank
                        || cards_bob[1].rank == *rank
                    {
                        continue;
                    }

                    cnt += 1;

                    write!(out, "{}{} ", rank, suits[cnt % 4]).unwrap();
                }

                writeln!(out).unwrap();
            }
        }

        // How to make the game in a draw
        if cards_alice[0].rank == cards_alice[1].rank
            && !(cards_bob[0].rank == cards_bob[1].rank && cards_alice[0].rank == cards_bob[0].rank)
        {
            writeln!(out, "NO").unwrap();
        } else if cards_alice[0].rank != cards_alice[1].rank
            && cards_alice[0].rank != cards_bob[0].rank
            && cards_alice[1].rank != cards_bob[0].rank
            && cards_bob[0].rank == cards_bob[1].rank
        {
            writeln!(out, "NO").unwrap();
        } else if cards_alice[0].rank != cards_alice[1].rank
            && cards_alice[0].rank != cards_bob[0].rank
            && cards_alice[0].rank != cards_bob[1].rank
            && cards_alice[1].rank != cards_bob[0].rank
            && cards_alice[1].rank != cards_bob[1].rank
            && cards_bob[0].rank != cards_bob[1].rank
        {
            if cards_bob[0].rank > cards_bob[1].rank
                && cards_bob[1].rank > cards_alice[0].rank
                && cards_alice[0].rank > cards_alice[1].rank
            {
                writeln!(out, "NO").unwrap();
            } else if cards_bob[0].rank > cards_alice[0].rank
                && cards_alice[0].rank > cards_alice[1].rank
                && cards_alice[1].rank > cards_bob[1].rank
            {
                writeln!(out, "NO").unwrap();
            } else if cards_alice[0].rank > cards_alice[1].rank
                && cards_alice[1].rank > cards_bob[0].rank
                && cards_bob[0].rank > cards_bob[1].rank
            {
                writeln!(out, "NO").unwrap();
            } else {
                write!(out, "YES ").unwrap();

                // Same rank cards (1)
                for suit in suits.iter() {
                    if cards_alice[0].suit == *suit {
                        continue;
                    }

                    write!(out, "{}{} ", cards_alice[0].rank, suit).unwrap();
                    break;
                }

                // Same rank cards (2)
                for suit in suits.iter() {
                    if cards_alice[1].suit == *suit {
                        continue;
                    }

                    write!(out, "{}{} ", cards_alice[1].rank, suit).unwrap();
                    break;
                }

                // Same rank cards (3)
                for suit in suits.iter() {
                    if cards_bob[0].suit == *suit {
                        continue;
                    }

                    write!(out, "{}{} ", cards_bob[0].rank, suit).unwrap();
                }

                // Same rank cards (4)
                for suit in suits.iter() {
                    if cards_bob[1].suit == *suit {
                        continue;
                    }

                    write!(out, "{}{} ", cards_bob[1].rank, suit).unwrap();
                    break;
                }
            }

            writeln!(out).unwrap();
        } else {
            write!(out, "YES ").unwrap();

            let mut vec_rank = vec![Vec::new(); 13];

            for card in cards_alice.iter() {
                vec_rank[card.rank as usize].push(card.suit.clone());
            }

            for card in cards_bob.iter() {
                vec_rank[card.rank as usize].push(card.suit.clone());
            }

            let mut cnt = 0;

            for (i, rank) in vec_rank.iter().enumerate() {
                if cnt == 6 {
                    break;
                }

                if rank.is_empty() || rank.len() == 4 {
                    continue;
                }

                let mut cnt_suit = 0;

                for suit in suits.iter() {
                    if cnt == 6 {
                        break;
                    }

                    if rank.len() == 1 && cnt_suit == 1 {
                        break;
                    }

                    if rank.len() % 2 == 0 || rank.contains(suit) {
                        continue;
                    }

                    write!(out, "{}{} ", ranks[i], suit).unwrap();
                    cnt += 1;
                    cnt_suit += 1;
                }
            }

            // Random two same rank cards
            for rank in ranks.iter() {
                if cnt == 6 {
                    break;
                }

                if cards_alice[0].rank == *rank
                    || cards_alice[1].rank == *rank
                    || cards_bob[0].rank == *rank
                    || cards_bob[1].rank == *rank
                {
                    continue;
                }

                write!(out, "{}{} ", rank, suits[cnt % 4]).unwrap();
                write!(out, "{}{} ", rank, suits[(cnt + 1) % 4]).unwrap();

                cnt += 2;
            }

            writeln!(out).unwrap();
        }
    }
}
