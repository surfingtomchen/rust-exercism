pub struct PascalsTriangle {
    row_count: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle {
            row_count
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut result: Vec<Vec<u32>> = vec![];

        for _ in 0..self.row_count as usize {
            let mut current_row: Vec<u32> = vec![];
            current_row.push(1);
            match result.last() {
                None => {}
                Some(last_row) => {
                    for n in last_row.windows(2) {
                        current_row.push(n[0] + n[1]);
                    }
                    current_row.push(1)
                }
            }
            result.push(current_row)
        }
        result
    }
}
