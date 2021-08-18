#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

pub fn convert_numbers_in_one_line(input: &[&str], line_length: usize) -> Result<String, Error> {
    if line_length % 3 != 0 {
        return Err(Error::InvalidColumnCount(line_length));
    }

    let mut numbers: Vec<Vec<u8>> = vec![vec![]; line_length / 3];

    input
        .into_iter()
        .flat_map(|x| x.as_bytes().chunks(3).zip(0usize..).collect::<Vec<_>>())
        .for_each(|(u, i)| numbers[i].extend_from_slice(u));

    Ok(numbers
        .iter()
        .cloned()
        .map(|u| {
            let x = String::from_utf8(u).unwrap();
            match &x as &str {
                " _ | ||_|   " => '0',
                "     |  |   " => '1',
                " _  _||_    " => '2',
                " _  _| _|   " => '3',
                "   |_|  |   " => '4',
                " _ |_  _|   " => '5',
                " _ |_ |_|   " => '6',
                " _   |  |   " => '7',
                " _ |_||_|   " => '8',
                " _ |_| _|   " => '9',
                _ => '?',
            }
        })
        .collect::<String>())
}

pub fn convert(input: &str) -> Result<String, Error> {
    Ok(input
        .split("\n")
        .collect::<Vec<_>>()
        .chunks(4)
        .map(|str| {
            if str.len() != 4 {
                Err(Error::InvalidRowCount(str.len()))
            } else {
                convert_numbers_in_one_line(str, str[0].len())
            }
        })
        .collect::<Result<Vec<_>, Error>>()?
        .join(","))
}
