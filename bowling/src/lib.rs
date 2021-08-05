use crate::Error::{GameComplete, NotEnoughPinsLeft};
use crate::RoundStatus::{General, Incomplete, Spare, Strike};

#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(PartialEq)]
pub enum RoundStatus {
    Incomplete,
    General,
    Spare,
    Strike,
}

pub fn is_valid(pins: u16) -> Result<u16, Error> {
    if pins > 10 {
        Err(NotEnoughPinsLeft)
    } else {
        Ok(pins)
    }
}

pub struct BonusRound {
    final_status: RoundStatus,
    bonus: (Option<u16>, Option<u16>),
}

impl BonusRound {
    pub fn new(pins: u16, final_status: RoundStatus) -> Result<Self, Error> {
        is_valid(pins)?;
        if final_status == RoundStatus::Incomplete || final_status == RoundStatus::General {
            return Err(GameComplete);
        }

        Ok(Self {
            final_status,
            bonus: (Some(pins), None),
        })
    }

    pub fn add(&mut self, pins: u16) -> Result<(), Error> {
        is_valid(pins)?;

        match (&self.final_status, self.bonus) {
            (Strike, (Some(bonus_0), None)) => {
                if bonus_0 != 10 && bonus_0 + pins > 10 {
                    return Err(NotEnoughPinsLeft);
                } else {
                    self.bonus.1 = Some(pins);
                }
            }
            (Strike, (None, None)) | (Spare, (None, None)) => self.bonus.0 = Some(pins),
            (Spare, (Some(_), _)) | (Strike, (Some(_), Some(_))) => return Err(GameComplete),
            _ => return Err(GameComplete),
        }

        Ok(())
    }

    pub fn score(&self, score: u16, first_factor: u16, second_factor: u16) -> Option<u16> {
        match self.final_status {
            Strike => self.bonus.0.and_then(|bonus_0| {
                self.bonus.1.and_then(|bonus_1| {
                    Some(score + bonus_0 * (first_factor - 1) + bonus_1 * (second_factor - 1))
                })
            }),
            Spare => self
                .bonus
                .0
                .and_then(|bonus_0| Some(score + bonus_0 * (first_factor - 1))),
            _ => None,
        }
    }
}

pub struct Round(u16, Option<u16>);

impl Round {
    pub fn new(pins: u16) -> Result<Self, Error> {
        match pins {
            10 => Ok(Self(10, Some(0))),
            0..=9 => Ok(Self(pins, None)),
            _ => Err(Error::NotEnoughPinsLeft),
        }
    }

    pub fn accumulated_score(
        &self,
        last_score: Option<u16>,
        first_factor: u16,
        second_factor: u16,
    ) -> (Option<u16>, u16, u16) {
        if let Some(score) = last_score {
            match self.status() {
                Incomplete => (None, 0, 0),
                Strike => (Some(score + self.0 * first_factor), second_factor + 1, 2),
                Spare => (
                    Some(score + self.0 * first_factor + self.1.unwrap_or(0) * second_factor),
                    2,
                    1,
                ),
                General => (
                    Some(score + self.0 * first_factor + self.1.unwrap_or(0) * second_factor),
                    1,
                    1,
                ),
            }
        } else {
            (None, 0, 0)
        }
    }

    pub fn round_complete(&self) -> bool {
        self.1.is_some()
    }

    pub fn pins_10(&self) -> bool {
        self.0 + self.1.unwrap_or(0) == 10
    }

    pub fn status(&self) -> RoundStatus {
        match (self.0, self.1) {
            (_, None) => Incomplete,
            (10, Some(0)) => Strike,
            (a, Some(b)) => {
                if a + b == 10 {
                    Spare
                } else {
                    General
                }
            }
        }
    }

    pub fn add(&mut self, pins: u16) -> Result<(), Error> {
        match self.status() {
            Incomplete => {
                if self.0 + pins > 10 {
                    Err(NotEnoughPinsLeft)
                } else {
                    self.1 = Some(pins);
                    Ok(())
                }
            }
            _ => Err(NotEnoughPinsLeft),
        }
    }
}

pub struct BowlingGame {
    records: Vec<Round>,
    bonus: Option<BonusRound>,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            records: vec![],
            bonus: None,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        match self.records.len() {
            10 => {
                if let Some(last) = self.records.last_mut() {
                    match last.status() {
                        Incomplete => last.add(pins),
                        General => Err(GameComplete),
                        s => {
                            if let Some(bonus) = &mut self.bonus {
                                bonus.add(pins)
                            } else {
                                self.bonus = Some(BonusRound::new(pins, s)?);
                                Ok(())
                            }
                        }
                    }
                } else {
                    unreachable!()
                }
            }
            _ => {
                if let Some(last) = self.records.last_mut() {
                    if last.round_complete() {
                        self.records.push(Round::new(pins)?);
                        Ok(())
                    } else {
                        last.add(pins)
                    }
                } else {
                    self.records.push(Round::new(pins)?);
                    Ok(())
                }
            }
        }
    }

    pub fn score(&self) -> Option<u16> {
        match self.records.len() {
            10 => {
                let (score, first, second) = self.records.iter().fold(
                    (Some(0), 1, 1),
                    |(score, first_factor, second_factor), this_round| {
                        this_round.accumulated_score(score, first_factor, second_factor)
                    },
                );

                score.and_then(|score| {
                    if let Some(last) = self.records.last() {
                        if let Some(bonus) = &self.bonus {
                            bonus.score(score, first, second)
                        } else if last.pins_10() {
                            None
                        } else {
                            Some(score)
                        }
                    } else {
                        None
                    }
                })
            }
            _ => None,
        }
    }
}
