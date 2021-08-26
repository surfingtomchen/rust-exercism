pub fn get_diamond(c: char) -> Vec<String> {
    let length = c as u8 - b'A';
    ('A'..=c)
        .chain(('A'..=c).rev().skip(1))
        .map(|ch| {
            let row = ch as u8 - b'A';
            (0..2 * length + 1)
                .map(|col| {
                    if col + row == length || col == length + row {
                        ch
                    } else {
                        ' '
                    }
                })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
}
