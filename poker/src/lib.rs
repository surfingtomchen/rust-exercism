use crate::Category::{
    Flush, FourOfAKind, FullHouse, HighCard, OnePair, Straight, StraightFlush, ThreeOfAKind,
    TwoPair,
};
use std::cmp::*;
use std::collections::{HashMap, HashSet};

type FiveCards = [u8; 5];

enum Category {
    HighCard(FiveCards),
    OnePair(FiveCards),
    TwoPair(FiveCards),
    ThreeOfAKind(FiveCards),
    Straight(FiveCards),
    Flush(FiveCards),
    FullHouse(FiveCards),
    FourOfAKind(FiveCards),
    StraightFlush(FiveCards),
}

impl PartialEq<Self> for Category {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (HighCard(s), HighCard(o))
            | (OnePair(s), OnePair(o))
            | (TwoPair(s), TwoPair(o))
            | (ThreeOfAKind(s), ThreeOfAKind(o))
            | (Straight(s), Straight(o))
            | (Flush(s), Flush(o))
            | (FullHouse(s), FullHouse(o))
            | (FourOfAKind(s), FourOfAKind(o))
            | (StraightFlush(s), StraightFlush(o)) => s == o,
            _ => false,
        }
    }
}

impl PartialOrd for Category {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (HighCard(s), HighCard(o))
            | (OnePair(s), OnePair(o))
            | (TwoPair(s), TwoPair(o))
            | (ThreeOfAKind(s), ThreeOfAKind(o))
            | (Straight(s), Straight(o))
            | (Flush(s), Flush(o))
            | (FullHouse(s), FullHouse(o))
            | (FourOfAKind(s), FourOfAKind(o))
            | (StraightFlush(s), StraightFlush(o)) => s.partial_cmp(o),

            (StraightFlush(_), _) => Some(Ordering::Greater),
            (_, StraightFlush(_)) => Some(Ordering::Less),
            (FourOfAKind(_), _) => Some(Ordering::Greater),
            (_, FourOfAKind(_)) => Some(Ordering::Less),
            (FullHouse(_), _) => Some(Ordering::Greater),
            (_, FullHouse(_)) => Some(Ordering::Less),
            (Flush(_), _) => Some(Ordering::Greater),
            (_, Flush(_)) => Some(Ordering::Less),
            (Straight(_), _) => Some(Ordering::Greater),
            (_, Straight(_)) => Some(Ordering::Less),
            (ThreeOfAKind(_), _) => Some(Ordering::Greater),
            (_, ThreeOfAKind(_)) => Some(Ordering::Less),
            (TwoPair(_), _) => Some(Ordering::Greater),
            (_, TwoPair(_)) => Some(Ordering::Less),
            (OnePair(_), _) => Some(Ordering::Greater),
            (_, OnePair(_)) => Some(Ordering::Less),
        }
    }
}

struct Hand<'a> {
    raw: &'a str,
    category: Category,
}

impl<'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.category.partial_cmp(&self.category)
    }
}

impl<'a> Eq for Hand<'a> {}

impl<'a> Ord for Hand<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.category.partial_cmp(&other.category).unwrap()
    }
}

impl<'a> PartialEq<Self> for Hand<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.category == other.category
    }
}

impl<'a> From<&'a str> for Hand<'a> {
    fn from(s: &'a str) -> Self {
        let mut suit_set = HashSet::new();
        let mut ranks = HashMap::new();
        let mut five_cards: FiveCards = [0u8; 5];
        for (i, (rank, suit)) in s
            .split_whitespace()
            .map(|s| (&s[..s.len() - 1], &s[s.len() - 1..]))
            .enumerate()
        {
            let r = match rank {
                "A" => 14,
                "K" => 13,
                "Q" => 12,
                "J" => 11,
                _ => rank.parse().unwrap(),
            } as u8;

            suit_set.insert(suit);
            *ranks.entry(r).or_insert(0) += 1;
            five_cards[i] = r;
        }

        five_cards.sort_by(|a, b| {
            match ranks.get(a).unwrap_or(&0).cmp(ranks.get(b).unwrap_or(&0)) {
                Ordering::Equal => b.cmp(a),
                Ordering::Greater => Ordering::Less,
                Ordering::Less => Ordering::Greater,
            }
        });

        if five_cards[0] == 14 && five_cards[1] == 5 && five_cards[4] == 2 {
            // A as 1
            five_cards = [5, 4, 3, 2, 1];
        }

        let mut sorted_ranks = ranks.values().cloned().collect::<Vec<i32>>();
        sorted_ranks.sort();

        let category = match sorted_ranks.as_slice() {
            [1, 4] => FourOfAKind(five_cards),
            [2, 3] => FullHouse(five_cards),
            [1, 1, 3] => ThreeOfAKind(five_cards),
            [1, 2, 2] => TwoPair(five_cards),
            [1, 1, 1, 2] => OnePair(five_cards),
            _ => match (suit_set.len(), five_cards[0] - five_cards[4]) {
                (1, 4) => StraightFlush(five_cards),
                (1, _) => Flush(five_cards),
                (_, 4) => Straight(five_cards),
                _ => HighCard(five_cards),
            },
        };

        Self { raw: s, category }
    }
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut hands_vec = hands
        .iter()
        .map(|s| Hand::from(s.to_owned()))
        .collect::<Vec<_>>();
    hands_vec.sort();
    hands_vec
        .iter()
        .filter(|x| **x == hands_vec[0])
        .map(|x| x.raw)
        .collect::<Vec<_>>()
}
