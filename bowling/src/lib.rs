use crate::Error::{GameComplete, NotEnoughPinsLeft};

#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    scores: Vec<u16>,
    is_new_round: bool,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            scores: vec![],
            is_new_round: true,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.score().is_some() {
            return Err(GameComplete);
        }
        if pins > 10 || !self.is_new_round && self.scores.last().unwrap() + pins > 10 {
            return Err(NotEnoughPinsLeft);
        }

        self.is_new_round = if self.is_new_round && pins == 10 {
            true
        } else {
            !self.is_new_round
        };

        self.scores.push(pins);
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        let mut round = 0;
        let mut i = 0;
        let mut score = 0;

        while round < 10 {
            match (
                self.scores.get(i),
                self.scores.get(i + 1),
                self.scores.get(i + 2),
            ) {
                (Some(10), Some(a), Some(b)) => {
                    score += 10 + a + b;
                    round += 1;
                    i += 1;
                }
                (Some(a), Some(b), Some(c)) => {
                    if a + b == 10 {
                        score += 10 + c;
                    } else {
                        score += a + b;
                    }
                    round += 1;
                    i += 2;
                }
                (Some(a), Some(b), _) => {
                    if a == &10 || a + b == 10 {
                        return None;
                    }
                    score += a + b;
                    round += 1;
                    i += 2;
                }
                (_, _, _) => {
                    return None;
                }
            }
        }

        Some(score)
    }
}
