use once_cell::sync::Lazy;
use rand::random;
use std::collections::HashSet;
use std::sync::Mutex;

static ROBOT_NAMES: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));

pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Self {
        let mut r = Robot {
            name: String::new(),
        };
        r.reset_name();
        r
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn random_name() -> String {
        format!(
            "{}{}{:03}",
            (random::<u8>() % 26 + b'A') as char,
            (random::<u8>() % 26 + b'A') as char,
            random::<u16>() % 1000
        )
    }

    pub fn reset_name(&mut self) {
        let mut name_set = ROBOT_NAMES.lock().unwrap();
        let mut new_name = String::new();
        while new_name.is_empty() || name_set.contains(&new_name) {
            new_name = Self::random_name();
        }
        name_set.remove(&self.name as &str);
        name_set.insert(new_name.clone());
        self.name = new_name;
    }
}
