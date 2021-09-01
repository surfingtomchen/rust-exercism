use std::collections::HashMap;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut result: HashMap<char, usize> = HashMap::new();
    let mut handles = vec![];

    for lines in input.chunks(input.len() / worker_count + 1) {
        let v = lines.iter().map(|&x| x.to_owned()).collect::<Vec<_>>();
        handles.push(thread::spawn(move || {
            let mut worker_result: HashMap<char, usize> = HashMap::new();
            v.iter().for_each(|line| {
                line.chars().filter(|c| c.is_alphabetic()).for_each(|c| {
                    *worker_result.entry(c.to_ascii_lowercase()).or_insert(0) += 1;
                })
            });
            worker_result
        }));
    }

    for h in handles {
        let worker_result = h.join().unwrap();
        worker_result.iter().for_each(|(char, count)| {
            *result.entry(*char).or_insert(0) += *count;
        });
    }

    result
}
