use std::collections::HashSet;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

impl BucketStats {
    pub fn new(moves: u8, goal_bucket: Bucket, other_bucket: u8) -> Self {
        Self {
            moves,
            goal_bucket,
            other_bucket,
        }
    }
}

pub struct Status {
    pub buckets: (u8, u8),
    pub capacity: (u8, u8),
    pub moves: u8,
    pub goal: u8,
    pub start: Bucket,
    pub answer: Option<BucketStats>,
}

impl Status {
    pub fn new(capacity: (u8, u8), goal: u8, start: &Bucket) -> Self {
        Self {
            buckets: (0, 0),
            capacity,
            moves: 0,
            goal,
            start: start.clone(),
            answer: None,
        }
    }

    pub fn next_status(&self, i: u8) -> Option<(u8, u8)> {
        let result = match i {
            0 => (self.buckets.0 != 0).then(|| (0, self.buckets.1)),
            1 => (self.buckets.1 != 0).then(|| (self.buckets.0, 0)),
            2 => (self.buckets.0 != self.capacity.0).then(|| (self.capacity.0, self.buckets.1)),
            3 => (self.buckets.1 != self.capacity.1).then(|| (self.buckets.0, self.capacity.1)),
            4 => {
                if self.buckets.0 == 0 || self.buckets.1 == self.capacity.1 {
                    return None;
                }
                let sum = self.buckets.1 + self.buckets.0;
                if sum > self.capacity.1 {
                    Some((sum - self.capacity.1, self.capacity.1))
                } else {
                    Some((0, sum))
                }
            }
            _ => {
                if self.buckets.1 == 0 || self.buckets.0 == self.capacity.0 {
                    return None;
                }
                let sum = self.buckets.1 + self.buckets.0;
                if sum > self.capacity.0 {
                    Some((self.capacity.0, sum - self.capacity.0))
                } else {
                    Some((sum, 0))
                }
            }
        };
        if self.start == Bucket::Two && (self.buckets.0, self.buckets.1) == (self.capacity.0, 0) {
            return None;
        } else if self.start == Bucket::One && (self.buckets.0, self.buckets.1) == (0, self.capacity.1) {
            return None;
        } else {
            result
        }
    }

    pub fn change_to_status(&mut self, start: &Bucket) {
        self.moves += 1;
        if start == &Bucket::One {
            self.buckets = self.next_status(2).unwrap();
        } else {
            self.buckets = self.next_status(3).unwrap();
        }
    }

    pub fn find(&mut self, stored_status: &mut HashSet<(u8, u8)>) {
        if self.buckets.0 == self.goal || self.buckets.1 == self.goal {
            let potential_answer = BucketStats::new(
                self.moves,
                if self.buckets.0 == self.goal {
                    Bucket::One
                } else {
                    Bucket::Two
                },
                self.buckets.1 + self.buckets.0 - self.goal);
            if let Some(answer) = &self.answer {
                if answer.moves < self.moves {
                    return;
                }
            }
            self.answer = Some(potential_answer);
            return;
        }

        for i in 0..6 {
            if let Some(next) = self.next_status(i) {
                if stored_status.contains(&next) {
                    continue;
                }
                let old = self.buckets;
                self.buckets = next;
                self.moves += 1;
                stored_status.insert(self.buckets);
                self.find(stored_status);
                stored_status.remove(&self.buckets);
                self.buckets = old;
                self.moves -= 1;
            }
        }
    }
}

/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    let mut stored_status: HashSet<(u8, u8)> = HashSet::new();
    let mut status = Status::new((capacity_1, capacity_2), goal, start_bucket);
    status.change_to_status(start_bucket);
    stored_status.insert((0, 0));
    if start_bucket == &Bucket::Two {
        stored_status.insert((0, capacity_2));
    } else {
        stored_status.insert((capacity_1, 0));
    }
    status.find(&mut stored_status);
    status.answer
}
