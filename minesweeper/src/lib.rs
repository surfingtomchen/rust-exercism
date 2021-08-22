pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let row = minefield.len() as isize;
    let neighbors = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    minefield
        .iter()
        .zip(0..)
        .map(|(str, i)| {
            let column = str.len() as isize;
            str.chars()
                .zip(0..)
                .map(|(char, j)| {
                    if char == '*' {
                        '*'
                    } else {
                        let count = neighbors
                            .iter()
                            .map(|(r, c)| (*r + i, *c + j))
                            .filter(|(r, c)| *r >= 0 && *c >= 0 && *r < row && *c < column)
                            .filter(|(r, c)| minefield[*r as usize].as_bytes()[*c as usize] == b'*')
                            .count() as u8;
                        if count == 0 {
                            ' '
                        } else {
                            (count + b'0') as char
                        }
                    }
                })
                .collect()
        })
        .collect()
}
