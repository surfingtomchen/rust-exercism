fn change(c: char) -> Option<char> {
    match c {
        '0'..='9' => Some(c),
        'a'..='z' => Some(c),
        'A'..='Z' => Some(c.to_ascii_lowercase()),
        _ => None
    }
}

pub fn encrypt(input: &str) -> String {
    let raw = input
        .chars()
        .filter_map(|x| change(x))
        .collect::<Vec<char>>();
    let len = raw.len();
    if len == 0 {
        return "".to_string();
    }

    let mut r = (len as f32).sqrt() as usize;
    let mut c = r;
    if r * c != len {
        if r * (c + 1) >= len {
            c += 1;
        } else {
            r += 1;
            c += 1;
        }
    }
    let v = raw.chunks(c).collect::<Vec<&[char]>>();
    (0..c)
        .into_iter()
        .map(|i| {
            (0..r)
                .into_iter()
                .map(|j| v[j].get(i).unwrap_or(&' '))
                .collect()
        })
        .collect::<Vec<String>>()
        .join(" ")
}
