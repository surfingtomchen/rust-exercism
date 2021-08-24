pub fn count(lines: &[&str]) -> u32 {
    let is_corner = |i, j| (lines[i] as &str).as_bytes()[j] == b'+';
    let is_edge_or_corner = |i, j| {
        let a = (lines[i] as &str).as_bytes()[j];
        a == b'|' || a == b'+'
    };

    (0..lines.len())
        .map(|row| {
            let column = lines[row].len();
            (0..column)
                .flat_map(|col| {
                    (col..col + 1).cycle().zip(col + 1..column) // all two pairs
                })
                .filter(|&(l, r)| is_corner(row, l) && is_corner(row, r))
                .map(|(l, r)| {
                    (row + 1..lines.len())
                        .take_while(|&i| is_edge_or_corner(i, l) && is_edge_or_corner(i, r))
                        .filter(|&i| is_corner(i, l) && is_corner(i, r))
                        .count()
                })
                .sum::<usize>()
        })
        .sum::<usize>() as u32
}
