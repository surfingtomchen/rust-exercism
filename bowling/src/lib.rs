use crate::Error::{GameComplete, NotEnoughPinsLeft};

#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    scores: Vec<u16>,
    is_new_frame: bool,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            scores: vec![],
            is_new_frame: true,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.score().is_some() {
            return Err(GameComplete);
        }
        if pins > 10 || !self.is_new_frame && self.scores.last().unwrap() + pins > 10 {
            return Err(NotEnoughPinsLeft);
        }

        self.is_new_frame = if self.is_new_frame && pins == 10 {
            true
        } else {
            !self.is_new_frame
        };

        self.scores.push(pins);
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        let mut frame = 0;
        let mut i = 0;
        let mut score = 0;

        while frame < 10 {
            match (
                self.scores.get(i),
                self.scores.get(i + 1),
                self.scores.get(i + 2),
            ) {
                (Some(10), Some(a), Some(b)) => {
                    score += 10 + a + b;
                    i += 1;
                }
                (Some(a), Some(b), Some(c)) => {
                    score += a + b;
                    if a + b == 10 {
                        score += c;
                    }
                    i += 2;
                }
                (Some(a), Some(b), _) => {
                    if a == &10 || a + b == 10 {
                        return None;
                    }
                    score += a + b;
                    i += 2;
                }
                (_, _, _) => {
                    return None;
                }
            }
            frame += 1;
        }

        Some(score)
    }
}
