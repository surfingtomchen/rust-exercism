use std::collections::BTreeMap;
use std::iter::FromIterator;

pub struct ScoreCard {
    mp: u16,
    w: u16,
    d: u16,
    l: u16,
}

pub enum GameResult {
    Win,
    Draw,
    Loss,
}

impl ScoreCard {
    pub fn new() -> Self {
        Self {
            mp: 0,
            w: 0,
            d: 0,
            l: 0,
        }
    }

    pub fn add(&mut self, result: GameResult) {
        self.mp += 1;
        match result {
            GameResult::Win => self.w += 1,
            GameResult::Draw => self.d += 1,
            GameResult::Loss => self.l += 1,
        }
    }

    pub fn score(&self) -> u16 {
        self.w * 3 + self.d
    }
}

pub fn tally(match_results: &str) -> String {
    let mut teams: BTreeMap<&str, ScoreCard> = BTreeMap::new();
    match_results.split("\n").for_each(|l| {
        if !l.is_empty() {
            let v = l.split(";").collect::<Vec<&str>>();
            match v[2] {
                "win" => {
                    teams
                        .entry(v[0])
                        .or_insert(ScoreCard::new())
                        .add(GameResult::Win);
                    teams
                        .entry(v[1])
                        .or_insert(ScoreCard::new())
                        .add(GameResult::Loss);
                }
                "loss" => {
                    teams
                        .entry(v[1])
                        .or_insert(ScoreCard::new())
                        .add(GameResult::Win);
                    teams
                        .entry(v[0])
                        .or_insert(ScoreCard::new())
                        .add(GameResult::Loss);
                }
                "draw" => {
                    teams
                        .entry(v[0])
                        .or_insert(ScoreCard::new())
                        .add(GameResult::Draw);
                    teams
                        .entry(v[1])
                        .or_insert(ScoreCard::new())
                        .add(GameResult::Draw);
                }
                _ => {
                    unreachable!()
                }
            }
        }
    });
    let mut v: Vec<(&str, ScoreCard)> = Vec::from_iter(teams);
    v.sort_by(|(_, a), (_, b)| b.score().cmp(&a.score()));

    vec![String::from(
        "Team                           | MP |  W |  D |  L |  P",
    )]
        .into_iter()
        .chain(v.iter().map(|(name, result)| {
            format!(
                "{:31}|{:3} |{:3} |{:3} |{:3} |{:3}",
                name,
                result.mp,
                result.w,
                result.d,
                result.l,
                result.score()
            )
        }))
        .collect::<Vec<String>>()
        .join("\n")
}
