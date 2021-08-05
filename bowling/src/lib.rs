use crate::Error::{GameComplete, NotEnoughPinsLeft};
use crate::Status::{One, Spare, SpareFull, Strike, StrikeFull, StrikeNext, Two};

#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(PartialEq)]
pub enum Status {
    One,
    Two,
    Spare,
    SpareFull,
    Strike,
    StrikeNext,
    StrikeFull,
}

impl Status {
    pub fn next(&self) -> Self {
        match self {
            Two | One => Two,
            SpareFull | Spare => SpareFull,
            Strike => StrikeNext,
            StrikeFull | StrikeNext => StrikeFull,
        }
    }
}

pub struct Round {
    score: (u16, u16, u16),
    status: Status,
    is_last: bool,
}

impl Round {
    pub fn score(&self) -> Option<u16> {
        match &self.status {
            One | Strike | StrikeNext | Spare => None,
            Two => Some(self.score.0 + self.score.1),
            StrikeFull | SpareFull => Some(self.score.0 + self.score.1 + self.score.2),
        }
    }

    pub fn new(pins: u16, is_last: bool) -> Result<Self, Error> {
        match pins {
            0..=9 => Ok(Self {
                score: (pins, 0, 0),
                status: One,
                is_last,
            }),
            10 => Ok(Self {
                score: (10, 0, 0),
                status: Strike,
                is_last,
            }),
            _ => Err(NotEnoughPinsLeft),
        }
    }

    pub fn add(&mut self, pins: u16) -> Result<bool, Error> {
        if pins > 10 {
            return Err(NotEnoughPinsLeft);
        }

        match (&self.status, pins) {
            (One, p) => {
                if p + self.score.0 > 10 {
                    return Err(NotEnoughPinsLeft);
                } else {
                    self.score.1 = p;
                    if p + self.score.0 == 10 {
                        self.status = Spare;
                        return Ok(false);
                    }
                }
            }
            (Strike, p) => self.score.1 = p,
            (StrikeNext, p) => {
                if self.is_last && self.score.1 != 10 && self.score.1 + p > 10 {
                    return Err(NotEnoughPinsLeft);
                }
                self.score.2 = p;
            }
            (Spare, p) => {
                self.score.2 = p;
            }
            (StrikeFull, _) | (SpareFull, _) | (Two, _) => {
                return Ok(true);
            }
        }
        self.status = self.status.next();
        Ok(false)
    }
}

pub struct BowlingGame {
    records: Vec<Round>,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self { records: vec![] }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        match self.records.len() {
            0 => self.records.push(Round::new(pins, false)?),
            10 => {
                let result: Vec<bool> = self
                    .records
                    .iter_mut()
                    .rev()
                    .take(3)
                    .map(|r| r.add(pins))
                    .collect::<Result<Vec<bool>, Error>>()?;
                if 3 == result.iter().map(|x| if *x { 1 } else { 0 }).sum() {
                    return Err(GameComplete);
                }
            }
            n => {
                if let Some(last) = self.records.last() {
                    if last.status != One {
                        self.records
                            .iter_mut()
                            .rev()
                            .take(2)
                            .map(|r| r.add(pins))
                            .collect::<Result<Vec<bool>, Error>>()?;
                        self.records.push(Round::new(pins, n == 9)?);
                    } else {
                        self.records
                            .iter_mut()
                            .rev()
                            .skip(1)
                            .take(2)
                            .map(|r| r.add(pins))
                            .collect::<Result<Vec<bool>, Error>>()?;
                        self.records.last_mut().unwrap().add(pins)?;
                    }
                }
            }
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if self.records.len() < 10 {
            return None;
        }
        self.records.iter().fold(Some(0), |score, r| {
            r.score()
                .and_then(|r_score| score.and_then(|s| Some(s + r_score)))
        })
    }
}
