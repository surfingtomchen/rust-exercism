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

#[derive(Eq, Ord, PartialOrd, PartialEq, Clone)]
pub struct State(u8, u8);

impl State {
    pub fn next_states(&self, capacity: (u8, u8), start: Bucket, stack: &Vec<State>) -> Vec<State> {
        let sum = self.1 + self.0;
        vec![
            Self(0, self.1),
            Self(self.0, 0),
            Self(capacity.0, self.1),
            Self(self.0, capacity.1),
            if sum > capacity.1 {
                Self(sum - capacity.1, capacity.1)
            } else {
                Self(0, sum)
            },
            if sum > capacity.0 {
                Self(capacity.0, sum - capacity.0)
            } else {
                Self(sum, 0)
            },
        ]
        .into_iter()
        .filter(|x| {
            *x != State(0, 0)
                && x != self
                && (start == Bucket::One && *x != State(0, capacity.1)
                    || start == Bucket::Two && *x != State(capacity.0, 0))
                && !stack.contains(&*x)
        })
        .collect()
    }
}

pub struct DFS {
    pub capacity: (u8, u8),
    pub goal: u8,
    pub start_bucket: Bucket,
    pub stats: Option<BucketStats>,
    pub stack: Vec<State>,
}

impl DFS {
    pub fn new(capacity: (u8, u8), goal: u8, start_bucket: &Bucket) -> Self {
        let mut vec = vec![];
        if *start_bucket == Bucket::One {
            vec.push(State(capacity.0, 0));
        } else {
            vec.push(State(0, capacity.1));
        }

        Self {
            capacity,
            goal,
            start_bucket: start_bucket.clone(),
            stats: None,
            stack: vec,
        }
    }

    pub fn check_goal(&mut self, state: &State) -> bool {
        if state.0 == self.goal || state.1 == self.goal {
            if let Some(answer) = &self.stats {
                if answer.moves < self.stack.len() as u8 {
                    return true;
                }
            }
            self.stats = Some(BucketStats::new(
                self.stack.len() as u8,
                if state.0 == self.goal {
                    Bucket::One
                } else {
                    Bucket::Two
                },
                state.1 + state.0 - self.goal,
            ));
            return true;
        }

        false
    }

    pub fn search(&mut self) {
        let one_state = self.stack.last().unwrap().clone();
        if self.check_goal(&one_state) {
            return;
        }

        let states = one_state.next_states(self.capacity, self.start_bucket.clone(), &self.stack);
        for next in states {
            self.stack.push(next);
            self.search();
            self.stack.pop();
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
    let mut dfs = DFS::new((capacity_1, capacity_2), goal, start_bucket);
    dfs.search();
    dfs.stats
}
