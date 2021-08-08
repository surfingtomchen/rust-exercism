use std::collections::HashMap;

pub struct Solution {
    addend: u64,
    subtotal: u64,
    sum: Option<u64>,
    all_chars: Vec<char>,
    all_char_index: usize,
    digit_used: u64,
    answer: HashMap<char, u8>,
}

impl Solution {
    pub fn new(all_chars: Vec<char>) -> Self {
        Self {
            addend: 0,
            subtotal: 0,
            sum: None,
            all_chars,
            all_char_index: 0,
            digit_used: 0,
            answer: HashMap::new(),
        }
    }

    pub fn finished(&mut self) -> Option<bool> {
        (self.all_char_index >= self.all_chars.len()).then(|| self.sum == Some(self.subtotal))
    }

    pub fn next(&mut self, digit: u8, digit_used: u64) {
        if let Some(mut sum) = self.sum {
            sum -= self.addend;
            self.sum = Some(sum);
        } else {
            self.subtotal -= self.addend;
        }
        self.addend = 10 * self.addend + digit as u64;
        if let Some(mut sum) = self.sum {
            sum += self.addend;
            self.sum = Some(sum);
        } else {
            self.subtotal += self.addend;
        }
        self.all_char_index += 1;
        self.digit_used = digit_used;
    }

    pub fn restore(&mut self, digit_used: u64) {
        if let Some(mut sum) = self.sum {
            sum -= self.addend;
            self.addend = self.addend / 10;
            sum += self.addend;
            self.sum = Some(sum);
        } else {
            self.subtotal -= self.addend;
            self.addend = self.addend / 10;
            self.subtotal += self.addend;
        }
        self.all_char_index -= 1;
        self.digit_used = digit_used;
    }

    pub fn find_solution(&mut self) -> bool {
        if let Some(is_finished) = self.finished() {
            return is_finished;
        }

        match self.all_chars[self.all_char_index] {
            '+' | '=' => {
                let addend = self.addend;
                self.addend = 0;
                if self.all_chars[self.all_char_index] == '=' {
                    self.sum = Some(0);
                }
                self.all_char_index += 1;
                return if self.find_solution() {
                    true
                } else {
                    self.all_char_index -= 1;
                    self.addend = addend;
                    if self.sum.is_some() {
                        self.sum = None;
                    }
                    false
                };
            }
            c => {
                let used = self.digit_used;
                if let Some(value) = self.answer.get(&c) {
                    if self.addend == 0 && *value == 0 {
                        return false;
                    }

                    self.next(*value, used);
                    return if self.find_solution() {
                        true
                    } else {
                        self.restore(used);
                        false
                    };
                }

                for u in 0..=9u8 {
                    if self.addend == 0 && u == 0 {
                        continue;
                    }

                    if used & 0x01u64 << u > 0 {
                        continue;
                    } else {
                        self.next(u, self.digit_used | 0x01u64 << u);
                        self.answer.insert(c, u);
                    }
                    if self.find_solution() {
                        return true;
                    } else {
                        self.restore(used);
                        self.answer.remove(&c);
                        continue;
                    }
                }

                false
            }
        }
    }
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let all_chars = input.chars().filter(|x| *x != ' ').collect::<Vec<char>>();
    let mut solution = Solution::new(all_chars);
    solution.find_solution().then(|| solution.answer)
}
