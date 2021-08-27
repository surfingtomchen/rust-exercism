use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let result: Arc<Mutex<HashMap<char, usize>>> = Arc::new(Mutex::new(HashMap::new()));
    let mut handles = vec![];

    for lines in input.chunks(input.len() / worker_count + 1) {
        let map = Arc::clone(&result);
        let mut v = vec![];
        lines.iter().for_each(|line| v.push(line.to_lowercase()));

        handles.push(thread::spawn(move || {
            let mut worker_result: HashMap<char, usize> = HashMap::new();
            v.iter().for_each(|line| {
                line.chars().filter(|c| c.is_alphabetic()).for_each(|c| {
                    *worker_result.entry(c).or_insert(0) += 1;
                })
            });
            let mut r = map.lock().unwrap();
            worker_result
                .iter()
                .for_each(|(c, i)| *r.entry(*c).or_insert(0) += *i)
        }));
    }

    for h in handles {
        h.join().unwrap()
    }

    let hm = result.lock().unwrap().clone();
    hm
}
