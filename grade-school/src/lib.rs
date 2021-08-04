use std::collections::{BTreeMap, BTreeSet};

pub struct Grade {
    roster: BTreeSet<String>,
}

pub struct School {
    grades: BTreeMap<u32, Grade>,
}

impl Grade {
    pub fn new() -> Self {
        Self {
            roster: BTreeSet::new()
        }
    }

    pub fn add(&mut self, student: &str) {
        self.roster.insert(student.to_owned());
    }
}

impl School {
    pub fn new() -> School {
        Self {
            grades: BTreeMap::new()
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        let grade = self.grades.entry(grade).or_insert(Grade::new());
        grade.add(student);
    }

    pub fn grades(&self) -> Vec<u32> {
        self.grades.keys().map(|x| *x).collect()
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        if let Some(grade) = self.grades.get(&grade) {
            grade.roster.iter().map(|name| name.to_owned()).collect()
        } else {
            return vec![];
        }
    }
}
