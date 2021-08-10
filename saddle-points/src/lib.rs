pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut found: Vec<(usize, usize)> = vec![];

    for (row_index, row) in input.iter().enumerate() {
        for (col_index, item) in row.iter().enumerate() {
            if row.iter().all(|x| x <= item) && (0..input.len()).into_iter().all(|x| input[x][col_index] >= *item) {
                found.push((row_index, col_index));
            }
        }
    }

    found
}
